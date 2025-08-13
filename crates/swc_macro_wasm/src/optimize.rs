use rustc_hash::FxHashSet;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::time::{Duration, Instant};
use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::atoms::Atom;
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{Visit, VisitMut, VisitMutWith, VisitWith};
use swc_ecma_ast::{
    CallExpr, Expr, ExprOrSpread, FnDecl, Ident, MemberExpr, 
    MemberProp, Module, ModuleItem, ObjectLit, Pat, Program, Prop, PropName, PropOrSpread, 
    Script, Stmt, VarDecl,
};
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{text_writer, Emitter};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use thiserror::Error;
// Re-enabling webpack_analyzer_v2 step by step
use webpack_analyzer_v2::{
    ChunkCharacteristics, ChunkType, DependencyGraph, ShareUsageConfig, WebpackAnalyzer,
    WebpackChunk,
};

/// Error types for optimization operations
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Failed to parse JavaScript: {0}")]
    ParseError(String),

    #[error("Failed to emit JavaScript: {0}")]
    EmitError(String),

    #[error("Invalid UTF-8 encoding in generated code")]
    Utf8Error,

    #[error("Webpack analysis failed: {0}")]
    WebpackAnalysisError(String),

    #[error("Numeric conversion failed: {0}")]
    NumericConversionError(String),
}

type OptimizationResult<T> = Result<T, OptimizationError>;

pub fn optimize(source: String, config: serde_json::Value) -> OptimizationResult<String> {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .map_err(|e| OptimizationError::ParseError(format!("Parser error: {:?}", e)))?;
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    let program = {
        let mut transformer = condition_transform(config.clone(), macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        // This worked fine in WASM from the beginning (June 2025)
        // Only the TreeShaker for webpack bundles causes WASM panics
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));
            
            // TreeShaker will handle entry point preservation
            
            perform_dce(&mut program, comments.clone(), unresolved_mark);
            
            // Use TreeShaker.optimize directly
            if has_macro_processing_config(&config) {
                let mut shaker = TreeShaker::new(config.clone());
                shaker.optimize(&mut program, cm.clone(), &comments);
                // Don't call simple tree shaking since TreeShaker should handle it
            }
            
            program.mutate(fixer(Some(&comments)));
            
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter
            .emit_program(&program)
            .map_err(|e| OptimizationError::EmitError(format!("Emit error: {:?}", e)))?;
        drop(emitter);

        String::from_utf8(buf).map_err(|_| OptimizationError::Utf8Error)?
    };

    Ok(ret)
}

fn perform_dce(m: &mut Program, comments: SingleThreadedComments, unresolved_mark: Mark) {
    let mut visitor = crate::dce::dce(
        comments,
        crate::dce::Config {
            module_mark: None,
            top_level: true,
            top_retain: Vec::new(),
            preserve_imports_with_side_effects: true,
        },
        unresolved_mark,
    );

    loop {
        m.visit_mut_with(&mut visitor);

        if !visitor.changed() {
            break;
        }

        visitor.reset();
    }
}

/// Check if the config contains macro processing directives that might create orphaned modules
fn has_macro_processing_config(config: &serde_json::Value) -> bool {
    // Check if there are macro processing directives like features, treeShake, etc.
    // Also check for entryModules which enables tree shaking with specific entry points
    config.get("treeShake").is_some() || config.get("entryModules").is_some()
}

// Simple tree shaking removed - using webpack_analyzer_v2 TreeShaker instead

// Synthetic entry point calls removed - TreeShaker handles this properly

// The complex TreeShaker using webpack_analyzer_v2 - re-enabling step by step

