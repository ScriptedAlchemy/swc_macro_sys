use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_skip_tree_shaking_without_entry_modules() {
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

    // Disable both exports but provide no explicit entry modules
    let config = json!({
        "treeShake": {
            "test": {
                "a": false,
                "b": false
            }
        }
    });

    let optimized = optimize(chunk.to_string(), &config.to_string());

    // Without explicit entry modules, tree shaking should be skipped and all modules remain
    assert!(optimized.contains("test/b.js"), "b.js should remain when no explicit entry modules are provided");
    assert!(optimized.contains("test/a.js"), "a.js should remain when no explicit entry modules are provided");
}
