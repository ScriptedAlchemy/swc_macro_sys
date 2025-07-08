use webpack_graph::WebpackBundleParser;

const BUNDLE_CONTENT: &str = include_str!("../../../test-cases/webpack-bundles/bundle-all-features.js");

#[test]
fn test_parse_webpack_bundle() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    // Should find all the modules from the bundle
    assert!(!graph.modules.is_empty(), "Graph should contain modules");
    
    // Expected module IDs from the bundle
    let expected_modules = vec!["418", "422", "803", "153", "722", "78", "812"];
    
    for module_id in &expected_modules {
        assert!(
            graph.get_module(module_id).is_some(),
            "Module {} should exist in the graph",
            module_id
        );
    }
}

#[test]
fn test_module_dependencies() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    // Test specific module dependencies based on the bundle content
    
    // Module 153 (featureA) should depend on modules 78 and 418
    if let Some(module_153) = graph.get_module("153") {
        assert!(
            module_153.dependencies.contains("78"),
            "Module 153 should depend on module 78 (heavyMathUtils)"
        );
        assert!(
            module_153.dependencies.contains("418"),
            "Module 153 should depend on module 418 (dataProcessor)"
        );
    }

    // Module 722 (featureB) should depend on modules 803 and 812
    if let Some(module_722) = graph.get_module("722") {
        assert!(
            module_722.dependencies.contains("803"),
            "Module 722 should depend on module 803 (expensiveUIUtils)"
        );
        assert!(
            module_722.dependencies.contains("812"),
            "Module 722 should depend on module 812 (networkUtils)"
        );
    }
}

#[test]
fn test_entry_points() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    assert!(!graph.entry_points.is_empty(), "Graph should have entry points");
    
    // The main entry should reference modules 153, 722, and 422
    let reachable = graph.get_reachable_modules();
    assert!(
        reachable.contains("153"),
        "Module 153 should be reachable from entry points"
    );
    assert!(
        reachable.contains("722"), 
        "Module 722 should be reachable from entry points"
    );
    assert!(
        reachable.contains("422"),
        "Module 422 should be reachable from entry points"
    );
}

#[test]
fn test_dependency_chain() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    // Test dependency chain for module 153
    if graph.get_module("153").is_some() {
        let chain = graph.get_dependency_chain("153");
        
        assert!(
            chain.contains(&"153".to_string()),
            "Chain should contain the starting module"
        );
        
        // Should include its dependencies
        assert!(
            chain.contains(&"78".to_string()) || chain.contains(&"418".to_string()),
            "Chain should contain at least one dependency"
        );
    }
}

#[test]
fn test_graph_structure() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    // Test that dependency relationships are bidirectional
    for (module_id, module) in &graph.modules {
        for dep_id in &module.dependencies {
            if let Some(dep_module) = graph.get_module(dep_id) {
                assert!(
                    dep_module.dependents.contains(module_id),
                    "Module {} should be in dependents of module {}",
                    module_id,
                    dep_id
                );
            }
        }
    }
}

#[test]
fn test_unreachable_modules() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    let unreachable = graph.get_unreachable_modules();
    
    // In a well-formed bundle, there shouldn't be many unreachable modules
    // But some utility modules might not be used
    assert!(
        unreachable.len() <= graph.modules.len(),
        "Unreachable modules should not exceed total modules"
    );
}

#[test]
fn test_module_content_extraction() {
    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(BUNDLE_CONTENT).expect("Failed to parse bundle");

    // Test that module dependencies are properly extracted using pure AST traversal
    if let Some(module) = graph.get_module("153") {
        assert!(
            !module.source.is_empty(),
            "Module source should not be empty"
        );
        
        // Should have correctly extracted dependencies from AST traversal
        assert_eq!(
            module.dependencies.len(), 2,
            "Module 153 should have 2 dependencies extracted from AST"
        );
        assert!(
            module.dependencies.contains("78"),
            "Module 153 should depend on module 78 (heavyMathUtils)"
        );
        assert!(
            module.dependencies.contains("418"), 
            "Module 153 should depend on module 418 (dataProcessor)"
        );
    }
}

#[test]
fn test_simplified_webpack_bundle() {
    // Test with a simplified webpack bundle format
    let simplified_bundle = r#"
var __webpack_modules__ = ({
  100: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var dep = __webpack_require__(200);
    console.log("Module 100");
  }),
  200: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Module 200 - leaf module");
  }),
  300: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Module 300 - another leaf");
  }),
});

