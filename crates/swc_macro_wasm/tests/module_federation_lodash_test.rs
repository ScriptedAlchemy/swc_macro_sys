use std::fs;
use std::path::Path;
use std::time::Instant;
use swc_macro_wasm::optimize;

#[test]
fn test_module_federation_lodash_optimization() {
    println!("\n=== MODULE FEDERATION LODASH OPTIMIZATION TEST ===");

    // Path to our Module Federation example lodash chunks (use original, not optimized)
    let module_federation_dir = Path::new("../../module-federation-example");
    let host_chunk_path = module_federation_dir.join("host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let remote_chunk_path = module_federation_dir.join("remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let host_usage_path = module_federation_dir.join("host/dist/share-usage.json");
    let remote_usage_path = module_federation_dir.join("remote/dist/share-usage.json");

    // Check if files exist
    if !host_chunk_path.exists() || !remote_chunk_path.exists() {
        println!("⚠️  Module Federation chunks not found. Run 'pnpm run build' in module-federation-example first.");
        println!("Expected paths:");
        println!("  Host chunk: {}", host_chunk_path.display());
        println!("  Remote chunk: {}", remote_chunk_path.display());
        return;
    }

    if !host_usage_path.exists() || !remote_usage_path.exists() {
        println!("⚠️  Share usage files not found. Make sure build completed successfully.");
        return;
    }

    // Read and merge usage data
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&host_usage_path).expect("Failed to read host usage")
    ).expect("Failed to parse host usage JSON");

    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&remote_usage_path).expect("Failed to read remote usage")
    ).expect("Failed to parse remote usage JSON");

    // Extract lodash usage
    let host_lodash = &host_usage["consume_shared_modules"]["lodash-es"];
    let remote_lodash = &remote_usage["consume_shared_modules"]["lodash-es"];

    let host_used = host_lodash["used_exports"].as_array().unwrap();
    let remote_used = remote_lodash["used_exports"].as_array().unwrap();

    println!("Host app uses {} lodash exports: {:?}", host_used.len(), 
        host_used.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());
    println!("Remote app uses {} lodash exports: {:?}", remote_used.len(),
        remote_used.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());

    // Merge used exports (union)
    let mut all_used_exports = std::collections::HashSet::new();
    for export in host_used {
        all_used_exports.insert(export.as_str().unwrap().to_string());
    }
    for export in remote_used {
        all_used_exports.insert(export.as_str().unwrap().to_string());
    }

    println!("Combined used exports: {} total", all_used_exports.len());
    println!("Combined exports: {:?}", all_used_exports.iter().collect::<Vec<_>>());

    // Create tree shake config
    let mut tree_shake_config = serde_json::Map::new();
    let unused_exports = host_lodash["unused_exports"].as_array().unwrap();
    
    // Mark used exports as true, unused as false
    for export in &all_used_exports {
        tree_shake_config.insert(export.clone(), serde_json::Value::Bool(true));
    }
    for export in unused_exports {
        let export_name = export.as_str().unwrap();
        if !all_used_exports.contains(export_name) {
            tree_shake_config.insert(export_name.to_string(), serde_json::Value::Bool(false));
        }
    }

    let config = serde_json::json!({
        "treeShake": {
            "lodash-es": tree_shake_config
        },
        "entryModules": {
            "lodash-es": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
        }
    });

    println!("Tree shake config includes {} exports ({} used, {} unused)",
        tree_shake_config.len(),
        all_used_exports.len(),
        tree_shake_config.len() - all_used_exports.len()
    );

    // Test host chunk optimization
    println!("\n=== TESTING HOST CHUNK ===");
    test_chunk_optimization(&host_chunk_path, &config, "HOST");

    // Test remote chunk optimization  
    println!("\n=== TESTING REMOTE CHUNK ===");
    test_chunk_optimization(&remote_chunk_path, &config, "REMOTE");

    println!("\n✅ Module Federation lodash optimization test completed!");
}

