use swc_macro_wasm::optimize;
use serde_json::json;

/// Tests that tree shaking is skipped when no entry_module_id is provided
#[test]
fn test_tree_shaking_requires_explicit_entries() {
    println!("\n=== TESTING TREE SHAKING REQUIRES EXPLICIT ENTRIES ===");
    
    let chunk = r#"
"use strict";
exports.ids = ["chunk-vendors"];
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.test.featureA"] */
        var featureA = __webpack_require__("featureA.js");
        exports.featureA = featureA.init;
        /* @common:endif */
        
        /* @common:if [condition="treeShake.test.featureB"] */
        var featureB = __webpack_require__("featureB.js");
        exports.featureB = featureB.init;
        /* @common:endif */
    },
    "featureA.js": function(module, exports, __webpack_require__) {
        exports.init = function() { return "featureA"; };
    },
    "featureB.js": function(module, exports, __webpack_require__) {
        exports.init = function() { return "featureB"; };
    },
    "unused.js": function(module, exports, __webpack_require__) {
        exports.unused = function() { return "unused"; };
    }
};
"#;
    
    // Config with treeShake but NO entryModules - should skip tree shaking
    let config = json!({
        "treeShake": {
            "test": {
                "featureA": true,
                "featureB": false
            }
        }
        // NOTE: No entryModules specified!
    });
    
    println!("Original chunk: {} bytes", chunk.len());
    println!("Configuration (NO entryModules): {}", serde_json::to_string_pretty(&config).unwrap());
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    println!("Optimized chunk: {} bytes", optimized.len());
    
    // Count modules before and after
    let original_modules = chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    
    println!("Module count: {} -> {}", original_modules, optimized_modules);
    
    // Verify all modules are preserved (no tree shaking)
    assert!(optimized.contains("main.js"), "Main module should be preserved");
    assert!(optimized.contains("featureA.js"), "Feature A should be preserved");
    assert!(optimized.contains("featureB.js"), "Feature B should be preserved");
    assert!(optimized.contains("unused.js"), "Unused module should be preserved (no tree shaking)");
    
    // Verify macro conditions are still processed
    let original_conditions = chunk.matches("@common:if").count();
    let optimized_conditions = optimized.matches("@common:if").count();
    let conditions_processed = original_conditions - optimized_conditions;
    
    println!("Macro conditions: {} -> {} (processed: {})", 
        original_conditions, optimized_conditions, conditions_processed);
    
    assert!(conditions_processed > 0, "Macro conditions should still be processed");
    
    // Module count should remain the same (no tree shaking)
    assert_eq!(original_modules, optimized_modules, 
        "Module count should remain the same without explicit entry points");
    
    println!("✅ Tree shaking correctly skipped without explicit entries!");
}

#[test]
fn test_tree_shaking_with_invalid_entry_points() {
    println!("\n=== TESTING TREE SHAKING WITH INVALID ENTRY POINTS ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "moduleA.js": function(module, exports, __webpack_require__) {
        exports.a = function() { return "A"; };
    },
    "moduleB.js": function(module, exports, __webpack_require__) {
        exports.b = function() { return "B"; };
    },
    "moduleC.js": function(module, exports, __webpack_require__) {
        exports.c = function() { return "C"; };
    }
};
"#;
    
    // Config with entryModules pointing to non-existent modules
    let config = json!({
        "treeShake": {
            "test": {
                "a": true,
                "b": false,
                "c": false
            }
        },
        "entryModules": {
            "test": "nonexistent.js"  // This module doesn't exist!
        }
    });
    
    println!("Configuration with invalid entry point: {}", serde_json::to_string_pretty(&config).unwrap());
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Count modules - should preserve all since entry point is invalid
    let original_modules = chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    
    println!("Module count with invalid entry: {} -> {}", original_modules, optimized_modules);
    
    // All modules should be preserved since the entry point doesn't exist
    assert!(optimized.contains("moduleA.js"), "Module A should be preserved");
    assert!(optimized.contains("moduleB.js"), "Module B should be preserved"); 
    assert!(optimized.contains("moduleC.js"), "Module C should be preserved");
    
    // Should not crash or remove modules when entry point is invalid
    assert_eq!(original_modules, optimized_modules, 
        "All modules should be preserved with invalid entry point");
    
    println!("✅ Invalid entry points handled gracefully!");
}

