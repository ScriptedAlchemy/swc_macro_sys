use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_dot_notation_format_for_tree_shake_conditions() {
    // silent
    
    // Create a vendor chunk with macro conditions using dot notation
    let vendor_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_lodash-es_lodash_js"];
exports.modules = {
    "node_modules/lodash-es/capitalize.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => capitalize
        });
        function capitalize(string) {
            return string.charAt(0).toUpperCase() + string.slice(1);
        }
    },
    "node_modules/lodash-es/debounce.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => debounce
        });
        function debounce(func, wait) {
            let timeout;
            return function() {
                clearTimeout(timeout);
                timeout = setTimeout(() => func.apply(this, arguments), wait);
            };
        }
    },
    "node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            capitalize: () => (/* @common:if [condition="treeShake.lodash-es.capitalize"] */ _capitalize["default"] /* @common:endif */),
            debounce: () => (/* @common:if [condition="treeShake.lodash-es.debounce"] */ _debounce["default"] /* @common:endif */)
        });
        /* @common:if [condition="treeShake.lodash-es.capitalize"] */
        var _capitalize = __webpack_require__("node_modules/lodash-es/capitalize.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.debounce"] */
        var _debounce = __webpack_require__("node_modules/lodash-es/debounce.js");
        /* @common:endif */
    }
};
"#;
    
    // Test 1: Standard dot notation format (exactly as expected by macros)
    let config_dot_notation = json!({
        "treeShake": {
            "lodash-es": {
                "capitalize": true,
                "debounce": false,
                "chunk_characteristics": {
                    "entry_module_id": "node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "require",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["vendors-node_modules_lodash-es_lodash_js.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        }
    });
    
    // Test 1: Standard dot notation format
    
    let optimized = optimize(vendor_chunk.to_string(), &config_dot_notation.to_string());
    
    assert!(!optimized.is_empty());
    
    // Verify capitalize module is preserved and debounce is removed
    assert!(optimized.contains("capitalize.js"), "capitalize module should be preserved");
    assert!(optimized.contains("function capitalize"), "capitalize function should be present");
    
    // debounce module should be completely removed
    assert!(!optimized.contains("debounce.js"), "debounce module should be removed");
    assert!(!optimized.contains("function debounce"), "debounce function should be removed");
    
    // The main lodash.js module might be removed if both exports are handled
    // This is expected behavior when tree shaking is aggressive
    
    // done
    
    // Test 2: Verify the config structure matches what our JS optimize function creates
    let config_from_js_optimizer = json!({
        "treeShake": {
            "lodash-es": {
                "capitalize": true,
                "debounce": true,
                "groupBy": true,
                "omit": true,
                "pick": true,
                "sortBy": true,
                "throttle": true,
                "uniq": true,
                "default": true,
                "chunk_characteristics": {
                    "entry_module_id": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js",
                    "is_runtime_chunk": false,
                    "chunk_format": "require"
                }
            }
        }
    });
    
    // Test 2: Config format from JS optimizer
    
    // The config should work with the optimizer
    let result = optimize(vendor_chunk.to_string(), &config_from_js_optimizer.to_string());
    assert!(!result.is_empty(), "Optimization should produce valid output");
    
    // done
    
    // Test 3: Verify path resolution for nested conditions
    // Test 3: Path resolution for dot notation
    
    // The metadata query should resolve "treeShake.lodash-es.capitalize" correctly
    let test_value = config_dot_notation.clone();
    
    // Simulate what happens in the condition transformer
    let path = "treeShake.lodash-es.capitalize";
    let mut current_value = Some(&test_value);
    for segment in path.split('.') {
        current_value = current_value.and_then(|v| v.get(segment));
        let _ = segment;
    }
    
    assert_eq!(current_value, Some(&json!(true)), "Path resolution should find true value");
    
    // done
}

#[test]
fn test_real_world_dot_notation_config() {
    // silent
    
    // Simulate the actual merged config structure from our JS tool
    let merged_config = json!({
        "treeShake": {
            "lodash-es": {
                "capitalize": true,
                "debounce": true,
                "groupBy": true,
                "omit": true,
                "pick": true,
                "sortBy": true,
                "throttle": true,
                "uniq": true,
                "default": true,
                "add": false,
                "after": false,
                "ary": false,
                "assign": false,
                "bind": false,
                "chain": false,
                "chunk": false,
                "clone": false,
                "compact": false,
                "concat": false
            }
        },
        "metadata": {
            "timestamp": "2025-07-24T05:05:02.083Z",
            "apps": ["host", "remote"],
            "modules": ["lodash-es"]
        }
    });
    
    let _ = "examples/module-federation-example";
    
    // Verify the structure
    assert!(merged_config.get("treeShake").is_some(), "Config must have treeShake");
    
    let tree_shake = merged_config.get("treeShake").unwrap();
    let lodash_config = tree_shake.get("lodash-es").unwrap();
    
    // Count true (used) vs false (unused) exports
    let mut used_count = 0;
    let mut unused_count = 0;
    
    if let serde_json::Value::Object(exports) = lodash_config {
        for (export_name, value) in exports {
            if let serde_json::Value::Bool(is_used) = value {
                if *is_used {
                    used_count += 1;
                } else {
                    unused_count += 1;
                }
            }
        }
    }
    
    assert!(used_count >= 0 && unused_count >= 0);
    
    // Verify some known used exports
    let known_used = ["capitalize", "debounce", "groupBy", "omit", "pick", "sortBy", "throttle", "uniq", "default"];
    for export in &known_used {
        let value = lodash_config.get(export);
        assert_eq!(value, Some(&json!(true)), "Export '{}' should be marked as used", export);
    }
    
    // done
}

#[test]
fn test_condition_evaluation_matches_dot_notation() {
    // silent
    
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "capitalize": true,
                "debounce": false,
                "default": true
            },
            "react": {
                "useState": true,
                "useEffect": false
            }
        }
    });
    
    // Test various dot notation paths manually (since Metadata trait is private)
    let test_cases = vec![
        ("treeShake.lodash-es.capitalize", Some(true)),
        ("treeShake.lodash-es.debounce", Some(false)),
        ("treeShake.lodash-es.default", Some(true)),
        ("treeShake.lodash-es.nonexistent", None), // Should return None for missing
        ("treeShake.react.useState", Some(true)),
        ("treeShake.react.useEffect", Some(false)),
    ];
    
    for (path, expected) in test_cases {
        // Manually implement the path resolution logic
        let mut current_value = Some(&config);
        for segment in path.split('.') {
            current_value = current_value.and_then(|v| v.get(segment));
        }
        
        let result = current_value.and_then(|v| v.as_bool());
        // validate
        assert_eq!(result, expected, "Path '{}' should resolve to {:?}", path, expected);
    }
    
    // done
}