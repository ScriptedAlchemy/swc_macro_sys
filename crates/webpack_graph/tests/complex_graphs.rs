use webpack_graph::{WebpackBundleParser, TreeShaker};

#[test]
fn test_complex_dependency_graph() {
    // Complex real-world scenario with multiple entry points and shared dependencies
    // 
    // Dependency Graph:
    // Entry: 100 (main app), 200 (admin panel)
    // 100 → [101, 102, 103]         (main app uses auth, utils, components)
    // 200 → [201, 102, 103]         (admin shares utils & components with main)
    // 101 → [104, 105]              (auth uses crypto & user api)
    // 102 → [106]                   (utils uses common)
    // 103 → [106, 107]              (components use common & ui)
    // 201 → [104, 107]              (admin utils shares crypto & ui)
    // 104 → [108]                   (crypto uses hash functions)
    // 105 → [106, 108]              (user api uses common & hash)
    // 107 → [106]                   (ui components use common)
    // 106, 108 are leaves
    
    let complex_bundle = r#"
var __webpack_modules__ = ({
  // Entry point modules
  100: (function(module, exports, __webpack_require__) {
    // Main application entry
    var auth = __webpack_require__(101);
    var utils = __webpack_require__(102);
    var components = __webpack_require__(103);
    console.log("Main app loaded");
  }),
  
  200: (function(module, exports, __webpack_require__) {
    // Admin panel entry  
    var adminUtils = __webpack_require__(201);
    var utils = __webpack_require__(102);  // shared with main
    var components = __webpack_require__(103);  // shared with main
    console.log("Admin panel loaded");
  }),

  // Level 1 dependencies
  101: (function(module, exports, __webpack_require__) {
    // Authentication module
    var crypto = __webpack_require__(104);
    var userApi = __webpack_require__(105);
    console.log("Auth module");
  }),

  102: (function(module, exports, __webpack_require__) {
    // Shared utilities
    var common = __webpack_require__(106);
    console.log("Utils module");
  }),

  103: (function(module, exports, __webpack_require__) {
    // Shared UI components
    var common = __webpack_require__(106);
    var uiLib = __webpack_require__(107);
    console.log("Components module");
  }),

  201: (function(module, exports, __webpack_require__) {
    // Admin-specific utilities
    var crypto = __webpack_require__(104);  // shared with auth
    var uiLib = __webpack_require__(107);   // shared with components
    console.log("Admin utils module");
  }),

  // Level 2 dependencies
  104: (function(module, exports, __webpack_require__) {
    // Cryptographic utilities
    var hash = __webpack_require__(108);
    console.log("Crypto module");
  }),

  105: (function(module, exports, __webpack_require__) {
    // User API client
    var common = __webpack_require__(106);  // shared utility
    var hash = __webpack_require__(108);    // shared with crypto
    console.log("User API module");
  }),

  106: (function(module, exports, __webpack_require__) {
    // Common utilities (leaf node - heavily shared)
    console.log("Common utils - no dependencies");
  }),

  107: (function(module, exports, __webpack_require__) {
    // UI component library
    var common = __webpack_require__(106);  // depends on common utils
    console.log("UI library module");
  }),

  // Level 3 dependencies  
  108: (function(module, exports, __webpack_require__) {
    // Hash functions (leaf node)
    console.log("Hash functions - no dependencies");
  }),
});

// Multiple entry points loaded from different contexts
(function() {
  // Main application bootstrap
  var mainApp = __webpack_require__(100);
})();

(function() {
  // Admin panel bootstrap (separate context)
  var adminPanel = __webpack_require__(200);
})();
"#;

    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(complex_bundle).expect("Failed to parse complex bundle");

    // Verify basic structure - 11 modules total (100,101,102,103,104,105,106,107,108,200,201)
    assert_eq!(graph.modules.len(), 11, "Should have 11 modules total");
    assert_eq!(graph.entry_points.len(), 2, "Should have 2 entry points");
    assert!(graph.entry_points.contains(&"100".to_string()), "Module 100 should be entry point");
    assert!(graph.entry_points.contains(&"200".to_string()), "Module 200 should be entry point");

    // Verify entry point dependencies
    let main_app = graph.get_module("100").expect("Module 100 should exist");
    assert_eq!(main_app.dependencies.len(), 3, "Main app should have 3 direct dependencies");
    assert!(main_app.dependencies.contains("101"), "Main app should depend on auth");
    assert!(main_app.dependencies.contains("102"), "Main app should depend on utils");
    assert!(main_app.dependencies.contains("103"), "Main app should depend on components");

    let admin_panel = graph.get_module("200").expect("Module 200 should exist");
    assert_eq!(admin_panel.dependencies.len(), 3, "Admin panel should have 3 direct dependencies");
    assert!(admin_panel.dependencies.contains("201"), "Admin panel should depend on admin utils");
    assert!(admin_panel.dependencies.contains("102"), "Admin panel should share utils with main");
    assert!(admin_panel.dependencies.contains("103"), "Admin panel should share components with main");

    // Verify shared dependencies
    let utils = graph.get_module("102").expect("Module 102 should exist");
    assert_eq!(utils.dependents.len(), 2, "Utils should be used by 2 modules");
    assert!(utils.dependents.contains("100"), "Utils used by main app");
    assert!(utils.dependents.contains("200"), "Utils used by admin panel");

    let components = graph.get_module("103").expect("Module 103 should exist");
    assert_eq!(components.dependents.len(), 2, "Components should be used by 2 modules");
    assert!(components.dependents.contains("100"), "Components used by main app");
    assert!(components.dependents.contains("200"), "Components used by admin panel");

    // Verify deep dependency sharing
    let crypto = graph.get_module("104").expect("Module 104 should exist");
    assert_eq!(crypto.dependents.len(), 2, "Crypto should be used by 2 modules");
    assert!(crypto.dependents.contains("101"), "Crypto used by auth");
    assert!(crypto.dependents.contains("201"), "Crypto used by admin utils");

    // Verify heavily shared leaf node
    let common = graph.get_module("106").expect("Module 106 should exist");
    assert_eq!(common.dependencies.len(), 0, "Common utils should be leaf node");
    assert_eq!(common.dependents.len(), 4, "Common utils should be used by 4 modules");
    assert!(common.dependents.contains("102"), "Common used by utils");
    assert!(common.dependents.contains("103"), "Common used by components");
    assert!(common.dependents.contains("105"), "Common used by user API");
    assert!(common.dependents.contains("107"), "Common used by UI lib");

    // Verify hash functions sharing
    let hash = graph.get_module("108").expect("Module 108 should exist");
    assert_eq!(hash.dependencies.len(), 0, "Hash functions should be leaf node");
    assert_eq!(hash.dependents.len(), 2, "Hash functions should be used by 2 modules");
    assert!(hash.dependents.contains("104"), "Hash used by crypto");
    assert!(hash.dependents.contains("105"), "Hash used by user API");

    // Verify reachability - all modules should be reachable from entry points
    let reachable = graph.get_reachable_modules();
    assert_eq!(reachable.len(), 11, "All 11 modules should be reachable from entry points");

    // Verify dependency chains
    let main_chain = graph.get_dependency_chain("100");
    assert!(main_chain.len() >= 4, "Main app chain should be at least 4 levels deep");
    assert!(main_chain.contains(&"100".to_string()), "Chain should include entry point");
    assert!(main_chain.contains(&"106".to_string()), "Chain should reach common utils");
    assert!(main_chain.contains(&"108".to_string()), "Chain should reach hash functions");

    // Verify entry points make sense
    let reachable = graph.get_reachable_modules();
    assert_eq!(
        reachable.len(),
        graph.modules.len(),
        "All parsed modules should be reachable from entry points"
    );

    // Check for shared dependencies (modules used by multiple others)
    let shared_deps = graph.modules.values()
        .filter(|m| m.dependents.len() > 1)
        .count();
    assert!(shared_deps >= 4, "Complex graph should have at least 4 shared dependencies");

    // Analyze dependency depth
    let max_depth = graph.entry_points.iter()
        .map(|entry| graph.get_dependency_chain(entry).len())
        .max()
        .unwrap_or(0);

    // Dynamic threshold: any real bundle should have some dependency chains
    // Scale expectation based on project complexity (number of modules)
    let expected_min_depth = if graph.modules.len() > 20 { 3 } else { 2 };
    assert!(max_depth >= expected_min_depth, 
        "Real bundle with {} modules should have dependency chains of at least depth {} (found: {})",
        graph.modules.len(), expected_min_depth, max_depth);
}