#[test]
fn test_tree_shaking_empty_chunk() {
    println!("\n=== TESTING TREE SHAKING WITH EMPTY CHUNK ===");
    
    let empty_chunk = r#"
"use strict";
exports.modules = {};
"#;
    
    let config = json!({
        "treeShake": {
            "test": {
                "something": true
            }
        },
        "entryModules": {
            "test": "main.js"
        }
    });
    
    let optimized = optimize(empty_chunk.to_string(), &config.to_string());
    
    // Should handle empty chunks gracefully
    assert!(optimized.contains("exports.modules"), "Should maintain structure");
    assert!(!optimized.contains("main.js"), "Should not contain non-existent modules");
    
    println!("Empty chunk handled: {} bytes -> {} bytes", empty_chunk.len(), optimized.len());
    println!("✅ Empty chunk test passed!");
}

#[test]
fn test_tree_shaking_skip_without_macro_config() {
    println!("\n=== TESTING TREE SHAKING SKIPPED WITHOUT MACRO CONFIG ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "app.js": function(module, exports, __webpack_require__) {
        var utils = __webpack_require__("utils.js");
        exports.app = function() { return utils.helper(); };
    },
    "utils.js": function(module, exports, __webpack_require__) {
        exports.helper = function() { return "helper"; };
    },
    "unused.js": function(module, exports, __webpack_require__) {
        exports.unused = function() { return "unused"; };
    }
};
"#;
    
    // Config with entryModules but NO treeShake config
    let config = json!({
        "entryModules": {
            "app": "app.js"
        }
        // NOTE: No treeShake config!
    });
    
    println!("Configuration without treeShake: {}", serde_json::to_string_pretty(&config).unwrap());
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    let original_modules = chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    
    println!("Module count without macro config: {} -> {}", original_modules, optimized_modules);
    
    // All modules should be preserved - no tree shaking without macro config
    assert!(optimized.contains("app.js"), "App module should be preserved");
    assert!(optimized.contains("utils.js"), "Utils module should be preserved");
    assert!(optimized.contains("unused.js"), "Unused module should be preserved (no macro config)");
    
    assert_eq!(original_modules, optimized_modules, 
        "All modules should be preserved without macro processing config");
    
    println!("✅ Tree shaking correctly skipped without macro config!");
}

#[test]
fn test_tree_shaking_utf8_handling() {
    println!("\n=== TESTING TREE SHAKING UTF-8 HANDLING ===");
    
    // Chunk with UTF-8 characters and emoji
    let chunk_with_utf8 = r#"
"use strict";
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.app.feature"] */
        var feature = __webpack_require__("feature.js");
        exports.feature = feature;
        /* @common:endif */
        exports.greeting = "🚀 Hello, 世界!";
    },
    "feature.js": function(module, exports, __webpack_require__) {
        exports.message = "Feature with émojis: 🎉✨";
    },
    "unused.js": function(module, exports, __webpack_require__) {
        exports.message = "Unused with ñ and ü characters";
    }
};
"#;
    
    let config = json!({
        "treeShake": {
            "app": {
                "feature": true
            }
        },
        "entryModules": {
            "app": "main.js"
        }
    });
    
    let optimized = optimize(chunk_with_utf8.to_string(), &config.to_string());
    
    // Verify UTF-8 characters are preserved
    assert!(optimized.contains("🚀"), "Emoji should be preserved");
    assert!(optimized.contains("世界"), "Chinese characters should be preserved");
    assert!(optimized.contains("émojis"), "Accented characters should be preserved");
    assert!(optimized.contains("🎉✨"), "Multiple emojis should be preserved");
    
    // Verify basic tree shaking still works with UTF-8
    assert!(optimized.contains("main.js"), "Main module should be preserved");
    assert!(optimized.contains("feature.js"), "Feature module should be preserved");
    
    println!("UTF-8 content preserved in {} byte output", optimized.len());
    println!("✅ UTF-8 handling test passed!");
}

