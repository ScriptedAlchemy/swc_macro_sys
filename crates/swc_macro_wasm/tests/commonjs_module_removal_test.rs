#[test]
fn test_commonjs_module_removal() {
    use swc_macro_wasm::optimize;
    use serde_json::json;
    
    println!("\n=== TESTING COMMONJS MODULE REMOVAL SPECIFICALLY ===");
    
    // Simple CommonJS chunk with 3 modules
    let chunk = r#"
    "use strict";
    exports.ids = ["test-chunk"];
    exports.modules = {
        "moduleA.js": function() { console.log("A"); },
        "moduleB.js": function() { console.log("B"); },
        "moduleC.js": function() { console.log("C"); }
    };
    "#;
    
    println!("Original chunk has {} modules", chunk.matches(".js\":").count());
    
    // Provide required chunk characteristics with explicit entry module id
    let config = json!({
        "treeShake": {
            "test": {
                "chunk_characteristics": {
                    "entry_module_id": "moduleA.js",
                    "is_runtime_chunk": false,
                    "has_runtime": false,
                    "is_entrypoint": false,
                    "can_be_initial": false,
                    "is_only_initial": false,
                    "chunk_format": "require",
                    "chunk_loading_type": null,
                    "runtime_names": ["main"],
                    "entry_name": null,
                    "has_async_chunks": false,
                    "chunk_files": ["test-chunk.js"],
                    "is_shared_chunk": false,
                    "shared_modules": []
                }
            }
        }
    });
    
    let optimized = optimize(chunk.to_string(), &config.to_string());
    println!("Optimized chunk has {} modules", optimized.matches(".js\":").count());
    
    println!("\nOriginal chunk:\n{}", chunk);
    println!("\nOptimized chunk:\n{}", optimized);
    
    // The WebpackModuleRemover should handle exports.modules
    let has_module_a = optimized.contains("moduleA.js");
    let has_module_b = optimized.contains("moduleB.js");
    let has_module_c = optimized.contains("moduleC.js");
    
    println!("\nModule presence check:");
    println!("- moduleA.js: {}", if has_module_a { "⚠️ still present" } else { "✅ removed" });
    println!("- moduleB.js: {}", if has_module_b { "⚠️ still present" } else { "✅ removed" });
    println!("- moduleC.js: {}", if has_module_c { "⚠️ still present" } else { "✅ removed" });
    
    // This will fail if WebpackModuleRemover doesn't handle exports.modules
    if has_module_b || has_module_c {
        println!("\n❌ WebpackModuleRemover does not handle exports.modules format!");
        println!("   Need to add support for CommonJS exports.modules assignments");
        panic!("WebpackModuleRemover failed to remove orphaned modules from exports.modules");
    } else {
        println!("\n✅ WebpackModuleRemover correctly handles exports.modules format");
    }
    
    // moduleA should be preserved (fallback main export)
    assert!(has_module_a, "moduleA.js should be preserved as main export module");
    // moduleB and moduleC should be removed (orphaned)
    assert!(!has_module_b, "moduleB.js should be removed as orphaned module");
    assert!(!has_module_c, "moduleC.js should be removed as orphaned module");
}