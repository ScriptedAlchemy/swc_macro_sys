use std::fs;
use std::path::Path;
use swc_macro_wasm::optimize;

#[test]
fn test_webpack_tree_shaker_on_standard_webpack_chunk() {
    println!("\n=== TESTING WEBPACK TREE SHAKER ON STANDARD WEBPACK CHUNK ===");
    
    let standard_chunk_path = Path::new("../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    if !standard_chunk_path.exists() {
        println!("⚠️  Standard webpack chunk not found: {}", standard_chunk_path.display());
        return;
    }
    
    let original_code = fs::read_to_string(standard_chunk_path).expect("Failed to read standard chunk");
    let original_size = original_code.len();
    
    println!("Original standard chunk size: {} bytes ({:.2} KB)", 
        original_size, original_size as f64 / 1024.0);
    
    // Test with minimal tree shaking config (only keep 'default' export)
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true,
                "map": false,
                "filter": false,
                "VERSION": false
            }
        }
    });
    
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    
    println!("Optimized standard chunk size: {} bytes ({:.2} KB)", 
        optimized_size, optimized_size as f64 / 1024.0);
    println!("Size reduction: {:.2}% ({} bytes saved)", 
        reduction, original_size - optimized_size);
    
    // Standard webpack chunks should achieve high reduction rates
    assert!(reduction > 50.0, 
        "Standard webpack chunk should achieve >50% reduction, got {:.2}%", reduction);
    
    // Check that the optimized code still contains webpack runtime structure
    assert!(optimized_code.contains("webpackChunk") || optimized_code.contains("__webpack"),
        "Optimized standard chunk should maintain webpack structure");
    
    println!("✅ Standard webpack chunk optimization successful!");
}

#[test]
fn test_webpack_tree_shaker_on_module_federation_chunk() {
    println!("\n=== TESTING WEBPACK TREE SHAKER ON MODULE FEDERATION CHUNK ===");
    
    let mf_chunk_path = Path::new("../../module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    
    if !mf_chunk_path.exists() {
        println!("⚠️  Module Federation chunk not found: {}", mf_chunk_path.display());
        println!("Run 'pnpm run build' in module-federation-example first.");
        return;
    }
    
    let original_code = fs::read_to_string(mf_chunk_path).expect("Failed to read MF chunk");
    let original_size = original_code.len();
    
    println!("Original MF chunk size: {} bytes ({:.2} KB)", 
        original_size, original_size as f64 / 1024.0);
    
    // Test with same minimal config as standard webpack test
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true,
                "map": false,
                "filter": false,
                "VERSION": false,
                "uniq": false,
                "sortBy": false,
                "omit": false,
                "capitalize": false,
                "pick": false,
                "groupBy": false,
                "throttle": false,
                "debounce": false
            }
        }
    });
    
    let config_str = serde_json::to_string(&config).expect("Failed to serialize config");
    let optimized_code = optimize(original_code.clone(), &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    
    println!("Optimized MF chunk size: {} bytes ({:.2} KB)", 
        optimized_size, optimized_size as f64 / 1024.0);
    println!("Size reduction: {:.2}% ({} bytes saved)", 
        reduction, original_size - optimized_size);
    
    // Check that the optimized code maintains CommonJS structure
    assert!(optimized_code.contains("exports.modules") || optimized_code.contains("exports.ids"),
        "Optimized MF chunk should maintain CommonJS exports structure");
    
    println!("✅ Module Federation chunk optimization completed!");
    
    // Print analysis of chunk format detection
    analyze_chunk_format(&original_code, "Module Federation");
}

