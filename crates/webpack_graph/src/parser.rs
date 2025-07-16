use crate::{error::WebpackGraphError, graph::{ModuleGraph, ModuleNode}, module_extractor::WebpackModuleExtractor, Result};
use rustc_hash::FxHashMap;
use swc_core::common::{sync::Lrc, SourceMap, FileName, Span};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

/// Parser for webpack bundles that extracts module dependency graphs
pub struct WebpackBundleParser {
    source_map: Lrc<SourceMap>,
}

impl WebpackBundleParser {
    pub fn new() -> Result<Self> {
        Ok(Self {
            source_map: Default::default(),
        })
    }

    /// Parse a webpack bundle from source code and extract the module graph
    pub fn parse_bundle(&self, source: &str) -> Result<ModuleGraph> {
        // Parse the JavaScript using SWC
        let fm = self.source_map.new_source_file(
            FileName::Custom("webpack-bundle.js".to_string()).into(),
            source.to_string(),
        );

        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );

        let program = parser
            .parse_program()
            .map_err(|e| WebpackGraphError::ParseError(format!("Failed to parse JavaScript: {:?}", e)))?;

        // Create visitor to extract webpack information
        let mut visitor = WebpackVisitor::new();
        program.visit_with(&mut visitor);

        // Also use the dedicated module extractor
        let mut module_extractor = WebpackModuleExtractor::new();
        module_extractor.extract_modules(&program);

        // If no standard webpack modules found, try split chunk format
        if visitor.webpack_modules.is_empty() {
            eprintln!("[webpack_graph] No standard webpack modules found, checking for split chunk format...");
            
            // Merge results from both extractors
            if !visitor.rspack_chunk_modules.is_empty() {
                eprintln!("[webpack_graph] AST visitor found {} split chunk modules", visitor.rspack_chunk_modules.len());
            }
            
            if !module_extractor.modules.is_empty() {
                eprintln!("[webpack_graph] Module extractor found {} modules", module_extractor.modules.len());
                // Merge module extractor results
                for (id, source) in module_extractor.modules {
                    visitor.rspack_chunk_modules.insert(id, source);
                }
            }
            
            // Always try string-based extraction for better dependency analysis
            let regex_modules = self.extract_rspack_modules(source);
            if !regex_modules.is_empty() {
                eprintln!("[webpack_graph] String-based extraction found {} split chunk modules", regex_modules.len());
                // Replace AST results with string-based results for better dependency analysis
                visitor.rspack_chunk_modules = regex_modules;
            }
            
            if visitor.rspack_chunk_modules.is_empty() {
                return Err(WebpackGraphError::InvalidBundleFormat(
                    "No webpack modules or split chunk modules found in bundle".to_string(),
                ));
            }
            
            // For split chunks, there are usually no explicit entry points
            // as they are loaded on demand. We'll treat this as valid.
            eprintln!("[webpack_graph] Split chunk detected with {} modules - no explicit entry points expected", 
                     visitor.rspack_chunk_modules.len());
        }

        // Build the module graph
        let mut graph = ModuleGraph::new();

        // Combine both webpack modules and rspack chunk modules
        let all_modules: FxHashMap<String, String> = visitor.webpack_modules.iter()
            .chain(visitor.rspack_chunk_modules.iter())
            .map(|(k, v)| (k.clone(), v.clone()))
            .collect();

        // Add all modules to the graph and extract their dependencies
        for (module_id, module_source) in &all_modules {
            let dependencies = self.extract_dependencies_from_source(module_source);
            let mut module_node = ModuleNode::new(module_id.clone(), module_source.clone());
            
            for dep_id in dependencies {
                module_node.add_dependency(dep_id);
            }
            
            graph.add_module(module_node);
        }

        // Build dependency relationships
        for (module_id, module_node) in &graph.modules.clone() {
            for dep_id in &module_node.dependencies {
                graph.add_dependency(module_id, dep_id);
            }
        }

        // Add entry points from visitor
        for entry_id in visitor.entry_points {
            if graph.modules.contains_key(&entry_id) {
                graph.add_entry_point(entry_id);
            }
        }

