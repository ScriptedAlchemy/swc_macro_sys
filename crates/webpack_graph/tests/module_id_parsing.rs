use webpack_graph::{WebpackBundleParser, TreeShaker, Result};

#[test]
fn test_string_module_ids_simple() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "main": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var dep = __webpack_require__("utils");
    console.log("Main module");
  }),
  "utils": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Utils module - leaf");
  }),
  "unused": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Unused module");
  }),
});

__webpack_require__("main");
"#;

    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Should parse all 3 modules with string IDs
    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("main").is_some());
    assert!(graph.get_module("utils").is_some());
    assert!(graph.get_module("unused").is_some());

    // Should detect main as entry point
    assert_eq!(graph.entry_points.len(), 1);
    assert!(graph.entry_points.contains(&"main".to_string()));

    // Should detect dependency relationship
    let main_module = graph.get_module("main").unwrap();
    assert!(main_module.dependencies.contains("utils"));

    let utils_module = graph.get_module("utils").unwrap();
    assert!(utils_module.dependents.contains("main"));

    // Should identify unused module
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["unused".to_string()]);

    Ok(())
}

#[test]
fn test_mixed_module_id_formats() -> Result<()> {
    // Test bundle with mixed numeric and string module IDs
    let bundle_content = r#"
var __webpack_modules__ = ({
  100: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var stringDep = __webpack_require__("stringModule");
    var numericDep = __webpack_require__(200);
  }),
  "stringModule": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var numericDep = __webpack_require__(200);
  }),
  200: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Numeric module - shared");
  }),
  "isolated": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Isolated string module");
  }),
});

__webpack_require__(100);
__webpack_require__("stringModule");
"#;

    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Should parse all 4 modules
    assert_eq!(graph.modules.len(), 4);
    assert!(graph.get_module("100").is_some());
    assert!(graph.get_module("stringModule").is_some());
    assert!(graph.get_module("200").is_some());
    assert!(graph.get_module("isolated").is_some());

    // Should have 2 entry points
    assert_eq!(graph.entry_points.len(), 2);
    assert!(graph.entry_points.contains(&"100".to_string()));
    assert!(graph.entry_points.contains(&"stringModule".to_string()));

    // Should detect shared dependencies
    let module_200 = graph.get_module("200").unwrap();
    assert_eq!(module_200.dependents.len(), 2);
    assert!(module_200.dependents.contains("100"));
    assert!(module_200.dependents.contains("stringModule"));

    // Should identify isolated module
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["isolated".to_string()]);

    Ok(())
}

#[test]
fn test_string_module_ids_complex_names() -> Result<()> {
    // Test with realistic module names
    let bundle_content = r#"
var __webpack_modules__ = ({
  "src/components/Header": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var utils = __webpack_require__("src/utils/helpers");
    console.log("Header component");
  }),
  "src/utils/helpers": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    var config = __webpack_require__("src/config/app");
    console.log("Helper utilities");
  }),
  "src/config/app": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("App configuration");
  }),
  "src/components/Footer": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log("Footer component - unused");
  }),
});

__webpack_require__("src/components/Header");
"#;

    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Should parse all 4 modules
    assert_eq!(graph.modules.len(), 4);
    
    // Should have complex string IDs
    assert!(graph.get_module("src/components/Header").is_some());
    assert!(graph.get_module("src/utils/helpers").is_some());
    assert!(graph.get_module("src/config/app").is_some());
    assert!(graph.get_module("src/components/Footer").is_some());

    // Should detect dependency chain
    let header = graph.get_module("src/components/Header").unwrap();
    assert!(header.dependencies.contains("src/utils/helpers"));

    let helpers = graph.get_module("src/utils/helpers").unwrap();
    assert!(helpers.dependencies.contains("src/config/app"));
    assert!(helpers.dependents.contains("src/components/Header"));

    // Should identify Footer as unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["src/components/Footer".to_string()]);

    Ok(())
}

#[test]
fn test_string_vs_numeric_dependency_extraction() -> Result<()> {
    // Test that dependency extraction works for both formats
    let numeric_bundle = r#"
var __webpack_modules__ = ({
  100: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__(200);
    __webpack_require__(300);
  }),
  200: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {}),
  300: (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {}),
});
__webpack_require__(100);
"#;

    let string_bundle = r#"
