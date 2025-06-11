use webpack_graph::{parser::WebpackBundleParser, tree_shaker::TreeShaker, Result};

#[test]
fn test_shake_simple_bundle() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  100: (function (module, exports, __webpack_require__) { // entry
    var dep = __webpack_require__(200);
  }),
  200: (function (module, exports, __webpack_require__) { // dependency
    console.log("Module 200");
  }),
  300: (function (module, exports, __webpack_require__) { // dead code
    console.log("Module 300 - unreachable");
  })
});
__webpack_require__(100);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("300").is_some());

    let shaken_ids = TreeShaker::new(&mut graph).shake();

    assert_eq!(shaken_ids, vec!["300".to_string()]);
    assert_eq!(graph.modules.len(), 2);
    assert!(graph.get_module("100").is_some());
    assert!(graph.get_module("200").is_some());
    assert!(graph.get_module("300").is_none());

    Ok(())
}

#[test]
fn test_shake_dependency_chain_with_dead_branch() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); }), // entry
  2: (function(m,e,__webpack_require__){ __webpack_require__(3); }), // A
  3: (function(m,e,__webpack_require__){}),       // B (leaf)
  4: (function(m,e,__webpack_require__){ __webpack_require__(5); }), // C (unreachable branch root)
  5: (function(m,e,__webpack_require__){})        // D (unreachable leaf)
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 5);
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["4".to_string(), "5".to_string()]);

    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();

    assert_eq!(shaken_ids, vec!["4".to_string(), "5".to_string()]);
    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("1").is_some());
    assert!(graph.get_module("2").is_some());
    assert!(graph.get_module("3").is_some());
    assert!(graph.get_module("4").is_none());
    assert!(graph.get_module("5").is_none());

    Ok(())
}

#[test]
fn test_shake_no_unreachable_modules() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); __webpack_require__(3); }), // entry
  2: (function(m,e,__webpack_require__){}),
  3: (function(m,e,__webpack_require__){})
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_unreachable_modules().is_empty());

    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert!(shaken_ids.is_empty());
    assert_eq!(graph.modules.len(), 3);

    Ok(())
}

#[test]
fn test_remove_module_and_check_graph_integrity() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  10: (function(m,e,__webpack_require__){ __webpack_require__(20); __webpack_require__(30); }), // entry
  20: (function(m,e,__webpack_require__){ __webpack_require__(40); }),      // A -> C
  30: (function(m,e,__webpack_require__){ __webpack_require__(40); }),      // B -> C
  40: (function(m,e,__webpack_require__){})              // C (shared)
});
__webpack_require__(10);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Initial state: C (40) is shared by A (20) and B (30)
    assert_eq!(graph.modules.len(), 4);
    let module_c = graph.get_module("40").unwrap();
    assert_eq!(module_c.dependents.len(), 2);
    assert!(module_c.dependents.contains("20"));
    assert!(module_c.dependents.contains("30"));
    
    // Remove module A (20)
    let removed = TreeShaker::new(&mut graph).remove_module("20");
    assert!(removed);
    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("20").is_none());

    // Check C's dependents: A should be gone, B should remain
    let module_c_after_a = graph.get_module("40").unwrap();
    assert_eq!(module_c_after_a.dependents.len(), 1);
    assert!(module_c_after_a.dependents.contains("30"));

    // C should still be reachable via B
    assert!(graph.get_unreachable_modules().is_empty(), "C should still be reachable via B");

    // Now remove module B (30)
    let removed_b = TreeShaker::new(&mut graph).remove_module("30");
    assert!(removed_b);

    // Now C (40) should be unreachable
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["40".to_string()]);

    // Shake the graph
    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();
    assert_eq!(shaken_ids, vec!["40".to_string()]);
    assert_eq!(graph.modules.len(), 1);
    assert!(graph.get_module("10").is_some());
    assert!(graph.get_module("30").is_none());
    assert!(graph.get_module("40").is_none());
    
    Ok(())
}

