use serde_json::json;
use swc_macro_wasm::optimize::optimize;

/// Test that verifies lodash modules are removed when their exports are disabled
/// This tests the complete flow: macro conditions -> DCE -> tree shaking
#[test]
fn test_lodash_module_removal_with_disabled_exports() {
    let source = include_str!("fixtures/module_federation_lodash_chunk.js");
    
    // Count original modules
    let original_module_count = source.matches("function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__)").count();
    
    // Debug: Check what module IDs are in the chunk
    println!("\n=== DEBUG: Checking module IDs in the chunk ===");
    println!("Looking for entry: ../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js");
    println!("Contains lodash.js entry: {}", source.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js""#));
    println!("Contains map.js: {}", source.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js""#));
    println!("Contains filter.js: {}", source.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/filter.js""#));
    println!("Contains sortBy.js: {}", source.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js""#));
    
    // Test with several exports enabled to maintain chunk structure
    let config = json!({
        "treeShake": {
            "lodash-es": {
                // Keep enough exports to maintain chunk structure
                "sortBy": true,
                "uniq": true,
                "debounce": true,
                "throttle": true,
                "merge": true,
                "clone": true,
                "isEqual": true,
                // Explicitly disable these
                "map": false,
                "filter": false,
                "reduce": false,
                "forEach": false,
                "find": false,
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "jsonp",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["test.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        },
        "minify": false
    });

    // Use the native optimize function
    let optimized = optimize(source.to_string(), config).expect("Optimization should succeed");
    
    // Debug: Check optimized output structure
    println!("\n=== DEBUG: Optimized output info ===");
    println!("Optimized size: {} bytes", optimized.len());
    println!("Original was: {} bytes", source.len());
    println!("Reduction: {} bytes", source.len() - optimized.len());
    println!("Still has webpack modules: {}", optimized.contains("__webpack_modules__"));
    println!("Still has JSONP structure: {}", optimized.contains("push([["));
    
    // Check if specific modules still appear as keys
    println!("\n=== Module presence in optimized ===");
    println!("Has map.js key: {}", optimized.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js""#));
    println!("Has sortBy.js key: {}", optimized.contains(r#""../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js""#));
    
    // Show first 500 chars of optimized to see structure
    println!("\n=== First 500 chars of optimized ===");
    println!("{}", &optimized[..500.min(optimized.len())]);
    
    // Count optimized modules
    let optimized_module_count = optimized.matches("function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__)").count();
    let modules_removed = original_module_count.saturating_sub(optimized_module_count);
    
    // Verify that map, filter, reduce modules are removed
    assert!(!optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js"),
            "map.js module should be removed when map export is disabled");
    assert!(!optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/filter.js"),
            "filter.js module should be removed when filter export is disabled");
    assert!(!optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/reduce.js"),
            "reduce.js module should be removed when reduce export is disabled");
    
    // Verify that enabled exports' modules are kept
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js"),
            "sortBy.js module should be kept when sortBy export is enabled");
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/uniq.js"),
            "uniq.js module should be kept when uniq export is enabled");
    
    // Verify metrics show module removal
    assert!(modules_removed > 0, "Should have removed some modules");
    println!("Removed {} modules", modules_removed);
}

#[test]
fn test_lodash_module_removal_with_default_false() {
    let source = include_str!("fixtures/module_federation_lodash_chunk.js");
    
    // Test with default export disabled - this should remove the main lodash re-export logic
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "map": false,
                "filter": false,
                "reduce": false,
                "default": false, // Disable default export
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "jsonp",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["test.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        },
        "minify": false
    });

    let optimized = optimize(source.to_string(), config).expect("Optimization should succeed");
    
    // When default is false, the main lodash wrapper logic should be removed
    // but individual enabled exports should still work
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/sortBy.js"),
            "sortBy.js should still be present when explicitly enabled");
    assert!(optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/uniq.js"),
            "uniq.js should still be present when explicitly enabled");
    
    // Disabled exports should still be removed
    assert!(!optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js"),
            "map.js should be removed");
    assert!(!optimized.contains("../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/filter.js"),
            "filter.js should be removed");
}

#[test]
fn test_deep_dependency_removal() {
    let source = include_str!("fixtures/module_federation_lodash_chunk.js");
    
    // Count original modules
    let original_module_count = source.matches("function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__)").count();
    
    // Test that dependencies of removed modules are also removed
    let config = json!({
        "treeShake": {
            "lodash-es": {
                // Only enable a few functions
                "debounce": true,
                "throttle": true,
                "default": true,
                // Disable everything else
                "map": false,
                "filter": false,
                "reduce": false,
                "forEach": false,
                "find": false,
                "findIndex": false,
                "sortBy": false,
                "groupBy": false,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "jsonp",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["test.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        },
        "minify": false
    });

    let optimized = optimize(source.to_string(), config).expect("Optimization should succeed");
    
    // Count modules after optimization
    let optimized_module_count = optimized.matches("function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__)").count();
    let modules_removed = original_module_count.saturating_sub(optimized_module_count);
    
    println!("Original modules: {}, Optimized modules: {}, Removed: {}", 
             original_module_count, optimized_module_count, modules_removed);
    
    // Should have removed a significant number of modules
    assert!(modules_removed > 100, 
            "Should remove many modules when most exports are disabled. Removed: {}", modules_removed);
    assert!(optimized_module_count < original_module_count / 2,
            "Should have less than half the original modules when most exports are disabled");
}

#[test]
fn test_module_removal_with_macro_conditions() {
    // Test a simple chunk with macro conditions
    let source = r#"
(self["webpackChunktest"] = self["webpackChunktest"] || []).push([["chunk"], {
    "./entry.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        
        /* @common:if [condition="treeShake.mylib.featureA"] */
        var featureA = __webpack_require__("./featureA.js");
        /* @common:endif */
        
        /* @common:if [condition="treeShake.mylib.featureB"] */
        var featureB = __webpack_require__("./featureB.js");
        /* @common:endif */
        
        /* @common:if [condition="treeShake.mylib.featureC"] */
        var featureC = __webpack_require__("./featureC.js");
        /* @common:endif */
    },
    "./featureA.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        var helper = __webpack_require__("./helperA.js");
        __webpack_exports__["default"] = function() { return "Feature A"; };
    },
    "./featureB.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        var helper = __webpack_require__("./helperB.js");
        __webpack_exports__["default"] = function() { return "Feature B"; };
    },
    "./featureC.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_exports__["default"] = function() { return "Feature C"; };
    },
    "./helperA.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_exports__["default"] = function() { return "Helper A"; };
    },
    "./helperB.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_exports__["default"] = function() { return "Helper B"; };
    }
}]);
"#;

    let config = json!({
        "treeShake": {
            "mylib": {
                "featureA": true,
                "featureB": false,
                "featureC": false,
                "chunk_characteristics": {
                    "entry_module_id": "./entry.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "jsonp",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["chunk.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        },
        "minify": false
    });

    let optimized = optimize(source.to_string(), config).expect("Optimization should succeed");
    
    // featureA and helperA should be kept
    assert!(optimized.contains(r#""./featureA.js":"#), "featureA should be kept");
    assert!(optimized.contains(r#""./helperA.js":"#), "helperA should be kept");
    
    // featureB, featureC, and helperB should be removed
    assert!(!optimized.contains(r#""./featureB.js":"#), "featureB should be removed");
    assert!(!optimized.contains(r#""./featureC.js":"#), "featureC should be removed");
    assert!(!optimized.contains(r#""./helperB.js":"#), "helperB should be removed");
    
    // Verify module count
    let original_count = 6; // entry + 3 features + 2 helpers
    let optimized_count = optimized.matches("function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__)").count();
    let expected_kept = 3; // entry + featureA + helperA
    assert_eq!(optimized_count, expected_kept,
               "Should have {} modules remaining", expected_kept);
}