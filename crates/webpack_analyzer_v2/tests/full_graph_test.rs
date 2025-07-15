use webpack_analyzer_v2::*;
use std::fs;

#[test]
fn test_full_module_graph_real_world_lodash() {
    let analyzer = WebpackAnalyzer::new();
    
    // Load the real-world lodash chunk
    let source = fs::read_to_string("../../test-cases/rspack-cjs-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js")
        .expect("Failed to read lodash chunk file");
    
    let chunk = analyzer.analyze_chunk(&source).unwrap();
    
    println!("📊 Full Module Graph Analysis:");
    println!("   - Total modules: {}", chunk.module_count());
    println!("   - Chunk type: {:?}", chunk.chunk_type);
    
    let module_count = chunk.module_count();
    
    // Build complete dependency graph
    let mut graph = DependencyGraph::new();
    for (_, module) in chunk.modules {
        graph.add_module(module);
    }
    
    // Print comprehensive graph statistics
    println!("   - Total dependencies: {}", graph.total_dependencies());
    
    // Find modules with no dependencies (leaf modules)
    let leaf_modules: Vec<_> = graph.modules.values()
        .filter(|m| !m.has_dependencies())
        .map(|m| m.id.clone())
        .collect();
    
    println!("   - Leaf modules (no dependencies): {}", leaf_modules.len());
    
    // Find modules with no dependents (potential unused)
    let no_dependents: Vec<_> = graph.modules.values()
        .filter(|m| !m.has_dependents())
        .map(|m| m.id.clone())
        .collect();
    
    println!("   - Modules with no dependents: {}", no_dependents.len());
    
    // Find modules with the most dependencies
    let mut modules_by_deps: Vec<_> = graph.modules.values()
        .map(|m| (m.id.clone(), m.dependencies.len()))
        .collect();
    modules_by_deps.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("   - Top 5 modules by dependency count:");
    for (module_id, dep_count) in modules_by_deps.iter().take(5) {
        let short_name = module_id.split('/').last().unwrap_or(module_id);
        println!("     * {} -> {} deps", short_name, dep_count);
    }
    
    // Find modules with the most dependents
    let mut modules_by_dependents: Vec<_> = graph.modules.values()
        .map(|m| (m.id.clone(), m.dependents.len()))
        .collect();
    modules_by_dependents.sort_by(|a, b| b.1.cmp(&a.1));
    
    println!("   - Top 5 modules by dependent count:");
    for (module_id, dependent_count) in modules_by_dependents.iter().take(5) {
        let short_name = module_id.split('/').last().unwrap_or(module_id);
        println!("     * {} <- {} dependents", short_name, dependent_count);
    }
    
    // Test module removal impact analysis
    println!("\n🔍 Module Removal Impact Analysis:");
    
    // Test removing a highly depended-upon module
    if let Some((most_depended_module, _)) = modules_by_dependents.first() {
        let impact = graph.simulate_module_removal(most_depended_module);
        let short_name = most_depended_module.split('/').last().unwrap_or(most_depended_module);
        println!("   - Impact of removing most depended module '{}':", short_name);
        println!("     * Would break {} modules", impact.broken_modules.len());
        println!("     * Would orphan {} modules", impact.potentially_orphaned.len());
        
        // Show first few broken modules
        if !impact.broken_modules.is_empty() {
            println!("     * First few broken modules:");
            for broken in impact.broken_modules.iter().take(3) {
                let broken_short = broken.split('/').last().unwrap_or(broken);
                println!("       - {}", broken_short);
            }
        }
    }
    
    // Test removing a leaf module (should have minimal impact)
    if let Some(leaf_module) = leaf_modules.first() {
        let impact = graph.simulate_module_removal(leaf_module);
        let short_name = leaf_module.split('/').last().unwrap_or(leaf_module);
        println!("   - Impact of removing leaf module '{}':", short_name);
        println!("     * Would break {} modules", impact.broken_modules.len());
        println!("     * Would orphan {} modules", impact.potentially_orphaned.len());
    }
    
    assert!(module_count > 100, "Should have many modules in lodash chunk");
    assert!(graph.total_dependencies() > 0, "Should have dependencies");
    
    println!("✅ Full module graph analysis complete!");
}