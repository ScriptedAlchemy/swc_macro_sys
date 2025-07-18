use webpack_analyzer_v2::*;
use std::fs;
use swc_core::atoms::Atom;

#[test]
fn test_host_vendor_chunk() {
    let chunk_path = "/Users/bytedance/dev/swc_macro_sys/module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js";
    let chunk_content = fs::read_to_string(chunk_path).expect("Failed to read host vendor chunk");
    
    println!("Testing host vendor chunk...");
    
    let analyzer = WebpackAnalyzer::new();
    let result = analyzer.analyze_chunk(&chunk_content);
    
    match result {
        Ok(chunk) => {
            println!("✓ Successfully analyzed host vendor chunk");
            println!("  Chunk type: {:?}", chunk.chunk_type);
            println!("  Module count: {}", chunk.module_count());
            
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            println!("  Total dependencies: {}", graph.total_dependencies());
            
            // Show some sample modules
            let sample_modules: Vec<_> = chunk.modules.keys().take(5).collect();
            println!("  Sample modules: {:?}", sample_modules);
            
            assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
            assert!(chunk.module_count() > 0, "Should have extracted modules");
        }
        Err(e) => {
            panic!("Failed to analyze host vendor chunk: {}", e);
        }
    }
}

#[test]
fn test_remote_vendor_chunk() {
    let chunk_path = "/Users/bytedance/dev/swc_macro_sys/module-federation-example/remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js";
    let chunk_content = fs::read_to_string(chunk_path).expect("Failed to read remote vendor chunk");
    
    println!("Testing remote vendor chunk...");
    
    let analyzer = WebpackAnalyzer::new();
    let result = analyzer.analyze_chunk(&chunk_content);
    
    match result {
        Ok(chunk) => {
            println!("✓ Successfully analyzed remote vendor chunk");
            println!("  Chunk type: {:?}", chunk.chunk_type);
            println!("  Module count: {}", chunk.module_count());
            
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            println!("  Total dependencies: {}", graph.total_dependencies());
            
            // Show some sample modules
            let sample_modules: Vec<_> = chunk.modules.keys().take(5).collect();
            println!("  Sample modules: {:?}", sample_modules);
            
            assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
            assert!(chunk.module_count() > 0, "Should have extracted modules");
        }
        Err(e) => {
            panic!("Failed to analyze remote vendor chunk: {}", e);
        }
    }
}

#[test]
fn test_source_utils_chunk() {
    let chunk_path = "/Users/bytedance/dev/swc_macro_sys/module-federation-example/remote/dist/src_utils_js.js";
    let chunk_content = fs::read_to_string(chunk_path).expect("Failed to read utils chunk");
    
    println!("Testing source utils chunk...");
    
    let analyzer = WebpackAnalyzer::new();
    let result = analyzer.analyze_chunk(&chunk_content);
    
    match result {
        Ok(chunk) => {
            println!("✓ Successfully analyzed source utils chunk");
            println!("  Chunk type: {:?}", chunk.chunk_type);
            println!("  Module count: {}", chunk.module_count());
            
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            println!("  Total dependencies: {}", graph.total_dependencies());
            
            // Show module details
            for (module_id, module) in &chunk.modules {
                println!("  Module: {}", module_id);
                println!("    Dependencies: {:?}", module.dependencies);
            }
            
            assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
            assert!(chunk.module_count() > 0, "Should have extracted modules");
        }
        Err(e) => {
            panic!("Failed to analyze source utils chunk: {}", e);
        }
    }
}

#[test]
fn test_source_button_chunk() {
    let chunk_path = "/Users/bytedance/dev/swc_macro_sys/module-federation-example/remote/dist/src_Button_js.js";
    let chunk_content = fs::read_to_string(chunk_path).expect("Failed to read button chunk");
    
    println!("Testing source button chunk...");
    
    let analyzer = WebpackAnalyzer::new();
    let result = analyzer.analyze_chunk(&chunk_content);
    
    match result {
        Ok(chunk) => {
            println!("✓ Successfully analyzed source button chunk");
            println!("  Chunk type: {:?}", chunk.chunk_type);
            println!("  Module count: {}", chunk.module_count());
            
            // Build dependency graph
            let mut graph = DependencyGraph::new();
            for module in chunk.modules.values() {
                graph.add_module(module.clone());
            }
            println!("  Total dependencies: {}", graph.total_dependencies());
            
            // Show module details
            for (module_id, module) in &chunk.modules {
                println!("  Module: {}", module_id);
                println!("    Dependencies: {:?}", module.dependencies);
            }
            
            assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
            assert!(chunk.module_count() > 0, "Should have extracted modules");
        }
        Err(e) => {
            panic!("Failed to analyze source button chunk: {}", e);
        }
    }
}

#[test]
fn test_chunk_type_detection() {
    println!("Testing chunk type detection...");
    
    let analyzer = WebpackAnalyzer::new();
    
    // Test CommonJS format detection
    let commonjs_sample = r#"
    "use strict";
    exports.ids = ["test"];
    exports.modules = {
        "module1": function() { return "test"; }
    };
    "#;
    
    let result = analyzer.analyze_chunk(commonjs_sample);
    match result {
        Ok(chunk) => {
            println!("✓ CommonJS detection works");
            assert_eq!(chunk.chunk_type, ChunkType::CommonJS);
        }
        Err(e) => {
            panic!("CommonJS detection failed: {}", e);
        }
    }
    
    // Test JSONP format detection
    let jsonp_sample = r#"
    (self["webpackChunktest"] = self["webpackChunktest"] || []).push([["test"], {
        "module1": function() { return "test"; }
    }]);
    "#;
    
    let result = analyzer.analyze_chunk(jsonp_sample);
    match result {
        Ok(chunk) => {
            println!("✓ JSONP detection works");
            assert_eq!(chunk.chunk_type, ChunkType::JSONP);
        }
        Err(e) => {
            panic!("JSONP detection failed: {}", e);
        }
    }
}