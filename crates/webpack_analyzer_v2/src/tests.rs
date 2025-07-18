use crate::*;
use swc_core::atoms::Atom;

#[test]
fn test_chunk_type_detection() {
    let analyzer = WebpackAnalyzer::new();
    
    // Test CommonJS detection
    let commonjs_source = r#"
        "use strict";
        exports.modules = {
            "module1.js": function() {},
            "module2.js": function() {}
        };
    "#;
    
    let chunk_type = analyzer.detect_chunk_type(commonjs_source).unwrap();
    assert_eq!(chunk_type, ChunkType::CommonJS);
    
    // Test JSONP detection
    let jsonp_source = r#"
        (self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["chunk1"], {
            "module1.js": function() {},
            "module2.js": function() {}
        }]);
    "#;
    
    let chunk_type = analyzer.detect_chunk_type(jsonp_source).unwrap();
    assert_eq!(chunk_type, ChunkType::JSONP);
}

#[test]
fn test_commonjs_module_extraction() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        "use strict";
        exports.modules = {
            "module1.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                var dep = __webpack_require__("module2.js");
                const __WEBPACK_DEFAULT_EXPORT__ = "module1";
            },
            "module2.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                const __WEBPACK_DEFAULT_EXPORT__ = "module2";
            }
        };
    "#;
    
    let chunk = analyzer.analyze_chunk(source).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
    assert_eq!(chunk.module_count(), 2);
    assert!(chunk.get_module(&Atom::from("module1.js")).is_some());
    assert!(chunk.get_module(&Atom::from("module2.js")).is_some());
}

#[test]
fn test_jsonp_module_extraction() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        (self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["chunk1"], {
            "module1.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                var dep = __webpack_require__("module2.js");
            },
            "module2.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
            }
        }]);
    "#;
    
    let chunk = analyzer.analyze_chunk(source).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::JSONP);
    assert_eq!(chunk.module_count(), 2);
    assert!(chunk.get_module(&Atom::from("module1.js")).is_some());
    assert!(chunk.get_module(&Atom::from("module2.js")).is_some());
}

#[test]
fn test_dependency_graph_building() {
    let analyzer = WebpackAnalyzer::new();
    
    let source = r#"
        "use strict";
        exports.modules = {
            "main.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                var util = __webpack_require__("util.js");
                var helper = __webpack_require__("helper.js");
            },
            "util.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                var helper = __webpack_require__("helper.js");
            },
            "helper.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                // No dependencies
            },
            "unused.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                // No dependencies and no dependents
            }
        };
    "#;
    
    let chunk = analyzer.analyze_chunk(source).unwrap();
    
    // Test module dependencies
    let main_module = chunk.get_module(&"main.js".to_string()).unwrap();
    assert!(main_module.depends_on(&"util.js".to_string()));
    assert!(main_module.depends_on(&"helper.js".to_string()));
    
    let util_module = chunk.get_module(&"util.js".to_string()).unwrap();
    assert!(util_module.depends_on(&"helper.js".to_string()));
    
    let helper_module = chunk.get_module(&"helper.js".to_string()).unwrap();
    assert!(helper_module.is_depended_on_by(&"main.js".to_string()));
    assert!(helper_module.is_depended_on_by(&"util.js".to_string()));
    
    let unused_module = chunk.get_module(&"unused.js".to_string()).unwrap();
    assert!(!unused_module.has_dependencies());
    assert!(!unused_module.has_dependents());
}

#[test]
fn test_real_world_lodash_chunk() {
    let analyzer = WebpackAnalyzer::new();
    
    // Simplified lodash chunk structure
    let source = r#"
        "use strict";
        exports.modules = {
            "../../node_modules/lodash-es/lodash.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__,
                    "map": () => reexport_map
                });
                var _map_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/map.js");
                var reexport_map = _map_js__WEBPACK_IMPORTED_MODULE_0__["default"];
                const __WEBPACK_DEFAULT_EXPORT__ = lodash;
            },
            "../../node_modules/lodash-es/map.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
                __webpack_require__.r(__webpack_exports__);
                __webpack_require__.d(__webpack_exports__, {
                    "default": () => __WEBPACK_DEFAULT_EXPORT__
                });
                const __WEBPACK_DEFAULT_EXPORT__ = map;
            }
        };
    "#;
    
    let chunk = analyzer.analyze_chunk(source).unwrap();
    
    assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
    assert_eq!(chunk.module_count(), 2);
    
    // Test dependency relationships
    let lodash_module = chunk.get_module(&"../../node_modules/lodash-es/lodash.js".to_string()).unwrap();
    assert!(lodash_module.depends_on(&"../../node_modules/lodash-es/map.js".to_string()));
    
    let map_module = chunk.get_module(&"../../node_modules/lodash-es/map.js".to_string()).unwrap();
    assert!(map_module.is_depended_on_by(&"../../node_modules/lodash-es/lodash.js".to_string()));
}

#[test]
fn test_dependency_graph_orphan_detection() {
    let mut graph = DependencyGraph::new();
    
    // Add modules
    graph.add_module(WebpackModule::new("main.js".to_string(), "main source".to_string()));
    graph.add_module(WebpackModule::new("util.js".to_string(), "util source".to_string()));
    graph.add_module(WebpackModule::new("helper.js".to_string(), "helper source".to_string()));
    graph.add_module(WebpackModule::new("orphan.js".to_string(), "orphan source".to_string()));
    
    // Add dependencies
    graph.add_dependency(&"main.js".to_string(), &"util.js".to_string());
    graph.add_dependency(&"util.js".to_string(), &"helper.js".to_string());
    // orphan.js has no dependencies or dependents
    
    let entry_points = vec!["main.js".to_string()];
    let orphaned = graph.find_orphaned_modules(&entry_points);
    
    assert_eq!(orphaned.len(), 1);
    assert!(orphaned.contains(&"orphan.js".to_string()));
}

#[test]
fn test_reachability_analysis() {
    let mut graph = DependencyGraph::new();
    
    // Create a chain: main -> util -> helper
    graph.add_module(WebpackModule::new("main.js".to_string(), "main source".to_string()));
    graph.add_module(WebpackModule::new("util.js".to_string(), "util source".to_string()));
    graph.add_module(WebpackModule::new("helper.js".to_string(), "helper source".to_string()));
    
    graph.add_dependency(&"main.js".to_string(), &"util.js".to_string());
    graph.add_dependency(&"util.js".to_string(), &"helper.js".to_string());
    
    let reachable = graph.get_reachable_modules(&"main.js".to_string());
    
    assert_eq!(reachable.len(), 3);
    assert!(reachable.contains(&"main.js".to_string()));
    assert!(reachable.contains(&"util.js".to_string()));
    assert!(reachable.contains(&"helper.js".to_string()));
}