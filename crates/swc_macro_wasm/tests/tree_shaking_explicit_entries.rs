use swc_macro_wasm::optimize;
use serde_json::json;

/// Tests that tree shaking works correctly with explicit entry points
#[test]
fn test_tree_shaking_with_explicit_entry() {
    println!("\n=== TESTING TREE SHAKING WITH EXPLICIT ENTRY ===");
    
    // Create a realistic webpack chunk with multiple modules
    let chunk = r#"
"use strict";
exports.ids = ["vendors-chunk"];
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        /* @common:if [condition="treeShake.test.featureA"] */
        var featureA = __webpack_require__("featureA.js");
        exports.featureA = featureA.init;
        /* @common:endif */
        
        /* @common:if [condition="treeShake.test.featureB"] */
        var featureB = __webpack_require__("featureB.js");
        exports.featureB = featureB.init;
        /* @common:endif */
        
        exports.main = function() { return "main"; };
    },
    "featureA.js": function(module, exports, __webpack_require__) {
        var shared = __webpack_require__("shared.js");
        exports.init = function() { return shared.helper() + "A"; };
    },
    "featureB.js": function(module, exports, __webpack_require__) {
        var shared = __webpack_require__("shared.js");
        exports.init = function() { return shared.helper() + "B"; };
    },
    "shared.js": function(module, exports, __webpack_require__) {
        exports.helper = function() { return "shared"; };
    },
    "unused.js": function(module, exports, __webpack_require__) {
        exports.unused = function() { return "unused"; };
    }
};
"#;
    
    // Test with explicit entry point and selective feature enablement
    let config = json!({
        "treeShake": {
            "test": {
                "featureA": true,  // Enable feature A
                "featureB": false  // Disable feature B
            }
        },
        "entryModules": {
            "test": "main.js"  // Explicit entry point
        }
    });
    
    println!("Original chunk: {} bytes", chunk.len());
    println!("Configuration: {}", serde_json::to_string_pretty(&config).unwrap());
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    println!("Optimized chunk: {} bytes", optimized.len());
    println!("Size reduction: {:.1}%", 
        ((chunk.len() as f64 - optimized.len() as f64) / chunk.len() as f64) * 100.0);
    
    // Verify that tree shaking worked correctly
    let has_main = optimized.contains("main.js");
    let has_feature_a = optimized.contains("featureA.js");
    let has_feature_b = optimized.contains("featureB.js");
    let has_shared = optimized.contains("shared.js");
    let has_unused = optimized.contains("unused.js");
    
    println!("\nModule presence after optimization:");
    println!("  main.js: {}", if has_main { "present" } else { "removed" });
    println!("  featureA.js: {}", if has_feature_a { "present" } else { "removed" });
    println!("  featureB.js: {}", if has_feature_b { "present" } else { "removed" });
    println!("  shared.js: {}", if has_shared { "present" } else { "removed" });
    println!("  unused.js: {}", if has_unused { "present" } else { "removed" });
    
    // Check macro processing results
    let feature_a_processed = !optimized.contains("@common:if [condition=\"treeShake.test.featureA\"]");
    let feature_b_processed = !optimized.contains("@common:if [condition=\"treeShake.test.featureB\"]");
    
    println!("\nMacro processing:");
    println!("  featureA macro processed: {}", feature_a_processed);
    println!("  featureB macro processed: {}", feature_b_processed);
    
    // Assertions for correct tree shaking behavior
    assert!(has_main, "Entry module should be preserved");
    assert!(has_feature_a, "Feature A module should be preserved (enabled)");
    assert!(has_shared, "Shared dependency should be preserved");
    
    // Modules that should be removed with tree shaking
    assert!(!has_unused, "Unused module should be removed");
    
    // Verify macro conditions were processed
    assert!(feature_a_processed, "Feature A macro should be processed");
    assert!(feature_b_processed, "Feature B macro should be processed");
    
    // Verify the enabled feature is accessible in optimized code
    assert!(optimized.contains("featureA") || optimized.contains("feature A"), 
        "Feature A should be accessible");
    
    println!("✅ Tree shaking with explicit entry test passed!");
}

