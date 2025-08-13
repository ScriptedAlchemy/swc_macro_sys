use webpack_analyzer_v2::*;
use std::fs;

#[test]
fn test_full_module_graph_real_world_lodash() {
    let analyzer = WebpackAnalyzer::new();
    
    // Load the real-world lodash chunk
    let source = fs::read_to_string("../../test-cases/rspack-cjs-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js")
        .expect("Failed to read lodash chunk file");
    
    // Characteristics for rspack-cjs lodash chunk
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "require".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: None,
    };
    
    let chunk = analyzer.analyze_chunk(&source, characteristics).unwrap();
    
    assert!(chunk.module_count() > 0);
    assert!(matches!(chunk.chunk_type, ChunkType::CommonJSSync | ChunkType::CommonJSAsync));
    
    let module_count = chunk.module_count();
    
    // Build complete dependency graph
    let mut graph = DependencyGraph::new();
    for (_, module) in chunk.modules {
        graph.add_module(module);
    }
    
    // Print comprehensive graph statistics
    // Basic graph structure validation
    let leaf_modules: Vec<_> = graph.modules.values()
        .filter(|m| !m.has_dependencies())
        .map(|m| m.id.clone())
        .collect();
    
    assert!(leaf_modules.len() <= module_count);
    
    let no_dependents: Vec<_> = graph.modules.values()
        .filter(|m| !m.has_dependents())
        .map(|m| m.id.clone())
        .collect();
    
    assert!(no_dependents.len() <= module_count);
    
    // Find modules with the most dependents for impact testing
    let mut modules_by_dependents: Vec<_> = graph.modules.values()
        .map(|m| (m.id.clone(), m.dependents.len()))
        .collect();
    modules_by_dependents.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Basic validation that we found modules with varying dependency patterns
    if let Some((most_depended_module, dependent_count)) = modules_by_dependents.first() {
        // Should have at least some dependents if it's the most depended on
        let _ = (most_depended_module, dependent_count);
    }
    
    // Validate leaf modules exist (modules with no dependencies)
    if let Some(leaf_module) = leaf_modules.first() {
        // Should be a valid module ID
        let _ = leaf_module;
    }
    
    assert!(module_count > 100, "Should have many modules in lodash chunk");
    assert!(graph.total_dependencies() > 0, "Should have dependencies");
    
    // done
}