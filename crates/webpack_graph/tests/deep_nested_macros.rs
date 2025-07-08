use webpack_graph::{WebpackBundleParser, TreeShaker, Result};

#[test]
fn test_deep_nested_macros_complete_graph() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");
    
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Should find all 21 modules
    assert_eq!(graph.modules.len(), 21);
    
    // Entry points should be moduleA, moduleB, moduleC
    assert_eq!(graph.entry_points.len(), 3);
    assert!(graph.entry_points.contains(&"moduleA".to_string()));
    assert!(graph.entry_points.contains(&"moduleB".to_string()));
    assert!(graph.entry_points.contains(&"moduleC".to_string()));

    // Verify key modules exist
    let key_modules = vec![
        "moduleA", "moduleB", "moduleC",
        "moduleA1", "moduleA2", "moduleB1", "moduleC1", "moduleC2",
        "moduleA1_1", "moduleA1_2", "moduleA2_1", "moduleA2_2", "moduleB1_1", "moduleB1_2",
        "sharedDeepUtility", "deepUtility1", "deepUtility2", "moduleB1_2_Deep",
        "leafUtility1", "leafUtility2", "leafUtility3"
    ];
    
    for module_id in &key_modules {
        assert!(graph.get_module(module_id).is_some(), "Module {} should exist", module_id);
    }

    // Verify complex dependency relationships
    // moduleA should depend on moduleA1 and moduleA2
    let module_a = graph.get_module("moduleA").unwrap();
    assert!(module_a.dependencies.contains("moduleA1"));
    assert!(module_a.dependencies.contains("moduleA2"));
    
    // moduleA1 should depend on moduleA1_1 and moduleA1_2
    let module_a1 = graph.get_module("moduleA1").unwrap();
    assert!(module_a1.dependencies.contains("moduleA1_1"));
    assert!(module_a1.dependencies.contains("moduleA1_2"));
    
    // Shared deep utility should be heavily used
    let shared_deep = graph.get_module("sharedDeepUtility").unwrap();
    assert!(shared_deep.dependents.len() >= 4); // Used by A1_1, A2_1, B1_1, C1
    assert!(shared_deep.dependencies.contains("deepUtility1"));
    assert!(shared_deep.dependencies.contains("deepUtility2"));
    
    // Verify deep dependency chains
    let deep_util1 = graph.get_module("deepUtility1").unwrap();
    assert!(deep_util1.dependencies.contains("leafUtility1"));
    
    // All modules should be reachable
    let reachable = graph.get_reachable_modules();
    assert_eq!(reachable.len(), 21);
    
    Ok(())
}

#[test]
fn test_deep_nested_macros_with_top_level_graph() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros-with-top-level.js");
    
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Should find all 21 modules (same structure as regular deep nested)
    assert_eq!(graph.modules.len(), 21);
    
    // Entry points should be moduleA, moduleB, moduleC
    assert_eq!(graph.entry_points.len(), 3);
    assert!(graph.entry_points.contains(&"moduleA".to_string()));
    assert!(graph.entry_points.contains(&"moduleB".to_string()));
    assert!(graph.entry_points.contains(&"moduleC".to_string()));

    // Same dependency structure should exist
    let module_a = graph.get_module("moduleA").unwrap();
    assert!(module_a.dependencies.contains("moduleA1"));
    assert!(module_a.dependencies.contains("moduleA2"));
    
    // All modules should be reachable (top-level macros don't affect dependency graph)
    let reachable = graph.get_reachable_modules();
    assert_eq!(reachable.len(), 21);
    
    Ok(())
}

