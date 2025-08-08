use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_cjs_split_chunk_optimization() {
    // Test CommonJS split chunk format (exports.modules)
    let cjs_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_lodash-es_lodash_js"];
exports.modules = {
    "node_modules/lodash-es/sortBy.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => sortBy
        });
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        function sortBy(collection, iteratee) {
            return collection.sort();
        }
        /* @common:endif */
    },
    "node_modules/lodash-es/uniq.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => uniq
        });
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        function uniq(array) {
            return [...new Set(array)];
        }
        /* @common:endif */
    },
    "node_modules/lodash-es/filter.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        __webpack_require__.d(exports, {
            "default": () => filter
        });
        /* @common:if [condition="treeShake.lodash-es.filter"] */
        function filter(collection, predicate) {
            return collection.filter(predicate);
        }
        /* @common:endif */
    },
    "node_modules/lodash-es/lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.r(exports);
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        __webpack_require__.d(exports, {
            sortBy: () => _sortBy["default"]
        });
        var _sortBy = __webpack_require__("node_modules/lodash-es/sortBy.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        __webpack_require__.d(exports, {
            uniq: () => _uniq["default"]
        });
        var _uniq = __webpack_require__("node_modules/lodash-es/uniq.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.filter"] */
        __webpack_require__.d(exports, {
            filter: () => _filter["default"]
        });
        var _filter = __webpack_require__("node_modules/lodash-es/filter.js");
        /* @common:endif */
    }
};
"#;

    println!("\n=== CJS SPLIT CHUNK OPTIMIZATION TEST ===");
    println!("Testing CommonJS split chunk format (exports.modules)");
    println!("Original size: {} bytes", cjs_chunk.len());
    
    // Config: keep sortBy and uniq, remove filter
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "filter": false
            }
        }
    });
    
    let optimized = optimize(cjs_chunk.to_string(), &config.to_string());
    
    println!("Optimized size: {} bytes", optimized.len());
    println!("Reduction: {:.1}%", 
        (1.0 - optimized.len() as f64 / cjs_chunk.len() as f64) * 100.0);
    
    // Verify structure is preserved
    assert!(optimized.contains("exports.ids ="), "Should preserve exports.ids");
    assert!(optimized.contains("exports.modules ="), "Should preserve exports.modules");
    
    // Verify main lodash module is preserved
    assert!(optimized.contains("lodash.js"), "Main lodash module should be preserved");
    
    // Check macro conditions worked
    assert!(optimized.contains("collection.sort()"), "sortBy implementation should be present");
    assert!(optimized.contains("new Set(array)"), "uniq implementation should be present");
    assert!(!optimized.contains("collection.filter(predicate)"), "filter implementation should be removed");
    
    // Check exports - check for different possible formats
    let has_sortby_export = optimized.contains("sortBy: () =>") || 
                           optimized.contains("sortBy: ()=>") || 
                           optimized.contains("sortBy:");
    assert!(has_sortby_export, "sortBy export should be present");
    
    let has_uniq_export = optimized.contains("uniq: () =>") || 
                         optimized.contains("uniq: ()=>") || 
                         optimized.contains("uniq:");
    assert!(has_uniq_export, "uniq export should be present");
    
    let has_filter_export = optimized.contains("filter: () =>") || 
                           optimized.contains("filter: ()=>") || 
                           (optimized.contains("filter:") && optimized.contains("_filter"));
    assert!(!has_filter_export, "filter export should be removed");
    
    println!("✅ CJS split chunk optimization test passed!");
}