#[test]
fn test_shake_complex_graph_with_dead_branches() -> Result<()> {
    let complex_bundle = r#"
var __webpack_modules__ = ({
  // === Reachable Modules ===
  // Entry Points
  100: (function(m,e,__webpack_require__){ __webpack_require__(300); __webpack_require__(400); }), // main -> shared_utils, feature_A
  200: (function(m,e,__webpack_require__){ __webpack_require__(300); __webpack_require__(500); }), // admin -> shared_utils, feature_B

  // Shared Libraries
  300: (function(m,e,__webpack_require__){ __webpack_require__(600); }),      // shared_utils -> common_lib
  
  // Features
  400: (function(m,e,__webpack_require__){ __webpack_require__(600); }),      // feature_A -> common_lib
  500: (function(m,e,__webpack_require__){ __webpack_require__(600); }),      // feature_B -> common_lib

  // Core library (heavily shared)
  600: (function(m,e,__webpack_require__){}),              // common_lib (leaf)

  // === Unreachable Modules ===
  // A dead branch that is internally connected but not called from any entry point
  700: (function(m,e,__webpack_require__){ __webpack_require__(800); }),      // dead_feature_root -> dead_feature_util
  800: (function(m,e,__webpack_require__){}),              // dead_feature_util

  // An isolated, completely unused module
  900: (function(m,e,__webpack_require__){})               // isolated_dead_module
});

// Bootstrap: Only main and admin are called, making 700, 800, 900 unreachable
__webpack_require__(100);
__webpack_require__(200);
"#;

    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(complex_bundle)?;

    // 1. Verify initial state
    assert_eq!(graph.modules.len(), 9, "Should parse all 9 modules");
    assert_eq!(graph.entry_points.len(), 2, "Should have 2 entry points");

    // 2. Verify unreachable modules are correctly identified
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(
        unreachable,
        vec!["700", "800", "900"],
        "Should identify the dead branch and the isolated module as unreachable"
    );
    
    // 3. Perform tree-shaking
    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();
    
    // 4. Assert that the correct modules were removed
    assert_eq!(
        shaken_ids,
        vec!["700", "800", "900"],
        "Should report the correct shaken module IDs"
    );
    
    // 5. Assert the final state of the graph
    assert_eq!(graph.modules.len(), 6, "Graph should have 6 modules remaining");
    assert!(graph.get_module("700").is_none());
    assert!(graph.get_module("800").is_none());
    assert!(graph.get_module("900").is_none());
    
    // 6. Verify integrity of the remaining graph
    let common_lib = graph.get_module("600").unwrap();
    assert_eq!(
        common_lib.dependents.len(), 
        3, 
        "common_lib should still have 3 dependents (shared_utils, feature_A, feature_B)"
    );
    assert!(common_lib.dependents.contains("300"));
    assert!(common_lib.dependents.contains("400"));
    assert!(common_lib.dependents.contains("500"));

    let shared_utils = graph.get_module("300").unwrap();
    assert_eq!(
        shared_utils.dependents.len(),
        2,
        "shared_utils should have 2 dependents (main, admin)"
    );
    assert!(shared_utils.dependents.contains("100"));
    assert!(shared_utils.dependents.contains("200"));
    
    Ok(())
}

#[test]
fn test_debug_tree_shaking_process() -> Result<()> {
    let complex_bundle = r#"
var __webpack_modules__ = ({
  // Reachable chain: 100 -> 300 -> 600
  100: (function(m,e,__webpack_require__){ 
    console.log("Entry point main");
    __webpack_require__(300); 
  }),
  300: (function(m,e,__webpack_require__){ 
    console.log("Shared utils");
    __webpack_require__(600); 
  }),
  600: (function(m,e,__webpack_require__){ 
    console.log("Common lib - leaf");
  }),
  
  // Unreachable chain: 700 -> 800
  700: (function(m,e,__webpack_require__){ 
    console.log("Dead feature");
    __webpack_require__(800); 
  }),
  800: (function(m,e,__webpack_require__){ 
    console.log("Dead utility");
  }),
  
  // Isolated unreachable module
  900: (function(m,e,__webpack_require__){ 
    console.log("Isolated dead module");
  })
});

__webpack_require__(100);
"#;

    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(complex_bundle)?;

    // Show reachability analysis
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    
    // Perform tree-shaking with step-by-step output
    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();
    
    // Verify integrity
    let final_reachable = graph.get_reachable_modules();
    let final_unreachable = graph.get_unreachable_modules();
    
    // Assertions
    assert_eq!(shaken_ids, vec!["700", "800", "900"]);
    assert_eq!(graph.modules.len(), 3);
    assert!(final_unreachable.is_empty());
    assert_eq!(final_reachable.len(), 3);
    
    Ok(())
}

