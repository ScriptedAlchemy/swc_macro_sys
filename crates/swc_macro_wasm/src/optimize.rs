use std::time::{Duration, Instant};
use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::VisitMutWith;
use swc_ecma_ast::Program;
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{text_writer, Emitter};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use thiserror::Error;
// Re-enabling webpack_analyzer_v2 step by step
use webpack_analyzer_v2::{ChunkCharacteristics, TreeShaker as AnalyzerTreeShaker};

/// Error types for optimization operations
#[derive(Error, Debug)]
pub enum OptimizationError {
    #[error("Failed to parse JavaScript: {0}")]
    ParseError(String),

    #[error("Failed to emit JavaScript: {0}")]
    EmitError(String),

    #[error("Invalid UTF-8 encoding in generated code")]
    Utf8Error,

    #[error("Webpack analysis failed: {0}")]
    WebpackAnalysisError(String),

    #[error("Numeric conversion failed: {0}")]
    NumericConversionError(String),
}

type OptimizationResult<T> = Result<T, OptimizationError>;

pub fn optimize(source: String, config: serde_json::Value) -> OptimizationResult<String> {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"optimize::optimize: Starting optimization".into());
    
    // Extract minify option from config (default to true for backward compatibility)
    let should_minify = config.get("minify")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);
    
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&format!("optimize::optimize: Minification enabled: {}", should_minify).into());
    
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"optimize::optimize: Creating source file".into());
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"optimize::optimize: About to parse with Parser::new".into());
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .map_err(|e| OptimizationError::ParseError(format!("Parser error: {:?}", e)))?;
        (program, comments)
    };

    let macros = {
        let parser = MacroParser::new("common");
        parser.parse(&comments)
    };

    let program = {
        let mut transformer = condition_transform(config.clone(), macros);
        program.visit_mut_with(&mut transformer);

        // Apply resolver and optimization
        // This worked fine in WASM from the beginning (June 2025)
        // Only the TreeShaker for webpack bundles causes WASM panics
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            // Run DCE after macro conditions so orphaned require calls are dropped
            perform_dce(&mut program, comments.clone(), unresolved_mark);

            // Prune unreachable modules by rewriting modules map
            if has_macro_processing_config(&config) {
                run_webpack_tree_shake(&mut program, cm.clone(), &comments, &config, should_minify);
            }

            // Final fixer after pruning to keep AST consistent
            program.mutate(fixer(Some(&comments)));
            
            program
        })
    };

    let ret = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(should_minify),
            comments: Some(&comments),
            cm: cm.clone(),
            wr,
        };
        emitter
            .emit_program(&program)
            .map_err(|e| OptimizationError::EmitError(format!("Emit error: {:?}", e)))?;
        drop(emitter);

        String::from_utf8(buf).map_err(|_| OptimizationError::Utf8Error)?
    };

    Ok(ret)
}

// DCE is intentionally not run before analysis to preserve the modules table structure
// for the analyzer. If needed in the future, a post-prune DCE pass that preserves
// top-level module containers can be added back with a conservative configuration.
fn perform_dce(_m: &mut Program, _comments: SingleThreadedComments, _unresolved_mark: Mark) { }

/// Check if the config contains macro processing directives that might create orphaned modules
fn has_macro_processing_config(config: &serde_json::Value) -> bool {
    // Check if there are macro processing directives like features, treeShake, etc.
    // Entry modules are no longer supported; rely strictly on chunk_characteristics
    config.get("treeShake").is_some()
}

// Simple tree shaking removed - using webpack_analyzer_v2 TreeShaker instead

// Synthetic entry point calls removed - TreeShaker handles this properly

// The complex TreeShaker using webpack_analyzer_v2 - re-enabling step by step

/// Extract chunk characteristics strictly from config
/// Falls back to a safe default (CommonJSSync) without inspecting source
fn get_chunk_characteristics(config: &serde_json::Value) -> Option<ChunkCharacteristics> {
    // First check if chunk_characteristics is at the root level (old format)
    if let Some(chars_value) = config.get("chunk_characteristics")
        && let Ok(characteristics) =
            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
        {
            return Some(characteristics);
        }

    // Check if chunk_characteristics is nested within treeShake (new format)
    if let Some(tree_shake_config) = config.get("treeShake")
        && let Some(tree_shake_obj) = tree_shake_config.as_object() {
            // Look for chunk_characteristics in any of the modules
            for (_, module_config) in tree_shake_obj {
                if let Some(module_obj) = module_config.as_object()
                    && let Some(chars_value) = module_obj.get("chunk_characteristics")
                        && let Ok(characteristics) =
                            serde_json::from_value::<ChunkCharacteristics>(chars_value.clone())
                        {
                            return Some(characteristics);
                        }
            }
        }

    // No fallback: hard requirement enforced
    None
}

/// Metrics collected during tree shaking optimization
#[derive(Debug, Clone)]
struct TreeShakeMetrics {
    modules_before: usize,
    modules_after: usize,
    modules_removed: usize,
    iterations: u32,
    time_taken: Duration,
    chunks_processed: usize,
}

impl TreeShakeMetrics {
    fn new() -> Self {
        Self {
            modules_before: 0,
            modules_after: 0,
            modules_removed: 0,
            iterations: 0,
            time_taken: Duration::new(0, 0),
            chunks_processed: 0,
        }
    }

    fn log_summary(&self) {
        if self.modules_removed > 0 {
            println!(
                "Tree shaking summary: Removed {} modules across {} iterations in {:?} ({}% reduction)",
                self.modules_removed,
                self.iterations,
                self.time_taken,
                (self.modules_removed * 100) / self.modules_before.max(1)
            );
        } else {
            println!(
                "Tree shaking summary: No modules removed in {} iterations ({:?})",
                self.iterations, self.time_taken
            );
        }
    }
}