var __webpack_modules__ = ({
  "entry": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__("moduleA");
    __webpack_require__("moduleB");
  }),
  "moduleA": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {}),
  "moduleB": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {}),
});
__webpack_require__("entry");
"#;

    let parser = WebpackBundleParser::new()?;
    
    // Test numeric bundle
    let numeric_graph = parser.parse_bundle(numeric_bundle)?;
    assert_eq!(numeric_graph.modules.len(), 3);
    let numeric_entry = numeric_graph.get_module("100").unwrap();
    assert_eq!(numeric_entry.dependencies.len(), 2);
    assert!(numeric_entry.dependencies.contains("200"));
    assert!(numeric_entry.dependencies.contains("300"));

    // Test string bundle
    let string_graph = parser.parse_bundle(string_bundle)?;
    assert_eq!(string_graph.modules.len(), 3);
    let string_entry = string_graph.get_module("entry").unwrap();
    assert_eq!(string_entry.dependencies.len(), 2);
    assert!(string_entry.dependencies.contains("moduleA"));
    assert!(string_entry.dependencies.contains("moduleB"));

    Ok(())
}

#[test]
fn test_shake_string_module_ids_simple() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "entry": (function (module, exports, __webpack_require__) {
    var dep = __webpack_require__("dependency");
  }),
  "dependency": (function (module, exports, __webpack_require__) {
    console.log("Dependency module");
  }),
  "deadCode": (function (module, exports, __webpack_require__) {
    console.log("Dead code module - unreachable");
  })
});
__webpack_require__("entry");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("deadCode").is_some());

    let shaken_ids = TreeShaker::new(&mut graph).shake();

    assert_eq!(shaken_ids, vec!["deadCode".to_string()]);
    assert_eq!(graph.modules.len(), 2);
    assert!(graph.get_module("entry").is_some());
    assert!(graph.get_module("dependency").is_some());
    assert!(graph.get_module("deadCode").is_none());

    Ok(())
}

#[test]
fn test_shake_mixed_module_id_formats() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  100: (function(m,e,__webpack_require__){ 
    __webpack_require__("stringDep"); 
    __webpack_require__(200);
  }),
  "stringDep": (function(m,e,__webpack_require__){ 
    __webpack_require__(200);
  }),
  200: (function(m,e,__webpack_require__){}),
  "isolatedString": (function(m,e,__webpack_require__){}),
  300: (function(m,e,__webpack_require__){})
});
__webpack_require__(100);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 5);
    
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["300", "isolatedString"]);

    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();

    assert_eq!(shaken_ids, vec!["300".to_string(), "isolatedString".to_string()]);
    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("100").is_some());
    assert!(graph.get_module("stringDep").is_some());
    assert!(graph.get_module("200").is_some());
    assert!(graph.get_module("isolatedString").is_none());
    assert!(graph.get_module("300").is_none());

    Ok(())
}

#[test]
fn test_shake_realistic_string_module_names() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "src/main": (function(m,e,__webpack_require__){ 
    __webpack_require__("src/components/App"); 
  }),
  "src/components/App": (function(m,e,__webpack_require__){ 
    __webpack_require__("src/utils/helpers");
    __webpack_require__("src/components/Header");
  }),
  "src/components/Header": (function(m,e,__webpack_require__){ 
    __webpack_require__("src/utils/helpers");
  }),
  "src/utils/helpers": (function(m,e,__webpack_require__){}),
  "src/components/Footer": (function(m,e,__webpack_require__){}), // unreachable
  "src/legacy/oldCode": (function(m,e,__webpack_require__){})     // unreachable
});
__webpack_require__("src/main");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 6);
    
    // Check dependency relationships
    let app_module = graph.get_module("src/components/App").unwrap();
    assert!(app_module.dependencies.contains("src/utils/helpers"));
    assert!(app_module.dependencies.contains("src/components/Header"));

    let helpers_module = graph.get_module("src/utils/helpers").unwrap();
    assert_eq!(helpers_module.dependents.len(), 2); // Used by App and Header

    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["src/components/Footer", "src/legacy/oldCode"]);

    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();

    assert_eq!(shaken_ids, vec!["src/components/Footer".to_string(), "src/legacy/oldCode".to_string()]);
    assert_eq!(graph.modules.len(), 4);
    
    // Verify remaining modules
    assert!(graph.get_module("src/main").is_some());
    assert!(graph.get_module("src/components/App").is_some());
    assert!(graph.get_module("src/components/Header").is_some());
    assert!(graph.get_module("src/utils/helpers").is_some());
    assert!(graph.get_module("src/components/Footer").is_none());
    assert!(graph.get_module("src/legacy/oldCode").is_none());

    Ok(())
}

