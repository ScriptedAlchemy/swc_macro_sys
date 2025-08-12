use std::fs;
use std::path::{Path, PathBuf};
use regex::Regex;
use serde_json::Value;
use swc_macro_wasm::optimize;
use webpack_analyzer_v2::{WebpackAnalyzer, ChunkCharacteristics};
use webpack_analyzer_v2::WebpackChunk;

fn host_dist_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent().unwrap() // crates/
        .parent().unwrap() // repo root
        .join("examples/module-federation-react-example/host/dist")
}

fn read_share_usage() -> Value {
    let path = host_dist_dir().join("share-usage.json");
    let content = fs::read_to_string(&path)
        .expect(&format!("Failed to read share-usage.json at {}", path.display()));
    serde_json::from_str(&content).expect("Failed to parse share-usage.json")
}

fn find_file_containing(substring: &str) -> Option<PathBuf> {
    let dist = host_dist_dir();
    let entries = fs::read_dir(&dist).ok()?;
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "js") {
            let name = path.file_name().unwrap().to_string_lossy();
            if name.contains(substring) {
                return Some(path);
            }
        }
    }
    None
}

fn read_file(path: &Path) -> String {
    fs::read_to_string(path).expect(&format!("Failed to read {}", path.display()))
}

fn analyze_jsonp_modules_count(source: &str) -> usize {
    let analyzer = WebpackAnalyzer::new();
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    match analyzer.analyze_chunk(source, characteristics) {
        Ok(chunk) => chunk.module_count(),
        Err(_) => 0,
    }
}

fn analyze_jsonp_chunk(source: &str) -> Option<WebpackChunk> {
    let analyzer = WebpackAnalyzer::new();
    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: true,
        is_only_initial: false,
        chunk_format: "jsonp".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec!["chunk.js".to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
    };
    analyzer.analyze_chunk(source, characteristics).ok()
}

fn print_chunk_modules_and_requires(label: &str, chunk: &WebpackChunk) {
    println!("\n===== {} =====", label);
    println!("Total modules: {}", chunk.module_count());
    let mut module_ids: Vec<_> = chunk.modules.keys().cloned().collect();
    module_ids.sort();
    for id in &module_ids {
        println!("  MODULE: {}", id);
        if let Some(m) = chunk.modules.get(id) {
            let mut deps: Vec<_> = m.get_dependencies();
            deps.sort();
            for dep in deps {
                println!("    require -> {}", dep);
            }
        }
    }
}