#[test]
fn test_circular_dependencies() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); }), // entry -> A
  2: (function(m,e,__webpack_require__){ __webpack_require__(3); }), // A -> B  
  3: (function(m,e,__webpack_require__){ __webpack_require__(2); }), // B -> A (circular)
  4: (function(m,e,__webpack_require__){})                           // isolated
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Show circular relationship
    assert!(graph.get_module("2").unwrap().dependencies.contains("3"));
    assert!(graph.get_module("3").unwrap().dependencies.contains("2"));
    assert!(graph.get_module("2").unwrap().dependents.contains("3"));
    assert!(graph.get_module("3").unwrap().dependents.contains("2"));
    
    // Only module 4 should be unreachable
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["4"]);
    
    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["4".to_string()]);
    assert_eq!(graph.modules.len(), 3); // 1, 2, 3 remain
    
    Ok(())
}

#[test]
fn test_entry_point_as_dependency() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  100: (function(m,e,__webpack_require__){ __webpack_require__(200); }), // entry1 -> shared
  200: (function(m,e,__webpack_require__){}),                            // shared module (also entry2)
  300: (function(m,e,__webpack_require__){})                             // isolated
});
__webpack_require__(100); // entry1
__webpack_require__(200); // entry2 (also dependency of entry1)
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Both 100 and 200 should be entry points
    assert_eq!(graph.entry_points.len(), 2);
    assert!(graph.entry_points.contains(&"100".to_string()));
    assert!(graph.entry_points.contains(&"200".to_string()));
    
    // Module 200 should be both entry point and dependency
    assert!(graph.get_module("100").unwrap().dependencies.contains("200"));
    assert!(graph.get_module("200").unwrap().dependents.contains("100"));
    
    // Only module 300 should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["300".to_string()]);
    
    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["300".to_string()]);
    assert_eq!(graph.modules.len(), 2); // 100, 200 remain
    
    Ok(())
}

#[test]
fn test_cascading_tree_shaking() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); }),   // entry -> bridge
  2: (function(m,e,__webpack_require__){ __webpack_require__(3); }),   // bridge -> A
  3: (function(m,e,__webpack_require__){ __webpack_require__(4); }),   // A -> B  
  4: (function(m,e,__webpack_require__){ __webpack_require__(5); }),   // B -> C
  5: (function(m,e,__webpack_require__){}),                            // C (leaf)
  6: (function(m,e,__webpack_require__){ __webpack_require__(7); }),   // isolated chain start
  7: (function(m,e,__webpack_require__){})                             // isolated chain end
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Initially, 6 and 7 should be unreachable
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["6", "7"]);
    
    // Remove bridge module 2 manually - this should make 3,4,5 unreachable too
    TreeShaker::new(&mut graph).remove_module("2");
    
    // Now 3,4,5,6,7 should be unreachable (cascading effect)
    let mut new_unreachable = graph.get_unreachable_modules();
    new_unreachable.sort();
    assert_eq!(new_unreachable, vec!["3", "4", "5", "6", "7"]);
    
    // Shake remaining dead code
    let mut final_shaken = TreeShaker::new(&mut graph).shake();
    final_shaken.sort();
    assert_eq!(final_shaken, vec!["3", "4", "5", "6", "7"]);
    assert_eq!(graph.modules.len(), 1); // Only entry module 1 remains
    
    Ok(())
}

