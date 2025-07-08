use crate::{error::WebpackGraphError, graph::{ModuleGraph, ModuleNode}, Result};
use rustc_hash::{FxHashMap, FxHashSet};
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

        if visitor.webpack_modules.is_empty() {
            return Err(WebpackGraphError::InvalidBundleFormat(
                "No __webpack_modules__ found in bundle".to_string(),
            ));
        }

        // Build the module graph
        let mut graph = ModuleGraph::new();

        // Add all modules to the graph with their dependencies extracted during AST traversal
        for (module_id, (_module_ast, dependencies)) in &visitor.webpack_modules {
            let mut module_node = ModuleNode::new(module_id.clone(), format!("/* Module {} */", module_id));
            
            for dep_id in dependencies {
                module_node.add_dependency(dep_id.clone());
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

        // Note: Empty entry points are allowed for tree shaking scenarios where DCE
        // has removed all entry point imports, making all modules unreachable

        Ok(graph)
    }
}

/// AST visitor to extract webpack module information
struct WebpackVisitor {
    webpack_modules: FxHashMap<String, (Box<Expr>, FxHashSet<String>)>,
    entry_points: Vec<String>,
    webpack_modules_span: Option<Span>,
}

impl WebpackVisitor {
    fn new() -> Self {
        Self {
            webpack_modules: FxHashMap::default(),
            entry_points: Vec::new(),
            webpack_modules_span: None,
        }
    }

    /// Extract module content from object property
    fn extract_module_content(&self, prop: &PropOrSpread) -> Option<(String, Box<Expr>, FxHashSet<String>)> {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                // Extract module ID
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next()?.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return None,
                };

                // Extract dependencies using pure AST traversal
                let mut dependencies = FxHashSet::default();
                self.extract_require_calls_from_expr(&kv.value, &mut dependencies);
                
