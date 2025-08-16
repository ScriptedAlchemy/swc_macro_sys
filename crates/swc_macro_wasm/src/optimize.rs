use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_ast::Program;
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use crate::webpack_parser::WebpackChunkParser;

pub fn optimize(source: String, config: serde_json::Value) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source.clone());
        let comments = SingleThreadedComments::default();
        
        // Handle parsing errors gracefully without panicking
        let program = match Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program() {
            Ok(program) => program,
            Err(e) => {
                eprintln!("SWC parsing failed: {:?}", e);
                eprintln!("Returning original source due to parsing error");
                // Return the original source if parsing fails
                return source;
            }
        };
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");

        parser.parse(&comments)
    };

    // Clone config so we can still access it after passing into the transformer
    let config_clone = config.clone();

    let program = {
        let mut transformer = condition_transform(config, macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, comments.clone(), unresolved_mark);

            program.mutate(fixer(Some(&comments)));

            // After DCE, run webpack parser to build dependency graph for potential pruning
            // Emit current program (post-DCE) to string and analyze
            let mut intermediate_buf = vec![];
            {
                let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut intermediate_buf, None))
                    as Box<dyn WriteJs>;
                let mut emitter = Emitter {
                    cfg: codegen::Config::default().with_minify(false),
                    comments: Some(&comments),
                    cm: cm.clone(),
                    wr,
                };
                // If emit fails here, continue with original flow without analysis
                if emitter.emit_program(&program).is_err() {
                    return program;
                }
            }
            let dce_output = match String::from_utf8(intermediate_buf) {
                Ok(s) => s,
                Err(_) => return program,
            };

            if let Ok(parser) = WebpackChunkParser::new() {
                if let Ok(chunk) = parser.parse_chunk_file(&dce_output) {
                    let _graph = parser.build_dependency_graph(&chunk);
                    // Optionally consider entry_module_id from config for future pruning
                    let _entry_module_id = config_clone
                        .get("treeShake")
                        .and_then(|ts| ts.as_object())
                        .and_then(|obj| obj.values().next())
                        .and_then(|pkg| pkg.get("chunk_characteristics"))
                        .and_then(|cc| cc.get("entry_module_id"))
                        .and_then(|v| v.as_str())
                        .map(|s| s.to_string());
                    // Future work: compute reachable set from entry_module_id and prune unreachable modules
                }
            }
 
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        if let Err(e) = emitter.emit_program(&program) {
            eprintln!("Failed to emit program: {:?}", e);
            return source;
        }
        drop(emitter);

        match String::from_utf8(buf) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("Failed to convert to UTF-8: {:?}", e);
                return source;
            }
        }
    };

    ret
}

fn perform_dce(m: &mut Program, comments: SingleThreadedComments, unresolved_mark: Mark) {
    let mut visitor = crate::dce::dce(
        comments,
        crate::dce::Config {
            module_mark: None,
            top_level: true,
            top_retain: Default::default(),
            preserve_imports_with_side_effects: true,
        },
        unresolved_mark,
    );

    loop {
        m.visit_mut_with(&mut visitor);

        if !visitor.changed() {
            break;
        }

        visitor.reset();
    }
}
