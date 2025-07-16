#[cfg(test)]
mod tree_shaker_tests {
    use crate::{parser::WebpackBundleParser, tree_shaker::TreeShaker, graph::ModuleGraph, Result};

    #[test]
    fn test_tree_shaker_with_no_entry_points() -> Result<()> {
        // Test tree shaker behavior when graph has no entry points
        // This tests the core tree shaker logic, not split chunk handling
        let mut graph = ModuleGraph::new();
        
        // Add modules with dependencies but no entry points
        let m1 = crate::graph::ModuleNode::new("module1".to_string(), "function() {}".to_string());
        let mut m2 = crate::graph::ModuleNode::new("module2".to_string(), "function() { __webpack_require__(\"module1\"); }".to_string());
        let m3 = crate::graph::ModuleNode::new("module3".to_string(), "function() {}".to_string());
        
        m2.add_dependency("module1".to_string());
        
        graph.add_module(m1);
        graph.add_module(m2);
        graph.add_module(m3);
        
        // Build dependency relationships
        graph.add_dependency("module2", "module1");
        
        assert_eq!(graph.modules.len(), 3, "Should have 3 modules");
        assert_eq!(graph.entry_points.len(), 0, "Should have no entry points");
        
        // All modules are unreachable without entry points
        let unreachable = graph.get_unreachable_modules();
        assert_eq!(unreachable.len(), 3, "All modules should be unreachable");
        
        // Tree shaker removes all unreachable modules
        let removed_modules = TreeShaker::new(&mut graph).shake();
        
        assert_eq!(removed_modules.len(), 3, 
            "Tree shaker removes all modules when there are no entry points");
        assert_eq!(graph.modules.len(), 0, "No modules remain");
        
        Ok(())
    }

    #[test]
    fn test_tree_shaker_with_entry_points() -> Result<()> {
        // Test tree shaker with entry points and unreachable modules
        let mut graph = ModuleGraph::new();
        
        // Create modules
        let mut entry = crate::graph::ModuleNode::new("1".to_string(), "function() { __webpack_require__(2); }".to_string());
        let used = crate::graph::ModuleNode::new("2".to_string(), "function() {}".to_string());
        let unused = crate::graph::ModuleNode::new("3".to_string(), "function() {}".to_string());
        
        entry.add_dependency("2".to_string());
        
        graph.add_module(entry);
        graph.add_module(used);
        graph.add_module(unused);
        
        // Build relationships and add entry point
        graph.add_dependency("1", "2");
        graph.add_entry_point("1".to_string());
        
        assert_eq!(graph.modules.len(), 3, "Should have 3 modules");
        assert_eq!(graph.entry_points.len(), 1, "Should have 1 entry point");
        
        // Run tree shaker
        let removed_modules = TreeShaker::new(&mut graph).shake();
        
        // Module 3 should be removed
        assert_eq!(removed_modules.len(), 1, "Should remove 1 module");
        assert!(removed_modules.contains(&"3".to_string()), "Module 3 should be removed");
        assert_eq!(graph.modules.len(), 2, "Should have 2 modules remaining");
        
        Ok(())
    }

    #[test]
    fn test_module_graph_dependency_extraction() -> Result<()> {
        // Test that the parser correctly extracts module dependencies
        let cjs_chunk = r#"
"use strict";
exports.ids = ["test-chunk"];
exports.modules = {
    "a.js": function(module, exports, __webpack_require__) {
        var b = __webpack_require__("b.js");
        var c = __webpack_require__("c.js");
    },
    "b.js": function(module, exports) {
        exports.value = "B";
    },
    "c.js": function(module, exports) {
        exports.value = "C";
    }
};
"#;

        let parser = WebpackBundleParser::new()?;
        let graph = parser.parse_bundle(cjs_chunk)?;
        
        // Verify modules were parsed
        assert_eq!(graph.modules.len(), 3, "Should have 3 modules");
        
        // Verify dependencies were extracted
        let module_a = graph.get_module("a.js").expect("Should have a.js");
        assert_eq!(module_a.dependencies.len(), 2, "Module a should have 2 dependencies");
        assert!(module_a.dependencies.contains("b.js"));
        assert!(module_a.dependencies.contains("c.js"));
        
        // Verify dependents were set up
        let module_b = graph.get_module("b.js").expect("Should have b.js");
        assert!(module_b.dependents.contains("a.js"));
        
        let module_c = graph.get_module("c.js").expect("Should have c.js");
        assert!(module_c.dependents.contains("a.js"));
        
        Ok(())
    }

