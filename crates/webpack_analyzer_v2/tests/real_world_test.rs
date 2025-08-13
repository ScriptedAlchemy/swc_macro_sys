use webpack_analyzer_v2::*;
use swc_core::atoms::Atom;

#[test]
fn test_real_world_webpack_chunk() {
    let analyzer = WebpackAnalyzer::new();
    
    // This is a simplified version of a real webpack chunk
    let source = r#"
        "use strict";
        exports.ids = ["vendors-lodash"];
        exports.modules = {
            "../../node_modules/lodash-es/lodash.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "map": () => reexport_map,
                    "filter": () => reexport_filter,
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                var _map_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/map.js");
                var _filter_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("../../node_modules/lodash-es/filter.js");
                var reexport_map = _map_js__WEBPACK_IMPORTED_MODULE_0__["default"];
                var reexport_filter = _filter_js__WEBPACK_IMPORTED_MODULE_1__["default"];
                const __WEBPACK_DEFAULT_EXPORT__ = {
                    map: reexport_map,
                    filter: reexport_filter
                };
            },
            "../../node_modules/lodash-es/map.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                var _baseMap_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/_baseMap.js");
                function map(collection, iteratee) {
                    return _baseMap_js__WEBPACK_IMPORTED_MODULE_0__["default"](collection, iteratee);
                }
                const __WEBPACK_DEFAULT_EXPORT__ = map;
            },
            "../../node_modules/lodash-es/filter.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                var _baseFilter_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/_baseFilter.js");
                function filter(collection, predicate) {
                    return _baseFilter_js__WEBPACK_IMPORTED_MODULE_0__["default"](collection, predicate);
                }
                const __WEBPACK_DEFAULT_EXPORT__ = filter;
            },
            "../../node_modules/lodash-es/_baseMap.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                function baseMap(collection, iteratee) {
                    return collection.map(iteratee);
                }
                const __WEBPACK_DEFAULT_EXPORT__ = baseMap;
            },
            "../../node_modules/lodash-es/_baseFilter.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                function baseFilter(collection, predicate) {
                    return collection.filter(predicate);
                }
                const __WEBPACK_DEFAULT_EXPORT__ = baseFilter;
            },
            "../../node_modules/lodash-es/reduce.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                function reduce(collection, iteratee, accumulator) {
                    return collection.reduce(iteratee, accumulator);
                }
                const __WEBPACK_DEFAULT_EXPORT__ = reduce;
            }
        };
    "#;
    
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-lodash.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let chunk = analyzer.analyze_chunk(source, characteristics).unwrap();
    
    // Test basic chunk properties
    // Updated for new enum variant - should be CommonJSSync due to "use strict" at start
    assert_eq!(chunk.chunk_type, ChunkType::CommonJSSync);
    assert_eq!(chunk.module_count(), 6);
    
    // Test main lodash module dependencies
    {
        let lodash_module = chunk.get_module(&Atom::from("../../node_modules/lodash-es/lodash.js")).unwrap();
        assert!(lodash_module.depends_on(&Atom::from("../../node_modules/lodash-es/map.js")));
        assert!(lodash_module.depends_on(&Atom::from("../../node_modules/lodash-es/filter.js")));
        
        // Test map module dependencies
        let map_module = chunk.get_module(&Atom::from("../../node_modules/lodash-es/map.js")).unwrap();
        assert!(map_module.depends_on(&Atom::from("../../node_modules/lodash-es/_baseMap.js")));
        
        // Test filter module dependencies
        let filter_module = chunk.get_module(&Atom::from("../../node_modules/lodash-es/filter.js")).unwrap();
        assert!(filter_module.depends_on(&Atom::from("../../node_modules/lodash-es/_baseFilter.js")));
    }
    
    // Test orphan detection
    let mut graph = DependencyGraph::new();
    for (_, module) in chunk.modules {
        graph.add_module(module);
    }
    
    // Use lodash.js as entry point
    let entry_points = vec![Atom::from("../../node_modules/lodash-es/lodash.js")];
    let reachable = graph.get_reachable_from_multiple(&entry_points);
    let orphaned = graph.find_orphaned_modules(&entry_points);
    
    // reduce.js should be orphaned since it's not used by lodash.js
    assert_eq!(orphaned.len(), 1);
    assert!(orphaned.contains(&Atom::from("../../node_modules/lodash-es/reduce.js")));
    
    // All other modules should be reachable
    assert_eq!(reachable.len(), 5);
    assert!(reachable.contains(&Atom::from("../../node_modules/lodash-es/lodash.js")));
    assert!(reachable.contains(&Atom::from("../../node_modules/lodash-es/map.js")));
    assert!(reachable.contains(&Atom::from("../../node_modules/lodash-es/filter.js")));
    assert!(reachable.contains(&Atom::from("../../node_modules/lodash-es/_baseMap.js")));
    assert!(reachable.contains(&Atom::from("../../node_modules/lodash-es/_baseFilter.js")));
    
    assert_eq!(6, 6);
    assert_eq!(reachable.len(), 5);
    assert_eq!(orphaned.len(), 1);
}