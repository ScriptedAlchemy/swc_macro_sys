use crate::{error::WebpackGraphError, graph::{ModuleGraph, ModuleNode}, Result};
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

        // If no standard webpack modules found, try split chunk format
        if visitor.webpack_modules.is_empty() {
            eprintln!("[webpack_graph] No standard webpack modules found, checking for split chunk format...");
            
            // Also check AST visitor results
            if !visitor.rspack_chunk_modules.is_empty() {
                eprintln!("[webpack_graph] AST visitor found {} split chunk modules", visitor.rspack_chunk_modules.len());
            }
            
            // Try regex extraction as fallback
            let regex_modules = self.extract_rspack_modules(source);
            if !regex_modules.is_empty() {
                eprintln!("[webpack_graph] Regex extraction found {} split chunk modules", regex_modules.len());
                // Merge both sources
                for (id, source) in regex_modules {
                    visitor.rspack_chunk_modules.insert(id, source);
                }
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
        
        println!("Extracted {} modules from split chunk format", modules.len());
        modules
    }
    
    /// Parse a modules object and extract module IDs and their source
    fn parse_modules_object(&self, modules_str: &str) -> FxHashMap<String, String> {
        let mut modules = FxHashMap::default();
        
        // Use AST visitor to properly parse the modules instead of regex
        // For now, use regex as fallback but this should be improved
        
        // Match module entries: "module_id": function(...) { ... }
        // Also handles entries with comments: "module_id": /*! comment */ function(...) { ... }
        let module_pattern = r#""([^"]+)":\s*(?:/\*[!*][^*]*\*+(?:[^/*][^*]*\*+)*/\s*)?(?:\()?function\s*\([^)]*\)\s*\{"#;
        let module_re = regex::Regex::new(module_pattern).unwrap();
        
        let _last_end = 0;
        for cap in module_re.find_iter(modules_str) {
            // Extract module ID from the capture
            if let Some(id_match) = regex::Regex::new(r#""([^"]+)""#).unwrap().captures(cap.as_str()) {
                if let Some(module_id) = id_match.get(1) {
                    let id = module_id.as_str().to_string();
                    
                    // Find the function body by counting braces
                    let start = cap.end();
                    let mut brace_count = 1;
                    let mut in_string = false;
                    let mut escape_next = false;
                    let mut body_end = start;
                    
                    for (i, ch) in modules_str[start..].chars().enumerate() {
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
                                    body_end = start + i;
                                    break;
                                }
                            },
                            _ => {}
                        }
                    }
                    
                    if body_end > start {
                        let body = modules_str[start..body_end].to_string();
                        modules.insert(id, body);
                    }
                }
            }
        }
        
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
        // Instead of trying to convert back to source, let's extract the webpack_require calls directly
        let mut dependencies = Vec::new();
        self.extract_require_calls_from_expr(expr, &mut dependencies);
        
        // Return a representation that includes the dependencies for our regex fallback
        let deps_string = dependencies.iter()
            .map(|dep| {
                // Check if dep is numeric or string
                if dep.chars().all(|c| c.is_numeric() || c == '.') {
                    format!("__webpack_require__({})", dep)
                } else {
                    format!(r#"__webpack_require__("{}")"#, dep)
                }
            })
            .collect::<Vec<_>>()
            .join("; ");
        
        Some(format!("function() {{ {} }}", deps_string))
    }

    /// Recursively extract webpack_require calls from an expression
    fn extract_require_calls_from_expr(&self, expr: &Expr, dependencies: &mut Vec<String>) {
        match expr {
            Expr::Paren(paren) => {
                self.extract_require_calls_from_expr(&paren.expr, dependencies);
            }
            Expr::Fn(func) => {
                if let Some(body) = &func.function.body {
                    for stmt in &body.stmts {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
            }
            Expr::Call(call) => {
                if let Some(module_id) = self.extract_webpack_require_call(call) {
                    if !dependencies.contains(&module_id) {
                        dependencies.push(module_id);
                    }
                }
                // Also check arguments for nested calls
                for arg in &call.args {
                    self.extract_require_calls_from_expr(&arg.expr, dependencies);
                }
            }
            _ => {}
        }
    }

    /// Extract webpack_require calls from a statement
    fn extract_require_calls_from_stmt(&self, stmt: &Stmt, dependencies: &mut Vec<String>) {
        match stmt {
            Stmt::Expr(expr_stmt) => {
                self.extract_require_calls_from_expr(&expr_stmt.expr, dependencies);
            }
            Stmt::Decl(Decl::Var(var_decl)) => {
                for declarator in &var_decl.decls {
                    if let Some(init) = &declarator.init {
                        self.extract_require_calls_from_expr(init, dependencies);
                    }
                }
            }
            _ => {}
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

impl WebpackBundleParser {
    // Removed analyze_split_chunk_reachability - let tree shaker handle it
}

impl Default for WebpackBundleParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebpackBundleParser")
    }
} 