use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{VisitMutWith, VisitMut};
use swc_core::atoms::Atom;
use swc_ecma_ast::{Program, Expr, VarDecl, Pat, ObjectLit, PropOrSpread, Prop, PropName};
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use webpack_analyzer_v2::{WebpackAnalyzer, ChunkType, ChunkCharacteristics, DependencyGraph, WebpackChunk};
use rustc_hash::FxHashSet;
use regex::Regex;
use std::collections::HashSet;
use std::time::Instant;

use crate::error::{OptimizationError, OptimizationResult, ErrorContext};
use crate::config::{OptimizationConfig, MemoryConfig};
use crate::cache::{OptimizationContext, MODULE_PATTERN_STR, MODULE_PATTERN_NUM, REQUIRE_PATTERN_STR, REQUIRE_PATTERN_NUM, ENTRY_POINT_REGEX};
use crate::convergence::{ConvergenceDetector, ConvergenceStatus};
use crate::performance::{PerformanceMonitor, PerformanceEstimator};

/// Main optimization entry point with fallback error handling
pub fn optimize(source: String, config: serde_json::Value) -> String {
    match optimize_with_error_handling(source.clone(), config) {
        Ok(result) => result,
        Err(err) => {
            eprintln!("Optimization error: {}", err);
            // Return original source as fallback
            source
        }
    }
}

/// Optimized version with comprehensive error handling and performance monitoring
pub fn optimize_with_error_handling(source: String, config: serde_json::Value) -> OptimizationResult<String> {
    // Parse optimization configuration
    let opt_config = OptimizationConfig::from_json(&config)?;
    opt_config.validate()?;
    
    // Initialize optimization context with caching and monitoring
    let mut context = OptimizationContext::new(opt_config.clone());
    let mut monitor = PerformanceMonitor::new(opt_config.enable_performance_monitoring);
    
    let _total_timer = monitor.scoped_timer("total_optimization");
    
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let _parse_timer = monitor.scoped_timer("parsing");
        
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        );
        
        let program = parser.parse_program()
            .map_err(|err| OptimizationError::parse_error(
                format!("Failed to parse program: {:?}", err),
                "Initial program parsing"
            ))?;
            
        monitor.record_memory("parsing");
        (program, comments)
    };

    let macros = {
        let _macro_timer = monitor.scoped_timer("macro_parsing");
        let parser = MacroParser::new("common");
        let result = parser.parse(&comments);
        monitor.record_memory("macro_parsing");
        result
    };

    let program = {
        let _transform_timer = monitor.scoped_timer("transformation");
        
        let mut transformer = condition_transform(config.clone(), macros);
        program.visit_mut_with(&mut transformer);
        monitor.increment("macro_transforms");

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || -> OptimizationResult<Program> {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));
            monitor.increment("resolver_passes");

            if opt_config.dce_config.enabled {
                perform_dce_with_convergence(&mut program, comments.clone(), unresolved_mark, &opt_config, &mut monitor)?;
            }

            if opt_config.tree_shaking.enabled {
                // Tree shake webpack modules after removing unused imports
                let modules_before = count_webpack_modules(&program);
                let start_time = Instant::now();
                
                perform_webpack_tree_shaking_optimized(&mut program, cm.clone(), &comments, &config, &opt_config, &mut context, &mut monitor)?;
                
                if opt_config.tree_shaking.enable_simple_orphan_removal {
                    // As a fallback, also try simple regex-based orphan removal
                    perform_simple_orphan_removal_safe(&mut program, cm.clone(), &comments, &config, &mut monitor)?;
                }
                
                let modules_after = count_webpack_modules(&program);
                let total_time = start_time.elapsed();
                
                if opt_config.enable_performance_monitoring {
                    let estimate = PerformanceEstimator::estimate_improvement(
                        modules_before, modules_after, 1, total_time
                    );
                    println!("{}", estimate);
                }
            }

            program.mutate(fixer(Some(&comments)));
            monitor.increment("fixer_passes");

            Ok(program)
        })?
    };

    let ret = {
        let _emit_timer = monitor.scoped_timer("emission");
        
        // Try to use cached emission if available
        if let Some(cached) = context.ast_cache.get(&program) {
            monitor.increment("cache_hits");
            cached
        } else {
            let mut buf = vec![];
            let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
                as Box<dyn WriteJs>;
            let mut emitter = Emitter {
                cfg: codegen::Config::default().with_minify(false),
                comments: Some(&comments),
                cm: cm.clone(),
                wr,
            };
            
            emitter.emit_program(&program)
                .map_err(|err| OptimizationError::emission_error(
                    format!("Failed to emit program: {:?}", err),
                    "final_emission",
                    "Program to string conversion"
                ))?;
            drop(emitter);

            let result = String::from_utf8(buf)
                .map_err(|err| OptimizationError::emission_error(
                    format!("Invalid UTF-8 in generated code: {}", err),
                    "utf8_conversion",
                    "String encoding validation"
                ))?;
                
            // Cache the result if caching is enabled
            if opt_config.memory_config.cache_ast_emission {
                context.ast_cache.store(&program, result.clone());
                monitor.increment("cache_stores");
            }
            
            monitor.record_memory("emission");
            result
        }
    };
    
    // Print performance stats if enabled
    if opt_config.enable_performance_monitoring {
        monitor.print_report();
        context.print_stats();
    }

    Ok(ret)
}

