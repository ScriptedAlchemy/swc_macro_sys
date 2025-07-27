use std::collections::{HashMap, HashSet};
use serde_json::Value;
use regex::Regex;

/// Analyzes module references in a webpack chunk and returns a map of module names to their dependencies
pub fn analyze_module_references(chunk: &str) -> HashMap<String, Vec<String>> {
    let mut references = HashMap::new();
    
    // Match webpack module definitions
    let module_pattern = Regex::new(r#"["']([^"']+)["']:\s*\(function\s*\([^)]*\)\s*\{|["']([^"']+)["']:\s*\(["']([^"']+)["']\)\s*=>\s*\{|["']([^"']+)["']:\s*\(["']([^"']+)["'],\s*["']([^"']+)["']\)\s*=>\s*\{"#).unwrap();
    let _require_pattern = Regex::new(r#"__webpack_require__\(["']([^"']+)["']\)|__webpack_require__\.o\(["']([^"']+)["']\)"#).unwrap();
    
    // First pass: identify all modules
    let mut current_module = None;
    let mut module_content = String::new();
    let mut in_module = false;
    let mut brace_count = 0;
    
    for line in chunk.lines() {
        if let Some(captures) = module_pattern.captures(line) {
            // Save previous module if any
            if let Some(module_name) = current_module.take() {
                let deps = extract_dependencies(&module_content);
                references.insert(module_name, deps);
            }
            
            // Start new module
            current_module = captures.get(1)
                .or(captures.get(2))
                .or(captures.get(4))
                .map(|m| m.as_str().to_string());
            module_content.clear();
            in_module = true;
            brace_count = line.matches('{').count() as i32 - line.matches('}').count() as i32;
        } else if in_module {
            module_content.push_str(line);
            module_content.push('\n');
            brace_count += line.matches('{').count() as i32 - line.matches('}').count() as i32;
            
            if brace_count <= 0 {
                // End of module
                if let Some(module_name) = current_module.take() {
                    let deps = extract_dependencies(&module_content);
                    references.insert(module_name, deps);
                }
                in_module = false;
                module_content.clear();
            }
        }
    }
    
    // Handle last module if any
    if let Some(module_name) = current_module {
        let deps = extract_dependencies(&module_content);
        references.insert(module_name, deps);
    }
    
    references
}

/// Extracts dependencies from module content
fn extract_dependencies(content: &str) -> Vec<String> {
    let mut deps = Vec::new();
    let require_pattern = Regex::new(r#"__webpack_require__\(["']([^"']+)["']\)"#).unwrap();
    
    for captures in require_pattern.captures_iter(content) {
        if let Some(dep) = captures.get(1) {
            deps.push(dep.as_str().to_string());
        }
    }
    
    deps.sort();
    deps.dedup();
    deps
}

/// Finds orphaned modules in a chunk based on the share usage config
pub fn find_orphaned_modules(chunk: &str, config: &Value) -> Vec<String> {
    let mut orphaned = Vec::new();
    let module_refs = analyze_module_references(chunk);
    
    // Get all modules that export something according to config
    let mut exported_modules = HashSet::new();
    
    // Handle new format where treeShake contains module data
    if let Some(tree_shake) = config.get("treeShake").and_then(|t| t.as_object()) {
        // For each library in treeShake
        for (_lib_name, lib_data) in tree_shake {
            if let Some(lib_obj) = lib_data.as_object() {
                // Look for exports in the library data
                for (key, value) in lib_obj {
                    // Skip chunk_characteristics and other metadata
                    if key == "chunk_characteristics" {
                        continue;
                    }
                    // If the export is used (true), add it to exported modules
                    if let Some(is_used) = value.as_bool() {
                        if is_used {
                            // The key is the export name, we need to find which modules provide it
                            // This is a simplified approach - in reality you might need more info
                            exported_modules.insert(key.clone());
                        }
                    }
                }
            }
        }
    }
    
    // Also check old format with exports field
    if let Some(exports) = config.get("exports").and_then(|e| e.as_object()) {
        for (_export_name, export_info) in exports {
            if let Some(modules) = export_info.get("modules").and_then(|m| m.as_array()) {
                for module in modules {
                    if let Some(module_name) = module.as_str() {
                        exported_modules.insert(module_name.to_string());
                    }
                }
            }
        }
    }
    
    // Find all modules that are referenced by exported modules
    let mut referenced_modules = HashSet::new();
    let mut to_process: Vec<String> = exported_modules.iter().cloned().collect();
    let mut processed = HashSet::new();
    
    while let Some(module) = to_process.pop() {
        if processed.contains(&module) {
            continue;
        }
        processed.insert(module.clone());
        
        if let Some(deps) = module_refs.get(&module) {
            for dep in deps {
                referenced_modules.insert(dep.clone());
                if !processed.contains(dep) {
                    to_process.push(dep.clone());
                }
            }
        }
    }
    
    // Find modules that exist but are not exported or referenced
    for module_name in module_refs.keys() {
        if !exported_modules.contains(module_name) && !referenced_modules.contains(module_name) {
            orphaned.push(module_name.clone());
        }
    }
    
    orphaned.sort();
    orphaned
}

/// Verifies that a specific module has been removed from the optimized chunk
pub fn verify_module_removed(optimized: &str, module_name: &str) -> bool {
    // Check for module definition
    let patterns = vec![
        format!(r#"["']{}["']:\s*\(function"#, regex::escape(module_name)),
        format!(r#"["']{}["']:\s*\("#, regex::escape(module_name)),
        format!(r#"["']{}["']:\s*function"#, regex::escape(module_name)),
    ];
    
    for pattern in patterns {
        if let Ok(re) = Regex::new(&pattern) {
            if re.is_match(optimized) {
                return false;
            }
        }
    }
    
    true
}

/// Counts how many times a module is required in the chunk
pub fn count_webpack_requires(chunk: &str, module_name: &str) -> usize {
    let pattern = format!(r#"__webpack_require__\(["']{}["']\)"#, regex::escape(module_name));
    if let Ok(re) = Regex::new(&pattern) {
        re.find_iter(chunk).count()
    } else {
        0
    }
}

/// Prints a visual dependency graph for debugging
pub fn print_dependency_graph(chunk: &str, config: &Value) {
    let module_refs = analyze_module_references(chunk);
    
    println!("\n=== Module Dependency Graph ===");
    
    // Get exported modules from config
    let mut exported_modules = HashSet::new();
    let mut used_exports = Vec::new();
    
    // Handle new format where treeShake contains module data
    if let Some(tree_shake) = config.get("treeShake").and_then(|t| t.as_object()) {
        for (lib_name, lib_data) in tree_shake {
            if let Some(lib_obj) = lib_data.as_object() {
                for (key, value) in lib_obj {
                    if key == "chunk_characteristics" {
                        continue;
                    }
                    if let Some(is_used) = value.as_bool() {
                        if is_used {
                            used_exports.push(format!("📦 Export '{}' from library '{}'", key, lib_name));
                            exported_modules.insert(key.clone());
                        }
                    }
                }
            }
        }
    }
    
    // Also check old format with exports field
    if let Some(exports) = config.get("exports").and_then(|e| e.as_object()) {
        for (export_name, export_info) in exports {
            if let Some(modules) = export_info.get("modules").and_then(|m| m.as_array()) {
                for module in modules {
                    if let Some(module_name) = module.as_str() {
                        exported_modules.insert(module_name.to_string());
                        used_exports.push(format!("📦 Export '{}' from module '{}'", export_name, module_name));
                    }
                }
            }
        }
    }
    
    // Print used exports
    for export in &used_exports {
        println!("{}", export);
    }
    
    println!("\n--- Module Dependencies ---");
    for (module, deps) in &module_refs {
        let is_exported = exported_modules.contains(module);
        let prefix = if is_exported { "✅" } else { "❌" };
        
        println!("{} {} ->", prefix, module);
        for dep in deps {
            println!("    └─> {}", dep);
        }
    }
    
    let orphaned = find_orphaned_modules(chunk, config);
    if !orphaned.is_empty() {
        println!("\n🗑️  Orphaned modules (should be removed):");
        for module in &orphaned {
            println!("   - {}", module);
        }
    }
}

/// Traces why a module wasn't removed by checking its dependency chain
pub fn trace_module_retention(chunk: &str, config: &Value, target_module: &str) -> Vec<String> {
    let module_refs = analyze_module_references(chunk);
    let mut trace = Vec::new();
    
    // Get all exported modules
    let mut exported_modules = HashMap::new();
    
    // Handle new format where treeShake contains module data
    if let Some(tree_shake) = config.get("treeShake").and_then(|t| t.as_object()) {
        for (lib_name, lib_data) in tree_shake {
            if let Some(lib_obj) = lib_data.as_object() {
                for (key, value) in lib_obj {
                    if key == "chunk_characteristics" {
                        continue;
                    }
                    if let Some(is_used) = value.as_bool() {
                        if is_used {
                            // In the new format, we track export names from libraries
                            exported_modules.insert(key.clone(), format!("{}:{}", lib_name, key));
                        }
                    }
                }
            }
        }
    }
    
    // Also check old format with exports field
    if let Some(exports) = config.get("exports").and_then(|e| e.as_object()) {
        for (export_name, export_info) in exports {
            if let Some(modules) = export_info.get("modules").and_then(|m| m.as_array()) {
                for module in modules {
                    if let Some(module_name) = module.as_str() {
                        exported_modules.insert(module_name.to_string(), export_name.clone());
                    }
                }
            }
        }
    }
    
    // If target module is directly exported
    if let Some(export_name) = exported_modules.get(target_module) {
        trace.push(format!("Module '{}' is directly exported as '{}'", target_module, export_name));
        return trace;
    }
    
    // Find all paths from exported modules to target
    let mut found_path = false;
    for (exported_module, export_name) in &exported_modules {
        if let Some(path) = find_path_to_module(&module_refs, exported_module, target_module) {
            trace.push(format!("Module '{}' is retained because:", target_module));
            trace.push(format!("  Export '{}' (module '{}') depends on it via:", export_name, exported_module));
            for (i, module) in path.iter().enumerate() {
                trace.push(format!("    {}{}", "  ".repeat(i + 1), module));
            }
            found_path = true;
        }
    }
    
    if !found_path {
        trace.push(format!("Module '{}' has no path from any exported module - it should be removed!", target_module));
    }
    
    trace
}

/// Finds a path from source module to target module
fn find_path_to_module(
    module_refs: &HashMap<String, Vec<String>>,
    source: &str,
    target: &str,
) -> Option<Vec<String>> {
    let mut visited = HashSet::new();
    let mut queue = vec![(source.to_string(), vec![source.to_string()])];
    
    while let Some((current, path)) = queue.pop() {
        if current == target {
            return Some(path);
        }
        
        if visited.contains(&current) {
            continue;
        }
        visited.insert(current.clone());
        
        if let Some(deps) = module_refs.get(&current) {
            for dep in deps {
                if !visited.contains(dep) {
                    let mut new_path = path.clone();
                    new_path.push(dep.clone());
                    queue.push((dep.clone(), new_path));
                }
            }
        }
    }
    
    None
}

/// Counts nullified exports in the optimized chunk
pub fn count_nullified_exports(chunk: &str) -> usize {
    let null_export_pattern = Regex::new(r#"\.exports\s*=\s*null[,;]"#).unwrap();
    null_export_pattern.find_iter(chunk).count()
}

/// Analyzes module characteristics to help with debugging
pub fn analyze_module_characteristics(chunk: &str, module_name: &str) -> HashMap<String, String> {
    let mut characteristics = HashMap::new();
    
    // Find module content
    let module_pattern = format!(r#"["']{}["']:\s*\(function.*?\n([\s\S]*?)\n\s*\}},"#, regex::escape(module_name));
    if let Ok(re) = Regex::new(&module_pattern) {
        if let Some(captures) = re.captures(chunk) {
            if let Some(content) = captures.get(1) {
                let module_content = content.as_str();
                
                // Check for various patterns
                if module_content.contains("module.exports") {
                    characteristics.insert("export_type".to_string(), "CommonJS".to_string());
                } else if module_content.contains("__webpack_exports__") {
                    characteristics.insert("export_type".to_string(), "ES Module".to_string());
                }
                
                // Count requires
                let require_count = count_webpack_requires(module_content, "");
                characteristics.insert("require_count".to_string(), require_count.to_string());
                
                // Check if it's a re-export module
                if module_content.contains("__webpack_require__.r") && module_content.contains("__webpack_require__.d") {
                    characteristics.insert("is_reexport".to_string(), "true".to_string());
                }
                
                // Module size
                characteristics.insert("size".to_string(), module_content.len().to_string());
            }
        }
    }
    
    characteristics
}

/// Gets chunk characteristics from the share usage config
pub fn get_chunk_characteristics<'a>(config: &'a Value, library_name: &str) -> Option<&'a Value> {
    // Try new format first
    if let Some(tree_shake) = config.get("treeShake") {
        if let Some(lib_data) = tree_shake.get(library_name) {
            if let Some(chunk_chars) = lib_data.get("chunk_characteristics") {
                return Some(chunk_chars);
            }
        }
    }
    
    // Try old format
    if let Some(chunk_chars) = config.get("chunk_characteristics") {
        return Some(chunk_chars);
    }
    
    None
}

/// Checks if a chunk is a shared chunk based on characteristics
pub fn is_shared_chunk(config: &Value, library_name: &str) -> bool {
    if let Some(chunk_chars) = get_chunk_characteristics(config, library_name) {
        if let Some(is_shared) = chunk_chars.get("is_shared_chunk").and_then(|v| v.as_bool()) {
            return is_shared;
        }
    }
    false
}

/// Gets the entry module ID for a library from the config
pub fn get_entry_module_id(config: &Value, library_name: &str) -> Option<String> {
    if let Some(chunk_chars) = get_chunk_characteristics(config, library_name) {
        if let Some(entry_id) = chunk_chars.get("entry_module_id").and_then(|v| v.as_str()) {
            return Some(entry_id.to_string());
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_analyze_module_references() {
        let chunk = r#"
"./src/add.js": (function(module, exports, __webpack_require__) {
    const utils = __webpack_require__("./src/utils.js");
    module.exports = function add(a, b) { return a + b; };
}),
"./src/utils.js": (function(module, exports, __webpack_require__) {
    module.exports = { helper: true };
}),
        "#;
        
        let refs = analyze_module_references(chunk);
        assert_eq!(refs.get("./src/add.js").unwrap(), &vec!["./src/utils.js"]);
        assert_eq!(refs.get("./src/utils.js").unwrap().len(), 0);
    }
    
    #[test]
    fn test_verify_module_removed() {
        let chunk_with_module = r#"
"./src/add.js": (function(module, exports, __webpack_require__) {
    module.exports = function add(a, b) { return a + b; };
}),
        "#;
        
        let chunk_without_module = r#"
"./src/subtract.js": (function(module, exports, __webpack_require__) {
    module.exports = function subtract(a, b) { return a - b; };
}),
        "#;
        
        assert!(!verify_module_removed(chunk_with_module, "./src/add.js"));
        assert!(verify_module_removed(chunk_without_module, "./src/add.js"));
    }
}