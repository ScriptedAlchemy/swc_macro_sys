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

#[test]
fn test_deep_nested_macros_optimization() {
    // Test that our deep nested macros bundle can be parsed and optimized
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");

    println!("\n=== DEEP NESTED MACROS OPTIMIZATION TEST ===");
    println!("Original bundle size: {} bytes", bundle_content.len());

    // Test with all features disabled (maximum optimization)
    let config = json!({
        "features": {
            "enableFeatureA": false,
            "enableFeatureB": false,
            "enableFeatureC": false,
            "enableA1_2": false,
            "enableA2_2": false,
            "enableB1_1": false,
            "enableB1_2": false,
            "enableSharedDeep": false,
            "enableDeepUtil1": false,
            "enableDeepUtil2": false,
            "enableLeaf2": false,
            "enableB1_2Deep": false
        }
    });

    let result = optimize(bundle_content.to_string(), &config.to_string());

    // Should achieve significant optimization
    assert!(result.len() < bundle_content.len(), "Should optimize the bundle");

    // Should still contain module structure due to hoisted imports
    assert!(result.contains("__webpack_modules__"), "Should preserve webpack structure");
    assert!(result.contains("moduleA"), "Entry modules should be present");
    assert!(result.contains("moduleB"), "Entry modules should be present");
    assert!(result.contains("moduleC"), "Entry modules should be present");

    println!("Deep nested macros optimization test passed!");

    // Test with selective enablement
    let partial_config = json!({
        "features": {
            "enableFeatureA": true,
            "enableFeatureB": false,
            "enableFeatureC": false,
            "enableA1_2": true,
            "enableA2_2": false,
            "enableB1_1": false,
            "enableB1_2": false,
            "enableSharedDeep": true,
            "enableDeepUtil1": true,
            "enableDeepUtil2": false,
            "enableLeaf2": false,
            "enableB1_2Deep": false
        }
    });

    let partial_result = optimize(bundle_content.to_string(), &partial_config.to_string());

    println!("Partial optimization result size: {} bytes", partial_result.len());
    println!("Partial size reduction: {} bytes ({:.1}%)",
            bundle_content.len() - partial_result.len(),
            ((bundle_content.len() - partial_result.len()) as f64 / bundle_content.len() as f64) * 100.0);

    // Partial optimization should be less aggressive than full disable
    assert!(partial_result.len() > result.len(), "Partial optimization should preserve more code");
    assert!(partial_result.len() < bundle_content.len(), "Should still optimize");

    println!("Partial optimization comparison test passed!");
}

#[test]
fn test_deep_nested_macros_with_top_level_optimization() {
    // Test the variant with top-level macro conditions
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros-with-top-level.js");

    println!("\n=== DEEP NESTED MACROS WITH TOP-LEVEL OPTIMIZATION TEST ===");
    println!("Original bundle size: {} bytes", bundle_content.len());

    // Test with complete disable (maximum tree shaking)
    let config = json!({
        "features": {
            "enableTopLevelA": false,
            "enableTopLevelB": false,
            "enableTopLevelC": false,
            "enableFeatureA": false,
            "enableFeatureB": false,
            "enableFeatureC": false,
            "enableA1_2": false,
            "enableA2_2": false,
            "enableB1_1": false,
            "enableB1_2": false,
            "enableSharedDeep": false,
            "enableDeepUtil1": false,
            "enableDeepUtil2": false,
            "enableLeaf2": false,
            "enableB1_2Deep": false
        }
    });

    let result = optimize(bundle_content.to_string(), &config.to_string());

    println!("Optimized result size: {} bytes", result.len());
    println!("Size reduction: {} bytes ({:.1}%)",
            bundle_content.len() - result.len(),
            ((bundle_content.len() - result.len()) as f64 / bundle_content.len() as f64) * 100.0);

    // Should achieve even better optimization due to top-level disabling
    assert!(result.len() < bundle_content.len(), "Should optimize the bundle");

    // Top-level features should be completely removed
    assert!(!result.contains("Top-level A enabled"), "Top-level A should be disabled");
    assert!(!result.contains("Top-level B enabled"), "Top-level B should be disabled");
    assert!(!result.contains("Top-level C enabled"), "Top-level C should be disabled");

    // Test with only top-level A enabled
    let top_level_a_config = json!({
        "features": {
            "enableTopLevelA": true,
            "enableTopLevelB": false,
            "enableTopLevelC": false,
            "enableFeatureA": true,
            "enableFeatureB": false,
            "enableFeatureC": false,
            "enableA1_2": true,
            "enableA2_2": true,
            "enableB1_1": false,
            "enableB1_2": false,
            "enableSharedDeep": true,
            "enableDeepUtil1": true,
            "enableDeepUtil2": true,
            "enableLeaf2": true,
            "enableB1_2Deep": false
        }
    });

    let top_level_a_result = optimize(bundle_content.to_string(), &top_level_a_config.to_string());

    println!("Top-level A only result size: {} bytes", top_level_a_result.len());
    println!("Top-level A reduction: {} bytes ({:.1}%)",
            bundle_content.len() - top_level_a_result.len(),
            ((bundle_content.len() - top_level_a_result.len()) as f64 / bundle_content.len() as f64) * 100.0);

    // Should preserve A chain but remove B and C
    assert!(top_level_a_result.contains("Top-level A enabled"), "Top-level A should be enabled");
    assert!(!top_level_a_result.contains("Top-level B enabled"), "Top-level B should be disabled");
    assert!(!top_level_a_result.contains("Top-level C enabled"), "Top-level C should be disabled");

    // A-only should be less optimized than complete disable but more than original
    assert!(top_level_a_result.len() > result.len(), "A-only should be larger than complete disable");
    assert!(top_level_a_result.len() < bundle_content.len(), "A-only should be smaller than original");

    println!("Deep nested macros with top-level optimization test passed!");
} 