#[test]
fn test_tree_shaking_mixed_chunk_formats() {
    println!("\n=== TESTING TREE SHAKING WITH MIXED CHUNK FORMATS ===");
    
    // Test with JSONP-style chunk format
    let jsonp_chunk = r#"
(self["webpackChunk"] = self["webpackChunk"] || []).push([["vendors"], {
    "lib/core.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lib.core"] */
        exports.core = function() { return "core"; };
        /* @common:endif */
    },
    "lib/addon.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lib.addon"] */
        exports.addon = function() { return "addon"; };
        /* @common:endif */
    }
}]);
"#;
    
    let config = json!({
        "treeShake": {
            "lib": {
                "core": true,
                "addon": false
            }
        },
        "entryModules": {
            "lib": "lib/core.js"
        }
    });
    
    let optimized = optimize(jsonp_chunk.to_string(), &config.to_string());
    
    // Verify JSONP structure is maintained
    assert!(optimized.contains("webpackChunk"), "JSONP structure should be preserved");
    assert!(optimized.contains(".push(["), "Push call should be preserved");
    
    // Verify module processing
    assert!(optimized.contains("lib/core.js"), "Core module should be preserved");
    
    println!("JSONP chunk format processed: {} bytes -> {} bytes", 
        jsonp_chunk.len(), optimized.len());
    
    println!("✅ Mixed chunk formats test passed!");
}

#[test]
fn test_tree_shaking_log_messages() {
    println!("\n=== TESTING TREE SHAKING LOG MESSAGES ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        exports.main = function() { return "main"; };
    }
};
"#;
    
    // Capture output to verify logging behavior
    // Note: This is a basic test - in real scenarios you'd need proper log capture
    
    // Test 1: With explicit entries (should show tree shaking activity)
    let config_with_entries = json!({
        "treeShake": {
            "test": {
                "main": true
            }
        },
        "entryModules": {
            "test": "main.js"
        }
    });
    
    let optimized1 = optimize(chunk.to_string(), &config_with_entries.to_string());
    assert!(optimized1.len() > 0, "Should produce valid output with entries");
    
    // Test 2: Without explicit entries (should skip tree shaking)
    let config_no_entries = json!({
        "treeShake": {
            "test": {
                "main": true
            }
        }
        // No entryModules
    });
    
    let optimized2 = optimize(chunk.to_string(), &config_no_entries.to_string());
    assert!(optimized2.len() > 0, "Should produce valid output without entries");
    
    // Both should preserve the main module
    assert!(optimized1.contains("main.js"), "Main module preserved with entries");
    assert!(optimized2.contains("main.js"), "Main module preserved without entries");
    
    println!("✅ Log messages test passed!");
}

#[test]
fn test_tree_shaking_edge_case_circular_deps() {
    println!("\n=== TESTING TREE SHAKING WITH CIRCULAR DEPENDENCIES ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.test.feature"] */
        var moduleA = __webpack_require__("moduleA.js");
        exports.feature = moduleA.funcA;
        /* @common:endif */
    },
    "moduleA.js": function(module, exports, __webpack_require__) {
        var moduleB = __webpack_require__("moduleB.js");
        exports.funcA = function() { return moduleB.funcB() + "A"; };
    },
    "moduleB.js": function(module, exports, __webpack_require__) {
        // Circular dependency back to moduleA
        var moduleA = __webpack_require__("moduleA.js");
        exports.funcB = function() { return "B"; };
    },
    "independent.js": function(module, exports, __webpack_require__) {
        exports.independent = function() { return "independent"; };
    }
};
"#;
    
    let config = json!({
        "treeShake": {
            "test": {
                "feature": true
            }
        },
        "entryModules": {
            "test": "main.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Should handle circular dependencies gracefully
    assert!(optimized.contains("main.js"), "Main module should be preserved");
    assert!(optimized.contains("moduleA.js"), "Module A should be preserved");
    assert!(optimized.contains("moduleB.js"), "Module B should be preserved");
    
    // Independent module should potentially be removed
    let has_independent = optimized.contains("independent.js");
    println!("Independent module preserved: {}", has_independent);
    
    // Should not crash or hang with circular dependencies
    assert!(optimized.len() > 0, "Should produce valid output with circular deps");
    
    println!("✅ Circular dependencies test passed!");
}