use swc_macro_wasm::optimize;
use serde_json::json;

/// Test that verifies the complete dead module removal flow:
/// 1. Macro conditions remove webpack_require calls
/// 2. DCE cleans up unreferenced code
/// 3. Tree shaker removes unreachable modules from the modules object
#[test]
fn test_dead_module_removal_after_macro_and_dce() {
    let source = r#"
var __webpack_modules__ = {
    "./entry.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            featureA: ()=>_featureA_js__WEBPACK_IMPORTED_MODULE_0__["default"],
            featureB: ()=>_featureB_js__WEBPACK_IMPORTED_MODULE_1__["default"],
            featureC: ()=>_featureC_js__WEBPACK_IMPORTED_MODULE_2__["default"]
        });
        
        /* @common:if [condition="treeShake.mylib.featureA"] */
        var _featureA_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./featureA.js");
        /* @common:endif */
        
        /* @common:if [condition="treeShake.mylib.featureB"] */
        var _featureB_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("./featureB.js");
        /* @common:endif */
        
        /* @common:if [condition="treeShake.mylib.featureC"] */
        var _featureC_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("./featureC.js");
        /* @common:endif */
    },
    "./featureA.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>featureA
        });
        // This module depends on helperA
        var _helperA_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./helperA.js");
        function featureA() {
            return "Feature A with " + (0, _helperA_js__WEBPACK_IMPORTED_MODULE_0__["default"])();
        }
    },
    "./featureB.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>featureB
        });
        // This module depends on helperB
        var _helperB_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./helperB.js");
        function featureB() {
            return "Feature B with " + (0, _helperB_js__WEBPACK_IMPORTED_MODULE_0__["default"])();
        }
    },
    "./featureC.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>featureC
        });
        // This module has no dependencies
        function featureC() {
            return "Feature C standalone";
        }
    },
    "./helperA.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>helperA
        });
        function helperA() {
            return "Helper A";
        }
    },
    "./helperB.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>helperB
        });
        function helperB() {
            return "Helper B";
        }
    }
};
"#;

    // Test 1: Only featureA is enabled
    let config = json!({
        "treeShake": {
            "mylib": {
                "featureA": true,
                "featureB": false,
                "featureC": false,
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "./entry.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "async-node",
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

    let optimized = optimize(source.to_string(), &config.to_string());

    // Verify that featureB and featureC webpack_require calls are removed
    assert!(!optimized.contains(r#"__webpack_require__("./featureB.js")"#), 
            "featureB require should be removed when disabled");
    assert!(!optimized.contains(r#"__webpack_require__("./featureC.js")"#),
            "featureC require should be removed when disabled");
    
    // featureA require should still be present
    assert!(optimized.contains(r#"__webpack_require__("./featureA.js")"#),
            "featureA require should be kept when enabled");

    // Verify module removal from __webpack_modules__
    assert!(!optimized.contains(r#""./featureB.js": function"#),
            "featureB module should be removed from modules object");
    assert!(!optimized.contains(r#""./featureC.js": function"#),
            "featureC module should be removed from modules object");
    assert!(!optimized.contains(r#""./helperB.js": function"#),
            "helperB module should be removed as it's only used by featureB");
    
    // featureA and helperA should still be present
    assert!(optimized.contains(r#""./featureA.js": function"#),
            "featureA module should be kept");
    assert!(optimized.contains(r#""./helperA.js": function"#),
            "helperA module should be kept as it's used by featureA");
}

#[test]
fn test_module_count_reduction() {
    let source = r#"
var __webpack_modules__ = {
    "./entry.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            utilA: ()=>_utilA_js__WEBPACK_IMPORTED_MODULE_0__["default"],
            utilB: ()=>_utilB_js__WEBPACK_IMPORTED_MODULE_1__["default"]
        });
        
        /* @common:if [condition="treeShake.utils.utilA"] */
        var _utilA_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./utilA.js");
        /* @common:endif */
        
        /* @common:if [condition="treeShake.utils.utilB"] */
        var _utilB_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("./utilB.js");
        /* @common:endif */
    },
    "./utilA.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>utilA
        });
        function utilA() { return "Util A"; }
    },
    "./utilB.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            default: ()=>utilB
        });
        function utilB() { return "Util B"; }
    }
};
"#;

    let config = json!({
        "treeShake": {
            "utils": {
                "utilA": true,
                "utilB": false,
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "./entry.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "async-node",
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

    let optimized = optimize(source.to_string(), &config.to_string());

    // Count modules in the original
    let original_module_count = source.matches(": function(").count();
    
    // Count modules in the optimized
    let optimized_module_count = optimized.matches(": function(").count();
    
    // Should have removed utilB module
    assert_eq!(optimized_module_count, original_module_count - 1,
               "Should have removed exactly 1 module (utilB)");
}

#[test]
fn test_deep_dependency_removal() {
    let source = r#"
var __webpack_modules__ = {
    "./entry.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        
        /* @common:if [condition="treeShake.app.feature"] */
        var feature = __webpack_require__("./feature.js");
        /* @common:endif */
    },
    "./feature.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        var dep1 = __webpack_require__("./dep1.js");
        var dep2 = __webpack_require__("./dep2.js");
    },
    "./dep1.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
        var subdep = __webpack_require__("./subdep.js");
    },
    "./dep2.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
    },
    "./subdep.js": function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
        __webpack_require__.r(__webpack_exports__);
    }
};
"#;

    let config = json!({
        "treeShake": {
            "app": {
                "feature": false,
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "./entry.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "async-node",
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

    let optimized = optimize(source.to_string(), &config.to_string());

    // When feature is disabled, the entire dependency tree should be removed
    assert!(!optimized.contains(r#""./feature.js": function"#),
            "feature.js should be removed");
    assert!(!optimized.contains(r#""./dep1.js": function"#),
            "dep1.js should be removed");
    assert!(!optimized.contains(r#""./dep2.js": function"#),
            "dep2.js should be removed");
    assert!(!optimized.contains(r#""./subdep.js": function"#),
            "subdep.js should be removed");
    
    // Only entry.js should remain
    assert!(optimized.contains(r#""./entry.js": function"#),
            "entry.js should be kept");
    
    // Verify module count
    let module_count = optimized.matches(": function(").count();
    assert_eq!(module_count, 1, "Only entry module should remain");
}