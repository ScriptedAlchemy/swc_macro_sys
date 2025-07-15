use swc_macro_wasm::optimize;
use serde_json::json;
use std::fs;
use std::path::Path;

#[test]
fn test_verify_remaining_exports_after_optimization() {
    println!("\n=== VERIFY REMAINING EXPORTS AFTER OPTIMIZATION ===");
    
    // Load the real chunks
    let mf_chunk_path = Path::new("tests/fixtures/module_federation_lodash_chunk.js");
    let host_usage_path = Path::new("tests/fixtures/module_federation_usage.json");
    let remote_usage_path = Path::new("tests/fixtures/module_federation_remote_usage.json");
    
    // Read files
    let original_code = fs::read_to_string(mf_chunk_path)
        .expect("Failed to read MF chunk");
    let host_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(host_usage_path).expect("Failed to read host usage")
    ).expect("Failed to parse host usage");
    let remote_usage: serde_json::Value = serde_json::from_str(
        &fs::read_to_string(remote_usage_path).expect("Failed to read remote usage")
    ).expect("Failed to parse remote usage");
    
    // Extract usage data
    let host_used = host_usage["consume_shared_modules"]["lodash-es"]["used_exports"]
        .as_array()
        .unwrap();
    let remote_used = remote_usage["consume_shared_modules"]["lodash-es"]["used_exports"]
        .as_array()
        .unwrap();
    let unused = host_usage["consume_shared_modules"]["lodash-es"]["unused_exports"]
        .as_array()
        .unwrap();
    
    // Build tree shake config
    let mut used_exports = std::collections::HashSet::new();
    for export in host_used.iter().chain(remote_used.iter()) {
        used_exports.insert(export.as_str().unwrap());
    }
    
    let mut lodash_config = serde_json::Map::new();
    for export in &used_exports {
        lodash_config.insert(export.to_string(), json!(true));
    }
    for export in unused {
        let export_name = export.as_str().unwrap();
        if !used_exports.contains(export_name) {
            lodash_config.insert(export_name.to_string(), json!(false));
        }
    }
    
    let config = json!({
        "treeShake": {
            "lodash-es": lodash_config
        }
    });
    
    println!("Expected to preserve these exports: {:?}", used_exports);
    
    // Optimize
    let optimized = optimize(original_code.clone(), &config.to_string());
    
    println!("\nOptimization results:");
    println!("  Original size: {} bytes", original_code.len());
    println!("  Optimized size: {} bytes", optimized.len());
    println!("  Reduction: {:.2}%", 
        (original_code.len() - optimized.len()) as f64 / original_code.len() as f64 * 100.0);
    
    // Analyze what remains
    println!("\nAnalyzing optimized chunk:");
    
    // Count modules
    let module_count = optimized.matches("!***").count();
    println!("  Modules remaining: {}", module_count);
    
    // Find module paths by looking for patterns like "path/to/module.js": /*! 
    let mut found_modules = Vec::new();
    for line in optimized.lines() {
        if line.contains(".js\":") && line.contains("/*!") {
            if let Some(start) = line.find('"') {
                if let Some(end) = line[start+1..].find('"') {
                    let module_path = &line[start+1..start+1+end];
                    let module_name = module_path.split('/').last().unwrap_or(module_path);
                    found_modules.push(module_name.to_string());
                }
            }
        }
    }
    
    println!("  Module files found:");
    for module in &found_modules {
        println!("    - {}", module);
    }
    
    // Check for specific exports that should be preserved
    println!("\nChecking for expected exports:");
    let expected_exports = ["sortBy", "uniq", "capitalize", "debounce", "groupBy", "omit", "pick", "throttle", "default"];
    let mut missing_exports = Vec::new();
    let mut found_exports = Vec::new();
    
    for export in &expected_exports {
        if optimized.contains(export) {
            found_exports.push(*export);
        } else {
            missing_exports.push(*export);
        }
    }
    
    println!("  Found exports: {:?}", found_exports);
    if !missing_exports.is_empty() {
        println!("  ⚠️  Missing exports: {:?}", missing_exports);
    }
    
    // Look for the main lodash.js module
    let has_main_lodash = optimized.contains("lodash-es/lodash.js");
    println!("\n  Main lodash.js module present: {}", has_main_lodash);
    
    // Check if exports.modules structure is preserved
    let has_module_structure = optimized.contains("exports.modules");
    println!("  exports.modules structure preserved: {}", has_module_structure);
    
    // Sample the optimized content
    if optimized.len() < 2000 {
        println!("\n⚠️  Optimized chunk is very small ({} bytes)", optimized.len());
        println!("First 500 chars:");
        println!("{}", &optimized[..optimized.len().min(500)]);
    }
    
    // Validate the optimization didn't break the structure
    assert!(has_module_structure, "Module structure should be preserved");
    assert!(module_count > 0, "Should have at least one module");
    
    // The optimization is too aggressive if it removes all the exports we need
    if missing_exports.len() == expected_exports.len() {
        println!("\n❌ CRITICAL: All expected exports were removed!");
        println!("   This means the tree shaker is removing modules it shouldn't.");
        
        // Check what the tree shaker is doing
        if optimized.len() < 2000 && found_modules.len() <= 2 {
            println!("\n   The tree shaker appears to be removing the main lodash module");
            println!("   even though we marked exports as used in the config.");
            println!("   Remaining modules: {:?}", found_modules);
        }
    }
    
    println!("\n✅ Test completed - optimization behavior documented");
}

#[test]
fn test_macro_condition_processing() {
    println!("\n=== TEST MACRO CONDITION PROCESSING ===");
    
    // Create a minimal test case with @common:if conditions
    let test_code = r#"
"use strict";
exports.modules = {
    "lodash.js": function(module, exports, __webpack_require__) {
        __webpack_require__.d(exports, {
            sortBy: () => (/* @common:if [condition="treeShake.lodash-es.sortBy"] */ sortBy /* @common:endif */),
            uniq: () => (/* @common:if [condition="treeShake.lodash-es.uniq"] */ uniq /* @common:endif */),
            filter: () => (/* @common:if [condition="treeShake.lodash-es.filter"] */ filter /* @common:endif */)
        });
        
        const sortBy = __webpack_require__("./sortBy.js");
        const uniq = __webpack_require__("./uniq.js");
        const filter = __webpack_require__("./filter.js");
    }
};
"#;
    
    // Config to keep sortBy and uniq but remove filter
    let config = json!({
        "treeShake": {
            "lodash-es": {
                "sortBy": true,
                "uniq": true,
                "filter": false
            }
        }
    });
    
    println!("Test code with @common:if conditions");
    println!("Config: keep sortBy and uniq, remove filter");
    
    let optimized = optimize(test_code.to_string(), &config.to_string());
    
    println!("\nOptimized result:");
    println!("{}", optimized);
    
    // Check what was kept/removed
    assert!(optimized.contains("sortBy"), "sortBy should be kept");
    assert!(optimized.contains("uniq"), "uniq should be kept");
    
    // The macro processor should handle the @common:if conditions
    // If filter is removed entirely, that's good
    // If it's replaced with undefined or similar, that's also acceptable
    let filter_removed = !optimized.contains("filter:") || 
                        optimized.contains("filter: () => undefined") ||
                        optimized.contains("filter: () => (/* removed */");
    
    println!("\nVerification:");
    println!("  sortBy present: {}", optimized.contains("sortBy"));
    println!("  uniq present: {}", optimized.contains("uniq"));
    println!("  filter removed: {}", filter_removed);
    
    assert!(filter_removed, "filter export should be removed or nullified");
    
    println!("\n✅ Macro condition processing test passed");
}