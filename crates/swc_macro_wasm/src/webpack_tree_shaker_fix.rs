// This file shows the fix needed for the webpack tree shaker
// The main issue is that export modules (like lodash.js) that have many exports
// and serve as the main API interface are being removed incorrectly

// Add this to the WebpackTreeShaker implementation:

impl WebpackTreeShaker {
    /// Detect if a module is an export module (main API interface)
    /// Export modules have many exports and primarily re-export from other modules
    fn is_export_module(&self, module_id: &str) -> bool {
        // Check if this module has been analyzed for exports
        if let Some(module_info) = self.module_graph.modules.get(module_id) {
            // Count how many modules depend on this one (imports)
            let import_count = module_info.dependencies.len();
            
            // For now, use a simple heuristic:
            // Export modules typically:
            // 1. Have many dependencies (imports many modules)
            // 2. Are not deeply nested (main entry points)
            // 3. Have names like index.js, lodash.js, or main export files
            
            if import_count > 50 {  // High number of imports suggests re-export module
                return true;
            }
            
            // Check for main entry point patterns
            if module_id.ends_with("/lodash.js") 
                || module_id.ends_with("/index.js")
                || module_id.ends_with("/_index.js") {
                return true;
            }
        }
        
        false
    }
    
    /// Modified compute_removable_modules that preserves export modules
    fn compute_removable_modules(&self) -> FxHashSet<String> {
        let mut reachable = FxHashSet::default();
        
        // Handle split chunks specially
        if self.module_graph.is_split_chunk && self.module_graph.entry_points.is_empty() {
            // In split chunks, we need to identify and preserve export modules
            // First, mark all export modules as reachable
            for (module_id, _) in &self.module_graph.modules {
                if self.is_export_module(module_id) {
                    self.mark_reachable(module_id, &mut reachable);
                }
            }
            
            // Then use the conservative split chunk logic for the rest
            let potentially_removable = self.compute_unused_modules_in_split_chunk();
            
            // But don't remove any modules that are reachable from export modules
            return potentially_removable
                .into_iter()
                .filter(|id| !reachable.contains(id))
                .collect();
        }
        
        // Normal logic for non-split chunks...
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
}

// Alternative approach: Detect export modules during analysis phase
impl WebpackModuleAnalyzer {
    /// Analyze a module to see if it's an export module
    fn analyze_module_exports(&self, module_id: &str, value: &Expr) -> usize {
        let mut export_count = 0;
        let mut visitor = ExportCountVisitor::new();
        value.visit_with(&mut visitor);
        visitor.export_count
    }
}

struct ExportCountVisitor {
    export_count: usize,
}

impl ExportCountVisitor {
    fn new() -> Self {
        Self { export_count: 0 }
    }
}

impl Visit for ExportCountVisitor {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Look for __webpack_require__.d(exports, { ... }) patterns
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(member) = expr.as_ref() {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym.as_ref() == "__webpack_require__" {
                        if let MemberProp::Ident(prop) = &member.prop {
                            if prop.sym.as_ref() == "d" {
                                // This is a __webpack_require__.d call
                                // Count exports in the second argument
                                if call.args.len() >= 2 {
                                    if let Expr::Object(obj) = call.args[1].expr.as_ref() {
                                        self.export_count += obj.props.len();
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        
        call.visit_children_with(self);
    }
}