#[test]
fn test_string_module_ids_integration() {
    // Test with string module IDs to ensure full integration works
    let string_bundle = r#"
var __webpack_modules__ = ({
  "app/main": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      main: () => (main)
    });
    var _utils = __webpack_require__("utils/helpers");
    var _api = __webpack_require__("services/api");
    function main() {
      console.log('Main app with string module IDs');
    }
  }),
  "utils/helpers": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      helpers: () => (helpers)
    });
    var _config = __webpack_require__("config/app");
    var helpers = {
      format: function(data) { return data; }
    };
  }),
  "services/api": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      api: () => (api)
    });
    var _config = __webpack_require__("config/app");
    var api = {
      fetch: function(url) { return fetch(url); }
    };
  }),
  "config/app": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    __webpack_require__.d(__webpack_exports__, {
      config: () => (config)
    });
    var config = {
      apiUrl: 'https://api.example.com',
      version: '1.0.0'
    };
  }),
  "legacy/oldModule": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
    console.log('Legacy module - should be tree shaken');
  }),
});

// Entry point
(function() {
  var main = __webpack_require__("app/main");
})();
"#;

    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let mut graph = parser.parse_bundle(string_bundle).expect("Failed to parse string module bundle");

    // Should find all 5 modules
    assert_eq!(graph.modules.len(), 5);
    assert!(graph.get_module("app/main").is_some());
    assert!(graph.get_module("utils/helpers").is_some());
    assert!(graph.get_module("services/api").is_some());
    assert!(graph.get_module("config/app").is_some());
    assert!(graph.get_module("legacy/oldModule").is_some());

    // Should detect app/main as entry point
    assert_eq!(graph.entry_points.len(), 1);
    assert!(graph.entry_points.contains(&"app/main".to_string()));

    // Verify dependency relationships
    let main_module = graph.get_module("app/main").unwrap();
    assert!(main_module.dependencies.contains("utils/helpers"));
    assert!(main_module.dependencies.contains("services/api"));

    let helpers_module = graph.get_module("utils/helpers").unwrap();
    assert!(helpers_module.dependencies.contains("config/app"));
    assert!(helpers_module.dependents.contains("app/main"));

    let api_module = graph.get_module("services/api").unwrap();
    assert!(api_module.dependencies.contains("config/app"));
    assert!(api_module.dependents.contains("app/main"));

    let config_module = graph.get_module("config/app").unwrap();
    assert_eq!(config_module.dependents.len(), 2); // Used by helpers and api
    assert!(config_module.dependents.contains("utils/helpers"));
    assert!(config_module.dependents.contains("services/api"));

    // Check reachability - legacy module should be unreachable
    let reachable = graph.get_reachable_modules();
    let unreachable = graph.get_unreachable_modules();
    
    assert_eq!(reachable.len(), 4);
    assert_eq!(unreachable.len(), 1);
    assert_eq!(unreachable[0], "legacy/oldModule");

    // Test tree shaking
    let shaken_modules = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_modules, vec!["legacy/oldModule".to_string()]);
    assert_eq!(graph.modules.len(), 4);
    assert!(graph.get_module("legacy/oldModule").is_none());

    // Verify final graph integrity
    let final_reachable = graph.get_reachable_modules();
    assert_eq!(final_reachable.len(), 4);
    assert_eq!(final_reachable.len(), graph.modules.len());
}

 