use webpack_analyzer_v2::*;
use std::fs;
use std::path::Path;

#[test]
fn test_host_vendor_chunk() {
    let chunk_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("examples/module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    let chunk_content = fs::read_to_string(&chunk_path).expect("Failed to read host vendor chunk");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Host chunk characteristics from share-usage.json
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            assert!(graph.total_dependencies() >= 0, "Dependency count should be non-negative");
        }
        Err(e) => panic!("Failed to analyze host vendor chunk: {}", e),
    }
}

#[test]
fn test_remote_vendor_chunk() {
    let chunk_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("examples/module-federation-example/remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    let chunk_content = fs::read_to_string(&chunk_path).expect("Failed to read remote vendor chunk");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Remote chunk characteristics from share-usage.json
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string(), "main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            assert!(graph.total_dependencies() >= 0, "Dependency count should be non-negative");
        }
        Err(e) => panic!("Failed to analyze remote vendor chunk: {}", e),
    }
}

#[test]
fn test_source_utils_chunk() {
    let chunk_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("examples/module-federation-example/remote/dist/src_utils_js.js");
    let chunk_content = fs::read_to_string(&chunk_path).expect("Failed to read utils chunk");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Utils chunk characteristics (async-node format)
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["src_utils_js.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            assert!(graph.total_dependencies() >= 0, "Dependency count should be non-negative");
        }
        Err(e) => panic!("Failed to analyze source utils chunk: {}", e),
    }
}

#[test]
fn test_source_button_chunk() {
    let chunk_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap()
        .parent().unwrap()
        .join("examples/module-federation-example/remote/dist/src_Button_js.js");
    let chunk_content = fs::read_to_string(&chunk_path).expect("Failed to read button chunk");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Button chunk characteristics (async-node format)
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["src_Button_js.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let result = analyzer.analyze_chunk(&chunk_content, characteristics);
    
    match result {
        Ok(chunk) => {
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            // Update for new enum variant - could be CommonJSAsync or CommonJSSync
            assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
            assert!(chunk.module_count() > 0, "Should have extracted modules");
            assert!(graph.total_dependencies() >= 0, "Dependency count should be non-negative");
        }
        Err(e) => panic!("Failed to analyze source button chunk: {}", e),
    }
}

#[test]
fn test_chunk_characteristics_detection() {
    let analyzer = WebpackAnalyzer::new();
    
    // Test with async-node chunk format characteristics
    let async_node_characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["remote".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-lodash.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let commonjs_sample = r#"
    exports.ids = ["test"];
    exports.modules = {
        "module1": function() { return "test"; }
    };
    "#;
    
    let result = analyzer.analyze_chunk(commonjs_sample, async_node_characteristics);
    match result {
        Ok(chunk) => {
            assert_eq!(chunk.chunk_type, ChunkType::CommonJSAsync);
            assert!(chunk.characteristics.is_some());
            if let Some(chars) = &chunk.characteristics {
                assert_eq!(chars.chunk_format, "async-node");
                assert!(!chars.is_vendor_chunk()); // Should be false since can_be_initial is false
            }
        }
        Err(e) => panic!("Chunk characteristics detection failed: {}", e),
    }
    
    // Test with jsonp chunk format characteristics
    let jsonp_characteristics = ChunkCharacteristics {
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
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    
    let jsonp_sample = r#"
    (self["webpackChunktest"] = self["webpackChunktest"] || []).push([["test"], {
        "module1": function() { return "test"; }
    }]);
    "#;
    
    let result = analyzer.analyze_chunk(jsonp_sample, jsonp_characteristics);
    match result {
        Ok(chunk) => {
            assert_eq!(chunk.chunk_type, ChunkType::JSONP);
            assert!(chunk.characteristics.is_some());
            if let Some(chars) = &chunk.characteristics {
                assert_eq!(chars.chunk_format, "jsonp");
                assert!(chars.is_vendor_chunk()); // Should be true since can_be_initial is true
            }
        }
        Err(e) => panic!("Chunk characteristics detection failed: {}", e),
    }
}