#[test]
fn test_diamond_dependency_pattern() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ 
    __webpack_require__(2); 
    __webpack_require__(3); 
  }),                                                                   // entry -> B, C
  2: (function(m,e,__webpack_require__){ __webpack_require__(4); }),   // B -> D
  3: (function(m,e,__webpack_require__){ __webpack_require__(4); }),   // C -> D  
  4: (function(m,e,__webpack_require__){}),                            // D (shared leaf)
  5: (function(m,e,__webpack_require__){})                             // isolated
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Module 4 should have 2 dependents (B and C)
    let module_d = graph.get_module("4").unwrap();
    assert_eq!(module_d.dependents.len(), 2);
    assert!(module_d.dependents.contains("2"));
    assert!(module_d.dependents.contains("3"));
    
    // Only module 5 should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["5".to_string()]);
    
    // Test removing one branch of the diamond
    TreeShaker::new(&mut graph).remove_module("2");
    
    // Module 4 should still be reachable via module 3
    assert!(graph.get_unreachable_modules().is_empty() || graph.get_unreachable_modules() == vec!["5".to_string()]);
    
    // Now remove the other branch
    TreeShaker::new(&mut graph).remove_module("3");
    
    // Now module 4 should become unreachable
    let mut new_unreachable = graph.get_unreachable_modules();
    new_unreachable.sort();
    assert_eq!(new_unreachable, vec!["4", "5"]);
    
    Ok(())
}

#[test]
fn test_deep_dependency_chain() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); }),
  2: (function(m,e,__webpack_require__){ __webpack_require__(3); }),  
  3: (function(m,e,__webpack_require__){ __webpack_require__(4); }),
  4: (function(m,e,__webpack_require__){ __webpack_require__(5); }),
  5: (function(m,e,__webpack_require__){ __webpack_require__(6); }),
  6: (function(m,e,__webpack_require__){ __webpack_require__(7); }),
  7: (function(m,e,__webpack_require__){ __webpack_require__(8); }),
  8: (function(m,e,__webpack_require__){}),                          // deep leaf
  9: (function(m,e,__webpack_require__){})                           // isolated
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Verify the chain is correctly built
    let dependency_chain = graph.get_dependency_chain("1");
    assert!(dependency_chain.len() >= 8); // Should include all modules in chain
    
    // Only module 9 should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["9".to_string()]);
    
    // Break the chain at module 4
    TreeShaker::new(&mut graph).remove_module("4");
    
    // Modules 5,6,7,8,9 should now be unreachable
    let mut new_unreachable = graph.get_unreachable_modules();
    new_unreachable.sort();
    assert_eq!(new_unreachable, vec!["5", "6", "7", "8", "9"]);
    
    // Shake remaining dead code
    let mut shaken = TreeShaker::new(&mut graph).shake();
    shaken.sort();
    assert_eq!(shaken, vec!["5", "6", "7", "8", "9"]);
    assert_eq!(graph.modules.len(), 3); // 1,2,3 remain
    
    Ok(())
}

#[test]
fn test_shake_string_module_ids_simple() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "./main.js": (function (module, exports, __webpack_require__) { 
    var dep = __webpack_require__("./utils.js");
  }),
  "./utils.js": (function (module, exports, __webpack_require__) { 
    console.log("Utils module");
  }),
  "./unused.js": (function (module, exports, __webpack_require__) { 
    console.log("Unused module - should be tree shaken");
  })
});
__webpack_require__("./main.js");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("./unused.js").is_some());

    let shaken_ids = TreeShaker::new(&mut graph).shake();

    assert_eq!(shaken_ids, vec!["./unused.js".to_string()]);
    assert_eq!(graph.modules.len(), 2);
    assert!(graph.get_module("./main.js").is_some());
    assert!(graph.get_module("./utils.js").is_some());
    assert!(graph.get_module("./unused.js").is_none());

    Ok(())
}

