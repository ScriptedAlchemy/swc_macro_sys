use crate::module_extractor::WebpackModuleExtractor;
use swc_core::common::{sync::Lrc, SourceMap, FileName};
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};

#[test]
fn test_split_chunk_format() {
    let split_chunk = r#"
(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"], {
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_DataView.js": function() { console.log("DataView"); },
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_Hash.js": function() { console.log("Hash"); },
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js": function() { console.log("lodash"); }
}]);
"#;

    let mut extractor = WebpackModuleExtractor::new();
    
    // Parse the chunk
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), split_chunk.to_string());
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        None,
    );
    
    let program = parser.parse_program().expect("Failed to parse");
    extractor.extract_modules(&program);
    
    // Check results
    assert!(extractor.is_split_chunk, "Should detect split chunk format");
    assert_eq!(extractor.modules.len(), 3, "Should extract 3 modules");
    
    let expected_modules = vec![
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_DataView.js",
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_Hash.js",
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
    ];
    
    for module_id in expected_modules {
        assert!(extractor.modules.contains_key(module_id), "Should contain module: {}", module_id);
    }
}

#[test]
fn test_commonjs_format() {
    let commonjs_chunk = r#"
"use strict";
exports.ids = ["vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js"];
exports.modules = {
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_DataView.js": function() { console.log("DataView"); },
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_Hash.js": function() { console.log("Hash"); },
  "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js": function() { console.log("lodash"); }
};
"#;

    let mut extractor = WebpackModuleExtractor::new();
    
    // Parse the chunk
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), commonjs_chunk.to_string());
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        None,
    );
    
    let program = parser.parse_program().expect("Failed to parse");
    extractor.extract_modules(&program);
    
    // Check results
    assert!(extractor.is_split_chunk, "Should detect split chunk format");
    assert_eq!(extractor.modules.len(), 3, "Should extract 3 modules");
    
    let expected_modules = vec![
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_DataView.js",
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/_Hash.js",
        "../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
    ];
    
    for module_id in expected_modules {
        assert!(extractor.modules.contains_key(module_id), "Should contain module: {}", module_id);
    }
}

#[test]
fn test_empty_chunk() {
    let empty_chunk = r#"
console.log("Not a webpack chunk");
"#;

    let mut extractor = WebpackModuleExtractor::new();
    
    // Parse the chunk
    let cm: Lrc<SourceMap> = Default::default();
    let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), empty_chunk.to_string());
    let mut parser = Parser::new(
        Syntax::Es(EsSyntax::default()),
        StringInput::from(&*fm),
        None,
    );
    
    let program = parser.parse_program().expect("Failed to parse");
    extractor.extract_modules(&program);
    
    // Check results
    assert!(!extractor.is_split_chunk, "Should not detect split chunk format");
    assert_eq!(extractor.modules.len(), 0, "Should extract 0 modules");
}