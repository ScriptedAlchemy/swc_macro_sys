use rustc_hash::FxHashMap;
use rustc_hash::FxHashSet;
use swc_common::pass::Repeated;
use swc_ecma_ast::*;
use swc_ecma_visit::{Visit, VisitMut, VisitMutWith, VisitWith};

/// DCE-style webpack tree shaker with automatic convergence detection
pub struct WebpackTreeShaker {
    /// Whether any changes were made in the current pass
    changed: bool,
    /// Current pass number for debugging
    pass: u16,
    /// Module graph built from AST analysis
    module_graph: WebpackModuleGraph,
    /// Modules removed in the current pass
    removed_modules: FxHashSet<String>,
}

/// Internal representation of webpack module graph
#[derive(Debug, Default)]
struct WebpackModuleGraph {
    /// All modules found in the bundle
    modules: FxHashMap<String, ModuleInfo>,
    /// Entry points (modules that should never be removed)
    entry_points: FxHashSet<String>,
    /// Dependencies between modules
    dependencies: FxHashMap<String, FxHashSet<String>>,
    /// Whether this appears to be a split chunk with no entry points
    is_split_chunk: bool,
}

#[derive(Debug)]
struct ModuleInfo {
    /// Module ID
    id: String,
    /// Whether module is reachable from entry points
    reachable: bool,
    /// Dependencies this module requires
    dependencies: FxHashSet<String>,
}

impl WebpackTreeShaker {
    pub fn new() -> Self {
        Self {
            changed: false,
            pass: 0,
            module_graph: WebpackModuleGraph::default(),
            removed_modules: FxHashSet::default(),
        }
    }
    
    /// Set tree shaking configuration
    pub fn set_config(&mut self, _config: serde_json::Value) {
        // For now, just acknowledge that config is received
        // TODO: Implement tree shaking configuration integration
        eprintln!("Tree shaking config received (integration pending)");
    }

    pub fn pass(&self) -> u16 {
        self.pass
    }

    /// Analyze AST to build webpack module graph
    fn analyze_modules(&mut self, program: &Program) {
        let mut analyzer = WebpackModuleAnalyzer::new();
        program.visit_with(&mut analyzer);
        self.module_graph = analyzer.into_graph();
    }

    /// Compute which modules can be removed
    fn compute_removable_modules(&self) -> FxHashSet<String> {
        let mut reachable = FxHashSet::default();

        // Handle split chunks specially
        if self.module_graph.is_split_chunk && self.module_graph.entry_points.is_empty() {
            // In split chunks with no entry points, all modules are potentially unreachable
            // But we need to be conservative and only remove clearly unused ones
            return self.compute_unused_modules_in_split_chunk();
        }

        // Mark all entry points as reachable
        for entry in &self.module_graph.entry_points {
            self.mark_reachable(entry, &mut reachable);
        }

        // Return unreachable modules
        self.module_graph
            .modules
            .keys()
            .filter(|id| !reachable.contains(*id))
            .cloned()
            .collect()
    }

    /// For split chunks, use more conservative analysis
    fn compute_unused_modules_in_split_chunk(&self) -> FxHashSet<String> {
        // In split chunks (vendor chunks), we need to identify export modules as implicit entry points
        let mut reachable = FxHashSet::default();
        
        // First, find modules that are likely export modules (high dependency count)
        for (module_id, info) in &self.module_graph.modules {
            // Export modules typically import many other modules to re-export them
            if info.dependencies.len() > 20 {
                // This is likely an export module, mark it and its dependencies as reachable
                self.mark_reachable(module_id, &mut reachable);
            }
        }
        
        // If we didn't find any export modules, be very conservative
        if reachable.is_empty() {
            // Only remove modules that are clearly internal helpers
            return self.module_graph
                .modules
                .iter()
                .filter(|(id, _)| {
                    let is_unused = !self.is_module_referenced(id);
                    let is_internal_helper = self.is_internal_helper_module(id);
                    is_unused && is_internal_helper
                })
                .map(|(id, _)| id.clone())
                .collect();
        }
        
        // Return modules that are not reachable from export modules
        self.module_graph
            .modules
            .keys()
            .filter(|id| !reachable.contains(*id))
            .cloned()
            .collect()
    }

