use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_enhanced_parser_export_definitions() {
    println!("\n=== ENHANCED PARSER: EXPORT DEFINITIONS TEST ===");
    
    // Test webpack_require calls inside __webpack_require__.d() export definitions
    let chunk = r#"
    "use strict";
    (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
        "main.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                utilA: () => (/* @common:if [condition="treeShake.utils.utilA"] */ __webpack_require__("utilA.js").default /* @common:endif */),
                utilB: () => (/* @common:if [condition="treeShake.utils.utilB"] */ __webpack_require__("utilB.js").default /* @common:endif */),
                utilC: () => (/* @common:if [condition="treeShake.utils.utilC"] */ __webpack_require__("utilC.js").default /* @common:endif */)
            });
        },
        "utilA.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.utils.utilA"] */
            const utilA = function() { return "utilA"; };
            const __WEBPACK_DEFAULT_EXPORT__ = utilA;
            /* @common:endif */
        },
        "utilB.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.utils.utilB"] */
            const utilB = function() { return "utilB"; };
            const __WEBPACK_DEFAULT_EXPORT__ = utilB;
            /* @common:endif */
        },
        "utilC.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.utils.utilC"] */
            const utilC = function() { return "utilC"; };
            const __WEBPACK_DEFAULT_EXPORT__ = utilC;
            /* @common:endif */
        }
    }]);
    "#;
    
    println!("Testing complex __webpack_require__ calls inside export definitions");
    println!("Original size: {} bytes", chunk.len());
    
    // Config: only enable utilA and utilC with entry module ID
    let config = json!({
        "treeShake": {
            "utils": {
                "utilA": true,
                "utilB": false,
                "utilC": true
            }
        },
        "entryModules": {
            "lodash-es": "main.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    println!("Optimized size: {} bytes", optimized.len());
    
    // Verify main.js is preserved
    assert!(optimized.contains("main.js"), "main.js should be preserved");
    
    // Verify utilA and utilC are preserved (enabled)
    assert!(optimized.contains("utilA.js"), "utilA.js should be preserved when enabled");
    assert!(optimized.contains("utilC.js"), "utilC.js should be preserved when enabled");
    
    // Verify utilB is removed (disabled)
    assert!(!optimized.contains("utilB.js"), "utilB.js should be removed when disabled");
    
    // NOTE: The macro conditions only remove the __webpack_require__ calls, not the function implementations
    // The key improvement is that the enhanced parser correctly identifies dependencies and removes orphaned modules
    
    // Verify that the key improvement is working - orphaned modules are removed
    let reduction = (chunk.len() - optimized.len()) as f64 / chunk.len() as f64 * 100.0;
    println!("Size reduction: {:.1}%", reduction);
    assert!(reduction > 10.0, "Should achieve meaningful size reduction by removing orphaned modules");
    
    println!("✅ Enhanced parser correctly handles export definitions");
}

#[test]
fn test_enhanced_parser_arrow_functions() {
    println!("\n=== ENHANCED PARSER: ARROW FUNCTIONS TEST ===");
    
    // Test webpack_require calls inside arrow functions
    let chunk = r#"
    "use strict";
    (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
        "main.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                getHelperA: () => /* @common:if [condition="treeShake.helpers.helperA"] */ () => __webpack_require__("helperA.js") /* @common:endif */,
                getHelperB: () => /* @common:if [condition="treeShake.helpers.helperB"] */ () => __webpack_require__("helperB.js") /* @common:endif */
            });
        },
        "helperA.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            /* @common:if [condition="treeShake.helpers.helperA"] */
            __webpack_require__.d(__webpack_exports__, {
                "default": () => "Helper A"
            });
            /* @common:endif */
        },
        "helperB.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            /* @common:if [condition="treeShake.helpers.helperB"] */
            __webpack_require__.d(__webpack_exports__, {
                "default": () => "Helper B"
            });
            /* @common:endif */
        }
    }]);
    "#;
    
    println!("Testing __webpack_require__ calls inside arrow functions");
    println!("Original size: {} bytes", chunk.len());
    
    // Config: only enable helperA with entry module ID
    let config = json!({
        "treeShake": {
            "helpers": {
                "helperA": true,
                "helperB": false
            }
        },
        "entryModules": {
            "lodash-es": "main.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    println!("Optimized size: {} bytes", optimized.len());
    
    // Verify main.js is preserved
    assert!(optimized.contains("main.js"), "main.js should be preserved");
    
    // Verify helperA is preserved (enabled)
    assert!(optimized.contains("helperA.js"), "helperA.js should be preserved when enabled");
    
    // Verify helperB is removed (disabled)
    assert!(!optimized.contains("helperB.js"), "helperB.js should be removed when disabled");
    
    // NOTE: The macro conditions only remove the __webpack_require__ calls, not the function implementations  
    // The key improvement is that the enhanced parser correctly identifies dependencies and removes orphaned modules
    
    // Verify that the key improvement is working - orphaned modules are removed
    let reduction = (chunk.len() - optimized.len()) as f64 / chunk.len() as f64 * 100.0;
    println!("Size reduction: {:.1}%", reduction);
    assert!(reduction > 10.0, "Should achieve meaningful size reduction by removing orphaned modules");
    
    println!("✅ Enhanced parser correctly handles arrow functions");
}

#[test]
fn test_enhanced_parser_nested_structures() {
    println!("\n=== ENHANCED PARSER: NESTED STRUCTURES TEST ===");
    
    // Test webpack_require calls in complex nested structures
    let chunk = r#"
    "use strict";
    (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
        "main.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            /* @common:if [condition="treeShake.features.featureA"] */
            var featureA = __webpack_require__("featureA.js");
            /* @common:endif */
            
            __webpack_require__.d(__webpack_exports__, {
                init: () => (function() {
                    var modules = {};
                    /* @common:if [condition="treeShake.features.featureA"] */
                    modules.featureA = __webpack_require__("featureA.js");
                    /* @common:endif */
                    /* @common:if [condition="treeShake.features.featureB"] */
                    modules.featureB = __webpack_require__("featureB.js");
                    /* @common:endif */
                    return modules;
                })
            });
        },
        "featureA.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            /* @common:if [condition="treeShake.features.featureA"] */
            __webpack_require__.d(__webpack_exports__, {
                "default": () => "Feature A"
            });
            /* @common:endif */
        },
        "featureB.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            /* @common:if [condition="treeShake.features.featureB"] */
            __webpack_require__.d(__webpack_exports__, {
                "default": () => "Feature B"
            });
            /* @common:endif */
        }
    }]);
    "#;
    
    println!("Testing __webpack_require__ calls in nested structures");
    println!("Original size: {} bytes", chunk.len());
    
    // Config: only enable featureA with entry module ID
    let config = json!({
        "treeShake": {
            "features": {
                "featureA": true,
                "featureB": false
            }
        },
        "entryModules": {
            "lodash-es": "main.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    println!("Optimized size: {} bytes", optimized.len());
    
    // Verify main.js is preserved
    assert!(optimized.contains("main.js"), "main.js should be preserved");
    
    // Verify featureA is preserved (enabled)
    assert!(optimized.contains("featureA.js"), "featureA.js should be preserved when enabled");
    
    // Verify featureB is removed (disabled)
    assert!(!optimized.contains("featureB.js"), "featureB.js should be removed when disabled");
    
    // NOTE: The macro conditions only remove the __webpack_require__ calls, not the function implementations
    // The key improvement is that the enhanced parser correctly identifies dependencies and removes orphaned modules
    
    // Verify that the key improvement is working - orphaned modules are removed
    let reduction = (chunk.len() - optimized.len()) as f64 / chunk.len() as f64 * 100.0;
    println!("Size reduction: {:.1}%", reduction);
    assert!(reduction > 10.0, "Should achieve meaningful size reduction by removing orphaned modules");
    
    println!("✅ Enhanced parser correctly handles nested structures");
}

#[test]
fn test_enhanced_parser_performance() {
    println!("\n=== ENHANCED PARSER: PERFORMANCE TEST ===");
    
    // Test with a simpler approach to avoid lifetime issues
    let mut chunk = r#""use strict";
    (self["webpackChunk"] = self["webpackChunk"] || []).push([["test"], {
        "main.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                module0: () => (/* @common:if [condition="treeShake.modules.module0"] */ __webpack_require__("module0.js").default /* @common:endif */),
                module1: () => (/* @common:if [condition="treeShake.modules.module1"] */ __webpack_require__("module1.js").default /* @common:endif */),
                module2: () => (/* @common:if [condition="treeShake.modules.module2"] */ __webpack_require__("module2.js").default /* @common:endif */),
                module3: () => (/* @common:if [condition="treeShake.modules.module3"] */ __webpack_require__("module3.js").default /* @common:endif */),
                module4: () => (/* @common:if [condition="treeShake.modules.module4"] */ __webpack_require__("module4.js").default /* @common:endif */)
            });
        },
        "module0.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.modules.module0"] */
            const module0 = function() { return "module0"; };
            const __WEBPACK_DEFAULT_EXPORT__ = module0;
            /* @common:endif */
        },
        "module1.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.modules.module1"] */
            const module1 = function() { return "module1"; };
            const __WEBPACK_DEFAULT_EXPORT__ = module1;
            /* @common:endif */
        },
        "module2.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.modules.module2"] */
            const module2 = function() { return "module2"; };
            const __WEBPACK_DEFAULT_EXPORT__ = module2;
            /* @common:endif */
        },
        "module3.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.modules.module3"] */
            const module3 = function() { return "module3"; };
            const __WEBPACK_DEFAULT_EXPORT__ = module3;
            /* @common:endif */
        },
        "module4.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => (__WEBPACK_DEFAULT_EXPORT__)
            });
            /* @common:if [condition="treeShake.modules.module4"] */
            const module4 = function() { return "module4"; };
            const __WEBPACK_DEFAULT_EXPORT__ = module4;
            /* @common:endif */
        }
    }]);
    "#;
    
    println!("Testing performance with 5 modules");
    println!("Original size: {} bytes", chunk.len());
    
    // Config: enable only module0 and module2 with entry module ID
    let config = json!({
        "treeShake": {
            "modules": {
                "module0": true,
                "module1": false,
                "module2": true,
                "module3": false,
                "module4": false
            }
        },
        "entryModules": {
            "lodash-es": "main.js"
        }
    });
    
    let start = std::time::Instant::now();
    let optimized = optimize(chunk.to_string(), &config.to_string());
    let duration = start.elapsed();
    
    println!("Optimized size: {} bytes", optimized.len());
    println!("Processing time: {:?}", duration);
    
    // Verify main.js is preserved
    assert!(optimized.contains("main.js"), "main.js should be preserved");
    
    // Verify enabled modules are preserved
    assert!(optimized.contains("module0.js"), "module0.js should be preserved when enabled");
    assert!(optimized.contains("module2.js"), "module2.js should be preserved when enabled");
    
    // Verify disabled modules are removed
    assert!(!optimized.contains("module1.js"), "module1.js should be removed when disabled");
    assert!(!optimized.contains("module3.js"), "module3.js should be removed when disabled");
    assert!(!optimized.contains("module4.js"), "module4.js should be removed when disabled");
    
    // NOTE: The macro conditions only remove the __webpack_require__ calls, not the function implementations
    // The key improvement is that the enhanced parser correctly identifies dependencies and removes orphaned modules
    
    // Verify that the key improvement is working - orphaned modules are removed
    let reduction = (chunk.len() - optimized.len()) as f64 / chunk.len() as f64 * 100.0;
    println!("Size reduction: {:.1}%", reduction);
    assert!(reduction > 30.0, "Should achieve significant size reduction by removing orphaned modules");
    
    // Performance should be reasonable (less than 1 second)
    assert!(duration.as_secs() < 1, "Processing should complete in reasonable time");
    
    println!("✅ Enhanced parser performance is acceptable");
}