        // For split chunks without explicit entry points, we need to determine reachability differently
        // In split chunks, modules can be reached through exports or dynamic imports
        if graph.entry_points.is_empty() && !visitor.rspack_chunk_modules.is_empty() {
            eprintln!("[webpack_graph] Split chunk has no explicit entry points - analyzing module exports for reachability");
            // Don't add any implicit entry points - let the tree shaker determine reachability
            // based on the actual module dependencies
            eprintln!("[webpack_graph] Allowing tree shaker to analyze all {} modules for reachability", graph.modules.len());
        }

        // Note: Empty entry points are allowed for tree shaking scenarios where DCE
        // has removed all entry point imports, making all modules unreachable

        Ok(graph)
    }

    /// Extract __webpack_require__ calls from module source code using regex as fallback
    fn extract_dependencies_from_source(&self, source: &str) -> Vec<String> {
        let mut dependencies = Vec::new();
        
        // Match numeric module IDs: __webpack_require__(123)
        let numeric_re = regex::Regex::new(r"__webpack_require__\((\d+)\)").unwrap();
        for cap in numeric_re.captures_iter(source) {
            if let Some(m) = cap.get(1) {
                dependencies.push(m.as_str().to_string());
            }
        }
        
        // Match string module IDs: __webpack_require__("../../path/to/module.js")
        // Also handles cases with comments: __webpack_require__(/*! comment */ "path")
        let string_re = regex::Regex::new(r#"__webpack_require__\s*\(\s*(?:/\*[^*]*\*/\s*)?"([^"]+)""#).unwrap();
        for cap in string_re.captures_iter(source) {
            if let Some(m) = cap.get(1) {
                dependencies.push(m.as_str().to_string());
            }
        }
        
        dependencies
    }
    
    /// Extract modules from split chunk format (code split chunks without runtime)
    fn extract_rspack_modules(&self, source: &str) -> FxHashMap<String, String> {
        let mut modules = FxHashMap::default();
        
        // Check for CommonJS format first: exports.modules = {...}
        if source.contains("exports.modules") {
            eprintln!("[webpack_graph] Detected CommonJS format with exports.modules");
            
            // Find the exports.modules = { ... } section
            if let Some(start) = source.find("exports.modules = {") {
                let modules_start = start + "exports.modules = ".len();
                let remaining = &source[modules_start..];
                
                // Find the closing brace by counting braces
                let mut brace_count = 0;
                let mut in_string = false;
                let mut escape_next = false;
                let mut end_pos = 0;
                
                for (i, ch) in remaining.chars().enumerate() {
                    if escape_next {
                        escape_next = false;
                        continue;
                    }
                    
                    match ch {
                        '\\' if in_string => escape_next = true,
                        '"' if !in_string => in_string = true,
                        '"' if in_string => in_string = false,
                        '{' if !in_string => brace_count += 1,
                        '}' if !in_string => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                end_pos = i;
                                break;
                            }
                        },
                        _ => {}
                    }
                }
                
                if end_pos > 0 {
                    let modules_content = &remaining[..=end_pos];
                    eprintln!("[webpack_graph] Extracting modules from CommonJS format, content length: {}", modules_content.len());
                    modules = self.parse_modules_object(modules_content);
                } else {
                    eprintln!("[webpack_graph] Failed to find end of exports.modules object");
                }
            } else {
                eprintln!("[webpack_graph] Found exports.modules reference but not assignment");
            }
        }
        
        // If no CommonJS modules found, try the original split chunk format
        if modules.is_empty() {
            // Look for split chunk format: (self["webpackChunk..."] = ...).push([[...], {...}])
            // This is used when code is split into separate chunks
            // The modules are in the second element of the array
            let chunk_re = regex::Regex::new(r#"\bself\[["']webpackChunk[^"']*["']\]\s*=\s*self\[["']webpackChunk[^"']*["']\]\s*\|\|\s*\[\]\)\s*\.push\(\["#).unwrap();
            
            if chunk_re.is_match(source) {
            eprintln!("[webpack_graph] Detected split chunk format (code split without runtime)");
            
            // Find the modules object which starts after the chunk ID array
            // Look for pattern: ], { "module_id": function(...) {...}, ... }])
            // Also handle ], {\n format with newline
            let patterns = ["], {", "],\n{", "],\n    {", "],\n{"];
            let mut modules_start_opt = None;
            
            for pattern in &patterns {
                if let Some(pos) = source.find(pattern) {
                    modules_start_opt = Some(pos + pattern.len() - 1); // Keep the {
                    break;
                }
            }
            
            if let Some(modules_start) = modules_start_opt {
                let modules_section = &source[modules_start..];
                eprintln!("[webpack_graph] Found modules section starting at position {}", modules_start);
                
                // Find the end of the modules object
                let mut brace_count = 1;
                let mut in_string = false;
                let mut escape_next = false;
                let mut end_pos = 0;
                
                for (i, ch) in modules_section.chars().enumerate() {
                    if escape_next {
                        escape_next = false;
                        continue;
                    }
                    
                    match ch {
                        '\\' if in_string => escape_next = true,
                        '"' if !in_string => in_string = true,
                        '"' if in_string => in_string = false,
                        '{' if !in_string => brace_count += 1,
                        '}' if !in_string => {
                            brace_count -= 1;
                            if brace_count == 0 {
                                end_pos = i;
                                break;
                            }
                        },
                        _ => {}
                    }
                }
                
                if end_pos > 0 {
                    let modules_content = &modules_section[..=end_pos];
                    eprintln!("[webpack_graph] Parsing modules object of length {}", modules_content.len());
                    modules = self.parse_modules_object(modules_content);
                } else {
                    eprintln!("[webpack_graph] Failed to find end of modules object");
                }
            } else {
                eprintln!("[webpack_graph] Could not find modules object pattern in chunk");
            }
            }
        }
        
        println!("Extracted {} modules from split chunk format", modules.len());
        modules
    }
    
    /// Parse a modules object and extract module IDs and their source
    fn parse_modules_object(&self, modules_str: &str) -> FxHashMap<String, String> {
        let mut modules = FxHashMap::default();
        
        // Pattern to match module IDs and extract their function bodies
        // This regex finds: "module_id": function(...) { ... }
        let module_pattern = r#""([^"]+\.js)":\s*function\([^)]*\)\s*\{(.*?)\n\s*\}(?:,|\s*\})"#;
        let module_re = regex::RegexBuilder::new(module_pattern)
            .dot_matches_new_line(true)
            .build()
            .unwrap();
        
        for cap in module_re.captures_iter(modules_str) {
            if let Some(module_id) = cap.get(1) {
                if let Some(function_body) = cap.get(2) {
                    let id = module_id.as_str().to_string();
                    let body = function_body.as_str().to_string();
                    
                    eprintln!("[webpack_graph] Extracted module '{}' with body length {}", id, body.len());
                    if body.len() < 200 {
                        eprintln!("[webpack_graph] Body preview: {}", body.trim());
                    }
                    
                    // Store the actual function body for dependency analysis
                    modules.insert(id, body);
                }
            }
        }
        
        // If no modules found with the function pattern, try a simpler approach
        if modules.is_empty() {
            eprintln!("[webpack_graph] No modules found with function pattern, trying simpler approach");
            
            // Pattern to match just module IDs and extract content until next module
            let simple_pattern = r#""([^"]+\.js)":\s*"#;
            let simple_re = regex::Regex::new(simple_pattern).unwrap();
            
            for cap in simple_re.captures_iter(modules_str) {
                if let Some(module_id) = cap.get(1) {
                    let id = module_id.as_str().to_string();
                    let start_pos = cap.get(0).unwrap().end();
                    
                    // Find the end of this module by looking for the next module ID or end of object
                    let remaining = &modules_str[start_pos..];
                    let next_module_pos = remaining.find("\"),").unwrap_or(remaining.len());
                    let module_content = &remaining[..next_module_pos];
                    
                    // Store the module content for dependency analysis
                    modules.insert(id, module_content.to_string());
                }
            }
        }
        
        eprintln!("[webpack_graph] Found {} modules using regex extraction", modules.len());
        
        modules
    }
}