#[test]
fn test_jsonp_split_chunk_optimization() {
    // Test JSONP split chunk format (push)
    let jsonp_chunk = r#"
(self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["vendors-lodash"], {
    "node_modules/lodash-es/sortBy.js": function(module, __webpack_exports__, __webpack_require__) {
        "use strict";
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            "default": () => __WEBPACK_DEFAULT_EXPORT__
        });
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        const sortBy = function(collection, iteratee) {
            return collection.sort();
        };
        const __WEBPACK_DEFAULT_EXPORT__ = sortBy;
        /* @common:endif */
    },
    "node_modules/lodash-es/uniq.js": function(module, __webpack_exports__, __webpack_require__) {
        "use strict";
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            "default": () => __WEBPACK_DEFAULT_EXPORT__
        });
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        const uniq = function(array) {
            return [...new Set(array)];
        };
        const __WEBPACK_DEFAULT_EXPORT__ = uniq;
        /* @common:endif */
    },
    "node_modules/lodash-es/filter.js": function(module, __webpack_exports__, __webpack_require__) {
        "use strict";
        __webpack_require__.r(__webpack_exports__);
        __webpack_require__.d(__webpack_exports__, {
            "default": () => __WEBPACK_DEFAULT_EXPORT__
        });
        /* @common:if [condition="treeShake.lodash-es.filter"] */
        const filter = function(collection, predicate) {
            return collection.filter(predicate);
        };
        const __WEBPACK_DEFAULT_EXPORT__ = filter;
        /* @common:endif */
    },
    "node_modules/lodash-es/lodash.js": function(module, __webpack_exports__, __webpack_require__) {
        "use strict";
        __webpack_require__.r(__webpack_exports__);
        /* @common:if [condition="treeShake.lodash-es.sortBy"] */
        __webpack_require__.d(__webpack_exports__, {
            sortBy: () => (/* reexport safe */ _sortBy__WEBPACK_IMPORTED_MODULE_0__["default"])
        });
        var _sortBy__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("node_modules/lodash-es/sortBy.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.uniq"] */
        __webpack_require__.d(__webpack_exports__, {
            uniq: () => (/* reexport safe */ _uniq__WEBPACK_IMPORTED_MODULE_1__["default"])
        });
        var _uniq__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("node_modules/lodash-es/uniq.js");
        /* @common:endif */
        /* @common:if [condition="treeShake.lodash-es.filter"] */
        __webpack_require__.d(__webpack_exports__, {
            filter: () => (/* reexport safe */ _filter__WEBPACK_IMPORTED_MODULE_2__["default"])
        });
        var _filter__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("node_modules/lodash-es/filter.js");
        /* @common:endif */
    }
}]);
"#;

    println!("\n=== JSONP SPLIT CHUNK OPTIMIZATION TEST ===");
    println!("Testing JSONP split chunk format (.push)");
    println!("Original size: {} bytes", jsonp_chunk.len());
    
    // Config: keep sortBy and uniq, remove filter
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "filter": false
            }
        }
    });
    
    let optimized = optimize(jsonp_chunk.to_string(), &config.to_string());
    
    println!("Optimized size: {} bytes", optimized.len());
    println!("Reduction: {:.1}%", 
        (1.0 - optimized.len() as f64 / jsonp_chunk.len() as f64) * 100.0);
    
    // Verify structure is preserved
    assert!(optimized.contains("self[\"webpackChunkapp\"]"), "Should preserve chunk assignment");
    assert!(optimized.contains(".push("), "Should preserve push call");
    
    // Verify main lodash module is preserved
    assert!(optimized.contains("lodash.js"), "Main lodash module should be preserved");
    
    // Check macro conditions worked
    assert!(optimized.contains("collection.sort()"), "sortBy implementation should be present");
    assert!(optimized.contains("new Set(array)"), "uniq implementation should be present");
    assert!(!optimized.contains("collection.filter(predicate)"), "filter implementation should be removed");
    
    // Check exports - check for different possible formats
    let has_sortby_export = optimized.contains("sortBy: () =>") || 
                           optimized.contains("sortBy: ()=>") || 
                           optimized.contains("sortBy:");
    assert!(has_sortby_export, "sortBy export should be present");
    
    let has_uniq_export = optimized.contains("uniq: () =>") || 
                         optimized.contains("uniq: ()=>") || 
                         optimized.contains("uniq:");
    assert!(has_uniq_export, "uniq export should be present");
    
    let has_filter_export = optimized.contains("filter: () =>") || 
                           optimized.contains("filter: ()=>") || 
                           (optimized.contains("filter:") && optimized.contains("_filter"));
    assert!(!has_filter_export, "filter export should be removed");
    
    println!("✅ JSONP split chunk optimization test passed!");
}

