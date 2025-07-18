use rustc_hash::FxHashMap;
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};

/// Extract webpack modules from different chunk formats
pub struct WebpackModuleExtractor {
    /// Modules found in the chunk
    pub modules: FxHashMap<String, String>,
    /// Whether this is a split chunk format
    pub is_split_chunk: bool,
}

impl Default for WebpackModuleExtractor {
    fn default() -> Self {
        Self::new()
    }
}

impl WebpackModuleExtractor {
    pub fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
            is_split_chunk: false,
        }
    }

    /// Extract modules from a webpack chunk
    pub fn extract_modules(&mut self, program: &Program) {
        program.visit_with(self);
    }

    /// Extract module ID from a property name
    fn extract_module_id(&self, prop_name: &PropName) -> Option<String> {
        match prop_name {
            PropName::Str(s) => Some(s.value.to_string()),
            PropName::Ident(ident) => Some(ident.sym.to_string()),
            PropName::Num(num) => Some(num.value.to_string()),
            _ => None,
        }
    }

    /// Extract module source code from a function expression
    fn extract_module_source(&self, expr: &Expr) -> Option<String> {
        // Extract the actual source code from the AST for dependency analysis
        match expr {
            Expr::Fn(func) => {
                // Extract the function body as source code
                self.extract_function_body(&func.function)
            }
            Expr::Arrow(arrow) => {
                // Extract arrow function body as source code  
                self.extract_arrow_body(arrow)
            }
            Expr::Paren(paren) => {
                // Handle parenthesized expressions
                self.extract_module_source(&paren.expr)
            }
            _ => Some("/* processed module content */".to_string()),
        }
    }
    
    /// Extract function body source code for dependency analysis
    fn extract_function_body(&self, func: &swc_core::ecma::ast::Function) -> Option<String> {
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
    fn extract_arrow_body(&self, arrow: &swc_core::ecma::ast::ArrowExpr) -> Option<String> {
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

    /// Process an object literal that contains webpack modules
    fn process_modules_object(&mut self, obj: &ObjectLit) {
        eprintln!("[WebpackModuleExtractor] Processing object with {} properties", obj.props.len());
        
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    if let Some(module_id) = self.extract_module_id(&kv.key) {
                        // Process both .js files (webpack modules) and numeric module IDs
                        if module_id.ends_with(".js") || module_id.chars().all(|c| c.is_ascii_digit()) {
                            let module_source = self.extract_module_source(&kv.value)
                                .unwrap_or_else(|| "/* empty module */".to_string());
                            
                            self.modules.insert(module_id.clone(), module_source);
                        }
                    }
                }
            }
        }
        
        eprintln!("[WebpackModuleExtractor] Extracted {} modules", self.modules.len());
    }
}

impl Visit for WebpackModuleExtractor {
    /// Visit assignment expressions to find exports.modules = {...}
    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        // Handle CommonJS format: exports.modules = {...}
        if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &node.left {
            if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                if obj_ident.sym == "exports" {
                    if let MemberProp::Ident(prop_ident) = &member.prop {
                        if prop_ident.sym == "modules" {
                            eprintln!("[WebpackModuleExtractor] Found CommonJS exports.modules assignment");
                            self.is_split_chunk = true;
                            
                            // Extract modules from the object literal
                            match node.right.as_ref() {
                                Expr::Object(obj) => {
                                    self.process_modules_object(obj);
                                }
                                Expr::Paren(paren) => {
                                    if let Expr::Object(obj) = paren.expr.as_ref() {
                                        self.process_modules_object(obj);
                                    }
                                }
                                _ => {}
                            }
                            return; // Don't visit children to avoid double processing
                        }
                    }
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }

    /// Visit call expressions to find chunk.push([[ids], {modules}])
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Handle split chunk format: (self["webpackChunk..."] = ...).push([[ids], {modules}])
        if let Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "push" {
                        // Check if this is a webpack chunk push call
                        if node.args.len() == 1 {
                            let ExprOrSpread { expr, .. } = &node.args[0];
                            if let Expr::Array(array) = expr.as_ref() {
                                // Expected format: [[chunk_ids], {modules}]
                                if array.elems.len() >= 2 {
                                    if let Some(Some(ExprOrSpread { expr: modules_expr, .. })) = array.elems.get(1) {
                                        if let Expr::Object(obj) = modules_expr.as_ref() {
                                            eprintln!("[WebpackModuleExtractor] Found split chunk .push() format");
                                            self.is_split_chunk = true;
                                            self.process_modules_object(obj);
                                            return; // Don't visit children
                                        }
                                    }
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

    /// Visit variable declarations to find __webpack_modules__ = {...}
    fn visit_var_decl(&mut self, node: &VarDecl) {
        for decl in &node.decls {
            if let Pat::Ident(ident) = &decl.name {
                if ident.sym == "__webpack_modules__" {
                    eprintln!("[WebpackModuleExtractor] Found __webpack_modules__ declaration");
                    
                    if let Some(init) = &decl.init {
                        if let Expr::Object(obj) = init.as_ref() {
                            self.process_modules_object(obj);
                        } else if let Expr::Paren(paren) = init.as_ref() {
                            if let Expr::Object(obj) = paren.expr.as_ref() {
                                self.process_modules_object(obj);
                            }
                        }
                    }
                    return; // Don't visit children
                }
            }
        }
        
        // Continue visiting children
        node.visit_children_with(self);
    }
}