fn test_chunk_optimization(chunk_path: &Path, config: &serde_json::Value, app_name: &str) {
    let original_code = fs::read_to_string(chunk_path).expect("Failed to read chunk");
    let original_size = original_code.len();

    println!("Original {} chunk size: {} bytes ({:.2} KB)", 
        app_name, original_size, original_size as f64 / 1024.0);

    let start_time = Instant::now();
    let config_str = serde_json::to_string(config).expect("Failed to serialize config");
    
    let optimized_code = optimize(original_code, &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    let duration = start_time.elapsed();

    println!("Optimized {} chunk size: {} bytes ({:.2} KB)", 
        app_name, optimized_size, optimized_size as f64 / 1024.0);
    println!("Size reduction: {:.2}% ({} bytes saved)", 
        reduction, original_size - optimized_size);
    println!("Optimization time: {:.2}ms", duration.as_millis());

    // Validate significant optimization occurred
    assert!(reduction > 20.0, 
        "{} chunk should have >20% reduction, got {:.2}%", app_name, reduction);

    // Check that optimized code is valid JavaScript (basic check)
    assert!(optimized_code.contains("exports.modules"), 
        "{} optimized chunk should maintain module structure", app_name);
    assert!(optimized_code.len() > 1000, 
        "{} optimized chunk should not be too small (likely broken)", app_name);

    println!("✅ {} chunk optimization successful!", app_name);
}

#[test]
fn test_module_federation_vs_standard_lodash() {
    println!("\n=== COMPARING MODULE FEDERATION VS STANDARD LODASH OPTIMIZATION ===");

    let module_federation_dir = Path::new("../../module-federation-example");
    let test_cases_dir = Path::new("../../test-cases");
    
    let mf_chunk_path = module_federation_dir.join("remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let standard_chunk_path = test_cases_dir.join("rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");

    if !mf_chunk_path.exists() || !standard_chunk_path.exists() {
        println!("⚠️  Comparison chunks not available, skipping test");
        return;
    }

    // Read both chunks
    let mf_code = fs::read_to_string(&mf_chunk_path).expect("Failed to read MF chunk");
    let standard_code = fs::read_to_string(&standard_chunk_path).expect("Failed to read standard chunk");

    println!("Module Federation chunk: {} bytes ({:.1} KB)", 
        mf_code.len(), mf_code.len() as f64 / 1024.0);
    println!("Standard test chunk: {} bytes ({:.1} KB)", 
        standard_code.len(), standard_code.len() as f64 / 1024.0);

    // Compare module counts (rough estimate)
    let mf_modules = mf_code.matches("\":function(").count();
    let standard_modules = standard_code.matches("\":function(").count();

    println!("Module Federation modules: ~{}", mf_modules);
    println!("Standard test modules: ~{}", standard_modules);

    // Use minimal config for comparison
    let minimal_config = serde_json::json!({
        "treeShake": {
            "lodash-es": {
                "map": true,
                "filter": true,
                "default": true
            }
        },
        "entryModules": {
            "lodash-es": "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
        }
    });

    let config_str = serde_json::to_string(&minimal_config).unwrap();

    // Test both with same config
    let mf_optimized = optimize(mf_code.clone(), &config_str);
    let standard_optimized = optimize(standard_code.clone(), &config_str);

    let mf_reduction = ((mf_code.len() - mf_optimized.len()) as f64 / mf_code.len() as f64) * 100.0;
    let standard_reduction = ((standard_code.len() - standard_optimized.len()) as f64 / standard_code.len() as f64) * 100.0;

    println!("Module Federation optimization: {:.1}%", mf_reduction);
    println!("Standard test optimization: {:.1}%", standard_reduction);

    // Both should achieve significant optimization
    assert!(mf_reduction > 20.0, "Module Federation should optimize well");
    assert!(standard_reduction > 20.0, "Standard test should optimize well");

    println!("✅ Both chunk types optimize successfully!");
}