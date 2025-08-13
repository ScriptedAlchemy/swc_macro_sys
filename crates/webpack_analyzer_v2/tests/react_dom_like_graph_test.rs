use webpack_analyzer_v2::*;
use swc_core::atoms::Atom;

fn jsonp_react_dom_like_source() -> String {
    r#"
    (self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["vendors-react-dom"], {
        "react-dom/index.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            var ReactDOM = __webpack_require__("react-dom/cjs/react-dom.development.js");
            var Scheduler = __webpack_require__("scheduler/index.js");
            module.exports = ReactDOM;
        },
        "react-dom/cjs/react-dom.development.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            // implementation
        },
        "scheduler/index.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            var Dev = __webpack_require__("scheduler/cjs/scheduler.development.js");
            module.exports = Dev;
        },
        "scheduler/cjs/scheduler.development.js": function(module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            // implementation
        }
    }]);
    "#.to_string()
}

fn jsonp_characteristics() -> ChunkCharacteristics {
    ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-react-dom.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    }
}

#[test]
fn test_dependency_extraction_react_dom_like() {
    let analyzer = WebpackAnalyzer::new();
    let src = jsonp_react_dom_like_source();
    let chunk = analyzer.analyze_chunk(&src, jsonp_characteristics()).expect("analyze");

    assert_eq!(chunk.module_count(), 4);

    // Validate dependencies
    let idx = chunk.get_module(&Atom::from("react-dom/index.js")).expect("index module");
    assert!(idx.depends_on(&Atom::from("react-dom/cjs/react-dom.development.js")));
    assert!(idx.depends_on(&Atom::from("scheduler/index.js")));

    let sched = chunk.get_module(&Atom::from("scheduler/index.js")).expect("scheduler index");
    assert!(sched.depends_on(&Atom::from("scheduler/cjs/scheduler.development.js")));
}

#[test]
fn test_dependency_graph_reachability_react_dom_like() {
    let analyzer = WebpackAnalyzer::new();
    let src = jsonp_react_dom_like_source();
    let chunk = analyzer.analyze_chunk(&src, jsonp_characteristics()).expect("analyze");

    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }

    // From react-dom/index.js, we should reach all modules
    let reachable = graph.get_reachable_modules(&Atom::from("react-dom/index.js"));
    assert_eq!(reachable.len(), 4);
    assert!(reachable.contains(&Atom::from("react-dom/index.js")));
    assert!(reachable.contains(&Atom::from("react-dom/cjs/react-dom.development.js")));
    assert!(reachable.contains(&Atom::from("scheduler/index.js")));
    assert!(reachable.contains(&Atom::from("scheduler/cjs/scheduler.development.js")));
}


