/// Simple regex-based tree shaking that works in WASM environments
/// This is a lightweight alternative to the full TreeShaker which has WASM compatibility issues

use regex::Regex;
use serde_json::Value;
use std::collections::HashSet;

/// Performs simple tree shaking by removing unused modules based on configuration
pub fn simple_tree_shake(source: &str, config: &Value) -> String {
    // Check if tree shaking is configured
    let tree_shake = match config.get("treeShake") {
        Some(ts) => ts,
        None => return source.to_string(), // No tree shaking config, return original
    };
    
    // Collect modules to remove
    let mut modules_to_remove = HashSet::new();
    
    // Process each package in the tree shake config
    if let Some(tree_shake_obj) = tree_shake.as_object() {
        for (package_name, exports_config) in tree_shake_obj {
            if let Some(exports_obj) = exports_config.as_object() {
                for (export_name, should_keep) in exports_obj {
                    // Skip chunk_characteristics metadata
                    if export_name == "chunk_characteristics" {
                        continue;
                    }
                    
                    // If the export should not be kept (false), mark for removal
                    if should_keep.as_bool() == Some(false) {
                        // Generate possible module paths for this export
                        let module_paths = vec![
                            format!("/{}.js", export_name),
                            format!("/{}/index.js", export_name),
                            format!("{}/{}.js", package_name, export_name),
                            format!("{}/{}/index.js", package_name, export_name),
                        ];
                        
                        for path in module_paths {
                            modules_to_remove.insert(path);
                        }
                    }
                }
            }
        }
    }
    
    // If no modules to remove, return original
    if modules_to_remove.is_empty() {
        return source.to_string();
    }
    
    // Try to remove modules from CommonJS format (exports.modules)
    if source.contains("exports.modules") {
        return remove_from_commonjs_format(source, &modules_to_remove);
    }
    
    // Try to remove modules from Webpack format (__webpack_modules__)
    if source.contains("__webpack_modules__") {
        return remove_from_webpack_format(source, &modules_to_remove);
    }
    
    // Unknown format, return original
    source.to_string()
}

/// Remove modules from CommonJS format (exports.modules = { ... })
fn remove_from_commonjs_format(source: &str, modules_to_remove: &HashSet<String>) -> String {
    // Use a simple approach: find module definitions and remove them
    let mut result = source.to_string();
    
    // Pattern to match module definitions in exports.modules
    // This is a simplified pattern that matches: "module_id": function(...) { ... }
    let module_pattern = match Regex::new(
        r#"(?s)"([^"]+)":\s*function\s*\([^)]*\)\s*\{[^}]*\}"#
    ) {
        Ok(re) => re,
        Err(_) => return source.to_string(),
    };
    
    for cap in module_pattern.captures_iter(source) {
        if let Some(module_id) = cap.get(1) {
            let module_id_str = module_id.as_str();
            
            // Check if this module should be removed
            let should_remove = modules_to_remove.iter().any(|path| {
                module_id_str.ends_with(path) || module_id_str.contains(path)
            });
            
            if should_remove {
                // Remove this module definition
                if let Some(full_match) = cap.get(0) {
                    result = result.replace(full_match.as_str(), "");
                }
            }
        }
    }
    
    // Clean up any double commas or trailing commas
    result = result.replace(",,", ",");
    result = result.replace(",}", "}");
    result = result.replace(",\n}", "\n}");
    
    result
}

/// Remove modules from Webpack format (var __webpack_modules__ = { ... })
fn remove_from_webpack_format(source: &str, modules_to_remove: &HashSet<String>) -> String {
    // Similar to CommonJS but for webpack format
    let mut result = source.to_string();
    
    // Pattern to match module definitions in __webpack_modules__
    let module_pattern = match Regex::new(
        r#"(?s)(?:"|')([^"']+)(?:"|'):\s*(?:function\s*\([^)]*\)|/\*[^*]*\*/)\s*\{[^}]*\}"#
    ) {
        Ok(re) => re,
        Err(_) => return source.to_string(),
    };
    
    for cap in module_pattern.captures_iter(source) {
        if let Some(module_id) = cap.get(1) {
            let module_id_str = module_id.as_str();
            
            // Check if this module should be removed
            let should_remove = modules_to_remove.iter().any(|path| {
                module_id_str.ends_with(path) || module_id_str.contains(path)
            });
            
            if should_remove {
                // Remove this module definition
                if let Some(full_match) = cap.get(0) {
                    result = result.replace(full_match.as_str(), "");
                }
            }
        }
    }
    
    // Clean up any double commas or trailing commas
    result = result.replace(",,", ",");
    result = result.replace(",}", "}");
    result = result.replace(",\n}", "\n}");
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    
    #[test]
    fn test_simple_tree_shake_commonjs() {
        let source = r#"
exports.modules = {
    "./debounce.js": function() { /* debounce */ },
    "./throttle.js": function() { /* throttle */ },
    "./isArray.js": function() { /* isArray */ }
};
"#;
        
        let config = json!({
            "treeShake": {
                "lodash-es": {
                    "debounce": true,
                    "throttle": true,
                    "isArray": false  // Remove this
                }
            }
        });
        
        let result = simple_tree_shake(source, &config);
        
        assert!(result.contains("debounce"));
        assert!(result.contains("throttle"));
        assert!(!result.contains("isArray"));
    }
    
    #[test]
    fn test_simple_tree_shake_webpack() {
        let source = r#"
var __webpack_modules__ = {
    "./node_modules/lodash-es/debounce.js": function() { /* debounce */ },
    "./node_modules/lodash-es/throttle.js": function() { /* throttle */ },
    "./node_modules/lodash-es/isArray.js": function() { /* isArray */ }
};
"#;
        
        let config = json!({
            "treeShake": {
                "lodash-es": {
                    "debounce": true,
                    "throttle": true,
                    "isArray": false  // Remove this
                }
            }
        });
        
        let result = simple_tree_shake(source, &config);
        
        assert!(result.contains("debounce"));
        assert!(result.contains("throttle"));
        assert!(!result.contains("isArray"));
    }
    
    #[test]
    fn test_no_tree_shake_config() {
        let source = "console.log('hello');";
        let config = json!({});
        
        let result = simple_tree_shake(source, &config);
        assert_eq!(result, source);
    }
}