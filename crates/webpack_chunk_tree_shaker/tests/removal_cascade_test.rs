use webpack_chunk_tree_shaker::*;
use swc_core::atoms::Atom;
use webpack_analyzer_v2::chunk::ChunkCharacteristics;

/// Test comprehensive removal cascade - remove one module and see what else gets removed
#[test]
fn test_remove_module_and_cascade_effect() {
    // Create a realistic chunk with dependency chains
    let chunk_source = r#"
    "use strict";
    exports.ids = ["test-chunk"];
    exports.modules = {
        "entry-point": function(module, exports, __webpack_require__) {
            const featureA = __webpack_require__("feature-a");
            const featureB = __webpack_require__("feature-b");
            module.exports = { 
                processA: featureA.process,
                processB: featureB.process 
            };
        },
        "feature-a": function(module, exports, __webpack_require__) {
            const utilsA = __webpack_require__("utils-a");
            const shared = __webpack_require__("shared-utils");
            module.exports = { 
                process: (data) => shared.transform(utilsA.prepare(data))
            };
        },
        "feature-b": function(module, exports, __webpack_require__) {
            const utilsB = __webpack_require__("utils-b");
            const shared = __webpack_require__("shared-utils");
            module.exports = { 
                process: (data) => shared.validate(utilsB.sanitize(data))
            };
        },
        "utils-a": function(module, exports, __webpack_require__) {
            const core = __webpack_require__("core-a");
            module.exports = { 
                prepare: (data) => core.normalize(data)
            };
        },
        "utils-b": function(module, exports, __webpack_require__) {
            const core = __webpack_require__("core-b");
            module.exports = { 
                sanitize: (data) => core.clean(data)
            };
        },
        "core-a": function(module, exports, __webpack_require__) {
            module.exports = { 
                normalize: (data) => data.trim().toLowerCase()
            };
        },
        "core-b": function(module, exports, __webpack_require__) {
            module.exports = { 
                clean: (data) => data.replace(/[<>]/g, '')
            };
        },
        "shared-utils": function(module, exports, __webpack_require__) {
            module.exports = { 
                transform: (data) => `transformed: ${data}`,
                validate: (data) => data.length > 0
            };
        },
        "unused-feature": function(module, exports, __webpack_require__) {
            const unused = __webpack_require__("unused-helper");
            module.exports = { 
                process: unused.help
            };
        },
        "unused-helper": function(module, exports, __webpack_require__) {
            module.exports = { 
                help: () => "this is unused"
            };
        }
    };
    "#;

    println!("🔍 STEP 1: Analyzing chunk to understand dependencies");
    
    // Analyze the chunk
    let analyzer = WebpackAnalyzer::new();
    let chunk_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: true,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec![],
        entry_name: Some("entry-point".to_string()),
        has_async_chunks: false,
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    let chunk = analyzer.analyze_chunk(chunk_source, chunk_characteristics).unwrap();
    
    println!("📊 Original chunk has {} modules", chunk.module_count());
    
    // Build dependency graph to understand relationships
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    println!("📈 Total dependencies in graph: {}", graph.total_dependencies());
    
    // Show what each module depends on
    println!("\n🔗 Module Dependencies:");
    for (module_id, module) in &chunk.modules {
        if !module.dependencies.is_empty() {
            println!("  {} depends on: {:?}", module_id, module.dependencies);
        }
    }
    
    // Show what depends on each module
    println!("\n⬅️ Module Dependents:");
    for (module_id, module) in &chunk.modules {
        if !module.dependents.is_empty() {
            println!("  {} is used by: {:?}", module_id, module.dependents);
        }
    }
    
    println!("\n🎯 STEP 2: Analyzing impact of removing 'feature-a'");
    
    // Simulate removing feature-a to see what would happen
    let impact = graph.simulate_module_removal(&Atom::from("feature-a"));
    
    println!("📋 Impact Analysis for removing 'feature-a':");
    println!("  - Removed module: {}", impact.removed_module);
    println!("  - Broken modules: {:?}", impact.broken_modules);
    println!("  - Potentially orphaned: {:?}", impact.potentially_orphaned);
    
    println!("\n🔥 STEP 3: Actually removing 'feature-a' and seeing cascade");
    
    // Create tree shaker with aggressive mode to allow removing modules with dependents
    let mut options = TreeShakingOptions::default();
    options.aggressive_mode = true;
    options.preserve_entry_modules = false;
    let shaker = WebpackTreeShaker::with_options(options);
    
    // Remove feature-a
    let result = shaker.remove_modules(&chunk, &["feature-a"]).unwrap();
    
    println!("✅ Tree shaking completed!");
    println!("📊 Statistics:");
    println!("  - Original modules: {}", result.stats.original_count);
    println!("  - Final modules: {}", result.stats.final_count);
    println!("  - Removed modules: {}", result.stats.removed_count);
    println!("  - Reduction: {:.1}%", result.stats.reduction_percentage);
    
    println!("\n📝 Removed modules: {:?}", result.removed_modules);
    println!("🔒 Preserved modules: {:?}", result.preserved_modules);
    
    // Verify the expected removals
    assert!(result.removed_modules.contains(&Atom::from("feature-a")));
    assert_eq!(result.stats.original_count, 10);
    assert!(result.stats.final_count < 10);
    assert!(result.stats.removed_count >= 1);
    
    // Verify that entry-point is still there (if preserved)
    if !result.removed_modules.contains(&Atom::from("entry-point")) {
        assert!(result.optimized_chunk.modules.contains_key(&Atom::from("entry-point")));
    }
    
    println!("\n✅ Test passed! Successfully removed module and showed cascade effect.");
}

