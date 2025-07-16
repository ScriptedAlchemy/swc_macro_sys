use swc_core::common::{sync::Lrc, SourceMap, FileName};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};
use swc_core::ecma::ast::*;
use swc_core::ecma::visit::{Visit, VisitWith};
use rustc_hash::FxHashMap;

use crate::{
    chunk::{ChunkType, WebpackChunk},
    module::{ModuleId, WebpackModule},
    Result,
};

/// Main analyzer for webpack chunks
pub struct WebpackAnalyzer {
    source_map: Lrc<SourceMap>,
}

impl WebpackAnalyzer {
    /// Create a new webpack analyzer
    pub fn new() -> Self {
        Self {
            source_map: Default::default(),
        }
    }

    /// Analyze a webpack chunk from source code
    pub fn analyze_chunk(&self, source: &str) -> Result<WebpackChunk> {
        // Step 1: Detect chunk type
        let chunk_type = self.detect_chunk_type(source)?;
        println!("[WebpackAnalyzer] Detected chunk type: {:?}", chunk_type);

        // Step 2: Parse the source code
        let program = self.parse_source(source)?;

        // Step 3: Extract modules based on chunk type
        let mut chunk = WebpackChunk::new(chunk_type.clone(), source.to_string());
        self.extract_modules(&program, &mut chunk)?;

        // Step 4: Build dependency graph
        self.build_dependency_graph(&mut chunk)?;

        println!("[WebpackAnalyzer] Analyzed chunk with {} modules", chunk.module_count());
        Ok(chunk)
    }

    /// Detect the type of webpack chunk
    pub fn detect_chunk_type(&self, source: &str) -> Result<ChunkType> {
        if source.contains("exports.modules") {
            Ok(ChunkType::CommonJS)
        } else if source.contains("webpackChunk") && source.contains(".push(") {
            Ok(ChunkType::JSONP)
        } else if source.contains("__webpack_modules__") {
            Ok(ChunkType::WebpackModules)
        } else {
            Err("Unknown chunk type - not a recognized webpack chunk format".into())
        }
    }

    /// Parse source code into AST
    fn parse_source(&self, source: &str) -> Result<Program> {
        let fm = self.source_map.new_source_file(FileName::Custom("chunk.js".to_string()).into(), source.to_string());
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );
        
        parser.parse_program()
            .map_err(|e| format!("Failed to parse source: {:?}", e).into())
    }

    /// Extract modules from the AST based on chunk type
    fn extract_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        match chunk.chunk_type {
            ChunkType::CommonJS => self.extract_commonjs_modules(program, chunk),
            ChunkType::JSONP => self.extract_jsonp_modules(program, chunk),
            ChunkType::WebpackModules => self.extract_webpack_modules(program, chunk),
        }
    }

    /// Extract modules from CommonJS format (exports.modules = {...})
    fn extract_commonjs_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = CommonJSVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        println!("[WebpackAnalyzer] Extracted {} CommonJS modules", chunk.module_count());
        Ok(())
    }

    /// Extract modules from JSONP format ((self["webpackChunk..."] = ...).push([[...], {...}]))
    fn extract_jsonp_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = JSONPVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        println!("[WebpackAnalyzer] Extracted {} JSONP modules", chunk.module_count());
        Ok(())
    }

    /// Extract modules from WebpackModules format (var __webpack_modules__ = {...})
    fn extract_webpack_modules(&self, program: &Program, chunk: &mut WebpackChunk) -> Result<()> {
        let mut visitor = WebpackModulesVisitor::new();
        program.visit_with(&mut visitor);

        for (module_id, module_source) in visitor.modules {
            let module = WebpackModule::new(module_id.clone(), module_source);
            chunk.add_module(module_id, module);
        }

        println!("[WebpackAnalyzer] Extracted {} WebpackModules modules", chunk.module_count());
        Ok(())
    }

    /// Build dependency graph by analyzing webpack_require calls
    fn build_dependency_graph(&self, chunk: &mut WebpackChunk) -> Result<()> {
        for (_module_id, module) in chunk.modules.iter_mut() {
            let dependencies = self.extract_webpack_require_calls(&module.source)?;
            for dep in dependencies {
                module.add_dependency(dep);
            }
        }

        // Build reverse dependencies
        let module_ids: Vec<_> = chunk.modules.keys().cloned().collect();
        for from_module in &module_ids {
            if let Some(from_mod) = chunk.modules.get(from_module) {
                let deps = from_mod.get_dependencies();
                for to_module in deps {
                    if let Some(to_mod) = chunk.modules.get_mut(&to_module) {
                        to_mod.add_dependent(from_module.clone());
                    }
                }
            }
        }

        println!("[WebpackAnalyzer] Built dependency graph");
        Ok(())
    }

    /// Extract webpack_require calls from module source
    fn extract_webpack_require_calls(&self, source: &str) -> Result<Vec<ModuleId>> {
        let mut dependencies = Vec::new();
        
        // If the source is empty, return empty dependencies
        if source.trim().is_empty() {
            return Ok(dependencies);
        }
        
        eprintln!("[WebpackAnalyzer] Extracting dependencies from source: {}", source);
        
        // Parse the module source to find webpack_require calls
        let fm = self.source_map.new_source_file(FileName::Custom("module.js".to_string()).into(), source.to_string());
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            None,
        );
        
        match parser.parse_program() {
            Ok(program) => {
                let mut visitor = RequireVisitor::new();
                program.visit_with(&mut visitor);
                dependencies = visitor.dependencies;
                eprintln!("[WebpackAnalyzer] Found {} dependencies: {:?}", dependencies.len(), dependencies);
            }
            Err(_) => {
                // If parsing fails, we'll return empty dependencies
                // This can happen with malformed or incomplete source
            }
        }

        Ok(dependencies)
    }
}

