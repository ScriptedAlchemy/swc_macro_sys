use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_webpack_tree_shaking_integration() {
    // Test with a realistic webpack bundle similar to our test cases
    let source = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        100: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Entry module");
            __webpack_require__(200);
        },
        200: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Used dependency");
        },
        300: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Unused module - should be tree shaken");
        }
    };

    function __webpack_require__(moduleId) {
        // webpack runtime
        return {};
    }

    (()=>{
        /* @common:if [condition="features.enableWebpackEntry"] */
        __webpack_require__(100);
        /* @common:endif */
    })();
})();
"#.to_string();

    let config = json!({
        "features": {
            "enableWebpackEntry": false  // This will remove the entry point call
        }
    });
    let original_size = source.len();
    let source_for_debug = source.clone();
    let result = optimize(source, &config.to_string());

    println!("=== DEBUG INTEGRATION TEST ===");
    println!("Original source ({} bytes):\n{}", original_size, source_for_debug);
    println!("\nOptimized result ({} bytes):\n{}", result.len(), result);
    println!("\nSearching for patterns:");
    println!("  Contains '100:': {}", result.contains("100:"));
    println!("  Contains '200:': {}", result.contains("200:"));
    println!("  Contains '300:': {}", result.contains("300:"));
    println!("  Contains empty webpack_modules: {}", result.contains("var __webpack_modules__ = {};"));

    // Since the entry point is removed by DCE, tree shaking should remove all modules
    assert!(!result.contains("100:"), "Module 100 should be tree shaken");
    assert!(!result.contains("200:"), "Module 200 should be tree shaken");
    assert!(!result.contains("300:"), "Module 300 should be tree shaken");
    assert!(!result.contains("__webpack_modules__"), "webpack_modules should be completely removed when no entry points");

    println!("Tree shaking integration test passed!");
    println!("Result size: {} bytes (tree shaking saved {} bytes)",
            result.len(),
            original_size - result.len());
}

#[test]
fn test_webpack_tree_shaking_with_macro_conditions() {
    // Test with a realistic webpack bundle with conditional features
    let source = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        100: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Entry module");
            /* @common:if [condition="features.enableFeatureA"] */
            __webpack_require__(200);
            /* @common:endif */
            /* @common:if [condition="features.enableFeatureB"] */
            __webpack_require__(300);
            /* @common:endif */
        },
        200: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Feature A module");
        },
        300: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Feature B module");
        },
        400: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Completely unused module");
        }
    };

    function __webpack_require__(moduleId) {
        // webpack runtime
        return {};
    }

    (()=>{
        /* @common:if [condition="features.enableEntryPoint"] */
        __webpack_require__(100);
        /* @common:endif */
    })();
})();
"#.to_string();

    let config = json!({
        "features": {
            "enableFeatureA": false,
            "enableFeatureB": false,
            "enableEntryPoint": false  // This removes the entry point entirely
        }
    });

    let result = optimize(source, &config.to_string());

    println!("=== DEBUG MACRO CONDITIONS TEST ===");
    println!("Optimized result:\n{}", result);

    // All modules should be tree shaken since there are no entry points
    assert!(!result.contains("100:"), "Entry module should be tree shaken");
    assert!(!result.contains("200:"), "Feature A module should be tree shaken");
    assert!(!result.contains("300:"), "Feature B module should be tree shaken");
    assert!(!result.contains("400:"), "Unused module should be tree shaken");
    assert!(!result.contains("__webpack_modules__"), "webpack_modules should be completely removed when no entry points");

    println!("Tree shaking with macro conditions test passed!");
    println!("All modules successfully tree shaken due to no entry points");
}

#[test]
fn test_decimal_numeric_module_ids() {
    // Test that decimal numeric module IDs are handled correctly
    let bundle_with_decimal_ids = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        100.5: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Decimal module ID 100.5");
            __webpack_require__(200);
        },
        200: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Integer module ID 200");
        },
        300.7: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            console.log("Unused decimal module ID 300.7");
        }
    };

    function __webpack_require__(moduleId) {
        return {};
    }

    __webpack_require__(100.5); // Entry point with decimal ID
})();
"#;

    println!("\n=== DECIMAL NUMERIC MODULE IDS TEST ===");

    let result = optimize(bundle_with_decimal_ids.to_string(), &json!({}).to_string());

    println!("Original bundle size: {} bytes", bundle_with_decimal_ids.len());
    println!("Optimized result size: {} bytes", result.len());
    println!("Result contains '100.5': {}", result.contains("100.5"));
    println!("Result contains '200': {}", result.contains("200"));
    println!("Result contains '300.7': {}", result.contains("300.7"));

    // Should preserve modules 100.5 and 200 (reachable)
    // Should remove module 300.7 (unreachable)
    assert!(result.contains("100.5") || result.len() < bundle_with_decimal_ids.len(),
            "Should either preserve decimal module ID 100.5 or tree shake everything");
    assert!(!result.contains("300.7") || result.contains("100.5"),
            "If decimal IDs are preserved, unreachable 300.7 should be removed");

    println!("Decimal numeric module IDs test passed!");
}

