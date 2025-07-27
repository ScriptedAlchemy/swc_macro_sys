use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_simple_tree_shaking() {
    println!("\n=== SIMPLE TREE SHAKING TEST ===");
    
    let chunk = r#"
"use strict";
exports.ids = ["test-chunk"];
exports.modules = {
    "test/a.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => a
        });
        function a() {
            return "A";
        }
    },
    "test/b.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => b
        });
        function b() {
            return "B";
        }
    },
    "test/index.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            a: () => (/* @common:if [condition="treeShake.test.a"] */ _a["default"] /* @common:endif */),
            b: () => (/* @common:if [condition="treeShake.test.b"] */ _b["default"] /* @common:endif */)
        });
        var _a = __webpack_require__("test/a.js");
        var _b = __webpack_require__("test/b.js");
    }
};
"#;
    
    // Keep only 'a', remove 'b'
    let config = json!({
        "treeShake": {
            "test": {
                "a": true,
                "b": false
            }
        },
        "entryModules": {
            "test": "test/index.js"
        }
    });
    
    println!("Original chunk: {} bytes", chunk.len());
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    println!("Optimized chunk: {} bytes", optimized.len());
    println!("Optimization applied: {}", chunk.len() != optimized.len());
    
    // Check results
    let has_a = optimized.contains("test/a.js");
    let has_b = optimized.contains("test/b.js");
    let has_index = optimized.contains("test/index.js");
    
    println!("\nModule presence:");
    println!("  a.js: {}", if has_a { "present" } else { "removed" });
    println!("  b.js: {}", if has_b { "present" } else { "removed" });
    println!("  index.js: {}", if has_index { "present" } else { "removed" });
    
    // Check export presence
    let has_a_export = optimized.contains("a: () =>");
    let has_b_export = optimized.contains("b: () =>");
    
    println!("\nExport presence:");
    println!("  export a: {}", if has_a_export { "present" } else { "removed" });
    println!("  export b: {}", if has_b_export { "present" } else { "removed" });
    
    // Print the optimized chunk for debugging
    println!("\nOptimized chunk:");
    println!("{}", optimized);
    
    assert!(has_a, "a.js should be kept");
    assert!(!has_b, "b.js should be removed");
    assert!(has_index, "index.js should be kept");
    assert!(has_a_export, "export a should be present");
    assert!(!has_b_export, "export b should be removed");
}