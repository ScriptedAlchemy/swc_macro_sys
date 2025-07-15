use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_webpack_dependency_preservation() {
    println!("\n=== WEBPACK DEPENDENCY PRESERVATION TEST ===");
    
    // Create a chunk where moduleA depends on moduleB and moduleC
    // If we remove moduleB, moduleA becomes broken
    let chunk = r#"
"use strict";
exports.modules = {
    "moduleA.js": function(module, exports, __webpack_require__) {
        // moduleA depends on moduleB and moduleC
        var moduleB = __webpack_require__("moduleB.js");
        var moduleC = __webpack_require__("moduleC.js");
        
        exports.functionA = function() {
            return moduleB.functionB() + moduleC.functionC();
        };
    },
    "moduleB.js": function(module, exports, __webpack_require__) {
        exports.functionB = function() {
            return "B";
        };
    },
    "moduleC.js": function(module, exports, __webpack_require__) {
        exports.functionC = function() {
            return "C";
        };
    },
    "unused.js": function(module, exports, __webpack_require__) {
        // This module is not used by anyone
        exports.unused = function() {
            return "unused";
        };
    }
};
"#;
    
    let config = json!({});
    
    println!("Testing dependency preservation:");
    println!("- moduleA depends on moduleB and moduleC");
    println!("- unused.js is not referenced by anyone");
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Check what was preserved
    let has_module_a = optimized.contains("moduleA.js");
    let has_module_b = optimized.contains("moduleB.js");
    let has_module_c = optimized.contains("moduleC.js");
    let has_unused = optimized.contains("unused.js");
    
    println!("\nResults:");
    println!("  moduleA.js: {}", if has_module_a { "✅ Preserved" } else { "❌ Removed" });
    println!("  moduleB.js: {}", if has_module_b { "✅ Preserved" } else { "❌ Removed" });
    println!("  moduleC.js: {}", if has_module_c { "✅ Preserved" } else { "❌ Removed" });
    println!("  unused.js: {}", if has_unused { "Preserved (should be removed)" } else { "✅ Removed" });
    
    // If moduleA is preserved, its dependencies should also be preserved
    if has_module_a {
        assert!(has_module_b, "moduleB should be preserved because moduleA depends on it");
        assert!(has_module_c, "moduleC should be preserved because moduleA depends on it");
    }
    
    assert!(!has_unused, "unused.js should be removed");
    
    println!("\n✅ Dependency preservation test passed");
}

#[test]
fn test_broken_dependency_detection() {
    println!("\n=== BROKEN DEPENDENCY DETECTION TEST ===");
    
    // Create a scenario where removing a module would break dependencies
    let chunk = r#"
"use strict";
exports.modules = {
    "entry.js": function(module, exports, __webpack_require__) {
        // This looks like an entry point
        var exporter = __webpack_require__("exporter.js");
        exports.default = exporter;
    },
    "exporter.js": function(module, exports, __webpack_require__) {
        // This module exports functions but also depends on helpers
        var helper1 = __webpack_require__("helper1.js");
        var helper2 = __webpack_require__("helper2.js");
        
        exports.exportedFunc = function() {
            return helper1() + helper2();
        };
    },
    "helper1.js": function(module, exports, __webpack_require__) {
        module.exports = function() { return "help1"; };
    },
    "helper2.js": function(module, exports, __webpack_require__) {
        module.exports = function() { return "help2"; };
    }
};
"#;
    
    let config = json!({});
    
    println!("Testing broken dependency detection:");
    println!("- entry.js requires exporter.js");
    println!("- exporter.js requires helper1.js and helper2.js");
    println!("- All modules should be preserved to maintain the dependency chain");
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    // Count how many modules remain
    let module_count = optimized.matches(".js\":").count();
    println!("\nModules remaining: {}", module_count);
    
    // All modules should be preserved because they form a dependency chain
    assert!(optimized.contains("entry.js"), "entry.js should be preserved");
    assert!(optimized.contains("exporter.js"), "exporter.js should be preserved");
    assert!(optimized.contains("helper1.js"), "helper1.js should be preserved");
    assert!(optimized.contains("helper2.js"), "helper2.js should be preserved");
    
    println!("✅ All modules in dependency chain preserved");
}

#[test]
fn test_module_federation_export_pattern() {
    println!("\n=== MODULE FEDERATION EXPORT PATTERN TEST ===");
    
    // Test the specific pattern used in Module Federation vendor chunks
    let chunk = r#"
"use strict";
exports.modules = {
    "lodash/sortBy.js": function(module, exports, __webpack_require__) {
        exports.default = function sortBy() { return "sortBy"; };
    },
    "lodash/uniq.js": function(module, exports, __webpack_require__) {
        exports.default = function uniq() { return "uniq"; };
    },
    "lodash/filter.js": function(module, exports, __webpack_require__) {
        exports.default = function filter() { return "filter"; };
    },
    "lodash/lodash.js": function(module, exports, __webpack_require__) {
        // This is the main export module that re-exports everything
        __webpack_require__.d(exports, {
            sortBy: () => (/* @common:if [condition="treeShake.lodash.sortBy"] */ __webpack_require__("lodash/sortBy.js").default /* @common:endif */),
            uniq: () => (/* @common:if [condition="treeShake.lodash.uniq"] */ __webpack_require__("lodash/uniq.js").default /* @common:endif */),
            filter: () => (/* @common:if [condition="treeShake.lodash.filter"] */ __webpack_require__("lodash/filter.js").default /* @common:endif */)
        });
    }
};
"#;
    
    let config = json!({
        "treeShake": {
            "lodash": {
                "sortBy": true,
                "uniq": true,
                "filter": false
            }
        }
    });
    
    println!("Testing Module Federation export pattern:");
    println!("- lodash/lodash.js is the main export module");
    println!("- It re-exports functions from other modules");
    println!("- Config: keep sortBy and uniq, remove filter");
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    
    println!("\nChecking preservation:");
    
    // The main export module MUST be preserved
    let has_main = optimized.contains("lodash/lodash.js");
    println!("  Main export module (lodash/lodash.js): {}", 
        if has_main { "✅ Preserved" } else { "❌ REMOVED (CRITICAL ERROR!)" });
    
    // Individual function modules
    let has_sortby = optimized.contains("lodash/sortBy.js");
    let has_uniq = optimized.contains("lodash/uniq.js");
    let has_filter = optimized.contains("lodash/filter.js");
    
    println!("  sortBy.js: {}", if has_sortby { "✅ Preserved" } else { "❌ Removed" });
    println!("  uniq.js: {}", if has_uniq { "✅ Preserved" } else { "❌ Removed" });
    println!("  filter.js: {}", if has_filter { "Preserved" } else { "✅ Removed" });
    
    // The main export module must ALWAYS be preserved
    assert!(has_main, "Main export module (lodash/lodash.js) must be preserved!");
    
    // If the main module is preserved and references other modules, they should be preserved too
    if has_main && optimized.contains("__webpack_require__(\"lodash/sortBy.js\")") {
        assert!(has_sortby, "sortBy.js should be preserved because main module requires it");
    }
    if has_main && optimized.contains("__webpack_require__(\"lodash/uniq.js\")") {
        assert!(has_uniq, "uniq.js should be preserved because main module requires it");
    }
    
    println!("\n✅ Module Federation export pattern test completed");
}