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
            .map_err(|e| WebpackGraphError::ParseError(format!("Failed to parse JavaScript: {e:?}")))?;

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
        
        if source.contains("featureC") {
            eprintln!("[webpack_graph] Extracting dependencies from source (contains featureC): {source}");
        }
        if source.contains("__webpack_require__.d") {
            eprintln!("[webpack_graph] Extracting dependencies from source (contains __webpack_require__.d): {source}");
        }
        
        // Match numeric module IDs: __webpack_require__(123)
        let numeric_re = regex::Regex::new(r"__webpack_require__\((\d+)\)").unwrap();
        for cap in numeric_re.captures_iter(source) {
            if let Some(m) = cap.get(1) {
                // eprintln!("[webpack_graph] Found numeric dependency: {}", m.as_str());
                dependencies.push(m.as_str().to_string());
            }
        }
        
        // Match string module IDs: __webpack_require__("../../path/to/module.js")
        // Also handles cases with comments: __webpack_require__(/*! comment */ "path")
        let string_re = regex::Regex::new(r#"__webpack_require__\s*\(\s*(?:/\*[^*]*\*/\s*)?"([^"]+)""#).unwrap();
        for cap in string_re.captures_iter(source) {
            if let Some(m) = cap.get(1) {
                if source.contains("featureC") {
                    eprintln!("[webpack_graph] Found string dependency: {}", m.as_str());
                }
                dependencies.push(m.as_str().to_string());
            }
        }
        
        // Debug: Test the regex pattern specifically with the known string
        if source.contains("featureC") {
            eprintln!("[webpack_graph] Testing direct regex on featureC source...");
            let test_string = r#"__webpack_require__("featureC.js")"#;
            let test_matches: Vec<_> = string_re.captures_iter(test_string).collect();
            eprintln!("[webpack_graph] Direct regex test matches: {}", test_matches.len());
        }
        
        // eprintln!("[webpack_graph] Total dependencies found: {}", dependencies.len());
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
                eprintln!("[webpack_graph] Found modules section starting at position {modules_start}");
                
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
                    .unwrap_or_else(|| format!("/* Module {module_id} */"));
                
                return Some((module_id, module_source));
            }
        }
        None
    }

    /// Extract source code from a function expression in the module value
    fn extract_function_source(&self, expr: &Expr) -> Option<String> {
        // Extract the actual source code from the AST for dependency analysis
        match expr {
            Expr::Fn(func) => {
                // Extract the function body as source code
                self.extract_function_body_source(&func.function)
            }
            Expr::Arrow(arrow) => {
                // Extract arrow function body as source code  
                self.extract_arrow_body_source(arrow)
            }
            Expr::Paren(paren) => {
                // Handle parenthesized expressions
                self.extract_function_source(&paren.expr)
            }
            _ => Some("/* module content */".to_string()),
        }
    }
    
    /// Extract function body source code for dependency analysis
    fn extract_function_body_source(&self, func: &swc_core::ecma::ast::Function) -> Option<String> {
        if let Some(body) = &func.body {
            // Convert the function body to source code
            let mut source_parts = Vec::new();
            
            for stmt in &body.stmts {
                if let Some(stmt_source) = self.extract_stmt_source(stmt) {
                    source_parts.push(stmt_source);
                }
            }
            
            Some(source_parts.join("\n"))
        } else {
            Some("/* no function body */".to_string())
        }
    }
    
    /// Extract arrow function body source code
    fn extract_arrow_body_source(&self, arrow: &swc_core::ecma::ast::ArrowExpr) -> Option<String> {
        match arrow.body.as_ref() {
            swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(block) => {
                let mut source_parts = Vec::new();
                for stmt in &block.stmts {
                    if let Some(stmt_source) = self.extract_stmt_source(stmt) {
                        source_parts.push(stmt_source);
                    }
                }
                Some(source_parts.join("\n"))
            }
            swc_core::ecma::ast::BlockStmtOrExpr::Expr(expr) => {
                self.extract_expr_source(expr)
            }
        }
    }
    
    /// Extract source code from a statement
    fn extract_stmt_source(&self, stmt: &swc_core::ecma::ast::Stmt) -> Option<String> {
        match stmt {
            swc_core::ecma::ast::Stmt::Expr(expr_stmt) => {
                self.extract_expr_source(&expr_stmt.expr)
            }
            swc_core::ecma::ast::Stmt::Decl(decl) => {
                match decl {
                    swc_core::ecma::ast::Decl::Var(var_decl) => {
                        let mut parts = Vec::new();
                        for declarator in &var_decl.decls {
                            if let Some(init) = &declarator.init {
                                if let Some(init_source) = self.extract_expr_source(init) {
                                    parts.push(init_source);
                                }
                            }
                        }
                        Some(parts.join(", "))
                    }
                    _ => Some("/* other declaration */".to_string()),
                }
            }
            _ => Some("/* statement */".to_string()),
        }
    }
    
    /// Extract source code from an expression
    fn extract_expr_source(&self, expr: &swc_core::ecma::ast::Expr) -> Option<String> {
        match expr {
            swc_core::ecma::ast::Expr::Call(call) => {
                // Check if this is a webpack_require call
                if let swc_core::ecma::ast::Callee::Expr(callee) = &call.callee {
                    if let swc_core::ecma::ast::Expr::Ident(ident) = callee.as_ref() {
                        if ident.sym == "__webpack_require__" {
                            // Extract the module ID from the call
                            if let Some(first_arg) = call.args.first() {
                                if let swc_core::ecma::ast::Expr::Lit(swc_core::ecma::ast::Lit::Num(num)) = first_arg.expr.as_ref() {
                                    return Some(format!("__webpack_require__({})", num.value));
                                }
                                if let swc_core::ecma::ast::Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) = first_arg.expr.as_ref() {
                                    return Some(format!("__webpack_require__(\"{}\")", s.value));
                                }
                            }
                        }
                    }
                    // Check if this is a __webpack_require__.d() call
                    if let swc_core::ecma::ast::Expr::Member(member) = callee.as_ref() {
                        if let swc_core::ecma::ast::Expr::Ident(ident) = member.obj.as_ref() {
                            if ident.sym == "__webpack_require__" {
                                if let swc_core::ecma::ast::MemberProp::Ident(prop) = &member.prop {
                                    if prop.sym == "d" {
                                        // This is a __webpack_require__.d() call
                                        // Extract dependencies from the second argument (object literal)
                                        if call.args.len() >= 2 {
                                            let mut deps = Vec::new();
                                            if let swc_core::ecma::ast::Expr::Object(obj) = call.args[1].expr.as_ref() {
                                                for prop in &obj.props {
                                                    if let swc_core::ecma::ast::PropOrSpread::Prop(prop) = prop {
                                                        if let swc_core::ecma::ast::Prop::KeyValue(kv) = prop.as_ref() {
                                                            if let Some(dep) = self.extract_expr_source(&kv.value) {
                                                                if dep.contains("__webpack_require__") {
                                                                    deps.push(dep);
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            return Some(deps.join("; "));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                Some("/* call expression */".to_string())
            }
            swc_core::ecma::ast::Expr::Member(member) => {
                // Handle member expressions which might contain webpack_require calls
                if let Some(obj_source) = self.extract_expr_source(&member.obj) {
                    if obj_source.contains("__webpack_require__") {
                        return Some(obj_source);
                    }
                }
                Some("/* member expression */".to_string())
            }
            swc_core::ecma::ast::Expr::Assign(assign) => {
                // Handle assignment expressions
                if let Some(right_source) = self.extract_expr_source(&assign.right) {
                    if right_source.contains("__webpack_require__") {
                        return Some(right_source);
                    }
                }
                Some("/* assignment */".to_string())
            }
            swc_core::ecma::ast::Expr::Arrow(arrow) => {
                // Handle arrow functions that might contain webpack_require calls
                match arrow.body.as_ref() {
                    swc_core::ecma::ast::BlockStmtOrExpr::BlockStmt(block) => {
                        let mut deps = Vec::new();
                        for stmt in &block.stmts {
                            if let Some(stmt_source) = self.extract_stmt_source(stmt) {
                                if stmt_source.contains("__webpack_require__") {
                                    deps.push(stmt_source);
                                }
                            }
                        }
                        if !deps.is_empty() {
                            return Some(deps.join("; "));
                        }
                    }
                    swc_core::ecma::ast::BlockStmtOrExpr::Expr(expr) => {
                        if let Some(expr_source) = self.extract_expr_source(expr) {
                            if expr_source.contains("__webpack_require__") {
                                return Some(expr_source);
                            }
                        }
                    }
                }
                Some("/* arrow function */".to_string())
            }
            _ => Some("/* expression */".to_string()),
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
        if let Decl::Var(var_decl) = node {
            self.process_var_declaration(var_decl);
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
        if !call.args.is_empty() {
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

// Removed unused RequireExtractor and ExtractionContext - dead code


