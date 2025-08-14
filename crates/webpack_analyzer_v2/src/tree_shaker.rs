use std::collections::HashSet;

use rustc_hash::FxHashMap;
use swc_core::ecma::ast::{CallExpr, Expr, ExprOrSpread, MemberProp, ObjectLit, Pat, Prop, PropName, PropOrSpread, VarDecl};
use swc_core::ecma::codegen::{self, text_writer::JsWriter, Emitter};
use swc_core::ecma::parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_core::ecma::visit::{VisitMut, VisitMutWith};
use swc_core::common::{sync::Lrc, SourceMap, FileName};

use crate::dependency_graph::DependencyGraph;
use crate::module::{ModuleId, WebpackModule};
use crate::{chunk::ShareUsageConfig, ChunkCharacteristics, WebpackChunk};

/// Result of a prune operation planned and/or applied to a chunk
#[derive(Debug, Clone)]
pub struct PruneResult {
    /// Module identifiers that will be kept
    pub kept_modules: HashSet<ModuleId>,
    /// Module identifiers that will be removed
    pub removed_modules: HashSet<ModuleId>,
    /// Number of modules before pruning
    pub original_count: usize,
    /// Number of modules after pruning (kept)
    pub pruned_count: usize,
    /// If pruning was skipped, this will contain the reason
    pub skip_reason: Option<String>,
    /// Optional pruned chunk view with only kept modules (analysis-level view)
    pub pruned_chunk: Option<WebpackChunk>,
}

impl PruneResult {
    fn skipped(reason: String, original_count: usize) -> Self {
        Self {
            kept_modules: HashSet::new(),
            removed_modules: HashSet::new(),
            original_count,
            pruned_count: original_count,
            skip_reason: Some(reason),
            pruned_chunk: None,
        }
    }
}

/// Conservative, analysis-only tree shaker that plans/prunes unreachable modules
///
/// Behavior:
/// - Uses ONLY explicit entry points from `ShareUsageConfig` and optional
///   `ChunkCharacteristics.entry_module_id`
/// - Skips pruning for runtime chunks (based on `ChunkCharacteristics` only)
/// - Never infers entries from filenames or heuristics
pub struct TreeShaker;

impl Default for TreeShaker {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeShaker {
    pub fn new() -> Self {
        Self
    }

    /// Apply pruning directly to a source string and return optimized code with plan
    pub fn prune_source(
        &self,
        source: &str,
        characteristics: &ChunkCharacteristics,
    ) -> Result<(String, PruneResult), Box<dyn std::error::Error>> {
        // Parse
        let cm: Lrc<SourceMap> = Default::default();
        let fm = cm.new_source_file(FileName::Custom("chunk.js".to_string()).into(), source.to_string());
        let program = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm), None)
            .parse_program()
            .map_err(|e| format!("Parse error: {:?}", e))?;

        // Analyze
        let analyzer = crate::analyzer::WebpackAnalyzer::new();
        let chunk = analyzer.analyze_chunk(source, characteristics.clone())?;
        let plan = self.plan_prune(&chunk, &crate::chunk::ShareUsageConfig { entry_module_ids: vec![] });
        if plan.skip_reason.is_some() || plan.removed_modules.is_empty() {
            return Ok((source.to_string(), plan));
        }

        // Build pruner and mutate AST
        let mut program = program;
        let unreachable: std::collections::HashSet<String> = plan
            .removed_modules
            .iter()
            .map(|a| a.to_string())
            .collect();
        let mut pruner = AstModulePruner::new(unreachable);
        program.visit_mut_with(&mut pruner);