#[test]
fn test_string_module_circular_dependencies() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "entry": (function(m,e,__webpack_require__){ __webpack_require__("moduleA"); }),
  "moduleA": (function(m,e,__webpack_require__){ __webpack_require__("moduleB"); }),
  "moduleB": (function(m,e,__webpack_require__){ __webpack_require__("moduleA"); }), // circular
  "isolated": (function(m,e,__webpack_require__){}) // unreachable
});
__webpack_require__("entry");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Verify circular relationship
    assert!(graph.get_module("moduleA").unwrap().dependencies.contains("moduleB"));
    assert!(graph.get_module("moduleB").unwrap().dependencies.contains("moduleA"));

    // Only isolated module should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["isolated".to_string()]);

    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["isolated".to_string()]);
    assert_eq!(graph.modules.len(), 3); // entry, moduleA, moduleB remain

    Ok(())
}

#[test]
fn test_string_module_entry_points_as_dependencies() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "app": (function(m,e,__webpack_require__){ __webpack_require__("shared"); }),
  "shared": (function(m,e,__webpack_require__){}), // also an entry point
  "admin": (function(m,e,__webpack_require__){ __webpack_require__("shared"); }),
  "unused": (function(m,e,__webpack_require__){})
});
__webpack_require__("app");
__webpack_require__("shared"); // shared is both entry and dependency
__webpack_require__("admin");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Should have 3 entry points
    assert_eq!(graph.entry_points.len(), 3);
    assert!(graph.entry_points.contains(&"app".to_string()));
    assert!(graph.entry_points.contains(&"shared".to_string()));
    assert!(graph.entry_points.contains(&"admin".to_string()));

    // Shared should be both entry and dependency
    let shared_module = graph.get_module("shared").unwrap();
    assert_eq!(shared_module.dependents.len(), 2); // app and admin depend on it

    // Only unused should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["unused".to_string()]);

    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["unused".to_string()]);
    assert_eq!(graph.modules.len(), 3); // app, shared, admin remain

    Ok(())
}

#[test]
fn test_string_vs_numeric_tree_shaking_comparison() -> Result<()> {
    // Equivalent bundles with numeric vs string IDs
    let numeric_bundle = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); }),
  2: (function(m,e,__webpack_require__){}),
  3: (function(m,e,__webpack_require__){}), // dead
  4: (function(m,e,__webpack_require__){}) // dead
});
__webpack_require__(1);
"#;

    let string_bundle = r#"
var __webpack_modules__ = ({
  "entry": (function(m,e,__webpack_require__){ __webpack_require__("used"); }),
  "used": (function(m,e,__webpack_require__){}),
  "dead1": (function(m,e,__webpack_require__){}), // dead
  "dead2": (function(m,e,__webpack_require__){}) // dead
});
__webpack_require__("entry");
"#;

    let parser = WebpackBundleParser::new()?;
    
    // Test numeric bundle
    let mut numeric_graph = parser.parse_bundle(numeric_bundle)?;
    let mut numeric_unreachable = numeric_graph.get_unreachable_modules();
    numeric_unreachable.sort();
    assert_eq!(numeric_unreachable, vec!["3", "4"]);
    
    let mut numeric_shaken = TreeShaker::new(&mut numeric_graph).shake();
    numeric_shaken.sort();
    assert_eq!(numeric_shaken, vec!["3", "4"]);
    assert_eq!(numeric_graph.modules.len(), 2);

    // Test string bundle
    let mut string_graph = parser.parse_bundle(string_bundle)?;
    let mut string_unreachable = string_graph.get_unreachable_modules();
    string_unreachable.sort();
    assert_eq!(string_unreachable, vec!["dead1", "dead2"]);
    
    let mut string_shaken = TreeShaker::new(&mut string_graph).shake();
    string_shaken.sort();
    assert_eq!(string_shaken, vec!["dead1", "dead2"]);
    assert_eq!(string_graph.modules.len(), 2);

    // Both should have equivalent results (2 modules remaining after shaking 2)
    assert_eq!(numeric_graph.modules.len(), string_graph.modules.len());
    assert_eq!(numeric_shaken.len(), string_shaken.len());

    Ok(())
} 