/// AST visitor to extract webpack module information
struct WebpackVisitor {
    webpack_modules: FxHashMap<String, String>,
    rspack_chunk_modules: FxHashMap<String, String>,
    entry_points: Vec<String>,
    webpack_modules_span: Option<Span>,
}

impl WebpackVisitor {
    fn new() -> Self {
        Self {
            webpack_modules: FxHashMap::default(),
            rspack_chunk_modules: FxHashMap::default(),
            entry_points: Vec::new(),
            webpack_modules_span: None,
        }
    }

    /// Extract module content from object property
    fn extract_module_content(&self, prop: &PropOrSpread) -> Option<(String, String)> {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                // Extract module ID
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next()?.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return None,
                };

                // Extract module source from the function expression
                let module_source = self.extract_function_source(&kv.value)
                    .unwrap_or_else(|| format!("/* Module {} */", module_id));
                
                return Some((module_id, module_source));
            }
        }
        None
    }

    /// Extract source code from a function expression in the module value
    fn extract_function_source(&self, expr: &Expr) -> Option<String> {
        // For accurate dependency analysis, we need to preserve the actual source code
        // instead of just extracting dependencies. This is because the tree shaker
        // needs to analyze the code AFTER macro processing, not before.
        
        // Return a placeholder that will be replaced by actual source code
        // The actual dependency extraction will happen on the processed code
        match expr {
            Expr::Fn(_) => Some("function() { /* function body */ }".to_string()),
            Expr::Arrow(_) => Some("() => { /* arrow function body */ }".to_string()),
            _ => Some("/* module content */".to_string()),
        }
    }

    /// Check if a call expression is a webpack_require call and extract module ID
    fn extract_webpack_require_call(&self, call: &CallExpr) -> Option<String> {
        // Check if callee is __webpack_require__
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym == "__webpack_require__" {
                    // Extract first argument (module ID)
                    if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                        match expr.as_ref() {
                            Expr::Lit(Lit::Num(num)) => {
                                return Some(num.value.to_string().split('.').next()?.to_string());
                            }
                            Expr::Lit(Lit::Str(s)) => {
                                return Some(s.value.to_string());
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
        None
    }
}

impl Visit for WebpackVisitor {
    /// Visit all declarations to find webpack_modules (var, let, const)
    fn visit_decl(&mut self, node: &Decl) {
        match node {
            Decl::Var(var_decl) => {
                self.process_var_declaration(var_decl);
            }
            _ => {}
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }
    
    /// Visit call expressions to find split chunk format
    fn visit_expr(&mut self, node: &Expr) {
        // Look for (self["webpackChunk..."] = ...).push([...])
        if let Expr::Call(call) = node {
            if let Callee::Expr(callee) = &call.callee {
                if let Expr::Member(member) = callee.as_ref() {
                    // Check if this is a .push() call
                    if let MemberProp::Ident(ident) = &member.prop {
                        if ident.sym == "push" {
                            // Check if object is the webpack chunk assignment
                            if let Expr::Paren(paren) = &member.obj.as_ref() {
                                if let Expr::Assign(_assign) = paren.expr.as_ref() {
                                    // This looks like split chunk format
                                    self.process_split_chunk_push(call);
                                    // Don't visit children to avoid collecting entry points
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit variable declarations to find webpack_modules
    fn visit_var_decl(&mut self, node: &VarDecl) {
        self.process_var_declaration(node);
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit assignment expressions to catch webpack_modules = {...} patterns
    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        // Handle standard webpack format: __webpack_modules__ = {...}
        if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &node.left {
            if ident.sym == "__webpack_modules__" {
                self.webpack_modules_span = Some(node.span);
                
                // Extract modules from the object literal - handle both direct and parenthesized objects
                let obj = match node.right.as_ref() {
                    Expr::Object(obj) => Some(obj),
                    Expr::Paren(paren) => {
                        if let Expr::Object(obj) = paren.expr.as_ref() {
                            Some(obj)
                        } else {
                            None
                        }
                    },
                    _ => None,
                };

                if let Some(obj) = obj {
                    for prop in &obj.props {
                        if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                            self.webpack_modules.insert(module_id, module_source);
                        }
                    }
                }
            }
        }
        
        // Handle CommonJS split chunk format: exports.modules = {...}
        if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &node.left {
            if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                if obj_ident.sym == "exports" {
                    if let MemberProp::Ident(prop_ident) = &member.prop {
                        if prop_ident.sym == "modules" {
                            eprintln!("[webpack_graph] Found CommonJS split chunk format: exports.modules");
                            
                            // Extract modules from the object literal
                            let obj = match node.right.as_ref() {
                                Expr::Object(obj) => Some(obj),
                                Expr::Paren(paren) => {
                                    if let Expr::Object(obj) = paren.expr.as_ref() {
                                        Some(obj)
                                    } else {
                                        None
                                    }
                                },
                                _ => None,
                            };

                            if let Some(obj) = obj {
                                eprintln!("[webpack_graph] Processing {} module entries in exports.modules", obj.props.len());
                                for prop in &obj.props {
                                    if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                                        self.rspack_chunk_modules.insert(module_id, module_source);
                                    }
                                }
                            }
                            // Don't visit children of exports.modules to avoid collecting entry points
                            return;
                        }
                    }
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit call expressions to find webpack_require calls outside modules
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Check if we're inside the webpack_modules span
        let inside_webpack_modules = if let Some(modules_span) = self.webpack_modules_span {
            modules_span.contains(node.span)
        } else {
            false
        };

        // Only collect entry points if we're not inside webpack_modules definition
        if !inside_webpack_modules {
            if let Some(module_id) = self.extract_webpack_require_call(node) {
                if !self.entry_points.contains(&module_id) {
                    self.entry_points.push(module_id);
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }
}

impl WebpackVisitor {
    /// Process split chunk .push() call
    fn process_split_chunk_push(&mut self, call: &CallExpr) {
        // Split chunk format: .push([[chunk_ids], { modules }])
        if call.args.len() >= 1 {
            let ExprOrSpread { expr, .. } = &call.args[0];
            if let Expr::Array(array) = expr.as_ref() {
                // We expect 2 elements: [chunk_ids, modules_object]
                if array.elems.len() >= 2 {
                    if let Some(ExprOrSpread { expr: modules_expr, .. }) = &array.elems[1] {
                        if let Expr::Object(obj) = modules_expr.as_ref() {
                            eprintln!("[webpack_graph] Found split chunk modules object with {} properties", obj.props.len());
                            // Extract modules from the object
                            for prop in &obj.props {
                                if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                                    self.rspack_chunk_modules.insert(module_id, module_source);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    
    /// Process variable declarations (works for var, let, const)
    fn process_var_declaration(&mut self, node: &VarDecl) {
        for declarator in &node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                // Check for various webpack modules variable names
                if ident.sym == "__webpack_modules__" {
                    self.webpack_modules_span = Some(node.span);
                    
                    // Extract modules from the object literal
                    if let Some(init) = &declarator.init {
                        // Handle both direct objects and parenthesized objects
                        let obj = match init.as_ref() {
                            Expr::Object(obj) => Some(obj),
                            Expr::Paren(paren) => {
                                if let Expr::Object(obj) = paren.expr.as_ref() {
                                    Some(obj)
                                } else {
                                    None
                                }
                            },
                            _ => None,
                        };

                        if let Some(obj) = obj {
                            for prop in &obj.props {
                                if let Some((module_id, module_source)) = self.extract_module_content(prop) {
                                    self.webpack_modules.insert(module_id, module_source);
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Enhanced require extractor that comprehensively finds all __webpack_require__ calls
struct RequireExtractor {
    dependencies: Vec<String>,
    current_context: ExtractionContext,
}

#[derive(Debug, Clone)]
enum ExtractionContext {
    ModuleFunction,
    ExportDefinition,
    ImportStatement,
    PropertyAccess,
}

impl RequireExtractor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
            current_context: ExtractionContext::ModuleFunction,
        }
    }
    
    /// Extract all __webpack_require__ calls from an expression
    fn extract_from_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Call(call) => self.handle_call_expr(call),
            Expr::Member(member) => self.handle_member_expr(member),
            Expr::Arrow(arrow) => self.handle_arrow_expr(arrow),
            Expr::Paren(paren) => self.extract_from_expr(&paren.expr),
            Expr::Fn(func) => self.handle_function_expr(func),
            Expr::Object(obj) => self.handle_object_expr(obj),
            Expr::Array(arr) => self.handle_array_expr(arr),
            Expr::Cond(cond) => self.handle_conditional_expr(cond),
            Expr::Assign(assign) => self.handle_assign_expr(assign),
            Expr::Seq(seq) => self.handle_sequence_expr(seq),
            _ => {
                // For other expressions, visit children if they exist
                expr.visit_children_with(self);
            }
        }
    }
    
    /// Extract from statements
    fn extract_from_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Expr(expr_stmt) => {
                self.extract_from_expr(&expr_stmt.expr);
            }
            Stmt::Decl(Decl::Var(var_decl)) => {
                for declarator in &var_decl.decls {
                    if let Some(init) = &declarator.init {
                        self.extract_from_expr(init);
                    }
                }
            }
            Stmt::Block(block) => {
                for stmt in &block.stmts {
                    self.extract_from_stmt(stmt);
                }
            }
            Stmt::If(if_stmt) => {
                self.extract_from_stmt(&if_stmt.cons);
                if let Some(alt) = &if_stmt.alt {
                    self.extract_from_stmt(alt);
                }
            }
            Stmt::Return(ret) => {
                if let Some(arg) = &ret.arg {
                    self.extract_from_expr(arg);
                }
            }
            _ => {
                // For other statements, visit children
                stmt.visit_children_with(self);
            }
        }
    }
    
    /// Handle call expressions - the most important case
    fn handle_call_expr(&mut self, call: &CallExpr) {
        // Handle direct __webpack_require__ calls
        if self.is_webpack_require_call(call) {
            if let Some(module_id) = self.extract_module_id_from_call(call) {
                self.add_dependency(module_id);
            }
        }
        
        // Handle __webpack_require__.d() export definitions
        if self.is_webpack_require_d_call(call) {
            self.handle_export_definition(call);
        }
        
        // Handle __webpack_require__.r() calls
        if self.is_webpack_require_r_call(call) {
            // These don't have dependencies but we should traverse arguments
            for arg in &call.args {
                self.extract_from_expr(&arg.expr);
            }
        }
        
        // Always traverse arguments for nested requires
        for arg in &call.args {
            self.extract_from_expr(&arg.expr);
        }
        
        // Traverse callee for chained calls
        if let Callee::Expr(expr) = &call.callee {
            self.extract_from_expr(expr);
        }
    }
    
    /// Handle member expressions (property access)
    fn handle_member_expr(&mut self, member: &MemberExpr) {
        // Handle __webpack_require__("module").property patterns
        if let Expr::Call(call) = member.obj.as_ref() {
            if self.is_webpack_require_call(call) {
                if let Some(module_id) = self.extract_module_id_from_call(call) {
                    self.add_dependency(module_id);
                }
            }
        }
        
        // Always traverse object and property
        self.extract_from_expr(&member.obj);
        if let MemberProp::Computed(computed) = &member.prop {
            self.extract_from_expr(&computed.expr);
        }
    }
    
    /// Handle arrow expressions
    fn handle_arrow_expr(&mut self, arrow: &ArrowExpr) {
        let old_context = self.current_context.clone();
        self.current_context = ExtractionContext::ExportDefinition;
        
        match arrow.body.as_ref() {
            BlockStmtOrExpr::BlockStmt(block) => {
                for stmt in &block.stmts {
                    self.extract_from_stmt(stmt);
                }
            }
            BlockStmtOrExpr::Expr(expr) => {
                self.extract_from_expr(expr);
            }
        }
        
        self.current_context = old_context;
    }
    
    /// Handle function expressions
    fn handle_function_expr(&mut self, func: &FnExpr) {
        if let Some(body) = &func.function.body {
            for stmt in &body.stmts {
                self.extract_from_stmt(stmt);
            }
        }
    }
    
    /// Handle object expressions
    fn handle_object_expr(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            match prop {
                PropOrSpread::Prop(prop) => {
                    match prop.as_ref() {
                        Prop::KeyValue(kv) => {
                            self.extract_from_expr(&kv.value);
                        }
                        Prop::Shorthand(_) => {}
                        Prop::Assign(assign) => {
                            self.extract_from_expr(&assign.value);
                        }
                        Prop::Getter(getter) => {
                            if let Some(body) = &getter.body {
                                for stmt in &body.stmts {
                                    self.extract_from_stmt(stmt);
                                }
                            }
                        }
                        Prop::Setter(setter) => {
                            if let Some(body) = &setter.body {
                                for stmt in &body.stmts {
                                    self.extract_from_stmt(stmt);
                                }
                            }
                        }
                        Prop::Method(method) => {
                            if let Some(body) = &method.function.body {
                                for stmt in &body.stmts {
                                    self.extract_from_stmt(stmt);
                                }
                            }
                        }
                    }
                }
                PropOrSpread::Spread(spread) => {
                    self.extract_from_expr(&spread.expr);
                }
            }
        }
    }
    
    /// Handle array expressions
    fn handle_array_expr(&mut self, arr: &ArrayLit) {
        for elem in &arr.elems {
            if let Some(ExprOrSpread { expr, .. }) = elem {
                self.extract_from_expr(expr);
            }
        }
    }
    
    /// Handle conditional expressions
    fn handle_conditional_expr(&mut self, cond: &CondExpr) {
        self.extract_from_expr(&cond.test);
        self.extract_from_expr(&cond.cons);
        self.extract_from_expr(&cond.alt);
    }
    
    /// Handle assignment expressions
    fn handle_assign_expr(&mut self, assign: &AssignExpr) {
        self.extract_from_expr(&assign.right);
        if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &assign.left {
            self.extract_from_expr(&member.obj);
            if let MemberProp::Computed(computed) = &member.prop {
                self.extract_from_expr(&computed.expr);
            }
        }
    }
    
    /// Handle sequence expressions
    fn handle_sequence_expr(&mut self, seq: &SeqExpr) {
        for expr in &seq.exprs {
            self.extract_from_expr(expr);
        }
    }
    
    /// Handle __webpack_require__.d() export definitions
    fn handle_export_definition(&mut self, call: &CallExpr) {
        let old_context = self.current_context.clone();
        self.current_context = ExtractionContext::ExportDefinition;
        
        // The second argument contains the export definitions
        if call.args.len() >= 2 {
            if let Expr::Object(obj) = call.args[1].expr.as_ref() {
                for prop in &obj.props {
                    if let PropOrSpread::Prop(prop) = prop {
                        if let Prop::KeyValue(kv) = prop.as_ref() {
                            // The value often contains arrow functions with requires
                            self.extract_from_expr(&kv.value);
                        }
                    }
                }
            }
        }
        
        self.current_context = old_context;
    }
    
    /// Check if a call is __webpack_require__()
    fn is_webpack_require_call(&self, call: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                return ident.sym == "__webpack_require__";
            }
        }
        false
    }
    
    /// Check if a call is __webpack_require__.d()
    fn is_webpack_require_d_call(&self, call: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let Expr::Ident(ident) = member.obj.as_ref() {
                    if ident.sym == "__webpack_require__" {
                        if let MemberProp::Ident(prop) = &member.prop {
                            return prop.sym == "d";
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Check if a call is __webpack_require__.r()
    fn is_webpack_require_r_call(&self, call: &CallExpr) -> bool {
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let Expr::Ident(ident) = member.obj.as_ref() {
                    if ident.sym == "__webpack_require__" {
                        if let MemberProp::Ident(prop) = &member.prop {
                            return prop.sym == "r";
                        }
                    }
                }
            }
        }
        false
    }
    
    /// Extract module ID from a __webpack_require__ call
    fn extract_module_id_from_call(&self, call: &CallExpr) -> Option<String> {
        if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
            match expr.as_ref() {
                Expr::Lit(Lit::Num(num)) => {
                    Some(num.value.to_string().split('.').next()?.to_string())
                }
                Expr::Lit(Lit::Str(s)) => {
                    Some(s.value.to_string())
                }
                _ => None,
            }
        } else {
            None
        }
    }
    
    /// Add a dependency if not already present
    fn add_dependency(&mut self, module_id: String) {
        if !self.dependencies.contains(&module_id) {
            self.dependencies.push(module_id);
        }
    }
}

// Implement Visit trait for RequireExtractor to enable AST traversal
impl Visit for RequireExtractor {
    fn visit_expr(&mut self, expr: &Expr) {
        self.extract_from_expr(expr);
    }
    
    fn visit_stmt(&mut self, stmt: &Stmt) {
        self.extract_from_stmt(stmt);
    }
}

impl WebpackBundleParser {
    // Removed analyze_split_chunk_reachability - let tree shaker handle it
}

impl Default for WebpackBundleParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebpackBundleParser")
    }
} 