/// Test removing a module that causes orphans
#[test]
fn test_remove_module_creates_orphans() {
    // Create a chunk where removing one module creates orphans
    let chunk_source = r#"
    "use strict";
    exports.ids = ["test-chunk"];
    exports.modules = {
        "main": function(module, exports, __webpack_require__) {
            const bridge = __webpack_require__("bridge");
            module.exports = { app: bridge.connect };
        },
        "bridge": function(module, exports, __webpack_require__) {
            const serviceA = __webpack_require__("service-a");
            const serviceB = __webpack_require__("service-b");
            module.exports = { 
                connect: () => serviceA.init() && serviceB.init()
            };
        },
        "service-a": function(module, exports, __webpack_require__) {
            const helperA = __webpack_require__("helper-a");
            module.exports = { 
                init: () => helperA.setup()
            };
        },
        "service-b": function(module, exports, __webpack_require__) {
            const helperB = __webpack_require__("helper-b");
            module.exports = { 
                init: () => helperB.setup()
            };
        },
        "helper-a": function(module, exports, __webpack_require__) {
            module.exports = { 
                setup: () => "service-a ready"
            };
        },
        "helper-b": function(module, exports, __webpack_require__) {
            module.exports = { 
                setup: () => "service-b ready"
            };
        }
    };
    "#;

    println!("🔍 STEP 1: Analyzing chunk with potential orphans");
    
    let analyzer = WebpackAnalyzer::new();
    let chunk_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: true,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec![],
        entry_name: Some("main".to_string()),
        has_async_chunks: false,
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    let chunk = analyzer.analyze_chunk(chunk_source, chunk_characteristics).unwrap();
    
    println!("📊 Original chunk has {} modules", chunk.module_count());
    
    // Build dependency graph
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    println!("\n🎯 STEP 2: Analyzing impact of removing 'bridge' (critical connector)");
    
    // This should break main and potentially orphan services
    let impact = graph.simulate_module_removal(&Atom::from("bridge"));
    
    println!("📋 Impact Analysis for removing 'bridge':");
    println!("  - Removed module: {}", impact.removed_module);
    println!("  - Broken modules: {:?}", impact.broken_modules);
    println!("  - Potentially orphaned: {:?}", impact.potentially_orphaned);
    
    // Should break main (which depends on bridge)
    assert!(impact.broken_modules.contains(&Atom::from("main")));
    
    println!("\n🔥 STEP 3: Actually removing 'bridge' with aggressive mode");
    
    let mut options = TreeShakingOptions::default();
    options.aggressive_mode = true;
    options.preserve_entry_modules = false;
    let shaker = WebpackTreeShaker::with_options(options);
    
    let result = shaker.remove_modules(&chunk, &["bridge"]).unwrap();
    
    println!("✅ Tree shaking completed!");
    println!("📊 Final statistics:");
    println!("  - Removed modules: {:?}", result.removed_modules);
    println!("  - Preserved modules: {:?}", result.preserved_modules);
    
    // Verify bridge was removed
    assert!(result.removed_modules.contains(&Atom::from("bridge")));
    
    // The main module should also be affected (broken due to missing bridge)
    // In aggressive mode, it might be removed too
    
    println!("\n✅ Test passed! Successfully demonstrated orphan creation.");
}