/// Extract chunk characteristics from config or create default ones
fn get_chunk_characteristics(config: &serde_json::Value, source: &str) -> ChunkCharacteristics {
    // First check if chunk_characteristics is at the root level (old format)
    if let Some(chars_value) = config.get("chunk_characteristics") {
        if let Ok(characteristics) =
            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
        {
            return characteristics;
        }
    }

    // Check if chunk_characteristics is nested within treeShake (new format)
    if let Some(tree_shake_config) = config.get("treeShake") {
        if let Some(tree_shake_obj) = tree_shake_config.as_object() {
            // Look for chunk_characteristics in any of the modules
            for (_, module_config) in tree_shake_obj {
                if let Some(module_obj) = module_config.as_object() {
                    if let Some(chars_value) = module_obj.get("chunk_characteristics") {
                        if let Ok(characteristics) =
                            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
                        {
                            return characteristics;
                        }
                    }
                }
            }
        }
    }

    // Detect chunk format from source code
    let chunk_format = if source.contains("exports.modules") {
        "require".to_string() // CommonJS format
    } else if source.contains("__webpack_modules__") {
        "webpack".to_string() // Standard webpack format
    } else if source.contains(".push([") {
        "jsonp".to_string() // JSONP format
    } else {
        "jsonp".to_string() // Default fallback
    };

    // Create default characteristics based on detected format
    // We no longer infer runtime or entrypoint; chunk_format drives type selection
    let (is_entrypoint, has_runtime) = (false, false);

    ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime,
        is_entrypoint,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format,
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        entry_module_id: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    }
}

/// Metrics collected during tree shaking optimization
#[derive(Debug, Clone)]
struct TreeShakeMetrics {
    modules_before: usize,
    modules_after: usize,
    modules_removed: usize,
    iterations: u32,
    time_taken: Duration,
    entry_points_found: usize,
    chunks_processed: usize,
}

impl TreeShakeMetrics {
    fn new() -> Self {
        Self {
            modules_before: 0,
            modules_after: 0,
            modules_removed: 0,
            iterations: 0,
            time_taken: Duration::new(0, 0),
            entry_points_found: 0,
            chunks_processed: 0,
        }
    }

    fn log_summary(&self) {
        if self.modules_removed > 0 {
            println!(
                "Tree shaking summary: Removed {} modules across {} iterations in {:?} ({}% reduction)",
                self.modules_removed,
                self.iterations,
                self.time_taken,
                (self.modules_removed * 100) / self.modules_before.max(1)
            );
        } else {
            println!(
                "Tree shaking summary: No modules removed in {} iterations ({:?})",
                self.iterations, self.time_taken
            );
        }
    }
}

/// Main tree shaker orchestration structure
pub struct TreeShaker {
    config: serde_json::Value,
    analyzer: WebpackAnalyzer,
}

impl TreeShaker {
    pub fn new(config: serde_json::Value) -> Self {
        Self {
            config,
            analyzer: WebpackAnalyzer::new(),
        }
    }

