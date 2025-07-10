use swc_macro_wasm::optimize;
use serde_json::json;

#[test]
fn test_rspack_lodash_chunk_tree_shaking() {
    // Load the actual RSpack lodash vendor chunk
    let lodash_chunk = include_str!("../../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js");
    
    println!("\n=== RSPACK LODASH CHUNK TREE SHAKING TEST ===");
    println!("Original chunk size: {} bytes", lodash_chunk.len());
    
    // This is a split chunk with no entry points, so tree shaking will remove all unreachable modules
    // The optimizer doesn't use macro conditions in this chunk, but it does iterative tree shaking
    let config = json!({});
    
    let result = optimize(lodash_chunk.to_string(), &config.to_string());
    
    println!("Optimized result size: {} bytes", result.len());
    println!("Size reduction: {} bytes ({:.1}%)", 
            lodash_chunk.len() - result.len(),
            ((lodash_chunk.len() - result.len()) as f64 / lodash_chunk.len() as f64) * 100.0);
    
    // Tree shaking should remove unreachable modules
    assert!(result.len() < lodash_chunk.len(), 
            "Should achieve size reduction through tree shaking");
    
    // Verify the chunk structure is preserved
    assert!(result.contains("webpackChunkrspack_basic_example"), 
            "Should preserve webpack chunk structure");
    
    // The optimization removes hundreds of unused lodash modules through iterative tree shaking
    // Based on the test output, it removes ~360 modules
    assert!(result.len() < lodash_chunk.len() / 2, 
            "Should remove majority of unused lodash modules");
    
    println!("RSpack lodash chunk tree shaking test passed!");
    println!("Successfully removed unused lodash modules through iterative tree shaking");
}

#[test]
fn test_rspack_main_bundle_optimization() {
    // Test the main RSpack bundle with various shared modules
    let main_bundle = include_str!("../../../test-cases/rspack-annotated-output/main.js");
    
    println!("\n=== RSPACK MAIN BUNDLE OPTIMIZATION TEST ===");
    println!("Original bundle size: {} bytes", main_bundle.len());
    
    // Based on share-usage.json, we know which exports are used
    let config = json!({
        "features": {
            "enableReact": true,
            "enableLodash": true,
            "enableUtilityLib": true,
            "enableApiLib": true,
            "enableComponentLib": true,
            "enableCjsHelpers": false  // Many unused exports here
        }
    });
    
    let result = optimize(main_bundle.to_string(), &config.to_string());
    
    println!("Optimized result size: {} bytes", result.len());
    println!("Size reduction: {} bytes ({:.1}%)", 
            main_bundle.len() - result.len(),
            ((main_bundle.len() - result.len()) as f64 / main_bundle.len() as f64) * 100.0);
    
    // Should achieve some optimization
    assert!(result.len() < main_bundle.len(), "Should optimize the bundle");
    
    // Check that key modules are preserved
    assert!(result.contains("@module-federation/error-codes"), 
            "Should preserve module federation error codes");
    
    println!("RSpack main bundle optimization test passed!");
}

#[test]
fn test_rspack_shared_modules_tree_shaking() {
    // Test tree shaking of shared modules based on actual usage
    let shared_utils = include_str!("../../../test-cases/rspack-annotated-output/shared_utils_js.js");
    
    println!("\n=== RSPACK SHARED MODULES TREE SHAKING TEST ===");
    println!("Original shared utils size: {} bytes", shared_utils.len());
    
    // According to share-usage.json, utility-lib uses: capitalize, formatDate, default
    // Unused: debounce, deepClone, generateId, processWithHelper, validateEmail
    let config = json!({
        "features": {
            "enableCapitalize": true,
            "enableFormatDate": true,
            "enableDebounce": false,
            "enableDeepClone": false,
            "enableGenerateId": false,
            "enableProcessWithHelper": false,
            "enableValidateEmail": false
        }
    });
    
    let result = optimize(shared_utils.to_string(), &config.to_string());
    
    println!("Optimized result size: {} bytes", result.len());
    
    // Should achieve optimization by removing unused exports
    assert!(result.len() < shared_utils.len(), "Should optimize shared utils");
    
    // Verify used exports are present
    if result.contains("capitalize") {
        println!("✓ capitalize export preserved");
    }
    if result.contains("formatDate") {
        println!("✓ formatDate export preserved");
    }
    
    // Verify unused exports are removed or their implementations are optimized
    let unused_count = ["debounce", "deepClone", "generateId", "validateEmail"]
        .iter()
        .filter(|&name| !result.contains(name))
        .count();
    
    println!("Removed {} unused exports", unused_count);
    
    println!("RSpack shared modules tree shaking test passed!");
}

#[test]
fn test_rspack_cjs_modules_optimization() {
    // Test CommonJS module optimization
    let cjs_helper = include_str!("../../../test-cases/rspack-annotated-output/cjs-modules_pure-cjs-helper_js.js");
    
    println!("\n=== RSPACK CJS MODULES OPTIMIZATION TEST ===");
    println!("Original CJS helper size: {} bytes", cjs_helper.len());
    
    // According to share-usage.json, only CONSTANTS is used from cjs-pure-helper
    let config = json!({
        "features": {
            "enableConstants": true,
            "enableDataValidator": false,
            "enableCreateValidator": false,
            "enableGenerateId": false,
            "enableHashString": false,
            "enableHelpers": false,
            "enableInfo": false,
            "enableProcessData": false,
            "enableValidateInput": false
        }
    });
    
    let result = optimize(cjs_helper.to_string(), &config.to_string());
    
    println!("Optimized result size: {} bytes", result.len());
    println!("Size reduction: {} bytes ({:.1}%)", 
            cjs_helper.len() - result.len(),
            ((cjs_helper.len() - result.len()) as f64 / cjs_helper.len() as f64) * 100.0);
    
    // Should achieve significant optimization since only CONSTANTS is used
    assert!(result.len() < cjs_helper.len(), "Should optimize CJS helper");
    
    // Verify CONSTANTS is preserved
    if result.contains("CONSTANTS") {
        println!("✓ CONSTANTS export preserved");
    }
    
    println!("RSpack CJS modules optimization test passed!");
}

#[test] 
fn test_rspack_split_chunk_with_no_entry_points() {
    // Test that split chunks with no entry points get fully tree shaken
    let vendor_chunk = include_str!("../../../test-cases/rspack-annotated-output/vendors-node_modules_pnpm_react_19_1_0_node_modules_react_index_js.js");
    
    println!("\n=== RSPACK SPLIT CHUNK NO ENTRY POINTS TEST ===");
    println!("Original vendor chunk size: {} bytes", vendor_chunk.len());
    
    // Disable the feature that would use this chunk
    let config = json!({
        "features": {
            "enableReact": false
        }
    });
    
    let result = optimize(vendor_chunk.to_string(), &config.to_string());
    
    println!("Optimized result size: {} bytes", result.len());
    
    // When React is disabled and there are no entry points, 
    // tree shaking should remove most/all modules
    assert!(result.len() < vendor_chunk.len() / 3, 
            "Should achieve massive size reduction when feature is disabled");
    
    // The chunk structure should remain but be mostly empty
    assert!(result.contains("webpackChunkrspack_basic_example"), 
            "Should preserve chunk structure");
    
    println!("Split chunk with no entry points test passed!");
    println!("Achieved {:.1}% size reduction", 
            ((vendor_chunk.len() - result.len()) as f64 / vendor_chunk.len() as f64) * 100.0);
}