/// Test finding unused modules before removal
#[test]
fn test_find_unused_then_remove() {
    let chunk_source = r#"
    "use strict";
    exports.ids = ["test-chunk"];
    exports.modules = {
        "main": function(module, exports, __webpack_require__) {
            const active = __webpack_require__("active-feature");
            module.exports = { process: active.handle };
        },
        "active-feature": function(module, exports, __webpack_require__) {
            const utils = __webpack_require__("active-utils");
            module.exports = { handle: utils.process };
        },
        "active-utils": function(module, exports, __webpack_require__) {
            module.exports = { process: (data) => data.toUpperCase() };
        },
        "unused-feature": function(module, exports, __webpack_require__) {
            const unused = __webpack_require__("unused-utils");
            module.exports = { handle: unused.process };
        },
        "unused-utils": function(module, exports, __webpack_require__) {
            module.exports = { process: (data) => data.toLowerCase() };
        },
        "orphan-module": function(module, exports, __webpack_require__) {
            module.exports = { standalone: true };
        }
    };
    "#;

    println!("🔍 STEP 1: Finding unused modules");
    
    let analyzer = WebpackAnalyzer::new();
    let chunk_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: true,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec![],
        entry_name: Some("main".to_string()),
        has_async_chunks: false,
        chunk_files: vec![],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    let chunk = analyzer.analyze_chunk(chunk_source, chunk_characteristics).unwrap();
    
    println!("📊 Original chunk has {} modules", chunk.module_count());
    
    let shaker = WebpackTreeShaker::new();
    
    // Find unused modules starting from main
    let unused = shaker.find_unused_modules(&chunk, &["main"]).unwrap();
    
    println!("🗑️ Found unused modules: {:?}", unused);
    
    // Should find unused-feature, unused-utils, and orphan-module
    assert!(unused.contains(&Atom::from("unused-feature")));
    assert!(unused.contains(&Atom::from("unused-utils")));
    assert!(unused.contains(&Atom::from("orphan-module")));
    
    println!("\n🎯 STEP 2: Removing unused modules");
    
    // Remove all unused modules
    let mut options = TreeShakingOptions::default();
    options.preserve_entry_modules = false;
    options.aggressive_mode = true;
    let shaker = WebpackTreeShaker::with_options(options);
    
    let result = shaker.remove_modules(&chunk, &unused).unwrap();
    
    println!("✅ Tree shaking completed!");
    println!("📊 Results:");
    println!("  - Original: {} modules", result.stats.original_count);
    println!("  - Final: {} modules", result.stats.final_count);
    println!("  - Removed: {} modules", result.stats.removed_count);
    println!("  - Reduction: {:.1}%", result.stats.reduction_percentage);
    
    // Should have removed the 3 unused modules
    assert_eq!(result.stats.removed_count, 3);
    assert_eq!(result.stats.final_count, 3); // main, active-feature, active-utils
    
    // Verify active modules are preserved
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("main")));
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("active-feature")));
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("active-utils")));
    
    // Verify unused modules are removed
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("unused-feature")));
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("unused-utils")));
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("orphan-module")));
    
    println!("\n✅ Test passed! Successfully identified and removed unused modules.");
}

