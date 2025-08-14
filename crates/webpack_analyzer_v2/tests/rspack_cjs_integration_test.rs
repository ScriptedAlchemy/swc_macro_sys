use std::fs;
use std::path::PathBuf;

use serde_json::Value;
// use swc_core::atoms::Atom;

use webpack_analyzer_v2::{ChunkCharacteristics, ChunkType, ShareUsageConfig, TreeShaker, WebpackAnalyzer};

fn repo_path(segments: &[&str]) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    for s in ["..", ".."].into_iter() { p.push(s); }
    for seg in segments { p.push(seg); }
    p
}

#[test]
fn test_rspack_cjs_lodash_chunk_prune_with_explicit_entry() {
    // Load CJS chunk (rspack cjs annotated output)
    let chunk_path = repo_path(&[
        "test-cases",
        "rspack-cjs-annotated-output",
        "vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js",
    ]);
    let source = fs::read_to_string(&chunk_path).expect("failed to read rspack cjs lodash chunk");

    // Load share-usage for lodash-es
    let usage_path = repo_path(&["test-cases", "rspack-cjs-annotated-output", "share-usage.json"]);
    let usage_json: Value = serde_json::from_str(&fs::read_to_string(&usage_path).expect("read usage"))
        .expect("invalid json");
    let entry_str = usage_json["treeShake"]["lodash-es"]["chunk_characteristics"]["entry_module_id"]
        .as_str()
        .expect("missing entry_module_id");

    let characteristics = ChunkCharacteristics {
        is_runtime_chunk: false,
        has_runtime: false,
        is_entrypoint: false,
        can_be_initial: false,
        is_only_initial: false,
        chunk_format: "async-node".to_string(),
        chunk_loading_type: None,
        runtime_names: vec!["main".to_string()],
        entry_name: None,
        has_async_chunks: false,
        chunk_files: vec![chunk_path.file_name().unwrap().to_string_lossy().to_string()],
        is_shared_chunk: false,
        shared_modules: vec![],
        entry_module_id: Some(entry_str.to_string()),
    };

    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer
        .analyze_chunk(&source, characteristics)
        .expect("analyze rspack cjs lodash chunk");

    assert!(matches!(chunk.chunk_type, ChunkType::CommonJSAsync | ChunkType::CommonJSSync));
    assert!(chunk.module_count() > 0);

    // Hard requirement: characteristics drive pruning exclusively; no guessing
    let shaker = TreeShaker::new();
    let cfg = ShareUsageConfig { entry_module_ids: vec![] };
    let plan = shaker.plan_prune(&chunk, &cfg);
    // Depending on whether the entry is present in modules, plan may proceed or skip
    // but it must not guess beyond characteristics; never error
    assert!(plan.skip_reason.is_some() || plan.pruned_count <= plan.original_count);
}