    fn is_module_referenced(&self, module_id: &str) -> bool {
        self.module_graph
            .dependencies
            .values()
            .any(|deps| deps.contains(module_id))
    }

    fn is_internal_helper_module(&self, module_id: &str) -> bool {
        // Check if this looks like an internal lodash helper
        module_id.starts_with("../../node_modules/.pnpm/lodash-es")
            && (module_id.contains("/_") || module_id.contains("/internal/"))
    }

    fn mark_reachable(&self, module_id: &str, reachable: &mut FxHashSet<String>) {
        if reachable.contains(module_id) {
            return; // Already processed
        }

        reachable.insert(module_id.to_string());

        // Mark all dependencies as reachable
        if let Some(deps) = self.module_graph.dependencies.get(module_id) {
            for dep in deps {
                self.mark_reachable(dep, reachable);
            }
        }
    }

    /// Remove modules from the AST
    fn remove_modules(&mut self, program: &mut Program, modules_to_remove: &FxHashSet<String>) {
        if modules_to_remove.is_empty() {
            return;
        }

        let mut remover = ModuleRemovalVisitor::new(modules_to_remove.clone());
        program.visit_mut_with(&mut remover);

        if remover.removed_count > 0 {
            self.changed = true;
            self.removed_modules = modules_to_remove.clone();
            
            eprintln!(
                "Webpack tree shaking pass {}: Removed {} modules: {:?}",
                self.pass + 1,
                remover.removed_count,
                modules_to_remove.iter().take(5).collect::<Vec<_>>()
            );
        }
    }
}

impl Repeated for WebpackTreeShaker {
    fn changed(&self) -> bool {
        self.changed
    }

    fn reset(&mut self) {
        self.pass += 1;
        self.changed = false;
        self.removed_modules.clear();
        // Keep module_graph for next iteration analysis
    }
}

impl VisitMut for WebpackTreeShaker {
    fn visit_mut_program(&mut self, program: &mut Program) {
        // Phase 1: Analyze the AST to build module graph
        self.analyze_modules(program);

        // Phase 2: Compute which modules can be removed
        let modules_to_remove = self.compute_removable_modules();

        // Phase 3: Remove unreachable modules
        self.remove_modules(program, &modules_to_remove);

        // Continue visiting children for completeness
        program.visit_mut_children_with(self);
    }
}

/// Visitor that analyzes AST to build webpack module graph
struct WebpackModuleAnalyzer {
    modules: FxHashMap<String, ModuleInfo>,
    entry_points: FxHashSet<String>,
    dependencies: FxHashMap<String, FxHashSet<String>>,
    is_split_chunk: bool,
    found_webpack_require: bool,
}

