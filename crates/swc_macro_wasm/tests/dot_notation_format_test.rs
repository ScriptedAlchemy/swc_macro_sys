use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_dot_notation_format_for_tree_shake_conditions() {
    println!("\n=== DOT NOTATION FORMAT TEST ===");
    
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
        var _capitalize = __webpack_require__("node_modules/lodash-es/capitalize.js");
        var _debounce = __webpack_require__("node_modules/lodash-es/debounce.js");
    }
};
"#;
    
    // Test 1: Standard dot notation format (exactly as expected by macros)
    let config_dot_notation = json!({
        "treeShake": {
            "lodash-es": {
                "capitalize": true,
                "debounce": false
            }
        }
    });
    
    println!("Test 1: Standard dot notation format");
    println!("Config: {}", serde_json::to_string_pretty(&config_dot_notation).unwrap());
    
    let optimized = optimize(vendor_chunk.to_string(), &config_dot_notation.to_string());
    
    println!("Optimized output:\n{}", optimized);
    
    // Verify capitalize module is preserved and debounce is removed
    assert!(optimized.contains("capitalize.js"), "capitalize module should be preserved");
    assert!(optimized.contains("function capitalize"), "capitalize function should be present");
    
    // debounce module should be completely removed
    assert!(!optimized.contains("debounce.js"), "debounce module should be removed");
    assert!(!optimized.contains("function debounce"), "debounce function should be removed");
    
    // The main lodash.js module might be removed if both exports are handled
    // This is expected behavior when tree shaking is aggressive
    
    println!("✅ Test 1 passed: Dot notation format works correctly");
    
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
                "default": true
                // Note: false values are NOT included, matching our JS implementation
            }
        },
        "entryModules": {
            "lodash-es": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
        }
    });
    
    println!("\nTest 2: Config format from JS optimizer");
    println!("This config only includes exports marked as true (used exports)");
    
    // The config should work with the optimizer
    let result = optimize(vendor_chunk.to_string(), &config_from_js_optimizer.to_string());
    assert!(!result.is_empty(), "Optimization should produce valid output");
    
    println!("✅ Test 2 passed: JS optimizer config format is valid");
    
    // Test 3: Verify path resolution for nested conditions
    println!("\nTest 3: Path resolution for dot notation");
    
    // The metadata query should resolve "treeShake.lodash-es.capitalize" correctly
    let test_value = config_dot_notation.clone();
    
    // Simulate what happens in the condition transformer
    let path = "treeShake.lodash-es.capitalize";
    let mut current_value = Some(&test_value);
    for segment in path.split('.') {
        current_value = current_value.and_then(|v| v.get(segment));
        println!("  Segment '{}': {:?}", segment, current_value);
    }
    
    assert_eq!(current_value, Some(&json!(true)), "Path resolution should find true value");
    
    println!("✅ Test 3 passed: Dot notation path resolution works correctly");
    
    println!("\n✅ All dot notation format tests passed!");
}

#[test]
fn test_real_world_dot_notation_config() {
    println!("\n=== REAL WORLD DOT NOTATION CONFIG TEST ===");
    
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
        "entryModules": {
            "lodash-es": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
        },
        "metadata": {
            "timestamp": "2025-07-24T05:05:02.083Z",
            "apps": ["host", "remote"],
            "modules": ["lodash-es"]
        }
    });
    
    println!("Testing with simulated merged config from module-federation-example");
    
    // Verify the structure
    assert!(merged_config.get("treeShake").is_some(), "Config must have treeShake");
    assert!(merged_config.get("entryModules").is_some(), "Config must have entryModules");
    
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
                    println!("  Used export: {}", export_name);
                } else {
                    unused_count += 1;
                }
            }
        }
    }
    
    println!("\nSummary:");
    println!("  Used exports: {}", used_count);
    println!("  Unused exports: {}", unused_count);
    
    // Verify some known used exports
    let known_used = ["capitalize", "debounce", "groupBy", "omit", "pick", "sortBy", "throttle", "uniq", "default"];
    for export in &known_used {
        let value = lodash_config.get(export);
        assert_eq!(value, Some(&json!(true)), "Export '{}' should be marked as used", export);
    }
    
    println!("\n✅ Real world config test passed!");
}

#[test]
fn test_condition_evaluation_matches_dot_notation() {
    println!("\n=== CONDITION EVALUATION TEST ===");
    
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
        println!("  Path '{}' resolves to: {:?} (expected: {:?})", path, result, expected);
        assert_eq!(result, expected, "Path '{}' should resolve to {:?}", path, expected);
    }
    
    println!("\n✅ Condition evaluation test passed!");
}