                return Some((module_id, kv.value.clone(), dependencies));
            }
        }
        None
    }

    /// Recursively extract webpack_require calls from an expression using pure AST traversal
    fn extract_require_calls_from_expr(&self, expr: &Expr, dependencies: &mut FxHashSet<String>) {
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
            Expr::Arrow(arrow) => {
                match arrow.body.as_ref() {
                    BlockStmtOrExpr::BlockStmt(block) => {
                        for stmt in &block.stmts {
                            self.extract_require_calls_from_stmt(stmt, dependencies);
                        }
                    }
                    BlockStmtOrExpr::Expr(expr) => {
                        self.extract_require_calls_from_expr(expr, dependencies);
                    }
                }
            }
            Expr::Call(call) => {
                if let Some(module_id) = self.extract_webpack_require_call(call) {
                    dependencies.insert(module_id);
                }
                // Also check arguments for nested calls
                for arg in &call.args {
                    self.extract_require_calls_from_expr(&arg.expr, dependencies);
                }
                // Check callee for nested expressions
                if let Callee::Expr(callee_expr) = &call.callee {
                    self.extract_require_calls_from_expr(callee_expr, dependencies);
                }
            }
            Expr::Assign(assign) => {
                self.extract_require_calls_from_expr(&assign.right, dependencies);
                if let AssignTarget::Simple(SimpleAssignTarget::Paren(paren)) = &assign.left {
                    self.extract_require_calls_from_expr(&paren.expr, dependencies);
                }
            }
            Expr::Seq(seq) => {
                for expr in &seq.exprs {
                    self.extract_require_calls_from_expr(expr, dependencies);
                }
            }
            Expr::Cond(cond) => {
                self.extract_require_calls_from_expr(&cond.test, dependencies);
                self.extract_require_calls_from_expr(&cond.cons, dependencies);
                self.extract_require_calls_from_expr(&cond.alt, dependencies);
            }
            Expr::Bin(bin) => {
                self.extract_require_calls_from_expr(&bin.left, dependencies);
                self.extract_require_calls_from_expr(&bin.right, dependencies);
            }
            Expr::Unary(unary) => {
                self.extract_require_calls_from_expr(&unary.arg, dependencies);
            }
            Expr::Update(update) => {
                self.extract_require_calls_from_expr(&update.arg, dependencies);
            }
            Expr::Member(member) => {
                self.extract_require_calls_from_expr(&member.obj, dependencies);
                if let MemberProp::Computed(computed) = &member.prop {
                    self.extract_require_calls_from_expr(&computed.expr, dependencies);
                }
            }
            Expr::SuperProp(super_prop) => {
                if let SuperProp::Computed(computed) = &super_prop.prop {
                    self.extract_require_calls_from_expr(&computed.expr, dependencies);
                }
            }
            Expr::Object(obj) => {
                for prop in &obj.props {
                    match prop {
                        PropOrSpread::Prop(prop) => {
                            match prop.as_ref() {
                                Prop::KeyValue(kv) => {
                                    self.extract_require_calls_from_expr(&kv.value, dependencies);
                                    if let PropName::Computed(computed) = &kv.key {
                                        self.extract_require_calls_from_expr(&computed.expr, dependencies);
                                    }
                                }
                                Prop::Assign(assign) => {
                                    self.extract_require_calls_from_expr(&assign.value, dependencies);
                                }
                                Prop::Getter(getter) => {
                                    if let Some(body) = &getter.body {
                                        for stmt in &body.stmts {
                                            self.extract_require_calls_from_stmt(stmt, dependencies);
                                        }
                                    }
                                }
                                Prop::Setter(setter) => {
                                    if let Some(body) = &setter.body {
                                        for stmt in &body.stmts {
                                            self.extract_require_calls_from_stmt(stmt, dependencies);
                                        }
                                    }
                                }
                                Prop::Method(method) => {
                                    if let Some(body) = &method.function.body {
                                        for stmt in &body.stmts {
                                            self.extract_require_calls_from_stmt(stmt, dependencies);
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                        PropOrSpread::Spread(spread) => {
                            self.extract_require_calls_from_expr(&spread.expr, dependencies);
                        }
                    }
                }
            }
            Expr::Array(array) => {
                for elem in &array.elems {
                    if let Some(elem) = elem {
                        match elem {
                            ExprOrSpread { expr, .. } => {
                                self.extract_require_calls_from_expr(expr, dependencies);
                            }
                        }
                    }
                }
            }
            Expr::Tpl(tpl) => {
                for expr in &tpl.exprs {
                    self.extract_require_calls_from_expr(expr, dependencies);
                }
            }
            Expr::TaggedTpl(tagged) => {
                self.extract_require_calls_from_expr(&tagged.tag, dependencies);
                for expr in &tagged.tpl.exprs {
                    self.extract_require_calls_from_expr(expr, dependencies);
                }
            }
            Expr::New(new_expr) => {
                self.extract_require_calls_from_expr(&new_expr.callee, dependencies);
                if let Some(args) = &new_expr.args {
                    for arg in args {
                        self.extract_require_calls_from_expr(&arg.expr, dependencies);
                    }
                }
            }
            Expr::Await(await_expr) => {
                self.extract_require_calls_from_expr(&await_expr.arg, dependencies);
            }
            Expr::Yield(yield_expr) => {
                if let Some(arg) = &yield_expr.arg {
                    self.extract_require_calls_from_expr(arg, dependencies);
                }
            }
            _ => {}
        }
    }

    /// Extract webpack_require calls from a statement using pure AST traversal
    fn extract_require_calls_from_stmt(&self, stmt: &Stmt, dependencies: &mut FxHashSet<String>) {
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
            Stmt::Decl(Decl::Fn(fn_decl)) => {
                if let Some(body) = &fn_decl.function.body {
                    for stmt in &body.stmts {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
            }
            Stmt::Decl(Decl::Class(class_decl)) => {
                for member in &class_decl.class.body {
                    match member {
                        ClassMember::Constructor(constructor) => {
                            if let Some(body) = &constructor.body {
                                for stmt in &body.stmts {
                                    self.extract_require_calls_from_stmt(stmt, dependencies);
                                }
                            }
                        }
                        ClassMember::Method(method) => {
                            if let Some(body) = &method.function.body {
                                for stmt in &body.stmts {
                                    self.extract_require_calls_from_stmt(stmt, dependencies);
                                }
                            }
                        }
                        ClassMember::PrivateMethod(method) => {
                            if let Some(body) = &method.function.body {
                                for stmt in &body.stmts {
                                    self.extract_require_calls_from_stmt(stmt, dependencies);
                                }
                            }
                        }
                        ClassMember::ClassProp(prop) => {
                            if let Some(value) = &prop.value {
                                self.extract_require_calls_from_expr(value, dependencies);
                            }
                        }
                        ClassMember::PrivateProp(prop) => {
                            if let Some(value) = &prop.value {
                                self.extract_require_calls_from_expr(value, dependencies);
                            }
                        }
                        _ => {}
                    }
                }
            }
            Stmt::Block(block) => {
                for stmt in &block.stmts {
                    self.extract_require_calls_from_stmt(stmt, dependencies);
                }
            }
            Stmt::If(if_stmt) => {
                self.extract_require_calls_from_expr(&if_stmt.test, dependencies);
                self.extract_require_calls_from_stmt(&if_stmt.cons, dependencies);
                if let Some(alt) = &if_stmt.alt {
                    self.extract_require_calls_from_stmt(alt, dependencies);
                }
            }
            Stmt::Switch(switch_stmt) => {
                self.extract_require_calls_from_expr(&switch_stmt.discriminant, dependencies);
                for case in &switch_stmt.cases {
                    if let Some(test) = &case.test {
                        self.extract_require_calls_from_expr(test, dependencies);
                    }
                    for stmt in &case.cons {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
            }
            Stmt::While(while_stmt) => {
                self.extract_require_calls_from_expr(&while_stmt.test, dependencies);
                self.extract_require_calls_from_stmt(&while_stmt.body, dependencies);
            }
            Stmt::DoWhile(do_while) => {
                self.extract_require_calls_from_stmt(&do_while.body, dependencies);
                self.extract_require_calls_from_expr(&do_while.test, dependencies);
            }
            Stmt::For(for_stmt) => {
                if let Some(init) = &for_stmt.init {
                    match init {
                        VarDeclOrExpr::VarDecl(var_decl) => {
                            for declarator in &var_decl.decls {
                                if let Some(init) = &declarator.init {
                                    self.extract_require_calls_from_expr(init, dependencies);
                                }
                            }
                        }
                        VarDeclOrExpr::Expr(expr) => {
                            self.extract_require_calls_from_expr(expr, dependencies);
                        }
                    }
                }
                if let Some(test) = &for_stmt.test {
                    self.extract_require_calls_from_expr(test, dependencies);
                }
                if let Some(update) = &for_stmt.update {
                    self.extract_require_calls_from_expr(update, dependencies);
                }
                self.extract_require_calls_from_stmt(&for_stmt.body, dependencies);
            }
            Stmt::ForIn(for_in) => {
                self.extract_require_calls_from_expr(&for_in.right, dependencies);
                self.extract_require_calls_from_stmt(&for_in.body, dependencies);
            }
            Stmt::ForOf(for_of) => {
                self.extract_require_calls_from_expr(&for_of.right, dependencies);
                self.extract_require_calls_from_stmt(&for_of.body, dependencies);
            }
            Stmt::Return(ret_stmt) => {
                if let Some(arg) = &ret_stmt.arg {
                    self.extract_require_calls_from_expr(arg, dependencies);
                }
            }
            Stmt::Throw(throw_stmt) => {
                self.extract_require_calls_from_expr(&throw_stmt.arg, dependencies);
            }
            Stmt::Try(try_stmt) => {
                for stmt in &try_stmt.block.stmts {
                    self.extract_require_calls_from_stmt(stmt, dependencies);
                }
                if let Some(handler) = &try_stmt.handler {
                    for stmt in &handler.body.stmts {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
                if let Some(finalizer) = &try_stmt.finalizer {
                    for stmt in &finalizer.stmts {
                        self.extract_require_calls_from_stmt(stmt, dependencies);
                    }
                }
            }
            Stmt::With(with_stmt) => {
                self.extract_require_calls_from_expr(&with_stmt.obj, dependencies);
                self.extract_require_calls_from_stmt(&with_stmt.body, dependencies);
            }
            Stmt::Labeled(labeled) => {
                self.extract_require_calls_from_stmt(&labeled.body, dependencies);
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
                        return match expr.as_ref() {
                            // Handle numeric module IDs: __webpack_require__(123)
                            Expr::Lit(Lit::Num(num)) => {
                                Some(num.value.to_string().split('.').next()?.to_string())
                            }
                            // Handle string module IDs: __webpack_require__("moduleId")
                            Expr::Lit(Lit::Str(string_lit)) => {
                                Some(string_lit.value.to_string())
                            }
                            _ => None,
                        };
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
                        if let Some((module_id, module_ast, dependencies)) = self.extract_module_content(prop) {
                            self.webpack_modules.insert(module_id, (module_ast, dependencies));
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
                                if let Some((module_id, module_ast, dependencies)) = self.extract_module_content(prop) {
                                    self.webpack_modules.insert(module_id, (module_ast, dependencies));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

impl Default for WebpackBundleParser {
    fn default() -> Self {
        Self::new().expect("Failed to create default WebpackBundleParser")
    }
} 