        // Emit
        let mut buf = vec![];
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: None,
            cm: cm.clone(),
            wr: Box::new(JsWriter::new(cm.clone(), "\n", &mut buf, None)),
        };
        emitter.emit_program(&program).map_err(|e| format!("Emit error: {:?}", e))?;
        let out = String::from_utf8(buf).map_err(|_| "Invalid UTF-8")?;
        Ok((out, plan))
    }

    /// Plan which modules could be removed based on reachability from explicit entries.
    /// Returns a `PruneResult` without modifying the input chunk.
    pub fn plan_prune(&self, chunk: &WebpackChunk, _config: &ShareUsageConfig) -> PruneResult {
        let original_count = chunk.modules.len();

        // Hard requirement: ChunkCharacteristics must be present
        let Some(characteristics) = &chunk.characteristics else {
            eprintln!("[TreeShaker] Skipping: missing ChunkCharacteristics");
            return PruneResult::skipped(
                "Missing ChunkCharacteristics; pruning skipped".to_string(),
                original_count,
            );
        };

        // If characteristics indicate runtime, skip pruning entirely
        if Self::is_runtime_chunk(characteristics) {
            eprintln!("[TreeShaker] Skipping: runtime chunk");
            return PruneResult::skipped(
                "Skipping pruning for runtime chunk as per characteristics".to_string(),
                original_count,
            );
        }

        // Entry points exclusively from characteristics.entry_module_id
        let Some(entry_id_str) = &characteristics.entry_module_id else {
            eprintln!("[TreeShaker] Skipping: entry_module_id missing in ChunkCharacteristics");
            return PruneResult::skipped(
                "ChunkCharacteristics.entry_module_id missing; pruning skipped".to_string(),
                original_count,
            );
        };
        let entry_id = swc_core::atoms::Atom::from(entry_id_str.as_str());
        if !chunk.modules.contains_key(&entry_id) {
            eprintln!("[TreeShaker] Skipping: entry_module_id not present in chunk modules");
            return PruneResult::skipped(
                "entry_module_id not present in chunk modules; pruning skipped".to_string(),
                original_count,
            );
        }
        let entry_points = vec![entry_id];

        // Build dependency graph from existing module records
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }

        // Compute reachability
        let reachable: HashSet<ModuleId> = graph.get_reachable_from_multiple(&entry_points);
        let all: HashSet<ModuleId> = chunk.modules.keys().cloned().collect();
        let removed: HashSet<ModuleId> = all.difference(&reachable).cloned().collect();

        PruneResult {
            kept_modules: reachable.clone(),
            removed_modules: removed,
            original_count,
            pruned_count: reachable.len(),
            skip_reason: None,
            pruned_chunk: None,
        }
    }

    /// Apply a previously planned prune (or plan on the fly) and return a pruned
    /// analysis-level view of the chunk containing only kept modules.
    ///
    /// This does not reconstruct or mutate source code; it filters the `modules`
    /// map to those that are reachable from explicit entries.
    pub fn prune_chunk(&self, chunk: &WebpackChunk, config: &ShareUsageConfig) -> PruneResult {
        let mut result = self.plan_prune(chunk, config);

        if result.skip_reason.is_some() {
            return result;
        }

        // Build filtered module map
        let mut filtered_modules: FxHashMap<ModuleId, WebpackModule> = FxHashMap::default();
        for module_id in &result.kept_modules {
            if let Some(module) = chunk.modules.get(module_id) {
                filtered_modules.insert(module_id.clone(), module.clone());
            }
        }

        // Construct pruned chunk view (source preserved, modules filtered)
        let mut pruned = WebpackChunk {
            chunk_type: chunk.chunk_type.clone(),
            modules: filtered_modules,
            source: chunk.source.clone(),
            characteristics: chunk.characteristics.clone(),
        };

        // Recompute dependency relationships within the pruned map:
        // Remove edges to modules that were dropped.
        Self::sanitize_dependencies(&mut pruned);

        result.pruned_chunk = Some(pruned);
        result
    }

    fn is_runtime_chunk(characteristics: &ChunkCharacteristics) -> bool {
        characteristics.is_runtime()
    }

    fn sanitize_dependencies(chunk: &mut WebpackChunk) {
        let kept: HashSet<ModuleId> = chunk.modules.keys().cloned().collect();
        for (_id, module) in chunk.modules.iter_mut() {
            module.dependencies.retain(|dep| kept.contains(dep));
            module.dependents.retain(|dep| kept.contains(dep));
        }
    }
}

/// Minimal AST pruner mirroring swc_macro_wasm logic for supported formats
struct AstModulePruner {
    to_remove: std::collections::HashSet<String>,
}