/// Perform webpack tree shaking by delegating to webpack_analyzer_v2's TreeShaker
fn run_webpack_tree_shake(
    program: &mut Program,
    cm: Lrc<SourceMap>,
    comments: &SingleThreadedComments,
    config: &serde_json::Value,
    should_minify: bool,
) {
    #[cfg(target_arch = "wasm32")]
    web_sys::console::log_1(&"TreeShaker::optimize: Starting tree shaking optimization".into());
    let timer = WasmTimer::start();
    let mut metrics = TreeShakeMetrics::new();

    let mut total_removed = 0;
    let max_iterations = 5; // Prevent infinite loops

    for iteration in 1..=max_iterations {
        // Step 1: Emit current AST to string for analysis
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&format!("TreeShaker: Starting iteration {}", iteration).into());
        let current_code = {
            let mut buf = vec![];
            let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
                as Box<dyn WriteJs>;
            let mut emitter = Emitter {
                cfg: codegen::Config::default().with_minify(should_minify),
                comments: Some(comments),
                cm: cm.clone(),
                wr,
            };
            if emitter.emit_program(program).is_err() {
                break; // Stop if emit fails
            }
            drop(emitter);

            match String::from_utf8(buf) {
                Ok(code) => code,
                Err(_) => break, // Stop if invalid UTF-8
            }
        };

        // Step 2: Analyze the chunk using webpack_analyzer_v2
        #[cfg(target_arch = "wasm32")]
        web_sys::console::log_1(&"TreeShaker: Getting chunk characteristics".into());
        let Some(characteristics) = get_chunk_characteristics(config) else {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"Tree shaking: No chunk_characteristics provided in config - skipping tree shaking".into());
            return;
        };
        // Skip entry or runtime chunks entirely based on characteristics
        if characteristics.is_entrypoint || characteristics.is_runtime_chunk {
            #[cfg(target_arch = "wasm32")]
            web_sys::console::log_1(&"Tree shaking: Skipping entry or runtime chunk".into());
            return;
        }

        // Delegate planning and pruning to analyzer's TreeShaker
        let analyzer_shaker = AnalyzerTreeShaker::new();
        match analyzer_shaker.prune_source(&current_code, &characteristics.clone()) {
            Ok((optimized_source, plan)) => {
                if iteration == 1 {
                    metrics.modules_before = plan.original_count;
                    metrics.chunks_processed = 1;
                }
                if let Some(reason) = &plan.skip_reason {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&format!("Tree shaking skipped: {}", reason).into());
                    return;
                }

                if plan.removed_modules.is_empty() || optimized_source == current_code {
                    if iteration == 1 {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(&format!("Tree shaking: No unreachable modules found. Kept {} modules, removed {}", plan.kept_modules.len(), plan.removed_modules.len()).into());
                    } else {
                        #[cfg(target_arch = "wasm32")]
                        web_sys::console::log_1(&format!(
                            "Tree shaking: Converged after {} iterations, removed {} total modules",
                            iteration - 1,
                            total_removed
                        ).into());
                    }
                    metrics.iterations = iteration;
                    break;
                }

                total_removed += plan.removed_modules.len();

                // Re-parse optimized source into AST and replace program
                let fm2 = cm
                    .new_source_file(FileName::Custom("optimized.js".to_string()).into(), optimized_source);
                let parsed = Parser::new(Syntax::Es(EsSyntax::default()), StringInput::from(&*fm2), Some(comments))
                    .parse_program()
                    .map_err(|e| OptimizationError::ParseError(format!("Parser error after prune: {:?}", e)));
                if let Ok(new_prog) = parsed {
                    *program = new_prog;
                    // Continue to next iteration to see if more become unreachable
                } else {
                    #[cfg(target_arch = "wasm32")]
                    web_sys::console::log_1(&"Tree shaking: Failed to reparse optimized source - stopping".into());
                    break;
                }
            }
            Err(err) => {
                #[cfg(target_arch = "wasm32")]
                web_sys::console::log_1(&format!("Tree shaking: Analyzer prune failed: {} - skipping", err).into());
                return;
            }
        }

        // Continue to next iteration to see if more modules become unreachable
    }

    // Update final metrics
    metrics.modules_removed = total_removed;
    metrics.modules_after = metrics.modules_before.saturating_sub(total_removed);
    metrics.time_taken = timer.elapsed();
    metrics.iterations = if metrics.iterations == 0 { max_iterations } else { metrics.iterations };

    // Log comprehensive summary
    metrics.log_summary();

    if total_removed > 0 {
        println!(
            "Tree shaking: Total removed {} modules across all iterations",
            total_removed
        );
    }
}

/// Cross-platform timer: uses std::time::Instant on native, js_sys::Date on wasm32
#[cfg(target_arch = "wasm32")]
struct WasmTimer {
    start_ms: f64,
}

#[cfg(target_arch = "wasm32")]
impl WasmTimer {
    fn start() -> Self {
        Self { start_ms: js_sys::Date::now() }
    }

    fn elapsed(&self) -> Duration {
        let now = js_sys::Date::now();
        let ms = if now >= self.start_ms { now - self.start_ms } else { 0.0 };
        // Convert milliseconds to seconds as f64, then to Duration
        Duration::from_secs_f64(ms / 1000.0)
    }
}

#[cfg(not(target_arch = "wasm32"))]
struct WasmTimer {
    start: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
impl WasmTimer {
    fn start() -> Self {
        Self { start: Instant::now() }
    }

    fn elapsed(&self) -> Duration {
        self.start.elapsed()
    }
}

// Legacy entry point extraction removed - explicit entry points required