    /// Performs iterative webpack module tree shaking after DCE
    pub fn optimize(
        &mut self,
        program: &mut Program,
        cm: Lrc<SourceMap>,
        comments: &SingleThreadedComments,
    ) {
        let start_time = Instant::now();
        let mut metrics = TreeShakeMetrics::new();

        let mut total_removed = 0;
        let max_iterations = 5; // Prevent infinite loops

        for iteration in 1..=max_iterations {
            // Step 1: Emit current AST to string for analysis
            let current_code = {
                let mut buf = vec![];
                let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
                    as Box<dyn WriteJs>;
                let mut emitter = Emitter {
                    cfg: codegen::Config::default().with_minify(false),
                    comments: Some(comments),
                    cm: cm.clone(),
                    wr,
                };
                if emitter.emit_program(program).is_err() {
                    break; // Stop if emit fails
                }
                drop(emitter);

                match String::from_utf8(buf) {
                    Ok(code) => code,
                    Err(_) => break, // Stop if invalid UTF-8
                }
            };

            // Step 2: Analyze the chunk using webpack_analyzer_v2
            let characteristics = get_chunk_characteristics(&self.config, &current_code);
            let mut chunk = match self.analyzer.analyze_chunk(&current_code, characteristics) {
                Ok(c) => c,
                Err(e) => {
                    if iteration == 1 {
                        println!(
                            "Tree shaking: Failed to analyze chunk on first iteration: {:?} - skipping tree shaking",
                            e
                        );
                        return; // Skip tree shaking entirely if first analysis fails - maybe it's not a webpack bundle
                    } else {
                        println!(
                            "Tree shaking: Failed to analyze chunk on iteration {}: {:?} - stopping",
                            iteration, e
                        );
                        break; // Stop iterations if analysis fails on subsequent attempts
                    }
                }
            };

            // Early return if no modules found
            if chunk.modules.is_empty() {
                println!("Tree shaking: No modules found in chunk - skipping tree shaking");
                return;
            }

            // Update metrics with initial module count
            if iteration == 1 {
                metrics.modules_before = chunk.modules.len();
                metrics.chunks_processed = 1;
            }

            // Step 2.5: Update the chunk's source to reflect the current code
            chunk.source = current_code.clone();

            // Step 2.6: Rebuild dependency graph after macro processing changes
            if iteration > 1 {
                if let Err(e) = self.analyzer.rebuild_dependency_graph(&mut chunk) {
                    println!(
                        "Tree shaking warning: Failed to rebuild dependency graph on iteration {}: {:?}",
                        iteration, e
                    );
                }
            }

            // Skip entry or runtime chunks entirely
            if let Some(chars) = &chunk.characteristics {
                if chars.is_entrypoint || chars.is_runtime_chunk {
                    println!("Tree shaking: Skipping entry or runtime chunk");
                    return;
                }
            }

            // Debug: Log dependency graph details (only in debug mode)
            #[cfg(debug_assertions)]
            if iteration == 1 && chunk.modules.len() > 0 {
                println!(
                    "Tree shaking debug: Chunk has {} modules",
                    chunk.modules.len()
                );
            }

            // Prefer provided characteristics to decide split vs entry bundles
            let is_split_chunk = match chunk.chunk_type {
                ChunkType::CommonJSAsync | ChunkType::CommonJSSync => true, // CommonJS exports.modules are always split chunks
                ChunkType::JSONP => true, // JSONP chunks are always split chunks
                ChunkType::ESModules => true, // ES modules chunks are typically split chunks
                ChunkType::WebpackModules => {
                    // WebpackModules support removed - treat as non-split chunk
                    false
                }
                ChunkType::Unknown => {
                    // Unknown chunk types - default to split chunk behavior
                    true
                }
            };

            let mut entry_points: Vec<Atom> = Vec::new();
            let unreachable_modules = if is_split_chunk {
                // Check if there are macro processing directives that might create orphaned modules
                let has_macro_config = has_macro_processing_config(&self.config);

                if has_macro_config {
                    // Build explicit entry point config
                    let share_config = ShareUsageConfig {
                        entry_module_ids: self
                            .config
                            .get("entryModules")
                            .and_then(|v| v.as_object())
                            .map(|obj| {
                                obj.values()
                                    .filter_map(|v| v.as_str())
                                    .map(Atom::from)
                                    .collect::<Vec<_>>()
                            })
                            .unwrap_or_default(),
                    };
                    entry_points = chunk.extract_explicit_entry_points(&share_config);

                    // IMPORTANT: Also add modules with preserved exports as entry points
                    if let Some(tree_shake_config) = self.config.get("treeShake") {
                        if let Some(tree_shake_obj) = tree_shake_config.as_object() {
                            for (package_name, exports_config) in tree_shake_obj {
                                if let Some(exports_obj) = exports_config.as_object() {
                                    for (export_name, should_preserve) in exports_obj {
                                        if export_name == "chunk_characteristics" {
                                            continue;
                                        }
                                        if should_preserve.as_bool() == Some(true) {
                                            for module_id in chunk.modules.keys() {
                                                let module_str = module_id.as_str();
                                                let matches_export = module_str
                                                    .contains(package_name)
                                                    && (module_str.ends_with(&format!(
                                                        "/{}.js",
                                                        export_name
                                                    )) || module_str
                                                        .contains(&format!("/{}/", export_name))
                                                        || module_str.ends_with(&format!(
                                                            "/{}/index.js",
                                                            export_name
                                                        )));
                                                if matches_export
                                                    && !entry_points.contains(module_id)
                                                {
                                                    entry_points.push(module_id.clone());
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if entry_points.is_empty() {
                        println!(
                            "Tree shaking: No explicit entry points found for split chunk with macro config - skipping tree shaking"
                        );
                        Vec::new()
                    } else {
                        println!(
                            "Tree shaking: Found {} entry points for split chunk: {:?}",
                            entry_points.len(),
                            entry_points.iter().map(|a| a.as_str()).collect::<Vec<_>>()
                        );
                        if iteration == 1 {
                            Vec::new()
                        } else {
                            compute_unreachable_modules_from_entries(&chunk, &entry_points)
                        }
                    }
                } else {
                    Vec::new()
                }
            } else {
                // Standard bundle: only use explicit entry modules, no inference
                let share_config = ShareUsageConfig {
                    entry_module_ids: self
                        .config
                        .get("entryModules")
                        .and_then(|v| v.as_object())
                        .map(|obj| {
                            obj.values()
                                .filter_map(|v| v.as_str())
                                .map(Atom::from)
                                .collect::<Vec<_>>()
                        })
                        .unwrap_or_default(),
                };
                entry_points = chunk.extract_explicit_entry_points(&share_config);

                if entry_points.is_empty() {
                    println!(
                        "Tree shaking: No explicit entry points found for standard bundle - skipping tree shaking"
                    );
                    Vec::new()
                } else {
                    println!(
                        "Tree shaking: Using {} entry points for standard bundle: {:?}",
                        entry_points.len(),
                        entry_points.iter().map(|a| a.as_str()).collect::<Vec<_>>()
                    );
                    compute_unreachable_modules_from_entries(&chunk, &entry_points)
                }
            };

            // Filter out modules that are referenced by reachable modules via require calls
            let mut graph = DependencyGraph::new();
            for (_id, module) in &chunk.modules {
                graph.add_module(module.clone());
            }
            let reachable_set = graph.get_reachable_from_multiple(&entry_points);

            let unreachable_modules: Vec<Atom> = unreachable_modules
                .into_iter()
                .filter(|m| {
                    // Skip if referenced by any reachable module's source code
                    let mut referenced = false;
                    for id in &reachable_set {
                        if let Some(module) = chunk.modules.get(id) {
                            if module
                                .source
                                .contains(&format!("__webpack_require__(\"{}\")", m))
                            {
                                referenced = true;
                                break;
                            }
                        }
                    }
                    !referenced
                })
                .collect();

            if unreachable_modules.is_empty() {
                // Check if this is the first iteration of a macro-enabled split chunk
                // In this case, we need to force a second iteration to perform tree shaking
                let should_continue_for_macros =
                    iteration == 1 && is_split_chunk && has_macro_processing_config(&self.config);

                if should_continue_for_macros {
                    // Continue to next iteration
                } else {
                    // No more modules to remove - convergence reached
                    if iteration == 1 {
                        if is_split_chunk {
                            println!(
                                "Tree shaking: Split chunk detected - preserving all modules (no tree shaking)"
                            );
                        } else {
                            println!("Tree shaking: No unreachable modules found on first pass");
                        }
                    } else {
                        println!(
                            "Tree shaking: Converged after {} iterations, removed {} total modules",
                            iteration - 1,
                            total_removed
                        );
                    }
                    metrics.iterations = iteration;
                    break;
                }
            }

            // Only remove modules if we have any to remove
            if !unreachable_modules.is_empty() {
                // Remove the unreachable modules

                total_removed += unreachable_modules.len();

                // Step 4: Remove unreachable modules from the AST
                let unreachable_set: FxHashSet<String> = unreachable_modules
                    .into_iter()
                    .map(|atom| atom.to_string())
                    .collect();
                let mut pruner = ModulePruner::new(unreachable_set);
                program.visit_mut_with(&mut pruner);

                let actually_removed = pruner.get_removed_count();
                if actually_removed > 0 {
                    println!(
                        "Tree shaking: Successfully removed {} module references from AST",
                        actually_removed
                    );
                }
            }

            // Continue to next iteration to see if more modules become unreachable
        }

        // Update final metrics
        metrics.modules_removed = total_removed;
        metrics.modules_after = metrics.modules_before.saturating_sub(total_removed);
        metrics.time_taken = start_time.elapsed();
        metrics.iterations = if metrics.iterations == 0 {
            max_iterations
        } else {
            metrics.iterations
        };

        // Log comprehensive summary
        metrics.log_summary();

        if total_removed > 0 {
            println!(
                "Tree shaking: Total removed {} modules across all iterations",
                total_removed
            );
        }
    }
}

/// Compute unreachable modules using a simple reachability analysis from the given entry modules
fn compute_unreachable_modules_from_entries(
    chunk: &WebpackChunk,
    entry_points: &[Atom],
) -> Vec<Atom> {
    if entry_points.is_empty() {
        println!(
            "Tree shaking warning: No entry points found - cannot determine reachable modules"
        );
        return Vec::new();
    }

    if chunk.modules.is_empty() {
        println!("Tree shaking warning: No modules found in chunk");
        return Vec::new();
    }

    let mut graph = DependencyGraph::new();
    for (_id, module) in &chunk.modules {
        graph.add_module(module.clone());
    }

    let reachable = graph.get_reachable_from_multiple(entry_points);
    let all: std::collections::HashSet<Atom> = graph.modules.keys().cloned().collect();
    all.difference(&reachable).cloned().collect()
}

/// AST visitor that removes specified webpack modules from various module formats
/// Supports: export const __webpack_modules__, exports.modules, AMD define, and SystemJS patterns
struct ModulePruner {
    modules_to_remove: FxHashSet<String>,
    removed_count: usize,
}

impl ModulePruner {
    fn new(modules_to_remove: FxHashSet<String>) -> Self {
        Self {
            modules_to_remove,
            removed_count: 0,
        }
    }

    fn get_removed_count(&self) -> usize {
        self.removed_count
    }

    /// Check if a property key matches a module ID that should be removed
    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(num) => {
                        let num_str = num.value.to_string();
                        num_str.split('.').next().unwrap_or("").to_string()
                    }
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return false,
                };
                return self.modules_to_remove.contains(&module_id);
            }
        }
        false
    }

    /// Remove modules from object literals in expressions
    fn remove_modules_from_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Object(obj) => {
                self.remove_modules_from_object(obj);
            }
            Expr::Paren(paren) => {
                if let Expr::Object(obj) = paren.expr.as_mut() {
                    self.remove_modules_from_object(obj);
                }
            }
            _ => {}
        }
    }

    /// Remove specified modules from an object literal
    fn remove_modules_from_object(&mut self, obj: &mut ObjectLit) {
        let before_count = obj.props.len();
        obj.props.retain(|prop| !self.should_remove_property(prop));
        let after_count = obj.props.len();
        let removed_count = before_count - after_count;
        if removed_count > 0 {
            self.removed_count += removed_count;
            println!(
                "Tree shaking: Removed {} modules from object literal",
                removed_count
            );
        }
    }

    /// Check if a call expression is an AMD define pattern: define(['dep1', 'dep2'], function() { ... })
    fn is_amd_define_call(&self, call_expr: &CallExpr) -> bool {
        if let swc_ecma_ast::Callee::Expr(callee) = &call_expr.callee {
            if let Expr::Ident(ident) = callee.as_ref() {
                return ident.sym == "define";
            }
        }
        false
    }

    /// Check if a call expression is a SystemJS pattern: System.register(['dep1'], function() { ... })
    fn is_systemjs_register_call(&self, call_expr: &CallExpr) -> bool {
        if let swc_ecma_ast::Callee::Expr(callee) = &call_expr.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let Expr::Ident(obj) = member.obj.as_ref() {
                    if let swc_ecma_ast::MemberProp::Ident(prop) = &member.prop {
                        return obj.sym == "System" && prop.sym == "register";
                    }
                }
            }
        }
        false
    }

