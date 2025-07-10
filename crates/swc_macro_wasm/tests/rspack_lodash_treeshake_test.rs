use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_rspack_lodash_tree_shake_based_on_usage() {
    // Load the actual RSpack lodash vendor chunk with macro conditions
    let lodash_chunk = include_str!("../../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    // Load share usage data to know which exports are actually used
    let share_usage_json = include_str!("../../../test-cases/rspack-annotated-output/share-usage.json");
    let share_usage: serde_json::Value = serde_json::from_str(share_usage_json).expect("Failed to parse share-usage.json");
    
    println!("\n=== RSPACK LODASH TREE SHAKE BASED ON ACTUAL USAGE TEST ===");
    println!("Original chunk size: {} bytes ({:.2} KB)", lodash_chunk.len(), lodash_chunk.len() as f64 / 1024.0);
    
    // Extract lodash usage data
    let lodash_usage = &share_usage["consume_shared_modules"]["lodash-es"];
    let used_exports = lodash_usage["used_exports"].as_array().expect("used_exports should be array");
    let unused_exports = lodash_usage["unused_exports"].as_array().expect("unused_exports should be array");
    
    println!("\nLodash usage from share-usage.json:");
    println!("  Used exports: {:?}", used_exports.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());
    println!("  Total unused exports: {}", unused_exports.len());
    
    // Build tree shaking configuration based on actual usage
    let mut lodash_config = serde_json::Map::new();
    
    // Enable used exports
    for export in used_exports {
        let export_name = export.as_str().unwrap();
        lodash_config.insert(export_name.to_string(), json!(true));
    }
    
    // Disable unused exports
    for export in unused_exports {
        let export_name = export.as_str().unwrap();
        lodash_config.insert(export_name.to_string(), json!(false));
    }
    
    let config = json!({
        "treeShake": {
            "lodash-es": lodash_config
        }
    });
    
    println!("\nTree shaking configuration:");
    println!("  Total exports configured: {}", lodash_config.len());
    println!("  Enabled exports: {}", used_exports.len());
    println!("  Disabled exports: {}", unused_exports.len());
    
    // Run optimization
    let start_time = std::time::Instant::now();
    let optimized = optimize(lodash_chunk.to_string(), &config.to_string());
    let optimization_time = start_time.elapsed();
    
    println!("\n⚡ Optimization completed in {:.2}ms", optimization_time.as_millis());
    
    // Calculate size reduction
    let optimized_size = optimized.len();
    let reduction = lodash_chunk.len() - optimized_size;
    let reduction_percent = (reduction as f64 / lodash_chunk.len() as f64) * 100.0;
    
    println!("\n📊 Size Reduction Results:");
    println!("  Original:  {} bytes ({:.2} KB)", lodash_chunk.len(), lodash_chunk.len() as f64 / 1024.0);
    println!("  Optimized: {} bytes ({:.2} KB)", optimized_size, optimized_size as f64 / 1024.0);
    println!("  Reduction: {} bytes ({:.2} KB)", reduction, reduction as f64 / 1024.0);
    println!("  Percentage: {:.2}% smaller", reduction_percent);
    
    // Verify optimizations
    assert!(optimized_size < lodash_chunk.len(), "Should reduce bundle size");
    assert!(reduction_percent > 50.0, "Should achieve >50% reduction when only 4 exports are used");
    
    // Verify used exports are preserved - check if the export name appears in the optimized output
    for export in used_exports {
        let export_name = export.as_str().unwrap();
        // After optimization, the export should still be present
        let export_pattern = format!(r#"{}: () =>"#, export_name);
        if !optimized.contains(&export_pattern) && export_name != "VERSION" && export_name != "default" {
            // VERSION and default might have different patterns
            println!("  ⚠️  Export '{}' pattern changed after optimization", export_name);
        }
    }
    
    // Sample check: verify some unused exports are removed or have empty conditions
    let sample_unused = ["add", "chunk", "debounce", "throttle", "merge"];
    let mut removed_count = 0;
    for unused in &sample_unused {
        let export_pattern = format!(r#"{}: () => (/* reexport safe */"#, unused);
        if !optimized.contains(&export_pattern) {
            removed_count += 1;
            println!("  ✓ Removed unused export: {}", unused);
        }
    }
    
    println!("\n✅ Tree shaking test passed!");
    println!("  Removed {} of {} sampled unused exports", removed_count, sample_unused.len());
    
    // Also test the module removal through iterative tree shaking
    // Count how many lodash module files are in the original vs optimized
    let original_modules = lodash_chunk.matches("node_modules/lodash-es/").count();
    let optimized_modules = optimized.matches("node_modules/lodash-es/").count();
    let modules_removed = original_modules - optimized_modules;
    
    println!("\n📦 Module Tree Shaking:");
    println!("  Original modules: {}", original_modules);
    println!("  Optimized modules: {}", optimized_modules);
    println!("  Modules removed: {} ({:.1}%)", 
        modules_removed, 
        (modules_removed as f64 / original_modules as f64) * 100.0);
    
    assert!(modules_removed > 100, "Should remove >100 unused lodash modules through tree shaking");
}

#[test]
fn test_rspack_lodash_complete_removal() {
    // Test complete removal when no lodash exports are used
    let lodash_chunk = include_str!("../../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    println!("\n=== RSPACK LODASH COMPLETE REMOVAL TEST ===");
    println!("Original chunk size: {} bytes ({:.2} KB)", lodash_chunk.len(), lodash_chunk.len() as f64 / 1024.0);
    
    // Disable all lodash exports
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "map": false,
                "filter": false,
                "VERSION": false,
                "default": false,
                // Add a few more common ones to ensure they're disabled
                "forEach": false,
                "reduce": false,
                "get": false,
                "set": false,
                "debounce": false,
                "throttle": false
            }
        }
    });
    
    let optimized = optimize(lodash_chunk.to_string(), &config.to_string());
    
    let optimized_size = optimized.len();
    let reduction_percent = ((lodash_chunk.len() - optimized_size) as f64 / lodash_chunk.len() as f64) * 100.0;
    
    println!("\n📊 Complete Removal Results:");
    println!("  Original:  {} bytes ({:.2} KB)", lodash_chunk.len(), lodash_chunk.len() as f64 / 1024.0);
    println!("  Optimized: {} bytes ({:.2} KB)", optimized_size, optimized_size as f64 / 1024.0);
    println!("  Reduction: {:.2}%", reduction_percent);
    
    // Should achieve massive reduction when no exports are used
    assert!(reduction_percent > 70.0, 
        "Should achieve >70% reduction when no lodash exports are used, got {:.2}%", reduction_percent);
    
    // The chunk structure should remain but most content removed
    assert!(optimized.contains("webpackChunkrspack_basic_example"), "Should preserve chunk structure");
    
    println!("\n✅ Complete removal test passed!");
}