#[test]
fn test_react_dom_index_chunk_preserved_after_optimize() {
    // Locate the React DOM index JSONP vendor chunk
    let index_chunk = find_file_containing("react-dom_index_js.js")
        .expect("Could not find react-dom index jsonp chunk in host/dist");

    let original = read_file(&index_chunk);
    assert!(original.contains(".push(["), "Expected JSONP push() format in index chunk");

    // Pre-analyze: must have at least one module
    let pre_modules = analyze_jsonp_modules_count(&original);
    assert!(pre_modules > 0, "Analyzer should find modules in the index JSONP chunk");

    // Build config from share-usage.json (as produced by build)
    let share_usage = read_share_usage();
    let config_json = serde_json::to_string(&share_usage).unwrap();

    // Optimize the index chunk
    let optimized = optimize(original.clone(), &config_json);
    assert!(!optimized.is_empty(), "Optimized output should not be empty");

    // Post-analyze: the index module should still be present in JSONP object
    let post_modules = analyze_jsonp_modules_count(&optimized);
    assert!(post_modules > 0, "Optimization should not strip all modules from index chunk");

    // Ensure the require to cjs/react-dom.development.js is still emitted
    let require_re = Regex::new(r#"__webpack_require__\(\"([^\"]+react-dom/(cjs/)?react-dom\.[^\"]+)\"\)"#).unwrap();
    let original_dep = require_re.captures(&original)
        .and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
        .expect("Could not find __webpack_require__ to react-dom cjs file in original index chunk");

    let optimized_dep = require_re.captures(&optimized)
        .and_then(|c| c.get(1)).map(|m| m.as_str().to_string())
        .expect("Optimized index chunk lost the require to react-dom cjs file");

    assert_eq!(original_dep, optimized_dep, "Required react-dom entry should be preserved");
}

#[test]
fn test_react_dom_development_chunk_preservation_if_present() {
    // Try to discover the cjs/react-dom.* chunk by scanning all dist .js files for the module id
    let index_chunk = match find_file_containing("react-dom_index_js.js") { Some(p) => p, None => return };
    let index_source = read_file(&index_chunk);

    let require_re = Regex::new(r#"__webpack_require__\(\"([^\"]+react-dom/(cjs/)?react-dom\.[^\"]+)\"\)"#).unwrap();
    let dev_module_id = match require_re.captures(&index_source).and_then(|c| c.get(1)).map(|m| m.as_str().to_string()) {
        Some(s) => s,
        None => {
            // If no cjs file is referenced, nothing to validate here
            return;
        }
    };

    // Find which chunk file contains that module id string
    let dist = host_dist_dir();
    let mut dev_chunk_path: Option<PathBuf> = None;
    for entry in fs::read_dir(&dist).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().map_or(false, |e| e == "js") {
            let content = read_file(&path);
            if content.contains(&format!("\"{}\"", dev_module_id)) || content.contains(&dev_module_id) {
                dev_chunk_path = Some(path);
                break;
            }
        }
    }

    let dev_chunk_path = match dev_chunk_path { Some(p) => p, None => return };
    let original_dev = read_file(&dev_chunk_path);

    // Analyze before: ensure the dev module id appears as a module key
    assert!(original_dev.contains(&dev_module_id), "Dev module id should appear in its chunk before optimization");

    // Optimize with real share usage config
    let share_usage = read_share_usage();
    let config_json = serde_json::to_string(&share_usage).unwrap();
    let optimized_dev = optimize(original_dev.clone(), &config_json);
    assert!(!optimized_dev.is_empty(), "Optimized dev chunk should not be empty");

    // After optimization, the module id must still be present (not removed)
    assert!(optimized_dev.contains(&dev_module_id),
        "Optimization removed react-dom dev module {}; this will cause __webpack_require__ to be undefined",
        dev_module_id);
}

#[test]
fn debug_react_dom_chunk_analysis() {
    // Locate index jsonp chunk
    let index_chunk = match find_file_containing("react-dom_index_js.js") { Some(p) => p, None => return };
    let original = read_file(&index_chunk);

    // Analyze before
    if let Some(pre_chunk) = analyze_jsonp_chunk(&original) {
        print_chunk_modules_and_requires("BEFORE OPTIMIZATION (react-dom index JSONP)", &pre_chunk);
    } else {
        println!("Analyzer failed on original react-dom index chunk");
    }

    // Optimize with real config
    let share_usage = read_share_usage();
    let config_json = serde_json::to_string(&share_usage).unwrap();
    let optimized = optimize(original.clone(), &config_json);

    // Analyze after
    if let Some(post_chunk) = analyze_jsonp_chunk(&optimized) {
        print_chunk_modules_and_requires("AFTER OPTIMIZATION (react-dom index JSONP)", &post_chunk);

        // Diff modules
        let mut pre_ids = Vec::new();
        if let Some(pre_chunk) = analyze_jsonp_chunk(&original) {
            pre_ids = pre_chunk.get_module_ids();
        }
        let post_ids = post_chunk.get_module_ids();
        let pre_set: std::collections::HashSet<_> = pre_ids.iter().cloned().collect();
        let post_set: std::collections::HashSet<_> = post_ids.iter().cloned().collect();
        let removed: Vec<_> = pre_set.difference(&post_set).cloned().collect();
        let added: Vec<_> = post_set.difference(&pre_set).cloned().collect();
        if !removed.is_empty() { println!("Removed modules: {:?}", removed); }
        if !added.is_empty() { println!("Added modules: {:?}", added); }
    } else {
        println!("Analyzer failed on optimized react-dom index chunk");
    }
}