    /// Remove modules from AMD/SystemJS dependency arrays
    fn remove_from_dependency_array(&mut self, args: &mut Vec<ExprOrSpread>) {
        // AMD define([deps], factory) or System.register([deps], factory)
        if let Some(first_arg) = args.get_mut(0) {
            if let Expr::Array(array) = first_arg.expr.as_mut() {
                let before_count = array.elems.len();
                array.elems.retain(|elem| {
                    if let Some(ExprOrSpread { expr, .. }) = elem.as_ref() {
                        if let Expr::Lit(swc_ecma_ast::Lit::Str(str_lit)) = expr.as_ref() {
                            return !self.modules_to_remove.contains(&str_lit.value.to_string());
                        }
                    }
                    true
                });
                let after_count = array.elems.len();
                let removed = before_count - after_count;
                if removed > 0 {
                    self.removed_count += removed;
                    println!(
                        "Tree shaking: Removed {} dependencies from AMD/SystemJS module",
                        removed
                    );
                }
            }
        }
    }

    /// Process split chunk .push() arguments
    fn visit_mut_split_chunk_args(&mut self, args: &mut Vec<swc_ecma_ast::ExprOrSpread>) {
        // Split chunk format: .push([[chunk_ids], { modules }])
        if args.len() >= 1 {
            let swc_ecma_ast::ExprOrSpread { expr, .. } = &mut args[0];
            if let Expr::Array(array) = expr.as_mut() {
                // We expect 2 elements: [chunk_ids, modules_object]
                if array.elems.len() >= 2 {
                    if let Some(Some(swc_ecma_ast::ExprOrSpread {
                        expr: modules_expr, ..
                    })) = array.elems.get_mut(1)
                    {
                        if let Expr::Object(obj) = modules_expr.as_mut() {
                            // Remove modules from the object
                            self.remove_modules_from_object(obj);
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for ModulePruner {
    /// Visit variable declarations to find and modify __webpack_modules__
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        if node.kind != swc_ecma_ast::VarDeclKind::Const {
            node.visit_mut_children_with(self);
            return;
        }

        for declarator in &mut node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &mut declarator.init {
                        self.remove_modules_from_expr(init);
                    }
                }
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }

    /// Visit call expressions to find split chunk .push() calls, AMD define, and SystemJS register
    fn visit_mut_call_expr(&mut self, node: &mut swc_ecma_ast::CallExpr) {
        // Check for AMD define(['dep1', 'dep2'], function() { ... })
        if self.is_amd_define_call(node) {
            println!("Tree shaking: Processing AMD define call");
            self.remove_from_dependency_array(&mut node.args);
        }
        // Check for System.register(['dep1'], function() { ... })
        else if self.is_systemjs_register_call(node) {
            println!("Tree shaking: Processing SystemJS register call");
            self.remove_from_dependency_array(&mut node.args);
        }
        // Look for (self["webpackChunk..."] = ...).push([...])
        else if let swc_ecma_ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                // Check if this is a .push() call
                if let swc_ecma_ast::MemberProp::Ident(ident) = &member.prop {
                    if ident.sym == "push" {
                        // Check if this looks like a webpack chunk push
                        self.visit_mut_split_chunk_args(&mut node.args);
                    }
                }
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }

    /// Visit assignment expressions to find and modify __webpack_modules__ and exports.modules
    fn visit_mut_assign_expr(&mut self, node: &mut swc_ecma_ast::AssignExpr) {
        match &node.left {
            // Handle exports.modules = { ... }
            swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Member(
                member,
            )) => {
                if let swc_ecma_ast::MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "modules" {
                        // Check if this is exports.modules
                        if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                            if obj_ident.sym == "exports" {
                                self.remove_modules_from_expr(&mut node.right);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }
}

// Legacy entry point extraction removed - explicit entry points required