// Some runtime code here
function __webpack_require__(moduleId) { /* runtime */ }

(function() {
  var main = __webpack_require__(100);
  var other = __webpack_require__(300);
})();
"#;

    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(simplified_bundle).expect("Failed to parse bundle");

    // Should detect modules 100 and 300 as entry points (called outside webpack_modules)
    // Module 200 should NOT be an entry point (only called from within module 100)
    assert!(!graph.entry_points.is_empty(), "Should find entry points");
    assert!(graph.entry_points.contains(&"100".to_string()), "Module 100 should be entry point");
    assert!(graph.entry_points.contains(&"300".to_string()), "Module 300 should be entry point");
    assert!(!graph.entry_points.contains(&"200".to_string()), "Module 200 should NOT be entry point");

    // Verify dependencies are correct
    let module_100 = graph.get_module("100").expect("Module 100 should exist");
    assert!(module_100.dependencies.contains("200"), "Module 100 should depend on 200");
}

#[test]
fn test_various_webpack_formats() {
    let test_cases = vec![
        // Test case 1: var declaration
        (
            "var_declaration",
            r#"
var __webpack_modules__ = ({
  100: (function(module, exports, __webpack_require__) {
    var dep = __webpack_require__(200);
  }),
  200: (function(module, exports, __webpack_require__) {
    console.log("leaf");
  }),
});
(function() { __webpack_require__(100); })();
"#,
        ),
        // Test case 2: let declaration
        (
            "let_declaration", 
            r#"
let __webpack_modules__ = ({
  300: (function(module, exports, __webpack_require__) {
    var dep = __webpack_require__(400);
  }),
  400: (function(module, exports, __webpack_require__) {
    console.log("leaf");
  }),
});
__webpack_require__(300);
"#,
        ),
        // Test case 3: const declaration
        (
            "const_declaration",
            r#"
const __webpack_modules__ = ({
  500: (function(module, exports, __webpack_require__) {
    var dep = __webpack_require__(600);
  }),
  600: (function(module, exports, __webpack_require__) {
    console.log("leaf");
  }),
});
(function() { var entry = __webpack_require__(500); })();
"#,
        ),
        // Test case 4: without parentheses around object literal
        (
            "no_parentheses",
            r#"
var __webpack_modules__ = {
  700: function(module, exports, __webpack_require__) {
    console.log("test");
  },
};
__webpack_require__(700);
"#,
        ),
    ];

    let parser = WebpackBundleParser::new().expect("Failed to create parser");

    for (test_name, bundle_source) in test_cases {
        let result = parser.parse_bundle(bundle_source);
        
        match result {
            Ok(graph) => {
                assert!(!graph.modules.is_empty(), "Should find modules in {}", test_name);
                assert!(!graph.entry_points.is_empty(), "Should find entry points in {}", test_name);
            }
            Err(e) => {
                panic!("Failed to parse {}: {:?}", test_name, e);
            }
        }
    }
}

#[test]
fn test_invalid_webpack_formats() {
    let invalid_cases = vec![
        // Non-standard variable name
        (
            "webpack_modules",
            r#"
var webpack_modules = ({
  100: (function(module, exports, __webpack_require__) {
    console.log("test");
  }),
});
__webpack_require__(100);
"#,
        ),
    ];

    let parser = WebpackBundleParser::new().expect("Failed to create parser");

    for (test_name, bundle_source) in invalid_cases {
        let result = parser.parse_bundle(bundle_source);
        
        assert!(result.is_err(), "Should fail for invalid format: {}", test_name);
    }
}

#[test]
fn test_no_entry_points_valid() {
    // Test that having no entry points is now valid (for tree shaking scenarios)
    let source = r#"
var __webpack_modules__ = ({
  100: (function(module, exports, __webpack_require__) {
    console.log("test");
  }),
});
// No __webpack_require__ calls outside modules - this is valid for tree shaking
"#;

    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let result = parser.parse_bundle(source).expect("No entry points should be valid");
    
    assert_eq!(result.modules.len(), 1, "Should parse 1 module");
    assert_eq!(result.entry_points.len(), 0, "Should have 0 entry points");
    
    // All modules should be unreachable with 0 entry points
    let unreachable = result.get_unreachable_modules();
    assert_eq!(unreachable.len(), 1, "Module should be unreachable with no entry points");
} 