#[test]
fn test_chunk_format_detection_and_parsing() {
    println!("\n=== TESTING CHUNK FORMAT DETECTION AND PARSING ===");
    
    // Test 1: Standard webpack chunk format detection
    let webpack_sample = r#"
        "use strict";
        (self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["vendors"], {
            "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": 
            (function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                console.log("webpack module");
            })
        }]);
    "#;
    
    println!("Testing standard webpack chunk format...");
    analyze_chunk_format(webpack_sample, "Standard Webpack");
    
    // Test 2: Module Federation CommonJS format detection
    let mf_sample = r#"
        "use strict";
        exports.ids = ["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"];
        exports.modules = {
            "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": 
            (function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                console.log("module federation module");
            })
        };
    "#;
    
    println!("Testing Module Federation CommonJS format...");
    analyze_chunk_format(mf_sample, "Module Federation");
    
    // Test 3: Optimization on both formats
    println!("\nTesting optimization on both formats...");
    
    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "default": true
            }
        }
    });
    let config_str = serde_json::to_string(&config).unwrap();
    
    let webpack_optimized = optimize(webpack_sample.to_string(), &config_str);
    let mf_optimized = optimize(mf_sample.to_string(), &config_str);
    
    let webpack_reduction = ((webpack_sample.len() - webpack_optimized.len()) as f64 / webpack_sample.len() as f64) * 100.0;
    let mf_reduction = ((mf_sample.len() - mf_optimized.len()) as f64 / mf_sample.len() as f64) * 100.0;
    
    println!("Standard webpack reduction: {:.1}%", webpack_reduction);
    println!("Module Federation reduction: {:.1}%", mf_reduction);
    
    // Both should achieve some reduction
    assert!(webpack_reduction > 0.0, "Standard webpack should achieve some optimization");
    assert!(mf_reduction > 0.0, "Module Federation should achieve some optimization");
    
    println!("✅ Chunk format detection and parsing test completed!");
}

#[test]
fn test_commonjs_exports_module_detection() {
    println!("\n=== TESTING COMMONJS EXPORTS MODULE DETECTION ===");
    
    // Test various CommonJS exports patterns that Module Federation uses
    let test_cases = vec![
        (
            "Basic exports.modules",
            r#"exports.modules = {"module1": function() { console.log("test"); }};"#
        ),
        (
            "exports.ids + exports.modules",
            r#"exports.ids = ["chunk1"]; exports.modules = {"module1": function() {}};"#
        ),
        (
            "Multi-line exports.modules",
            r#"
            exports.modules = {
                "module1": function() { return "test1"; },
                "module2": function() { return "test2"; }
            };
            "#
        ),
        (
            "Nested module paths",
            r#"
            exports.modules = {
                "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/map.js": function() {},
                "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/filter.js": function() {}
            };
            "#
        )
    ];
    
    for (test_name, code) in test_cases {
        println!("Testing: {}", test_name);
        
        let config = serde_json::json!({
            "treeShake": {
                "lodash-es": {
                    "map": true,
                    "filter": false
                }
            }
        });
        let config_str = serde_json::to_string(&config).unwrap();
        
        let original_size = code.len();
        let optimized_code = optimize(code.to_string(), &config_str);
        let optimized_size = optimized_code.len();
        let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
        
        println!("  Original: {} bytes", original_size);
        println!("  Optimized: {} bytes", optimized_size);
        println!("  Reduction: {:.1}%", reduction);
        
        // Should maintain exports.modules structure
        if code.contains("exports.modules") {
            assert!(optimized_code.contains("exports.modules"), 
                "Should maintain exports.modules structure in optimized code");
        }
        
        println!("  ✅ Test passed\n");
    }
    
    println!("✅ CommonJS exports module detection test completed!");
}