#[test]
fn test_shake_mixed_module_id_formats() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__("./feature.js"); }), // numeric entry
  "./feature.js": (function(m,e,__webpack_require__){ __webpack_require__(300); }), // string -> numeric
  300: (function(m,e,__webpack_require__){}),                                        // numeric dependency
  "./dead-feature.js": (function(m,e,__webpack_require__){}),                       // string dead
  999: (function(m,e,__webpack_require__){})                                        // numeric dead
});
__webpack_require__(1);
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 5);
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["./dead-feature.js".to_string(), "999".to_string()]);

    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();

    assert_eq!(shaken_ids, vec!["./dead-feature.js".to_string(), "999".to_string()]);
    assert_eq!(graph.modules.len(), 3);
    assert!(graph.get_module("1").is_some());
    assert!(graph.get_module("./feature.js").is_some());
    assert!(graph.get_module("300").is_some());
    assert!(graph.get_module("./dead-feature.js").is_none());
    assert!(graph.get_module("999").is_none());

    Ok(())
}

#[test]
fn test_shake_realistic_string_module_names() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "./src/index.js": (function(m,e,__webpack_require__){ 
    __webpack_require__("./src/components/App.jsx"); 
    __webpack_require__("./src/utils/helpers.js");
  }),
  "./src/components/App.jsx": (function(m,e,__webpack_require__){ 
    __webpack_require__("./src/components/Header.jsx");
  }),
  "./src/components/Header.jsx": (function(m,e,__webpack_require__){}),
  "./src/utils/helpers.js": (function(m,e,__webpack_require__){}),
  "./src/features/unused-feature.js": (function(m,e,__webpack_require__){ 
    __webpack_require__("./src/features/unused-helper.js");
  }),
  "./src/features/unused-helper.js": (function(m,e,__webpack_require__){}),
  "./src/legacy/old-code.js": (function(m,e,__webpack_require__){})
});
__webpack_require__("./src/index.js");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    assert_eq!(graph.modules.len(), 7, "Should parse all 7 modules");
    assert_eq!(graph.entry_points.len(), 1, "Should have 1 entry point");

    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(
        unreachable,
        vec![
            "./src/features/unused-feature.js", 
            "./src/features/unused-helper.js", 
            "./src/legacy/old-code.js"
        ],
        "Should identify 3 unreachable modules"
    );
    
    let mut shaken_ids = TreeShaker::new(&mut graph).shake();
    shaken_ids.sort();
    
    assert_eq!(
        shaken_ids,
        vec![
            "./src/features/unused-feature.js", 
            "./src/features/unused-helper.js", 
            "./src/legacy/old-code.js"
        ],
        "Should tree shake the 3 unreachable modules"
    );
    
    assert_eq!(graph.modules.len(), 4, "Should have 4 modules remaining");
    assert!(graph.get_module("./src/index.js").is_some());
    assert!(graph.get_module("./src/components/App.jsx").is_some());
    assert!(graph.get_module("./src/components/Header.jsx").is_some());
    assert!(graph.get_module("./src/utils/helpers.js").is_some());
    assert!(graph.get_module("./src/features/unused-feature.js").is_none());
    assert!(graph.get_module("./src/features/unused-helper.js").is_none());
    assert!(graph.get_module("./src/legacy/old-code.js").is_none());
    
    Ok(())
}

#[test]
fn test_string_module_circular_dependencies() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "./entry.js": (function(m,e,__webpack_require__){ __webpack_require__("./moduleA.js"); }),
  "./moduleA.js": (function(m,e,__webpack_require__){ __webpack_require__("./moduleB.js"); }),
  "./moduleB.js": (function(m,e,__webpack_require__){ __webpack_require__("./moduleA.js"); }), // circular
  "./isolated.js": (function(m,e,__webpack_require__){})
});
__webpack_require__("./entry.js");
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Verify circular dependency
    assert!(graph.get_module("./moduleA.js").unwrap().dependencies.contains("./moduleB.js"));
    assert!(graph.get_module("./moduleB.js").unwrap().dependencies.contains("./moduleA.js"));
    assert!(graph.get_module("./moduleA.js").unwrap().dependents.contains("./moduleB.js"));
    assert!(graph.get_module("./moduleB.js").unwrap().dependents.contains("./moduleA.js"));
    
    // Only isolated module should be unreachable
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    assert_eq!(unreachable, vec!["./isolated.js"]);
    
    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["./isolated.js".to_string()]);
    assert_eq!(graph.modules.len(), 3); // entry, moduleA, moduleB remain
    
    Ok(())
}