/// Enhanced DCE with convergence detection
fn perform_dce_with_convergence(
    program: &mut Program, 
    comments: SingleThreadedComments, 
    unresolved_mark: Mark,
    config: &OptimizationConfig,
    monitor: &mut PerformanceMonitor
) -> OptimizationResult<()> {
    let _timer = monitor.scoped_timer("dce_with_convergence");
    
    let mut visitor = crate::dce::dce(
        comments,
        crate::dce::Config {
            module_mark: None,
            top_level: config.dce_config.top_level,
            top_retain: Default::default(),
            preserve_imports_with_side_effects: config.dce_config.preserve_side_effect_imports,
        },
        unresolved_mark,
    );

    if config.convergence.enable_ast_hashing {
        let mut detector = ConvergenceDetector::new(config.convergence.clone());
        
        for iteration in 1..=config.max_iterations {
            let convergence_result = detector.record_iteration(program);
            
            if config.debug_output {
                println!("DCE iteration {}: {}", iteration, convergence_result.convergence_status);
            }
            
            if !detector.should_continue(config.max_iterations) {
                if config.debug_output {
                    println!("DCE converged: {}", detector.stats());
                }
                break;
            }
            
            program.visit_mut_with(&mut visitor);
            
            if !visitor.changed() {
                break;
            }
            
            visitor.reset();
            monitor.increment("dce_iterations");
        }
    } else {
        // Legacy approach without convergence detection
        for iteration in 1..=config.max_iterations {
            program.visit_mut_with(&mut visitor);

            if !visitor.changed() {
                break;
            }

            visitor.reset();
            monitor.increment("dce_iterations");
        }
    }
    
    Ok(())
}

