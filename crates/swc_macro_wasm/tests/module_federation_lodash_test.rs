use std::fs;
use std::path::Path;
use std::time::Instant;
use swc_macro_wasm::optimize;

#[test]
fn test_module_federation_lodash_optimization() {
    // silent

    // Path to our Module Federation example lodash chunks (use original, not optimized)
    let module_federation_dir = Path::new("../../examples/module-federation-example");
    let host_chunk_path = module_federation_dir.join("host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let remote_chunk_path = module_federation_dir.join("remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let host_usage_path = module_federation_dir.join("host/dist/share-usage.json");
    let remote_usage_path = module_federation_dir.join("remote/dist/share-usage.json");

    // Check if files exist
    if !host_chunk_path.exists() || !remote_chunk_path.exists() {
        panic!("Module Federation chunks not found. Build examples/module-federation-example first. Expected host: {}, remote: {}",
            host_chunk_path.display(), remote_chunk_path.display());
    }

    if !host_usage_path.exists() || !remote_usage_path.exists() {
        panic!("Share usage files not found. Make sure build completed successfully.");
    }

    // Read and merge usage data
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&host_usage_path).expect("Failed to read host usage")
    ).expect("Failed to parse host usage JSON");

    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(&remote_usage_path).expect("Failed to read remote usage")
    ).expect("Failed to parse remote usage JSON");

    // Extract lodash usage from new format - treeShake.lodash-es contains exports with boolean values
    let host_lodash = &host_usage["treeShake"]["lodash-es"];
    let remote_lodash = &remote_usage["treeShake"]["lodash-es"];

    let host_lodash_obj = host_lodash.as_object().expect("host_lodash should be object");
    let remote_lodash_obj = remote_lodash.as_object().expect("remote_lodash should be object");

    // Extract used exports (where value is true)
    let host_used: Vec<&str> = host_lodash_obj.iter()
        .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
        .map(|(k, _)| k.as_str())
        .collect();
    let remote_used: Vec<&str> = remote_lodash_obj.iter()
        .filter(|(k, v)| k.as_str() != "chunk_characteristics" && v.as_bool() == Some(true))
        .map(|(k, _)| k.as_str())
        .collect();

    assert!(host_used.len() > 0 || remote_used.len() > 0, "At least one export should be used");

    // Merge used exports (union)
    let mut all_used_exports = std::collections::HashSet::new();
    for export in &host_used {
        all_used_exports.insert(export.to_string());
    }
    for export in &remote_used {
        all_used_exports.insert(export.to_string());
    }

    assert!(all_used_exports.len() > 0, "Combined used exports should be non-empty");

    // Create tree shake config by merging both configs
    let mut tree_shake_config = serde_json::Map::new();
    
    // Start with all exports from host (includes both used and unused)
    for (key, value) in host_lodash_obj {
        if key != "chunk_characteristics" {
            tree_shake_config.insert(key.clone(), value.clone());
        }
    }
    
    // Update with remote's used exports (set to true if used in remote)
    for (key, value) in remote_lodash_obj {
        if key != "chunk_characteristics" && value.as_bool() == Some(true) {
            tree_shake_config.insert(key.clone(), serde_json::Value::Bool(true));
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

    assert!(tree_shake_config.len() >= all_used_exports.len());

    // Test host chunk optimization
    test_chunk_optimization(&host_chunk_path, &config, "HOST");

    // Test remote chunk optimization  
    test_chunk_optimization(&remote_chunk_path, &config, "REMOTE");

    // done
}

fn test_chunk_optimization(chunk_path: &Path, config: &serde_json::Value, app_name: &str) {
    let original_code = fs::read_to_string(chunk_path).expect("Failed to read chunk");
    let original_size = original_code.len();

    assert!(original_size > 0, "{} chunk should not be empty", app_name);

    let start_time = Instant::now();
    let config_str = serde_json::to_string(config).expect("Failed to serialize config");
    
    let optimized_code = optimize(original_code, &config_str);
    let optimized_size = optimized_code.len();
    let reduction = ((original_size - optimized_size) as f64 / original_size as f64) * 100.0;
    let duration = start_time.elapsed();

    assert!(optimized_size > 0, "{} optimized chunk should not be empty", app_name);
    assert!(duration.as_millis() >= 0);

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
    // silent

    let module_federation_dir = Path::new("../../examples/module-federation-example");
    let test_cases_dir = Path::new("../../test-cases");
    
    let mf_chunk_path = module_federation_dir.join("remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original");
    let standard_chunk_path = test_cases_dir.join("rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");

    assert!(mf_chunk_path.exists() && standard_chunk_path.exists(), "Comparison chunks not available");

    // Read both chunks
    let mf_code = fs::read_to_string(&mf_chunk_path).expect("Failed to read MF chunk");
    let standard_code = fs::read_to_string(&standard_chunk_path).expect("Failed to read standard chunk");

    assert!(mf_code.len() > 0 && standard_code.len() > 0);

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

    assert!(mf_reduction > 0.0 && standard_reduction > 0.0);

    // Both should achieve significant optimization
    assert!(mf_reduction > 20.0, "Module Federation should optimize well");
    assert!(standard_reduction > 20.0, "Standard test should optimize well");

    // done
}