#[test]
fn test_split_chunk_no_tree_shaking() {
    // Verify that split chunks don't get tree-shaken based on entry points
    let split_chunk = r#"
"use strict";
exports.ids = ["utilities"];
exports.modules = {
    "utils/helper1.js": function(module, exports, __webpack_require__) {
        exports.helper1 = function() { return "Helper 1"; };
    },
    "utils/helper2.js": function(module, exports, __webpack_require__) {
        // This module requires helper1
        var h1 = __webpack_require__("utils/helper1.js");
        exports.helper2 = function() { return h1.helper1() + " + Helper 2"; };
    },
    "utils/helper3.js": function(module, exports, __webpack_require__) {
        // This module has no dependencies
        exports.helper3 = function() { return "Helper 3"; };
    }
};
"#;

    println!("\n=== SPLIT CHUNK NO TREE SHAKING TEST ===");
    println!("Verifying split chunks preserve all modules (no entry-point-based tree shaking)");
    
    // Empty config - no macro conditions
    let config = json!({});
    
    let optimized = optimize(split_chunk.to_string(), &config.to_string());
    
    // All modules should be preserved because split chunks don't have entry points
    assert!(optimized.contains("helper1.js"), "Helper1 should be preserved");
    assert!(optimized.contains("helper2.js"), "Helper2 should be preserved");
    assert!(optimized.contains("helper3.js"), "Helper3 should be preserved");
    
    assert!(optimized.contains("Helper 1"), "Helper1 content should be preserved");
    assert!(optimized.contains("Helper 2"), "Helper2 content should be preserved");
    assert!(optimized.contains("Helper 3"), "Helper3 content should be preserved");
    
    println!("✅ Split chunks correctly preserve all modules!");
}

#[test]
fn test_real_world_cjs_chunk() {
    // Test with the actual CJS chunk format from the test-cases
    let chunk_content = include_str!("../../../test-cases/rspack-cjs-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    let share_usage = include_str!("../../../test-cases/rspack-cjs-annotated-output/share-usage.json");
    
    println!("\n=== REAL WORLD CJS CHUNK TEST ===");
    println!("Testing actual CJS vendor chunk from test-cases");
    println!("Original size: {} KB", chunk_content.len() / 1024);
    
    // Parse share usage to build config (supports legacy and new schemas)
    let usage: serde_json::Value = serde_json::from_str(share_usage).unwrap();
    let mut lodash_config = serde_json::Map::new();
    if usage.get("treeShake").is_some() {
        let obj = usage["treeShake"]["lodash-es"].as_object().expect("lodash-es object expected");
        let mut used_names: Vec<String> = Vec::new();
        let mut unused_count = 0usize;
        for (k, v) in obj {
            if k == "chunk_characteristics" { continue; }
            match v.as_bool() {
                Some(true) => { lodash_config.insert(k.clone(), json!(true)); used_names.push(k.clone()); }
                Some(false) => { lodash_config.insert(k.clone(), json!(false)); unused_count += 1; }
                _ => {}
            }
        }
        println!("Used exports: {:?}", used_names);
        println!("Unused exports: {}", unused_count);
    } else {
        let lodash_usage = &usage["consume_shared_modules"]["lodash-es"];
        let used_exports = lodash_usage["used_exports"].as_array().unwrap();
        let unused_exports = lodash_usage["unused_exports"].as_array().unwrap();
        println!("Used exports: {:?}", used_exports.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());
        println!("Unused exports: {}", unused_exports.len());
        for export in used_exports {
            lodash_config.insert(export.as_str().unwrap().to_string(), json!(true));
        }
        for export in unused_exports {
            lodash_config.insert(export.as_str().unwrap().to_string(), json!(false));
        }
    }
    
    let config = json!({
        "treeShake": {
            "lodash-es": lodash_config
        }
    });
    
    let optimized = optimize(chunk_content.to_string(), &config.to_string());
    
    let reduction_percent = (1.0 - optimized.len() as f64 / chunk_content.len() as f64) * 100.0;
    println!("Optimized size: {} KB", optimized.len() / 1024);
    println!("Reduction: {:.1}%", reduction_percent);
    
    // Verify it's a CJS chunk
    assert!(optimized.contains("exports.ids"), "Should be CJS format");
    assert!(optimized.contains("exports.modules"), "Should be CJS format");
    
    // Verify reasonable reduction
    assert!(reduction_percent > 30.0, "Should achieve significant reduction");
    
    println!("✅ Real world CJS chunk optimization successful!");
}