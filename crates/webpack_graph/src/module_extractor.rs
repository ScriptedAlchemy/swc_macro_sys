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
        // For now, return a placeholder since we need the actual processed source
        // The dependency extraction will be done on the actual string representation
        // of the processed code, not on the original AST
        match expr {
            Expr::Fn(_) => Some("function() { /* processed function body */ }".to_string()),
            Expr::Arrow(_) => Some("() => { /* processed arrow function body */ }".to_string()),
            _ => Some("/* processed module content */".to_string()),
        }
    }

    /// Process an object literal that contains webpack modules
    fn process_modules_object(&mut self, obj: &ObjectLit) {
        eprintln!("[WebpackModuleExtractor] Processing object with {} properties", obj.props.len());
        
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    if let Some(module_id) = self.extract_module_id(&kv.key) {
                        // Only process .js files (webpack modules)
                        if module_id.ends_with(".js") {
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
                            if let ExprOrSpread { expr, .. } = &node.args[0] {
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