#[test]
fn test_cascading_tree_shaking_scenarios() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");
    
    let parser = WebpackBundleParser::new()?;
    let mut graph = parser.parse_bundle(bundle_content)?;

    // Scenario 1: Remove sharedDeepUtility - should cascade to affect multiple chains
    let _initial_count = graph.modules.len();
    TreeShaker::new(&mut graph).remove_module("sharedDeepUtility");
    
    let after_shared_removal = graph.modules.len();
    // Should still have most modules due to hoisted imports
    assert!(after_shared_removal >= 15);
    assert!(graph.get_module("sharedDeepUtility").is_none());
    
    // Modules that used sharedDeepUtility should still exist but may have broken chains
    assert!(graph.get_module("moduleA1_1").is_some());
    assert!(graph.get_module("moduleA2_1").is_some());
    assert!(graph.get_module("moduleB1_1").is_some());
    
    // Scenario 2: Reset and test selective B chain removal
    let mut graph = parser.parse_bundle(bundle_content)?;
    TreeShaker::new(&mut graph).remove_module("moduleB");
    
    let unreachable = graph.get_unreachable_modules();
    
    // B1 should become unreachable
    assert!(unreachable.contains(&"moduleB1".to_string()) || graph.get_module("moduleB1").is_none());
    
    // Scenario 3: Test leaf utility removal effects
    let mut graph = parser.parse_bundle(bundle_content)?;
    TreeShaker::new(&mut graph).remove_module("leafUtility1");
    TreeShaker::new(&mut graph).remove_module("leafUtility2");
    TreeShaker::new(&mut graph).remove_module("leafUtility3");
    
    // Deep utilities might become unreachable if they only depend on removed leaves
    let remaining_leaves = vec!["leafUtility1", "leafUtility2", "leafUtility3"]
        .into_iter()
        .filter(|id| graph.get_module(id).is_some())
        .count();
    
    // Most should be removed
    assert!(remaining_leaves <= 1);
    
    Ok(())
}

#[test]
fn test_complex_dependency_patterns() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");
    
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Test 1: Shared dependency analysis
    let shared_deep = graph.get_module("sharedDeepUtility").unwrap();
    
    // Should be used by multiple modules
    assert!(shared_deep.dependents.len() >= 4);
    let expected_dependents = vec!["moduleA1_1".to_string(), "moduleA2_1".to_string(), "moduleB1_1".to_string(), "moduleC1".to_string()];
    for dependent in &expected_dependents {
        assert!(shared_deep.dependents.contains(dependent), 
               "sharedDeepUtility should be used by {}", dependent);
    }
    
    // Test 2: Multi-level dependency chains
    let module_a_chain = graph.get_dependency_chain("moduleA");
    
    // Should have a deep chain (A -> A1/A2 -> A1_1/A2_1 -> sharedDeepUtility -> deepUtil1/2 -> leafs)
    assert!(module_a_chain.len() >= 6);
    
    // Test 3: Verify hoisted imports create complete dependency graph
    let module_a1 = graph.get_module("moduleA1").unwrap();
    assert_eq!(module_a1.dependencies.len(), 2); // A1_1 and A1_2
    
    let module_b1 = graph.get_module("moduleB1").unwrap();
    assert_eq!(module_b1.dependencies.len(), 2); // B1_1 and B1_2
    
    // Test 4: Circular reference detection (should not exist in this bundle)
    for (module_id, module) in &graph.modules {
        for dep_id in &module.dependencies {
            if let Some(dep_module) = graph.get_module(dep_id) {
                assert!(!dep_module.dependencies.contains(module_id), 
                       "Circular dependency detected: {} <-> {}", module_id, dep_id);
            }
        }
    }
    
    // Test 5: Verify leaf nodes
    let leaf_nodes: Vec<_> = graph.modules.iter()
        .filter(|(_, module)| module.dependencies.is_empty())
        .map(|(id, _)| id.clone())
        .collect();
    
    // Should have the expected leaf utilities plus isolated modules
    let expected_leaves = vec!["leafUtility1".to_string(), "leafUtility2".to_string(), "leafUtility3".to_string(), "moduleA1_2".to_string(), "moduleA2_2".to_string(), "moduleC2".to_string()];
    for leaf in &expected_leaves {
        assert!(leaf_nodes.contains(leaf), "Expected leaf node {} not found", leaf);
    }
    
    Ok(())
}