/// Visitor for CommonJS format chunks
struct CommonJSVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl CommonJSVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for CommonJSVisitor {
    fn visit_assign_expr(&mut self, node: &AssignExpr) {
        // Look for exports.modules = {...}
        if let AssignTarget::Simple(SimpleAssignTarget::Member(member)) = &node.left {
            if let MemberProp::Ident(prop) = &member.prop {
                if prop.sym == "modules" {
                    if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                        if obj_ident.sym == "exports" {
                            // Found exports.modules = {...}
                            if let Expr::Object(obj) = node.right.as_ref() {
                                self.extract_modules_from_object(obj);
                            }
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}

impl CommonJSVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.to_string(),
                        PropName::Ident(ident) => ident.sym.to_string(),
                        _ => continue,
                    };

                    // Extract module source from the function expression
                    let module_source = self.extract_function_source(&kv.value);
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }

    fn extract_function_source(&self, expr: &Expr) -> String {
        // Handle different patterns found in webpack chunks
        match expr {
            // Pattern 1: Direct function expressions
            Expr::Fn(func) => {
                self.extract_webpack_requires_from_function_body(&func.function.body)
            }
            // Pattern 2: Function expressions wrapped in call expressions (real-world case)
            Expr::Call(call) => {
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Paren(paren) = callee_expr.as_ref() {
                        if let Expr::Fn(func) = paren.expr.as_ref() {
                            // This is a (function expression) being called
                            return self.extract_webpack_requires_from_function_body(&func.function.body);
                        }
                    }
                }
                "/* Non-function call */".to_string()
            }
            // Pattern 3: Paren expressions wrapping function expressions
            Expr::Paren(paren) => {
                self.extract_function_source(&paren.expr)
            }
            _ => "/* Non-function module */".to_string(),
        }
    }

    fn extract_webpack_requires_from_function_body(&self, body: &Option<BlockStmt>) -> String {
        let mut source = String::new();
        
        if let Some(body) = body {
            for stmt in &body.stmts {
                // Extract webpack_require calls from various statement types
                self.extract_webpack_requires_from_stmt(stmt, &mut source);
            }
        }
        
        source
    }

    fn extract_webpack_requires_from_stmt(&self, stmt: &Stmt, source: &mut String) {
        match stmt {
            Stmt::Expr(expr_stmt) => {
                self.extract_webpack_requires_from_expr(&expr_stmt.expr, source);
            }
            Stmt::Decl(decl) => {
                if let Decl::Var(var_decl) = decl {
                    for decl in &var_decl.decls {
                        if let Some(init) = &decl.init {
                            self.extract_webpack_requires_from_expr(init, source);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn extract_webpack_requires_from_expr(&self, expr: &Expr, source: &mut String) {
        match expr {
            Expr::Call(call) => {
                if let Callee::Expr(callee_expr) = &call.callee {
                    if let Expr::Ident(ident) = callee_expr.as_ref() {
                        if ident.sym == "__webpack_require__" {
                            // Found a webpack_require call
                            if let Some(ExprOrSpread { expr, .. }) = call.args.first() {
                                if let Expr::Lit(Lit::Str(s)) = expr.as_ref() {
                                    source.push_str(&format!("__webpack_require__(\"{}\");\n", s.value));
                                }
                            }
                        }
                    }
                }
            }
            // Handle other expression types that might contain webpack_require
            Expr::Assign(assign) => {
                self.extract_webpack_requires_from_expr(&assign.right, source);
            }
            Expr::Seq(seq) => {
                for expr in &seq.exprs {
                    self.extract_webpack_requires_from_expr(expr, source);
                }
            }
            _ => {}
        }
    }
}

/// Visitor for JSONP format chunks
struct JSONPVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl JSONPVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for JSONPVisitor {
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Look for .push() calls on webpackChunk arrays
        if let Callee::Expr(expr) = &node.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "push" {
                        // Found a .push() call, check if it's on a webpackChunk array
                        if node.args.len() >= 1 {
                            if let ExprOrSpread { expr, .. } = &node.args[0] {
                                if let Expr::Array(arr) = expr.as_ref() {
                                    // Look for the modules object (usually second element)
                                    if arr.elems.len() >= 2 {
                                        if let Some(Some(ExprOrSpread { expr, .. })) = arr.elems.get(1) {
                                            if let Expr::Object(obj) = expr.as_ref() {
                                                self.extract_modules_from_object(obj);
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
        
        node.visit_children_with(self);
    }
}

impl JSONPVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.to_string(),
                        PropName::Ident(ident) => ident.sym.to_string(),
                        PropName::Num(num) => num.value.to_string(),
                        _ => continue,
                    };

                    // Extract module source (simplified)
                    let module_source = format!("/* Module: {} */", module_id);
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }
}

/// Visitor for extracting modules from WebpackModules format
struct WebpackModulesVisitor {
    modules: FxHashMap<ModuleId, String>,
}

impl WebpackModulesVisitor {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }
}

impl Visit for WebpackModulesVisitor {
    fn visit_var_decl(&mut self, node: &VarDecl) {
        // Look for: var __webpack_modules__ = ({...})
        for declarator in &node.decls {
            if let Some(ident) = declarator.name.as_ident() {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &declarator.init {
                        if let Expr::Paren(paren) = init.as_ref() {
                            if let Expr::Object(obj) = paren.expr.as_ref() {
                                self.extract_modules_from_object(obj);
                            }
                        } else if let Expr::Object(obj) = init.as_ref() {
                            self.extract_modules_from_object(obj);
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}

impl WebpackModulesVisitor {
    fn extract_modules_from_object(&mut self, obj: &ObjectLit) {
        for prop in &obj.props {
            if let PropOrSpread::Prop(prop) = prop {
                if let Prop::KeyValue(kv) = prop.as_ref() {
                    // Extract module ID from key
                    let module_id = match &kv.key {
                        PropName::Str(s) => s.value.to_string(),
                        PropName::Ident(ident) => ident.sym.to_string(),
                        PropName::Num(num) => num.value.to_string(),
                        _ => continue,
                    };

                    // Extract module source from the function
                    let module_source = self.extract_function_source(&kv.value);
                    
                    self.modules.insert(module_id, module_source);
                }
            }
        }
    }

    fn extract_function_source(&self, expr: &Expr) -> String {
        match expr {
            Expr::Paren(paren) => {
                if let Expr::Fn(func) = paren.expr.as_ref() {
                    // For now, return a simplified representation
                    format!("function({}) {{ /* function body */ }}", 
                           func.function.params.len())
                } else {
                    "/* unknown function format */".to_string()
                }
            }
            Expr::Fn(func) => {
                format!("function({}) {{ /* function body */ }}", 
                       func.function.params.len())
            }
            _ => "/* non-function module */".to_string(),
        }
    }
}

/// Visitor for extracting webpack_require calls
struct RequireVisitor {
    dependencies: Vec<ModuleId>,
}

impl RequireVisitor {
    fn new() -> Self {
        Self {
            dependencies: Vec::new(),
        }
    }
}

impl Visit for RequireVisitor {
    fn visit_call_expr(&mut self, node: &CallExpr) {
        // Look for __webpack_require__("module_id") calls
        if let Callee::Expr(expr) = &node.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym == "__webpack_require__" {
                    // Found webpack_require call
                    if let Some(ExprOrSpread { expr, .. }) = node.args.first() {
                        if let Expr::Lit(Lit::Str(s)) = expr.as_ref() {
                            eprintln!("[RequireVisitor] Found __webpack_require__ call: {}", s.value);
                            self.dependencies.push(s.value.to_string());
                        }
                    }
                }
            }
        }
        
        node.visit_children_with(self);
    }
}