impl AstModulePruner {
    fn new(to_remove: std::collections::HashSet<String>) -> Self { Self { to_remove } }

    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(n) => n.value.to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(i) => i.sym.to_string(),
                    _ => return false,
                };
                return self.to_remove.contains(&module_id);
            }
        }
        false
    }

    fn strip_from_object(&self, obj: &mut ObjectLit) {
        obj.props.retain(|p| !self.should_remove_property(p));
    }

    fn strip_from_expr(&self, expr: &mut Expr) {
        match expr {
            Expr::Object(obj) => self.strip_from_object(obj),
            Expr::Paren(paren) => if let Expr::Object(obj) = paren.expr.as_mut() { self.strip_from_object(obj) },
            _ => {}
        }
    }
}

impl VisitMut for AstModulePruner {
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        for decl in &mut node.decls {
            // Only touch __webpack_modules__ initializers
            if let Pat::Ident(ident) = &decl.name {
                if ident.id.sym == "__webpack_modules__" {
                    if let Some(init) = &mut decl.init {
                        self.strip_from_expr(init);
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_assign_expr(&mut self, node: &mut swc_core::ecma::ast::AssignExpr) {
        // Exports/modules patterns
        if let swc_core::ecma::ast::AssignTarget::Simple(swc_core::ecma::ast::SimpleAssignTarget::Member(member)) = &node.left {
            // exports.modules = { ... }
            let is_modules = matches!(&member.prop, MemberProp::Ident(p) if p.sym == "modules")
                || matches!(&member.prop, MemberProp::Computed(c) if matches!(c.expr.as_ref(), Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) if s.value == *"modules"));
            if is_modules {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym == "exports" { self.strip_from_expr(&mut node.right); }
                }
                if let Expr::Member(inner) = member.obj.as_ref() {
                    if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (inner.obj.as_ref(), &inner.prop) {
                        if obj.sym == "module" && prop.sym == "exports" { self.strip_from_expr(&mut node.right); }
                    }
                }
            }
            // module.exports = { ... }
            let is_module_exports = matches!(&member.prop, MemberProp::Ident(p) if p.sym == "exports")
                || matches!(&member.prop, MemberProp::Computed(c) if matches!(c.expr.as_ref(), Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) if s.value == *"exports"));
            if is_module_exports {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if obj.sym == "module" { self.strip_from_expr(&mut node.right); }
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_call_expr(&mut self, node: &mut CallExpr) {
        // AMD define([...], fn)
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Ident(id) = callee.as_ref() {
                if id.sym == "define" {
                    // Strip removed ids from the dependency array (arg0)
                    if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                        if let Expr::Array(arr) = expr.as_mut() {
                            arr.elems.retain(|elem| {
                                if let Some(ExprOrSpread { expr, .. }) = elem {
                                    if let Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) = expr.as_ref() {
                                        return !self.to_remove.contains(&s.value.to_string());
                                    }
                                }
                                true
                            });
                        }
                    }
                }
            }
        }
        // System.register([...], fn)
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (member.obj.as_ref(), &member.prop) {
                    if obj.sym == "System" && prop.sym == "register" {
                        if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                            if let Expr::Array(arr) = expr.as_mut() {
                                arr.elems.retain(|elem| {
                                    if let Some(ExprOrSpread { expr, .. }) = elem {
                                        if let Expr::Lit(swc_core::ecma::ast::Lit::Str(s)) = expr.as_ref() {
                                            return !self.to_remove.contains(&s.value.to_string());
                                        }
                                    }
                                    true
                                });
                            }
                        }
                    }
                }
            }
        }
        // JSONP (...).push([[ids], { modules }, runtime?])
        if let swc_core::ecma::ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let MemberProp::Ident(p) = &member.prop {
                    if p.sym == "push" {
                        if let Some(ExprOrSpread { expr, .. }) = node.args.get_mut(0) {
                            if let Expr::Array(arr) = expr.as_mut() {
                                if let Some(Some(ExprOrSpread { expr: mods, .. })) = arr.elems.get_mut(1) {
                                    if let Expr::Object(obj) = mods.as_mut() {
                                        self.strip_from_object(obj);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }
}