/// Optimized tree shaking with performance monitoring and error handling
fn perform_webpack_tree_shaking_optimized(
    program: &mut Program, 
    cm: Lrc<SourceMap>, 
    comments: &SingleThreadedComments, 
    config: &serde_json::Value,
    opt_config: &OptimizationConfig,
    context: &mut OptimizationContext,
    monitor: &mut PerformanceMonitor
) -> OptimizationResult<()> {
    let _timer = monitor.scoped_timer("webpack_tree_shaking");
    
    let analyzer = WebpackAnalyzer::new();
    let mut total_removed = 0;
    let mut convergence_detector = if opt_config.convergence.enable_ast_hashing {
        Some(ConvergenceDetector::new(opt_config.convergence.clone()))
    } else {
        None
    };
    
    for iteration in 1..=opt_config.max_iterations {
        // Check convergence if enabled
        if let Some(ref mut detector) = convergence_detector {
            let convergence_result = detector.record_iteration(program);
            
            if opt_config.debug_output {
                println!("Tree shaking iteration {}: {}", iteration, convergence_result.convergence_status);
            }
            
            if !detector.should_continue(opt_config.max_iterations) {
                if opt_config.debug_output {
                    println!("Tree shaking converged: {}", detector.stats());
                }
                break;
            }
        }
        
        // Step 1: Emit current AST to string for analysis
        let current_code = emit_program_to_string(program, cm.clone(), comments, context, monitor)?;

        // Step 2: Analyze the chunk using webpack_analyzer_v2
        let characteristics = get_chunk_characteristics(config, &current_code);
        let mut chunk = analyzer.analyze_chunk(&current_code, characteristics)
            .map_err(|e| {
                if iteration == 1 {
                    // First iteration failure might not be an error - could be non-webpack bundle
                    return OptimizationError::analysis_error(
                        format!("Not a webpack bundle or analysis failed: {}", e),
                        "unknown",
                        "First iteration analysis"
                    );
                } else {
                    return OptimizationError::analysis_error(
                        format!("Webpack analysis failed: {}", e),
                        "webpack",
                        format!("Iteration {}", iteration)
                    );
                }
            })?;

        // Step 2.5: Update the chunk's source to reflect the current code
        chunk.source = current_code.clone();
        
        // Step 2.6: Rebuild dependency graph after macro processing changes
        if iteration > 1 {
            if let Err(e) = analyzer.rebuild_dependency_graph(&mut chunk) {
                if opt_config.debug_output {
                    println!("Warning: Failed to rebuild dependency graph: {}", e);
                }
            }
        }
        
        // Debug logging
        if opt_config.debug_output && iteration == 1 && chunk.modules.len() > 0 {
            println!("Tree shaking debug: Chunk has {} modules", chunk.modules.len());
        }

        let unreachable_modules = compute_unreachable_modules(&chunk, config, opt_config, iteration)?;
        
        if unreachable_modules.is_empty() {
            // Check if this is the first iteration of a macro-enabled split chunk
            let should_continue_for_macros = iteration == 1 && 
                is_split_chunk(&chunk) && 
                has_macro_processing_config(config);
                
            if should_continue_for_macros {
                // Continue to next iteration
                continue;
            } else {
                // No more modules to remove - convergence reached
                if opt_config.debug_output {
                    if iteration == 1 {
                        if is_split_chunk(&chunk) {
                            println!("Tree shaking: Split chunk detected - preserving all modules (no tree shaking)");
                        } else {
                            println!("Tree shaking: No unreachable modules found on first pass");
                        }
                    } else {
                        println!("Tree shaking: Converged after {} iterations, removed {} total modules", 
                                 iteration - 1, total_removed);
                    }
                }
                break;
            }
        }

        // Only remove modules if we have any to remove
        if !unreachable_modules.is_empty() {
            total_removed += unreachable_modules.len();
            
            // Step 4: Remove unreachable modules from the AST
            let unreachable_set: FxHashSet<String> = unreachable_modules
                .into_iter()
                .map(|atom| atom.to_string())
                .collect();
            let mut module_remover = WebpackModuleRemover::new(unreachable_set);
            program.visit_mut_with(&mut module_remover);
            
            monitor.add_to_counter("modules_removed", total_removed);
        }
    }
    
    if total_removed > 0 && opt_config.debug_output {
        println!("Tree shaking: Total removed {} modules across all iterations", total_removed);
    }
    
    Ok(())
}