impl WebpackModuleAnalyzer {
    fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
            entry_points: FxHashSet::default(),
            dependencies: FxHashMap::default(),
            is_split_chunk: false,
            found_webpack_require: false,
        }
    }

    fn into_graph(self) -> WebpackModuleGraph {
        WebpackModuleGraph {
            modules: self.modules,
            entry_points: self.entry_points,
            dependencies: self.dependencies,
            is_split_chunk: self.is_split_chunk,
        }
    }

    fn extract_module_id(&self, key: &PropName) -> Option<String> {
        match key {
            PropName::Str(s) => Some(s.value.to_string()),
            PropName::Computed(computed) => {
                if let Expr::Lit(Lit::Str(s)) = computed.expr.as_ref() {
                    Some(s.value.to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn extract_dependencies(&self, expr: &Expr) -> FxHashSet<String> {
        let mut deps = FxHashSet::default();
        let mut extractor = DependencyExtractor::new();
        expr.visit_with(&mut extractor);
        deps.extend(extractor.dependencies);
        deps
    }

    fn is_webpack_modules_object(&self, obj: &ObjectLit) -> bool {
        // Look for patterns indicating this is __webpack_modules__
        obj.props.len() > 1
            && obj.props.iter().any(|prop| {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(kv) = prop.as_ref() {
                        // Check if key looks like a module path
                        if let Some(id) = self.extract_module_id(&kv.key) {
                            return id.contains("node_modules") || id.contains(".js") || id.ends_with("/");
                        }
                    }
                }
                false
            })
    }

    fn analyze_webpack_call(&mut self, call: &CallExpr) {
        // Look for webpack entry point patterns
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym.as_ref() == "__webpack_require__" {
                    self.found_webpack_require = true;
                    // Extract entry point from first argument
                    if let Some(arg) = call.args.first() {
                        if let Expr::Lit(Lit::Str(s)) = arg.expr.as_ref() {
                            self.entry_points.insert(s.value.to_string());
                        }
                    }
                }
            }
        }
    }
}

impl Visit for WebpackModuleAnalyzer {
    fn visit_object_lit(&mut self, obj: &ObjectLit) {
        if self.is_webpack_modules_object(obj) {
            for prop in &obj.props {
                if let PropOrSpread::Prop(prop) = prop {
                    if let Prop::KeyValue(kv) = prop.as_ref() {
                        if let Some(module_id) = self.extract_module_id(&kv.key) {
                            let dependencies = self.extract_dependencies(&kv.value);

                            self.modules.insert(
                                module_id.clone(),
                                ModuleInfo {
                                    id: module_id.clone(),
                                    reachable: false,
                                    dependencies: dependencies.clone(),
                                },
                            );

                            self.dependencies.insert(module_id, dependencies);
                        }
                    }
                }
            }
        }

        obj.visit_children_with(self);
    }

    fn visit_call_expr(&mut self, call: &CallExpr) {
        self.analyze_webpack_call(call);
        call.visit_children_with(self);
    }

    fn visit_var_decl(&mut self, var_decl: &VarDecl) {
        // Check for webpack chunk patterns
        for decl in &var_decl.decls {
            if let Some(init) = &decl.init {
                if let Expr::Object(obj) = init.as_ref() {
                    if self.is_webpack_modules_object(obj) {
                        // This looks like a split chunk if we haven't found entry points
                        if self.entry_points.is_empty() && !self.found_webpack_require {
                            self.is_split_chunk = true;
                        }
                    }
                }
            }
        }

        var_decl.visit_children_with(self);
    }
}

/// Extract dependency information from module functions
struct DependencyExtractor {
    dependencies: FxHashSet<String>,
}

impl DependencyExtractor {
    fn new() -> Self {
        Self {
            dependencies: FxHashSet::default(),
        }
    }
}

impl Visit for DependencyExtractor {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Look for __webpack_require__ calls
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = expr.as_ref() {
                if ident.sym.as_ref() == "__webpack_require__" {
                    if let Some(arg) = call.args.first() {
                        if let Expr::Lit(Lit::Str(s)) = arg.expr.as_ref() {
                            self.dependencies.insert(s.value.to_string());
                        }
                    }
                }
            }
        }

        call.visit_children_with(self);
    }
}

/// Visitor that removes specific modules from AST
struct ModuleRemovalVisitor {
    modules_to_remove: FxHashSet<String>,
    removed_count: usize,
}

impl ModuleRemovalVisitor {
    fn new(modules_to_remove: FxHashSet<String>) -> Self {
        Self {
            modules_to_remove,
            removed_count: 0,
        }
    }

    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                if let PropName::Str(s) = &kv.key {
                    return self.modules_to_remove.contains(s.value.as_ref());
                }
                if let PropName::Computed(computed) = &kv.key {
                    if let Expr::Lit(Lit::Str(s)) = computed.expr.as_ref() {
                        return self.modules_to_remove.contains(s.value.as_ref());
                    }
                }
            }
        }
        false
    }
}

impl VisitMut for ModuleRemovalVisitor {
    fn visit_mut_object_lit(&mut self, obj: &mut ObjectLit) {
        let original_len = obj.props.len();
        obj.props.retain(|prop| !self.should_remove_property(prop));

        if obj.props.len() < original_len {
            self.removed_count += original_len - obj.props.len();
        }

        obj.visit_mut_children_with(self);
    }
}