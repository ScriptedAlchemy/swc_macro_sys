use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_tree_shaker_with_entry_points() {
    // Create a standard webpack bundle (not a split chunk) with entry points
    let test_chunk = r#"(function() {
var __webpack_modules__ = ({
    100: (function(module, exports, __webpack_require__) {
        // Entry point that only uses moduleA
        var a = __webpack_require__(200);
        console.log(a.value);
    }),
    200: (function(module, exports, __webpack_require__) {
        // Used module
        exports.value = "Module A";
    }),
    300: (function(module, exports, __webpack_require__) {
        // Unused module - should be removed by tree shaker
        exports.value = "Module B";
    }),
    400: (function(module, exports, __webpack_require__) {
        // Unused module - should be removed by tree shaker
        exports.value = "Module C";
    })
});

// Simulate webpack bootstrap
function __webpack_require__(moduleId) {
    var module = { exports: {} };
    __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
    return module.exports;
}

// Entry point
__webpack_require__(100);
})();
"#;

    println!("\n=== TREE SHAKER WITH ENTRY POINTS TEST ===");
    println!("Testing if unused modules are removed...");
    
    // Count modules before optimization
    let modules_before = test_chunk.matches("function(module, exports").count();
    println!("Modules before optimization: {}", modules_before);
    
    // Run optimization with empty config (no macro conditions)
    let config = json!({});
    let optimized = optimize(test_chunk.to_string(), &config.to_string());
    
    // Count modules after optimization
    let modules_after = optimized.matches("function(module, exports").count();
    println!("Modules after optimization: {}", modules_after);
    
    // Check specific modules
    println!("\nModule presence check:");
    println!("  Module 100 (entry): {}", if optimized.contains("100:") { "✅ Present" } else { "❌ Removed" });
    println!("  Module 200 (used): {}", if optimized.contains("200:") { "✅ Present" } else { "❌ Removed" });
    println!("  Module 300 (unused): {}", if optimized.contains("300:") { "❌ Present (should be removed)" } else { "✅ Removed" });
    println!("  Module 400 (unused): {}", if optimized.contains("400:") { "❌ Present (should be removed)" } else { "✅ Removed" });
    
    // Verify tree shaking worked
    assert!(optimized.contains("100:"), "Entry module should be preserved");
    assert!(optimized.contains("200:"), "Used module should be preserved");
    assert!(!optimized.contains("300:"), "Unused module 300 should be removed");
    assert!(!optimized.contains("400:"), "Unused module 400 should be removed");
    
    println!("\n✅ Tree shaker effectiveness test passed!");
}

#[test]
fn test_tree_shaker_with_cjs_lodash_modules() {
    // Test with a simplified CJS lodash chunk to verify module removal
    let lodash_chunk = r#"
"use strict";
exports.ids = ["vendors-lodash"];
exports.modules = {
    "lodash/sortBy.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lodash.sortBy"] */
        exports.sortBy = function(collection) { return collection.sort(); };
        /* @common:endif */
    },
    "lodash/uniq.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lodash.uniq"] */
        exports.uniq = function(array) { return [...new Set(array)]; };
        /* @common:endif */
    },
    "lodash/filter.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lodash.filter"] */
        exports.filter = function(collection, predicate) { return collection.filter(predicate); };
        /* @common:endif */
    },
    "lodash/map.js": function(module, exports, __webpack_require__) {
        /* @common:if [condition="treeShake.lodash.map"] */
        exports.map = function(collection, iteratee) { return collection.map(iteratee); };
        /* @common:endif */
    },
    "lodash/lodash.js": function(module, exports, __webpack_require__) {
        // Main export module
        /* @common:if [condition="treeShake.lodash.sortBy"] */
        exports.sortBy = __webpack_require__("lodash/sortBy.js").sortBy;
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash.uniq"] */
        exports.uniq = __webpack_require__("lodash/uniq.js").uniq;
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash.filter"] */
        exports.filter = __webpack_require__("lodash/filter.js").filter;
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash.map"] */
        exports.map = __webpack_require__("lodash/map.js").map;
        /* @common:endif */
    }
};
"#;

    println!("\n=== CJS LODASH MODULE REMOVAL TEST ===");
    
    // Config: only keep sortBy and uniq
    let config = json!({
        "treeShake": {
            "lodash": {
                "sortBy": true,
                "uniq": true,
                "filter": false,
                "map": false
            }
        }
    });
    
    let optimized = optimize(lodash_chunk.to_string(), &config.to_string());
    
    // Count function definitions
    let functions_before = lodash_chunk.matches("function(module, exports").count();
    let functions_after = optimized.matches("function(module, exports").count();
    
    println!("Functions before: {}", functions_before);
    println!("Functions after: {}", functions_after);
    
    // Check module presence
    println!("\nModule presence:");
    println!("  sortBy.js: {}", if optimized.contains("sortBy.js") { "✅" } else { "❌" });
    println!("  uniq.js: {}", if optimized.contains("uniq.js") { "✅" } else { "❌" });
    println!("  filter.js: {}", if optimized.contains("filter.js") { "❌ (should be removed)" } else { "✅ Removed" });
    println!("  map.js: {}", if optimized.contains("map.js") { "❌ (should be removed)" } else { "✅ Removed" });
    println!("  lodash.js: {}", if optimized.contains("lodash.js") { "✅" } else { "❌" });
    
    // Check exports
    println!("\nExports presence:");
    println!("  exports.sortBy: {}", if optimized.contains("exports.sortBy") { "✅" } else { "❌" });
    println!("  exports.uniq: {}", if optimized.contains("exports.uniq") { "✅" } else { "❌" });
    println!("  exports.filter: {}", if optimized.contains("exports.filter") { "❌ (should be removed)" } else { "✅ Removed" });
    println!("  exports.map: {}", if optimized.contains("exports.map") { "❌ (should be removed)" } else { "✅ Removed" });
    
    // The main lodash.js should be preserved when exports are used
    assert!(optimized.contains("lodash.js"), "Main lodash module should be preserved");
    
    // Enabled exports should have their content
    assert!(optimized.contains("return collection.sort()"), "sortBy implementation should be present");
    assert!(optimized.contains("new Set(array)"), "uniq implementation should be present");
    
    // Disabled exports should have their content removed
    assert!(!optimized.contains("collection.filter(predicate)"), "filter implementation should be removed");
    assert!(!optimized.contains("collection.map(iteratee)"), "map implementation should be removed");
    
    println!("\n✅ CJS lodash module removal test passed!");
}