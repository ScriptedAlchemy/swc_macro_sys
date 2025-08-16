//! Extract webpack_require__ dependencies and generate markdown report
//!
//! This utility extracts all __webpack_require__ calls from webpack chunk files
//! and generates a comprehensive markdown report showing dependencies for each module.

use std::fs;
use std::path::Path;
use swc_macro_wasm::webpack_parser::WebpackChunkParser;

/// Generate a markdown report of webpack dependencies for a single chunk
fn generate_chunk_markdown(chunk_name: &str, modules_with_deps: Vec<(String, Vec<String>)>) -> String {
    let mut markdown = String::new();
    
    // Add chunk header
    markdown.push_str(&format!("# {}\n\n", chunk_name));
    markdown.push_str(&format!("This chunk contains {} modules with webpack dependencies.\n\n", modules_with_deps.len()));
    markdown.push_str("---\n\n");
    
    // Sort modules by key for consistent output
    let mut sorted_modules = modules_with_deps;
    sorted_modules.sort_by(|a, b| a.0.cmp(&b.0));
    
    for (i, (module_key, dependencies)) in sorted_modules.iter().enumerate() {
        // Module header
        markdown.push_str(&format!("## Module {} of {}\n\n", i + 1, sorted_modules.len()));
        markdown.push_str(&format!("**Module Key:** `{}`\n\n", module_key));
        
        // Dependencies section
        if dependencies.is_empty() {
            markdown.push_str("**Dependencies:** None\n\n");
        } else {
            markdown.push_str(&format!("**Dependencies:** {} found\n\n", dependencies.len()));
            
            // List all dependencies
            for (dep_idx, dep) in dependencies.iter().enumerate() {
                markdown.push_str(&format!("{}. `{}`\n", dep_idx + 1, dep));
            }
            markdown.push_str("\n");
        }
        
        // Add separator between modules (except for the last one)
        if i < sorted_modules.len() - 1 {
            markdown.push_str("---\n\n");
        }
    }
    
    // Add chunk summary
    markdown.push_str("\n## Chunk Summary\n\n");
    let total_deps: usize = sorted_modules.iter().map(|(_, deps)| deps.len()).sum();
    let modules_with_deps_count = sorted_modules.iter().filter(|(_, deps)| !deps.is_empty()).count();
    
    markdown.push_str(&format!("- **Total Modules:** {}\n", sorted_modules.len()));
    markdown.push_str(&format!("- **Modules with Dependencies:** {}\n", modules_with_deps_count));
    markdown.push_str(&format!("- **Total Dependencies:** {}\n", total_deps));
    
    if total_deps > 0 {
        let avg_deps = total_deps as f64 / modules_with_deps_count as f64;
        markdown.push_str(&format!("- **Average Dependencies per Module:** {:.2}\n", avg_deps));
    }
    
    markdown.push_str("\n\n");
    markdown
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let jsonp_dir = "/Users/bytedance/dev/swc_macro_sys/tests/jsonp";
    
    println!("Extracting webpack dependencies from all chunks in: {}", jsonp_dir);
    
    // Read directory and find all .js files
    let entries = fs::read_dir(jsonp_dir)?;
    let mut js_files = Vec::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().map_or(false, |ext| ext == "js") {
            js_files.push(path);
        }
    }
    
    // Sort files for consistent output
    js_files.sort();
    
    println!("Found {} JavaScript chunk files", js_files.len());
    
    // Create the parser
    let parser = WebpackChunkParser::new()?;
    
    // Process each chunk file and collect markdown
    let mut full_markdown = String::new();
    let mut total_chunks_processed = 0;
    let mut total_modules_across_chunks = 0;
    let mut total_deps_across_chunks = 0;
    
    for js_file in &js_files {
        let file_name = js_file.file_name().unwrap().to_string_lossy();
        println!("Processing chunk: {}", file_name);
        
        // Read the file content
        match fs::read_to_string(js_file) {
            Ok(content) => {
                // Parse the chunk file
                match parser.parse_chunk_file(&content) {
                    Ok(chunk_info) => {
                        // Get modules with their dependencies
                        let modules_with_deps = parser.get_modules_with_dependencies(&chunk_info);
                        
                        if !modules_with_deps.is_empty() {
                            println!("  Found {} modules", modules_with_deps.len());
                            
                            // Generate markdown for this chunk
                            let chunk_markdown = generate_chunk_markdown(&file_name, modules_with_deps.clone());
                            full_markdown.push_str(&chunk_markdown);
                            
                            total_chunks_processed += 1;
                            total_modules_across_chunks += modules_with_deps.len();
                            total_deps_across_chunks += modules_with_deps.iter().map(|(_, deps)| deps.len()).sum::<usize>();
                        } else {
                            println!("  No modules found in this chunk");
                        }
                    }
                    Err(e) => {
                        println!("  Error parsing chunk {}: {}", file_name, e);
                    }
                }
            }
            Err(e) => {
                println!("  Error reading file {}: {}", file_name, e);
            }
        }
    }
    
    // Add overall summary at the end
    full_markdown.push_str("# Overall Summary\n\n");
    full_markdown.push_str(&format!("- **Total Chunk Files Processed:** {}\n", total_chunks_processed));
    full_markdown.push_str(&format!("- **Total Modules Across All Chunks:** {}\n", total_modules_across_chunks));
    full_markdown.push_str(&format!("- **Total Dependencies Across All Chunks:** {}\n", total_deps_across_chunks));
    
    if total_modules_across_chunks > 0 {
        let avg_deps_per_module = total_deps_across_chunks as f64 / total_modules_across_chunks as f64;
        full_markdown.push_str(&format!("- **Average Dependencies per Module:** {:.2}\n", avg_deps_per_module));
    }
    
    // Write to file
    let output_file = "webpack_all_chunks_dependencies_report.md";
    fs::write(output_file, full_markdown)?;
    
    println!("\nComprehensive markdown report generated: {}", output_file);
    println!("Processed {} chunks with {} total modules and {} total dependencies", 
             total_chunks_processed, total_modules_across_chunks, total_deps_across_chunks);
    
    Ok(())
}