#[test]
fn test_shared_vs_isolated_modules() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");
    
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Identify shared modules (used by multiple others)
    let shared_modules: Vec<_> = graph.modules.iter()
        .filter(|(_, module)| module.dependents.len() > 1)
        .map(|(id, module)| (id.clone(), module.dependents.len()))
        .collect();
    
    // Verify expected shared modules
    let expected_shared = vec![
        "sharedDeepUtility".to_string(), // Used by A1_1, A2_1, B1_1, C1
        "deepUtility1".to_string(),      // Used via sharedDeepUtility
        "deepUtility2".to_string(),      // Used via sharedDeepUtility
        "leafUtility1".to_string(),      // Used via deepUtility1
        "leafUtility2".to_string(),      // Used via deepUtility2
    ];
    
    for shared_id in &expected_shared {
        let module = graph.get_module(shared_id).unwrap();
        assert!(module.dependents.len() >= 1, "{} should be shared", shared_id);
    }
    
    // Identify isolated modules (not used by others OR don't use others)
    let _isolated_modules: Vec<_> = graph.modules.iter()
        .filter(|(_, module)| module.dependents.is_empty() || module.dependencies.is_empty())
        .map(|(id, module)| (id.clone(), module.dependencies.len(), module.dependents.len()))
        .collect();
    
    // Test tree shaking impact on shared vs isolated
    let mut graph_copy = graph.clone();
    let _initial_shared_count = shared_modules.len();
    
    // Remove an isolated module
    TreeShaker::new(&mut graph_copy).remove_module("moduleC2");
    
    let remaining_count = graph_copy.modules.len();
    
    // Should remove cleanly with minimal impact
    assert_eq!(remaining_count, graph.modules.len() - 1);
    
    // Try removing a shared module
    let mut graph_copy2 = graph.clone();
    TreeShaker::new(&mut graph_copy2).remove_module("sharedDeepUtility");
    
    Ok(())
}

#[test]
fn test_entry_point_variations() -> Result<()> {
    // Test both bundle variants
    let bundles = vec![
        ("deep-nested", include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js")),
        ("deep-nested-top-level", include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros-with-top-level.js")),
    ];
    
    let parser = WebpackBundleParser::new()?;
    
    for (_name, bundle_content) in bundles {
        let graph = parser.parse_bundle(bundle_content)?;
        
        // Both should have same module structure
        assert_eq!(graph.modules.len(), 21);
        assert_eq!(graph.entry_points.len(), 3);
        
        // Entry points should be consistent
        assert!(graph.entry_points.contains(&"moduleA".to_string()));
        assert!(graph.entry_points.contains(&"moduleB".to_string()));
        assert!(graph.entry_points.contains(&"moduleC".to_string()));
        
        // Dependency structure should be identical
        let module_a = graph.get_module("moduleA").unwrap();
        assert_eq!(module_a.dependencies.len(), 2);
        
        let shared_deep = graph.get_module("sharedDeepUtility").unwrap();
        assert!(shared_deep.dependents.len() >= 4);
        
        // All modules reachable
        let reachable = graph.get_reachable_modules();
        assert_eq!(reachable.len(), 21);
    }
    
    Ok(())
}

#[test]
fn test_dependency_depth_analysis() -> Result<()> {
    let bundle_content = include_str!("../../../test-cases/webpack-bundles/bundle-deep-nested-macros.js");
    
    let parser = WebpackBundleParser::new()?;
    let graph = parser.parse_bundle(bundle_content)?;

    // Calculate dependency depths for each entry point
    for entry_id in &graph.entry_points {
        let chain = graph.get_dependency_chain(entry_id);
        // Should have substantial depth due to nested structure
        assert!(chain.len() >= 5, "Entry {} should have deep dependencies", entry_id);
    }
    
    // Find the deepest dependency chain
    let max_depth = graph.entry_points.iter()
        .map(|entry| graph.get_dependency_chain(entry).len())
        .max()
        .unwrap_or(0);
    
    // Should have deep nesting due to A->A1->A1_1->shared->deep->leaf structure
    assert!(max_depth >= 6, "Should have deep dependency chains");
    
    // Verify specific depth expectations
    let module_a_depth = graph.get_dependency_chain("moduleA").len();
    let module_b_depth = graph.get_dependency_chain("moduleB").len();
    let module_c_depth = graph.get_dependency_chain("moduleC").len();
    
    // Module A should have deepest chain (A->A1->A1_1->shared->deep->leaf)
    assert!(module_a_depth >= 6);
    
    // Module B should have moderate depth (B->B1->B1_1->shared->deep->leaf)
    assert!(module_b_depth >= 4);
    
    // Module C should have moderate depth (C->C1->shared->deep->leaf)
    assert!(module_c_depth >= 4);
    
    Ok(())
} 