#[test]
fn test_string_module_entry_points_as_dependencies() -> Result<()> {
    let bundle_content = r#"
var __webpack_modules__ = ({
  "./main.js": (function(m,e,__webpack_require__){ __webpack_require__("./shared.js"); }),
  "./shared.js": (function(m,e,__webpack_require__){}),                                     // also an entry point
  "./unused.js": (function(m,e,__webpack_require__){})
});
__webpack_require__("./main.js");
__webpack_require__("./shared.js"); // also entry point
"#;
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Both should be entry points
    assert_eq!(graph.entry_points.len(), 2);
    assert!(graph.entry_points.contains(&"./main.js".to_string()));
    assert!(graph.entry_points.contains(&"./shared.js".to_string()));
    
    // Module shared.js should be both entry point and dependency
    assert!(graph.get_module("./main.js").unwrap().dependencies.contains("./shared.js"));
    assert!(graph.get_module("./shared.js").unwrap().dependents.contains("./main.js"));
    
    // Only unused should be unreachable
    let unreachable = graph.get_unreachable_modules();
    assert_eq!(unreachable, vec!["./unused.js".to_string()]);
    
    let shaken_ids = TreeShaker::new(&mut graph).shake();
    assert_eq!(shaken_ids, vec!["./unused.js".to_string()]);
    assert_eq!(graph.modules.len(), 2); // main.js, shared.js remain
    
    Ok(())
}

#[test]
fn test_string_vs_numeric_tree_shaking_comparison() -> Result<()> {
    let numeric_bundle = r#"
var __webpack_modules__ = ({
  1: (function(m,e,__webpack_require__){ __webpack_require__(2); __webpack_require__(3); }),
  2: (function(m,e,__webpack_require__){}),
  3: (function(m,e,__webpack_require__){}),
  4: (function(m,e,__webpack_require__){}) // dead code
});
__webpack_require__(1);
"#;

    let string_bundle = r#"
var __webpack_modules__ = ({
  "./main.js": (function(m,e,__webpack_require__){ __webpack_require__("./utils.js"); __webpack_require__("./components.js"); }),
  "./utils.js": (function(m,e,__webpack_require__){}),
  "./components.js": (function(m,e,__webpack_require__){}),
  "./dead.js": (function(m,e,__webpack_require__){}) // dead code
});
__webpack_require__("./main.js");
"#;

    let parser = WebpackBundleParser::new()?;
    
    // Test numeric IDs
    let mut numeric_graph = parser.parse_bundle(numeric_bundle)?;
    assert_eq!(numeric_graph.modules.len(), 4);
    let numeric_shaken = TreeShaker::new(&mut numeric_graph).shake();
    assert_eq!(numeric_shaken, vec!["4".to_string()]);
    assert_eq!(numeric_graph.modules.len(), 3);
    
    // Test string IDs  
    let mut string_graph = parser.parse_bundle(string_bundle)?;
    assert_eq!(string_graph.modules.len(), 4);
    let string_shaken = TreeShaker::new(&mut string_graph).shake();
    assert_eq!(string_shaken, vec!["./dead.js".to_string()]);
    assert_eq!(string_graph.modules.len(), 3);
    
    // Both should behave identically in terms of tree shaking effectiveness
    assert_eq!(numeric_shaken.len(), string_shaken.len(), "Both should shake same number of modules");
    
    Ok(())
} 