/// Safe wrapper for simple orphan removal with error handling
fn perform_simple_orphan_removal_safe(
    program: &mut Program, 
    cm: Lrc<SourceMap>, 
    comments: &SingleThreadedComments, 
    config: &serde_json::Value,
    monitor: &mut PerformanceMonitor
) -> OptimizationResult<()> {
    let _timer = monitor.scoped_timer("simple_orphan_removal");
    let mut temp_context = OptimizationContext::new(OptimizationConfig::default());
    
    // Emit the current AST to string for regex-based processing
    let current_code = emit_program_to_string(program, cm.clone(), comments, &mut temp_context, monitor)?;
    
    // Check if this is a CommonJS chunk with exports.modules
    if !current_code.contains("exports.modules") {
        return Ok(());
    }
    
    // Check if there are macro processing directives that might create orphaned modules
    if !has_macro_processing_config(config) {
        return Ok(());
    }
    
    // Find all module IDs and referenced modules using cached regexes
    let mut all_modules = temp_context.collection_pool.get_string_vec();
    let mut referenced_modules = temp_context.collection_pool.get_hash_set();
    
    // Use cached regex patterns
    for cap in MODULE_PATTERN_STR.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            all_modules.push(module_id.as_str().to_string());
        }
    }
    for cap in MODULE_PATTERN_NUM.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            all_modules.push(module_id.as_str().to_string());
        }
    }
    
    for cap in REQUIRE_PATTERN_STR.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            referenced_modules.insert(module_id.as_str().to_string());
        }
    }
    for cap in REQUIRE_PATTERN_NUM.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            referenced_modules.insert(module_id.as_str().to_string());
        }
    }
    
    // Find orphaned modules
    let mut orphaned_modules = temp_context.collection_pool.get_string_vec();
    for module_id in &all_modules {
        if !referenced_modules.contains(module_id) {
            orphaned_modules.push(module_id.clone());
        }
    }
    
    if orphaned_modules.is_empty() {
        // Return collections to pool
        temp_context.collection_pool.return_string_vec(all_modules);
        temp_context.collection_pool.return_hash_set(referenced_modules);
        temp_context.collection_pool.return_string_vec(orphaned_modules);
        return Ok(());
    }
    
    // Remove orphaned modules from the code using regex
    let mut modified_code = current_code.clone();
    let mut removed_count = 0;
    
    for module_id in &orphaned_modules {
        let string_pattern = format!(
            r#""{}"\s*:\s*\n(?:/\*![^*]*\*+(?:[^/*][^*]*\*+)*/\s*\n)?\(function[\s\S]*?\n\}}\),"#,
            regex::escape(module_id)
        );
        let numeric_pattern = format!(
            r#"(?m)^\s*{}\s*:\s*\n(?:/\*![^*]*\*+(?:[^/*][^*]*\*+)*/\s*\n)?\(function[\s\S]*?\n\}}\),"#,
            regex::escape(module_id)
        );
        
        if let Ok(re_str) = Regex::new(&string_pattern) {
            if re_str.is_match(&modified_code) {
                modified_code = re_str.replace(&modified_code, "").to_string();
                removed_count += 1;
                continue;
            }
        }
        
        if let Ok(re_num) = Regex::new(&numeric_pattern) {
            if re_num.is_match(&modified_code) {
                modified_code = re_num.replace(&modified_code, "").to_string();
                removed_count += 1;
            }
        }
    }
    
    // Return collections to pool
    temp_context.collection_pool.return_string_vec(all_modules);
    temp_context.collection_pool.return_hash_set(referenced_modules);
    temp_context.collection_pool.return_string_vec(orphaned_modules);
    
    if removed_count > 0 {
        monitor.add_to_counter("simple_orphan_removals", removed_count);
        
        // Parse the modified code back to AST
        let cm_new: Lrc<SourceMap> = Default::default();
        let fm = cm_new.new_source_file(FileName::Custom("optimized.js".to_string()).into(), modified_code);
        let comments_new = SingleThreadedComments::default();
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments_new),
        );
        
        let new_program = parser.parse_program()
            .map_err(|err| OptimizationError::parse_error(
                format!("Failed to parse modified code after orphan removal: {:?}", err),
                "orphan_removal_reparse"
            ))?;
        
        *program = new_program;
    }
    
    Ok(())
}

/// Helper function to emit program to string with caching
fn emit_program_to_string(
    program: &Program,
    cm: Lrc<SourceMap>,
    comments: &SingleThreadedComments,
    context: &mut OptimizationContext,
    monitor: &mut PerformanceMonitor
) -> OptimizationResult<String> {
    // Try cache first
    if let Some(cached) = context.ast_cache.get(program) {
        monitor.increment("emission_cache_hits");
        return Ok(cached);
    }
    
    let mut buf = vec![];
    let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
        as Box<dyn WriteJs>;
    let mut emitter = Emitter {
        cfg: codegen::Config::default().with_minify(false),
        comments: Some(comments),
        cm: cm.clone(),
        wr,
    };
    
    emitter.emit_program(program)
        .map_err(|err| OptimizationError::emission_error(
            format!("Failed to emit program during tree shaking: {:?}", err),
            "tree_shaking_emission",
            "AST to string conversion for analysis"
        ))?;
    drop(emitter);
    
    let result = String::from_utf8(buf)
        .map_err(|err| OptimizationError::emission_error(
            format!("Invalid UTF-8 in emitted code: {}", err),
            "utf8_conversion",
            "Emitted code encoding validation"
        ))?;
    
    // Cache the result
    if context.config.memory_config.cache_ast_emission {
        context.ast_cache.store(program, result.clone());
        monitor.increment("emission_cache_stores");
    }
    
    Ok(result)
}