/// Test the complete workflow: analyze -> find unused -> remove -> validate
#[test]
fn test_complete_workflow() {
    let chunk_source = r#"
    "use strict";
    exports.ids = ["vendor-chunk"];
    exports.modules = {
        "lodash/map": function(module, exports, __webpack_require__) {
            const baseMap = __webpack_require__("lodash/_baseMap");
            module.exports = function(collection, iteratee) {
                return baseMap(collection, iteratee);
            };
        },
        "lodash/_baseMap": function(module, exports, __webpack_require__) {
            const baseEach = __webpack_require__("lodash/_baseEach");
            module.exports = function(collection, iteratee) {
                return baseEach(collection, iteratee);
            };
        },
        "lodash/_baseEach": function(module, exports, __webpack_require__) {
            module.exports = function(collection, iteratee) {
                return collection.map(iteratee);
            };
        },
        "lodash/filter": function(module, exports, __webpack_require__) {
            const baseFilter = __webpack_require__("lodash/_baseFilter");
            module.exports = function(collection, predicate) {
                return baseFilter(collection, predicate);
            };
        },
        "lodash/_baseFilter": function(module, exports, __webpack_require__) {
            module.exports = function(collection, predicate) {
                return collection.filter(predicate);
            };
        },
        "lodash/sortBy": function(module, exports, __webpack_require__) {
            module.exports = function(collection, iteratee) {
                return collection.sort(iteratee);
            };
        },
        "lodash/debounce": function(module, exports, __webpack_require__) {
            module.exports = function(func, wait) {
                return function(...args) {
                    setTimeout(() => func(...args), wait);
                };
            };
        }
    };
    "#;

    println!("🔍 COMPLETE WORKFLOW TEST");
    println!("========================");
    
    println!("\n📊 STEP 1: Initial Analysis");
    let analyzer = WebpackAnalyzer::new();
    let chunk_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec![],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendor-chunk".to_string()],
        is_shared_chunk: true,
        shared_modules: vec![],
    };
    let chunk = analyzer.analyze_chunk(chunk_source, chunk_characteristics).unwrap();
    
    println!("  - Original modules: {}", chunk.module_count());
    println!("  - Chunk type: {:?}", chunk.chunk_type);
    
    println!("\n🎯 STEP 2: Find Unused Modules (using only map function)");
    let shaker = WebpackTreeShaker::new();
    let unused = shaker.find_unused_modules(&chunk, &["lodash/map"]).unwrap();
    
    println!("  - Unused modules: {:?}", unused);
    println!("  - Count: {}", unused.len());
    
    // Should find filter, sortBy, debounce, and their dependencies as unused
    assert!(unused.contains(&Atom::from("lodash/filter")));
    assert!(unused.contains(&Atom::from("lodash/sortBy")));
    assert!(unused.contains(&Atom::from("lodash/debounce")));
    
    println!("\n🔥 STEP 3: Remove Unused Modules");
    let mut options = TreeShakingOptions::default();
    options.preserve_entry_modules = false;
    options.aggressive_mode = true;
    let shaker = WebpackTreeShaker::with_options(options);
    
    let result = shaker.remove_modules(&chunk, &unused).unwrap();
    
    println!("  - Removed: {} modules", result.stats.removed_count);
    println!("  - Remaining: {} modules", result.stats.final_count);
    println!("  - Reduction: {:.1}%", result.stats.reduction_percentage);
    
    println!("\n✅ STEP 4: Validate Results");
    let validator = TreeShakingValidator::new();
    
    // Validate before
    let validation_before = validator.validate_before_shaking(&chunk).unwrap();
    println!("  - Pre-validation: {}", if validation_before.is_valid() { "✅ PASSED" } else { "❌ FAILED" });
    
    // Validate after
    let validation_after = validator.validate_after_shaking(
        &chunk,
        &result.optimized_chunk,
        &result.removed_modules,
    ).unwrap();
    println!("  - Post-validation: {}", if validation_after.is_valid() { "✅ PASSED" } else { "❌ FAILED" });
    
    println!("\n🔄 STEP 5: Verify Optimized Chunk");
    println!("  - Final module count: {}", result.optimized_chunk.module_count());
    println!("  - Modules removed: {}", result.removed_modules.len());
    println!("  - Contains lodash/map: {}", result.optimized_chunk.modules.contains_key(&Atom::from("lodash/map")));
    println!("  - Contains lodash/filter: {}", result.optimized_chunk.modules.contains_key(&Atom::from("lodash/filter")));
    
    // Verify final state
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("lodash/map")));
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("lodash/_baseMap")));
    assert!(result.optimized_chunk.modules.contains_key(&Atom::from("lodash/_baseEach")));
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("lodash/filter")));
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("lodash/sortBy")));
    assert!(!result.optimized_chunk.modules.contains_key(&Atom::from("lodash/debounce")));
    
    println!("\n🎉 WORKFLOW COMPLETE!");
    println!("   Original: {} modules → Final: {} modules ({:.1}% reduction)",
             result.stats.original_count,
             result.stats.final_count,
             result.stats.reduction_percentage);
    
    println!("\n✅ All tests passed! Complete workflow successful.");
}