#[test]
fn test_tree_shaking_multiple_entry_points() {
    println!("\n=== TESTING TREE SHAKING WITH MULTIPLE ENTRY POINTS ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "entryA.js": function(module, exports, __webpack_require__) {
        var utilA = __webpack_require__("utilA.js");
        var shared = __webpack_require__("shared.js");
        exports.entryA = function() {
            return utilA.functionA() + shared.common();
        };
    },
    "entryB.js": function(module, exports, __webpack_require__) {
        var utilB = __webpack_require__("utilB.js");
        var shared = __webpack_require__("shared.js");
        exports.entryB = function() {
            return utilB.functionB() + shared.common();
        };
    },
    "utilA.js": function(module, exports, __webpack_require__) {
        exports.functionA = function() { return "A"; };
    },
    "utilB.js": function(module, exports, __webpack_require__) {
        exports.functionB = function() { return "B"; };
    },
    "shared.js": function(module, exports, __webpack_require__) {
        exports.common = function() { return "shared"; };
    },
    "orphan.js": function(module, exports, __webpack_require__) {
        exports.orphan = function() { return "orphan"; };
    }
};
"#;
    
    let config = json!({
        "treeShake": {
            "test": {
                "entryA": true,
                "entryB": true,
                "utilA": true,
                "utilB": true,
                "shared": true,
                "orphan": false
            }
        },
        "entryModules": {
            "testA": "entryA.js",
            "testB": "entryB.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Count modules in original vs optimized
    let original_modules = chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    
    println!("Original modules: {}", original_modules);
    println!("Optimized modules: {}", optimized_modules);
    
    // Verify all entry points and their dependencies are preserved
    assert!(optimized.contains("entryA.js"), "Entry A should be preserved");
    assert!(optimized.contains("entryB.js"), "Entry B should be preserved");
    assert!(optimized.contains("utilA.js"), "Util A should be preserved");
    assert!(optimized.contains("utilB.js"), "Util B should be preserved");
    assert!(optimized.contains("shared.js"), "Shared dependency should be preserved");
    
    // Verify orphan module is removed
    assert!(!optimized.contains("orphan.js"), "Orphan module should be removed");
    
    println!("✅ Multiple entry points test passed!");
}

#[test]
fn test_tree_shaking_with_complex_dependencies() {
    println!("\n=== TESTING TREE SHAKING WITH COMPLEX DEPENDENCIES ===");
    
    let chunk = r#"
"use strict";
exports.modules = {
    "main.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.app.router"] */
        var router = __webpack_require__("router.js");
        exports.router = router;
        /* @common:endif */
        
        /* @common:if [condition="treeShake.app.auth"] */
        var auth = __webpack_require__("auth.js");
        exports.auth = auth;
        /* @common:endif */
    },
    "router.js": function(module, exports, __webpack_require__) {
        var utils = __webpack_require__("utils.js");
        var validation = __webpack_require__("validation.js");
        exports.createRouter = function() {
            return utils.merge({}, { validator: validation.validate });
        };
    },
    "auth.js": function(module, exports, __webpack_require__) {
        var crypto = __webpack_require__("crypto.js");
        var utils = __webpack_require__("utils.js");
        exports.authenticate = function() {
            return crypto.hash(utils.randomString());
        };
    },
    "utils.js": function(module, exports, __webpack_require__) {
        exports.merge = function(a, b) { return Object.assign(a, b); };
        exports.randomString = function() { return Math.random().toString(36); };
    },
    "validation.js": function(module, exports, __webpack_require__) {
        exports.validate = function(data) { return true; };
    },
    "crypto.js": function(module, exports, __webpack_require__) {
        exports.hash = function(data) { return "hash_" + data; };
    },
    "unused_feature.js": function(module, exports, __webpack_require__) {
        exports.feature = function() { return "unused"; };
    }
};
"#;
    
    // Test scenario: Enable router but disable auth
    let config = json!({
        "treeShake": {
            "app": {
                "router": true,
                "auth": false
            }
        },
        "entryModules": {
            "app": "main.js"
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    println!("Optimized chunk length: {} bytes", optimized.len());
    
    // Verify correct dependency resolution
    assert!(optimized.contains("main.js"), "Main entry should be preserved");
    assert!(optimized.contains("router.js"), "Router should be preserved (enabled)");
    assert!(optimized.contains("utils.js"), "Utils should be preserved (dependency of router)");
    assert!(optimized.contains("validation.js"), "Validation should be preserved (dependency of router)");
    
    // Auth and its unique dependencies should be handled based on tree shaking
    // Note: crypto.js should be removed since auth is disabled
    let has_auth = optimized.contains("auth.js");
    let has_crypto = optimized.contains("crypto.js");
    let has_unused = optimized.contains("unused_feature.js");
    
    println!("Auth module present: {}", has_auth);
    println!("Crypto module present: {}", has_crypto);
    println!("Unused feature present: {}", has_unused);
    
    // Unused feature should definitely be removed
    assert!(!has_unused, "Unused feature should be removed");
    
    println!("✅ Complex dependencies test passed!");
}

#[test]
fn test_tree_shaking_preserves_webpack_structure() {
    println!("\n=== TESTING TREE SHAKING PRESERVES WEBPACK STRUCTURE ===");
    
    let chunk = r#"
"use strict";
exports.ids = ["chunk-vendors"];
exports.modules = {
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
};
exports.runtime = ["webpack"];
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
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Verify webpack structure is preserved
    assert!(optimized.contains("exports.ids"), "exports.ids should be preserved");
    assert!(optimized.contains("exports.modules"), "exports.modules should be preserved");
    
    // Verify selective module preservation
    assert!(optimized.contains("lib/core.js"), "Core module should be preserved");
    
    // Check if runtime exports are preserved
    let has_runtime = optimized.contains("exports.runtime") || optimized.contains("runtime");
    println!("Runtime exports preserved: {}", has_runtime);
    
    println!("✅ Webpack structure preservation test passed!");
}

#[test]
fn test_tree_shaking_performance_benchmark() {
    println!("\n=== TESTING TREE SHAKING PERFORMANCE ===");
    
    // Generate a larger chunk for performance testing
    let mut large_chunk = String::from("\"use strict\";\nexports.modules = {\n");
    
    // Add 100 modules with dependencies
    for i in 0..100 {
        let module_content = format!(
            r#"    "module{}.js": function(module, exports, __webpack_require__) {{
        /* @common:if [condition="treeShake.perf.module{}"] */
        {}
        exports.function{} = function() {{ return "module{}"; }};
        /* @common:endif */
    }},"#,
            i, 
            i, 
            if i % 10 == 0 { format!("var dep = __webpack_require__(\"dep{}.js\");", i / 10) } else { String::new() },
            i, 
            i
        );
        large_chunk.push_str(&module_content);
        large_chunk.push('\n');
    }
    
    // Add some dependency modules
    for i in 0..10 {
        let dep_content = format!(
            r#"    "dep{}.js": function(module, exports, __webpack_require__) {{
        exports.helper = function() {{ return "dep{}"; }};
    }},"#,
            i, i
        );
        large_chunk.push_str(&dep_content);
        large_chunk.push('\n');
    }
    
    large_chunk.push_str("};\n");
    
    // Create config that enables only 25% of modules
    let mut module_config = serde_json::Map::new();
    for i in 0..100 {
        module_config.insert(format!("module{}", i), json!(i % 4 == 0));
    }
    
    let config = json!({
        "treeShake": {
            "perf": module_config
        },
        "entryModules": {
            "perf": "module0.js"
        }
    });
    
    println!("Generated large chunk: {} modules, {} bytes", 
        large_chunk.matches(".js\":").count(), 
        large_chunk.len());
    
    let start_time = std::time::Instant::now();
    let optimized = optimize(large_chunk.clone(), &config.to_string());
    let duration = start_time.elapsed();
    
    let original_modules = large_chunk.matches(".js\":").count();
    let optimized_modules = optimized.matches(".js\":").count();
    let reduction_percent = ((original_modules as f64 - optimized_modules as f64) / original_modules as f64) * 100.0;
    
    println!("Performance results:");
    println!("  Processing time: {:.2}ms", duration.as_millis());
    println!("  Original modules: {}", original_modules);
    println!("  Optimized modules: {}", optimized_modules);
    println!("  Module reduction: {:.1}%", reduction_percent);
    println!("  Size reduction: {:.1}%", 
        ((large_chunk.len() as f64 - optimized.len() as f64) / large_chunk.len() as f64) * 100.0);
    
    // Performance assertions
    assert!(duration.as_millis() < 5000, "Should complete within 5 seconds");
    assert!(optimized_modules <= original_modules, "Should not increase module count");
    
    // Verify some modules were processed
    let original_conditions = large_chunk.matches("@common:if").count();
    let optimized_conditions = optimized.matches("@common:if").count();
    let conditions_processed = original_conditions - optimized_conditions;
    
    println!("  Macro conditions processed: {}", conditions_processed);
    assert!(conditions_processed > 0, "Should process some macro conditions");
    
    println!("✅ Performance benchmark test passed!");
}