/// Count webpack modules in the AST for performance metrics
fn count_webpack_modules(program: &Program) -> usize {
    let mut count = 0;
    
    // This is a simplified count - in practice you'd traverse the AST
    // to count actual webpack module objects
    match program {
        Program::Module(module) => count = module.body.len(),
        Program::Script(script) => count = script.body.len(),
    }
    
    count
}

/// Helper functions that maintain the same interface as the original code
fn get_chunk_characteristics(config: &serde_json::Value, source: &str) -> ChunkCharacteristics {
    // Implementation stays the same as original
    if let Some(chars_value) = config.get("chunk_characteristics") {
        if let Ok(characteristics) = serde_json::from_value::<ChunkCharacteristics>(chars_value.clone()) {
            return characteristics;
        }
    }
    
    if let Some(tree_shake_config) = config.get("treeShake") {
        if let Some(tree_shake_obj) = tree_shake_config.as_object() {
            for (_, module_config) in tree_shake_obj {
                if let Some(module_obj) = module_config.as_object() {
                    if let Some(chars_value) = module_obj.get("chunk_characteristics") {
                        if let Ok(characteristics) = serde_json::from_value::<ChunkCharacteristics>(chars_value.clone()) {
                            return characteristics;
                        }
                    }
                }
            }
        }
    }
    
    let chunk_format = if source.contains("exports.modules") {
        "require".to_string()
    } else if source.contains("__webpack_modules__") {
        "webpack".to_string()
    } else if source.contains(".push([") {
        "jsonp".to_string()
    } else {
        "jsonp".to_string()
    };
    
    ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
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

fn has_macro_processing_config(config: &serde_json::Value) -> bool {
    config.get("treeShake").is_some() ||
    config.get("entryModules").is_some()
}

fn is_split_chunk(chunk: &WebpackChunk) -> bool {
    match chunk.chunk_type {
        ChunkType::CommonJSAsync | ChunkType::CommonJSSync => true,
        ChunkType::JSONP => true,
        ChunkType::ESModules => true,
        ChunkType::WebpackModules => false,
        ChunkType::Unknown => true,
    }
}

fn compute_unreachable_modules(
    chunk: &WebpackChunk, 
    config: &serde_json::Value,
    opt_config: &OptimizationConfig,
    iteration: usize
) -> OptimizationResult<Vec<Atom>> {
    let is_split = is_split_chunk(chunk);
    
    if is_split {
        let has_macro_config = has_macro_processing_config(config);
        
        if has_macro_config {
            let mut entry_points = Vec::new();
            
            // Get explicit entry modules from config
            if let Some(entry_modules) = config.get("entryModules") {
                if let Some(entry_obj) = entry_modules.as_object() {
                    for (_, entry_module) in entry_obj {
                        if let Some(entry_str) = entry_module.as_str() {
                            let entry_atom = Atom::from(entry_str);
                            if chunk.modules.contains_key(&entry_atom) {
                                entry_points.push(entry_atom);
                            }
                        }
                    }
                }
            }
            
            // Add modules with preserved exports as entry points
            if let Some(tree_shake_config) = config.get("treeShake") {
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
                                        
                                        if module_str.contains(package_name) {
                                            let matches_export = 
                                                module_str.ends_with(&format!("/{}.js", export_name)) ||
                                                module_str.contains(&format!("/{}/", export_name)) ||
                                                module_str.ends_with(&format!("/{}/index.js", export_name));
                                            
                                            if matches_export && !entry_points.contains(module_id) {
                                                entry_points.push(module_id.clone());
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            
            if entry_points.is_empty() {
                Ok(Vec::new())
            } else if iteration == 1 {
                Ok(Vec::new()) // First iteration for macro processing
            } else {
                Ok(compute_unreachable_modules_from_entries(chunk, &entry_points))
            }
        } else {
            Ok(Vec::new())
        }
    } else {
        // Standard bundle logic
        let mut entry_points: Vec<Atom> = Vec::new();
        
        if let Some(entry_modules) = config.get("entryModules") {
            if let Some(entry_obj) = entry_modules.as_object() {
                for (_, entry_module) in entry_obj {
                    if let Some(entry_str) = entry_module.as_str() {
                        let entry_atom = Atom::from(entry_str);
                        if chunk.modules.contains_key(&entry_atom) { 
                            entry_points.push(entry_atom); 
                        }
                    }
                }
            }
        }
        
        if entry_points.is_empty() {
            if let Some(chars) = &chunk.characteristics {
                if let Some(entry_name) = &chars.entry_name {
                    let atom = Atom::from(entry_name.as_str());
                    if chunk.modules.contains_key(&atom) { 
                        entry_points.push(atom); 
                    }
                }
            }
        }
        
        if entry_points.is_empty() {
            let extracted = extract_entry_points_from_source(&chunk.source);
            for ep in extracted {
                if chunk.modules.contains_key(&ep) {
                    entry_points.push(ep);
                }
            }
        }

        if entry_points.is_empty() {
            Ok(Vec::new())
        } else {
            Ok(compute_unreachable_modules_from_entries(chunk, &entry_points))
        }
    }
}

fn compute_unreachable_modules_from_entries(chunk: &WebpackChunk, entry_points: &[Atom]) -> Vec<Atom> {
    if entry_points.is_empty() {
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

fn extract_entry_points_from_source(source: &str) -> Vec<Atom> {
    let mut entry_points = Vec::new();
    
    for cap in ENTRY_POINT_REGEX.captures_iter(source) {
        if let Some(module_id) = cap.get(1) {
            let id_str = module_id.as_str().trim();
            let clean_id = if (id_str.starts_with('"') && id_str.ends_with('"')) || 
                             (id_str.starts_with('\'') && id_str.ends_with('\'')) {
                &id_str[1..id_str.len()-1]
            } else {
                id_str
            };
            
            if let Some(full_match) = cap.get(0) {
                if !source[..full_match.start()].ends_with("__webpack_require__.d(") &&
                   !source[..full_match.start()].ends_with("__webpack_require__.r(") {
                    entry_points.push(Atom::from(clean_id));
                }
            }
        }
    }
    
    entry_points
}

/// WebpackModuleRemover implementation (keeping the original logic)
struct WebpackModuleRemover {
    modules_to_remove: FxHashSet<String>,
}

impl WebpackModuleRemover {
    fn new(modules_to_remove: FxHashSet<String>) -> Self {
        Self { modules_to_remove }
    }

    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next().unwrap_or("").to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return false,
                };
                return self.modules_to_remove.contains(&module_id);
            }
        }
        false
    }

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

    fn remove_modules_from_object(&mut self, obj: &mut ObjectLit) {
        obj.props.retain(|prop| !self.should_remove_property(prop));
    }
    
    fn visit_mut_split_chunk_args(&mut self, args: &mut Vec<swc_ecma_ast::ExprOrSpread>) {
        if args.len() >= 1 {
            let swc_ecma_ast::ExprOrSpread { expr, .. } = &mut args[0];
            if let Expr::Array(array) = expr.as_mut() {
                if array.elems.len() >= 2 {
                    if let Some(Some(swc_ecma_ast::ExprOrSpread { expr: modules_expr, .. })) = array.elems.get_mut(1) {
                        if let Expr::Object(obj) = modules_expr.as_mut() {
                            self.remove_modules_from_object(obj);
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for WebpackModuleRemover {
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        for declarator in &mut node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &mut declarator.init {
                        self.remove_modules_from_expr(init);
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }
    
    fn visit_mut_call_expr(&mut self, node: &mut swc_ecma_ast::CallExpr) {
        if let swc_ecma_ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                if let swc_ecma_ast::MemberProp::Ident(ident) = &member.prop {
                    if ident.sym == "push" {
                        self.visit_mut_split_chunk_args(&mut node.args);
                    }
                }
            }
        }
        node.visit_mut_children_with(self);
    }

    fn visit_mut_assign_expr(&mut self, node: &mut swc_ecma_ast::AssignExpr) {
        match &node.left {
            swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Ident(ident)) => {
                if ident.sym == "__webpack_modules__" {
                    self.remove_modules_from_expr(&mut node.right);
                }
            }
            swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Member(member)) => {
                if let swc_ecma_ast::MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "modules" {
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
        node.visit_mut_children_with(self);
    }
}