    #[test]
    fn test_split_chunk_parsing() -> Result<()> {
        // Test that split chunks (both JSONP and CJS) are parsed correctly
        // This tests parser functionality, not tree shaking
        let jsonp_chunk = r#"
(self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([["vendor"], {
    "module1.js": function(module, exports) {
        exports.value = "Module 1";
    },
    "module2.js": function(module, exports, __webpack_require__) {
        var m1 = __webpack_require__("module1.js");
        exports.value = m1.value + " + Module 2";
    }
}]);
"#;

        let parser = WebpackBundleParser::new()?;
        let graph = parser.parse_bundle(jsonp_chunk)?;
        
        // Verify parsing
        assert_eq!(graph.modules.len(), 2, "Should parse 2 modules");
        assert_eq!(graph.entry_points.len(), 0, "Split chunks have no entry points");
        
        // Verify module graph was built correctly
        assert!(graph.get_module("module1.js").is_some());
        assert!(graph.get_module("module2.js").is_some());
        
        // Verify dependencies
        let m2 = graph.get_module("module2.js").expect("Should have module2");
        assert!(m2.dependencies.contains("module1.js"), 
            "Module 2 should depend on module 1");
        
        Ok(())
    }

    #[test]
    fn test_complex_dependency_graph() -> Result<()> {
        // Test complex dependency scenarios
        let bundle = r#"
var __webpack_modules__ = {
    1: function(module, exports, __webpack_require__) {
        // Entry point
        __webpack_require__(2);
        __webpack_require__(3);
    },
    2: function(module, exports, __webpack_require__) {
        __webpack_require__(4);
    },
    3: function(module, exports, __webpack_require__) {
        __webpack_require__(4);
        __webpack_require__(5);
    },
    4: function(module, exports) {
        // Shared dependency
        exports.shared = true;
    },
    5: function(module, exports) {
        exports.value = "Module 5";
    },
    6: function(module, exports) {
        // Unreachable
        exports.value = "Module 6";
    }
};
__webpack_require__(1);
"#;

        let parser = WebpackBundleParser::new()?;
        let mut graph = parser.parse_bundle(bundle)?;
        
        // Verify reachability
        let reachable = graph.get_reachable_modules();
        assert_eq!(reachable.len(), 5, "Should have 5 reachable modules");
        assert!(!reachable.contains("6"), "Module 6 should be unreachable");
        
        // Verify module 4 has multiple dependents
        let m4 = graph.get_module("4").expect("Should have module 4");
        assert_eq!(m4.dependents.len(), 2, "Module 4 should have 2 dependents");
        assert!(m4.dependents.contains("2"));
        assert!(m4.dependents.contains("3"));
        
        // Run tree shaker
        let removed = TreeShaker::new(&mut graph).shake();
        assert_eq!(removed.len(), 1, "Should remove 1 module");
        assert!(removed.contains(&"6".to_string()), "Module 6 should be removed");
        
        Ok(())
    }

    #[test]
    fn test_circular_dependencies() -> Result<()> {
        // Test that circular dependencies are handled correctly by tree shaker
        let mut graph = ModuleGraph::new();
        
        // Create circular dependency: a -> b -> c -> a
        let mut a = crate::graph::ModuleNode::new("a".to_string(), "function() { __webpack_require__(\"b\"); }".to_string());
        let mut b = crate::graph::ModuleNode::new("b".to_string(), "function() { __webpack_require__(\"c\"); }".to_string());
        let mut c = crate::graph::ModuleNode::new("c".to_string(), "function() { __webpack_require__(\"a\"); }".to_string());
        let d = crate::graph::ModuleNode::new("d".to_string(), "function() {}".to_string());
        
        a.add_dependency("b".to_string());
        b.add_dependency("c".to_string());
        c.add_dependency("a".to_string());
        
        graph.add_module(a);
        graph.add_module(b);
        graph.add_module(c);
        graph.add_module(d);
        
        // Build relationships
        graph.add_dependency("a", "b");
        graph.add_dependency("b", "c");
        graph.add_dependency("c", "a");
        
        // Add entry point to make circular group reachable
        graph.add_entry_point("a".to_string());
        
        // Only d should be unreachable
        let unreachable = graph.get_unreachable_modules();
        assert_eq!(unreachable, vec!["d".to_string()]);
        
        // Tree shaker should only remove d
        let removed = TreeShaker::new(&mut graph).shake();
        assert_eq!(removed, vec!["d".to_string()]);
        assert_eq!(graph.modules.len(), 3);
        
        Ok(())
    }

    #[test]
    fn test_dependency_chain_analysis() -> Result<()> {
        // Test get_dependency_chain functionality
        let bundle = r#"
var __webpack_modules__ = {
    1: function(module, exports, __webpack_require__) {
        __webpack_require__(2);
    },
    2: function(module, exports, __webpack_require__) {
        __webpack_require__(3);
    },
    3: function(module, exports, __webpack_require__) {
        __webpack_require__(4);
    },
    4: function(module, exports) {
        exports.value = "End of chain";
    }
};
"#;

        let parser = WebpackBundleParser::new()?;
        let graph = parser.parse_bundle(bundle)?;
        
        // Get dependency chain from module 1
        let chain = graph.get_dependency_chain("1");
        assert_eq!(chain.len(), 4, "Chain should include all 4 modules");
        assert_eq!(chain[0], "1");
        assert_eq!(chain[1], "2");
        assert_eq!(chain[2], "3");
        assert_eq!(chain[3], "4");
        
        Ok(())
    }
}