#[test]
fn test_module_federation_vs_webpack_chunk_optimization() {
    println!("\n=== COMPARING MODULE FEDERATION VS WEBPACK CHUNK OPTIMIZATION ===");
    
    // Create equivalent chunks in both formats
    let webpack_chunk = r#"
        "use strict";
        (self["webpackChunk"] = self["webpackChunk"] || []).push([["vendors"], {
            "lodash/map": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.map = function(arr, fn) { return arr.map(fn); };
            },
            "lodash/filter": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.filter = function(arr, pred) { return arr.filter(pred); };
            },
            "lodash/unused": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.unused = function() { return "unused"; };
            }
        }]);
    "#;
    
    let mf_chunk = r#"
        "use strict";
        exports.modules = {
            "lodash/map": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.map = function(arr, fn) { return arr.map(fn); };
            },
            "lodash/filter": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.filter = function(arr, pred) { return arr.filter(pred); };
            },
            "lodash/unused": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_exports__.unused = function() { return "unused"; };
            }
        };
    "#;
    
    // Same tree-shaking config: keep map, remove filter and unused
    let config = serde_json::json!({
        "treeShake": {
            "lodash": {
                "map": true,
                "filter": false,
                "unused": false
            }
        }
    });
    let config_str = serde_json::to_string(&config).unwrap();
    
    // Optimize both chunks
    let webpack_optimized = optimize(webpack_chunk.to_string(), &config_str);
    let mf_optimized = optimize(mf_chunk.to_string(), &config_str);
    
    let webpack_reduction = ((webpack_chunk.len() - webpack_optimized.len()) as f64 / webpack_chunk.len() as f64) * 100.0;
    let mf_reduction = ((mf_chunk.len() - mf_optimized.len()) as f64 / mf_chunk.len() as f64) * 100.0;
    
    println!("Webpack chunk:");
    println!("  Original size: {} bytes", webpack_chunk.len());
    println!("  Optimized size: {} bytes", webpack_optimized.len());
    println!("  Reduction: {:.1}%", webpack_reduction);
    
    println!("Module Federation chunk:");
    println!("  Original size: {} bytes", mf_chunk.len());
    println!("  Optimized size: {} bytes", mf_optimized.len());
    println!("  Reduction: {:.1}%", mf_reduction);
    
    // Both should achieve some optimization
    assert!(webpack_reduction > 0.0, "Webpack chunk should be optimized");
    assert!(mf_reduction > 0.0, "Module Federation chunk should be optimized");
    
    // Check that unused modules are removed from both
    assert!(!webpack_optimized.contains("lodash/unused"), 
        "Webpack chunk should not contain unused module");
    assert!(!mf_optimized.contains("lodash/unused"), 
        "Module Federation chunk should not contain unused module");
    
    // Check that used modules are preserved
    assert!(webpack_optimized.contains("lodash/map"), 
        "Webpack chunk should contain map module");
    assert!(mf_optimized.contains("lodash/map"), 
        "Module Federation chunk should contain map module");
    
    let reduction_diff = (webpack_reduction - mf_reduction).abs();
    println!("Reduction difference: {:.1}% points", reduction_diff);
    
    if reduction_diff > 10.0 {
        println!("⚠️  Significant difference in optimization rates detected!");
        println!("This suggests the tree shaker may not be handling both formats equally.");
    } else {
        println!("✅ Both formats show similar optimization rates");
    }
    
    println!("✅ Comparative optimization test completed!");
}

fn analyze_chunk_format(code: &str, format_name: &str) {
    println!("Analyzing {} chunk format:", format_name);
    
    // Check for webpack runtime patterns
    let has_webpack_chunk = code.contains("webpackChunk");
    let has_webpack_push = code.contains(".push(");
    let has_self_ref = code.contains("self[");
    
    // Check for CommonJS patterns
    let has_exports_modules = code.contains("exports.modules");
    let has_exports_ids = code.contains("exports.ids");
    let has_module_exports = code.contains("module.exports");
    
    // Check for module patterns
    let has_webpack_require = code.contains("__webpack_require__");
    let has_module_functions = code.contains("function(__unused_webpack");
    let module_count = code.matches("function(").count();
    
    println!("  Webpack runtime patterns:");
    println!("    webpackChunk: {}", has_webpack_chunk);
    println!("    .push(): {}", has_webpack_push);
    println!("    self[] reference: {}", has_self_ref);
    
    println!("  CommonJS patterns:");
    println!("    exports.modules: {}", has_exports_modules);
    println!("    exports.ids: {}", has_exports_ids);
    println!("    module.exports: {}", has_module_exports);
    
    println!("  Module patterns:");
    println!("    __webpack_require__: {}", has_webpack_require);
    println!("    Module functions: {}", has_module_functions);
    println!("    Estimated module count: {}", module_count);
    
    // Determine format
    if has_webpack_chunk && has_webpack_push {
        println!("  📊 Detected format: Standard Webpack Runtime");
    } else if has_exports_modules {
        println!("  📊 Detected format: CommonJS Exports (Module Federation)");
    } else {
        println!("  📊 Detected format: Unknown/Mixed");
    }
    
    println!("");
}