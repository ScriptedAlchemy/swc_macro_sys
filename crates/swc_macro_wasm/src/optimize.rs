use swc_common::comments::SingleThreadedComments;
use swc_common::pass::Repeated;
use swc_common::sync::Lrc;
use swc_common::{FileName, Mark, SourceMap};
use swc_core::ecma::codegen;
use swc_core::ecma::visit::{VisitMutWith, VisitMut};
use swc_core::atoms::Atom;
use swc_ecma_ast::{Program, Expr, VarDecl, Pat, ObjectLit, PropOrSpread, Prop, PropName};
use swc_ecma_codegen::text_writer::WriteJs;
use swc_ecma_codegen::{Emitter, text_writer};
use swc_ecma_parser::{EsSyntax, Parser, StringInput, Syntax};
use swc_ecma_transforms_base::fixer::fixer;
use swc_ecma_transforms_base::resolver;
use swc_macro_condition_transform::condition_transform;
use swc_macro_parser::MacroParser;
use webpack_analyzer_v2::{WebpackAnalyzer, ChunkType};
use webpack_chunk_tree_shaker::WebpackTreeShaker;
use rustc_hash::FxHashSet;
use regex::Regex;

pub fn optimize(source: String, config: serde_json::Value) -> String {
    let cm: Lrc<SourceMap> = Default::default();
    let (mut program, comments) = {
        let fm = cm.new_source_file(FileName::Custom("test.js".to_string()).into(), source);
        let comments = SingleThreadedComments::default();
        let program = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments),
        )
        .parse_program()
        .unwrap();
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
        swc_common::GLOBALS.set(&Default::default(), || {
            let unresolved_mark = Mark::new();
            let top_level_mark = Mark::new();

            program.mutate(resolver(unresolved_mark, top_level_mark, false));

            perform_dce(&mut program, comments.clone(), unresolved_mark);

            // Tree shake webpack modules after removing unused imports
            perform_webpack_tree_shaking(&mut program, cm.clone(), &comments, &config);
            
            // As a fallback, also try simple regex-based orphan removal
            // This handles cases where the AST-based approach fails on complex chunks
            perform_simple_orphan_removal(&mut program, cm.clone(), &comments, &config);

            program.mutate(fixer(Some(&comments)));

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
        emitter.emit_program(&program).unwrap();
        drop(emitter);

        unsafe { String::from_utf8_unchecked(buf) }
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

/// Check if the config contains macro processing directives that might create orphaned modules
fn has_macro_processing_config(config: &serde_json::Value) -> bool {
    // Check if there are macro processing directives like features, treeShake, etc.
    // Also check for entryModules which enables tree shaking with specific entry points
    config.get("features").is_some() || 
    config.get("treeShake").is_some() ||
    config.get("api").is_some() ||
    config.get("entryModules").is_some()
}

/// Performs iterative webpack module tree shaking after DCE
fn perform_webpack_tree_shaking(program: &mut Program, cm: Lrc<SourceMap>, comments: &SingleThreadedComments, config: &serde_json::Value) {
    let analyzer = WebpackAnalyzer::new();
    let shaker = WebpackTreeShaker::new();
    
    let mut total_removed = 0;
    let max_iterations = 5; // Prevent infinite loops
    
    for iteration in 1..=max_iterations {
        // Step 1: Emit current AST to string for analysis
        let current_code = {
            let mut buf = vec![];
            let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
                as Box<dyn WriteJs>;
            let mut emitter = Emitter {
                cfg: codegen::Config::default().with_minify(false),
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
        let mut chunk = match analyzer.analyze_chunk(&current_code) {
            Ok(c) => c,
            Err(_e) => {
                if iteration == 1 {
                    return; // Skip tree shaking entirely if first analysis fails - maybe it's not a webpack bundle
                } else {
                    break; // Stop iterations if analysis fails on subsequent attempts
                }
            }
        };

        // Step 2.5: Update the chunk's source to reflect the current code
        chunk.source = current_code.clone();
        
        // Step 2.6: Rebuild dependency graph after macro processing changes
        if iteration > 1 {
            if let Err(_e) = analyzer.rebuild_dependency_graph(&mut chunk) {
            }
        }
        
        // Debug: Log dependency graph details
        if iteration == 1 {
            for (_module_id, _module) in &chunk.modules {
            }
        }

        // Step 3: Check if this is a split chunk before tree shaking
        // For webpack chunks, we assume it's a split chunk if it has modules but no clear entry points
        // Check if this is a split chunk (no direct entry point calls) vs a bundle with entry points
        let _has_entry_point_calls = current_code.contains("__webpack_require__(") && 
            !current_code.contains("__webpack_require__.d(") && 
            !current_code.contains("__webpack_require__.r(");
            
        let is_split_chunk = match chunk.chunk_type {
            ChunkType::CommonJS => true,  // CommonJS exports.modules are always split chunks
            ChunkType::JSONP => true,     // JSONP chunks are always split chunks
            ChunkType::WebpackModules => {
                // WebpackModules support removed - treat as non-split chunk
                false
            }
        };
        
        let unreachable_modules = if is_split_chunk {
            
            // Check if there are macro processing directives that might create orphaned modules
            let has_macro_config = has_macro_processing_config(config);
            
            if has_macro_config {
                
                // Get explicit entry modules from config (required for tree shaking)
                let mut entry_points = Vec::new();
                if let Some(entry_modules) = config.get("entryModules") {
                    if let Some(entry_obj) = entry_modules.as_object() {
                        for (_, entry_module) in entry_obj {
                            if let Some(entry_str) = entry_module.as_str() {
                                let entry_atom = Atom::from(entry_str);
                                if chunk.modules.contains_key(&entry_atom) {
                                    entry_points.push(entry_atom);
                                }
                            }
                        }
                    }
                }
                
                if entry_points.is_empty() {
                    // Skip tree shaking but continue with other optimizations
                    Vec::new()
                } else {
                    // For split chunks with macro processing:
                    // - First iteration: Process macros to reveal dependencies
                    // - Second iteration: Perform tree shaking based on revealed dependencies
                    if iteration == 1 {
                        // We return an empty list but will force a second iteration by checking if macros changed anything
                        Vec::new()
                    } else {
                        // Proceed with tree shaking on subsequent iterations
                        
                        // Perform tree shaking from the specified entry points
                        match shaker.shake_tree(&chunk, &entry_points) {
                            Ok(result) => {
                                result.removed_modules
                            }
                            Err(_e) => {
                                Vec::new()
                            }
                        }
                    }
                }
            } else {
                Vec::new()
            }
        } else {
            // Standard bundle with entry points - perform tree shaking
            // Extract entry points from webpack require calls in the source
            let entry_points = extract_entry_points_from_source(&current_code);
            let entry_module_refs: Vec<&str> = if entry_points.is_empty() {
                // Fallback: assume all modules are entry points if we can't determine entry points
                chunk.modules.keys().map(|s| s.as_str()).collect()
            } else {
                entry_points.iter().map(|s| s.as_str()).collect()
            };
            
            match shaker.shake_tree(&chunk, &entry_module_refs) {
                Ok(result) => {
                    result.removed_modules
                }
                Err(_e) => {
                    Vec::new()
                }
            }
        };
        
        if unreachable_modules.is_empty() {
            // Check if this is the first iteration of a macro-enabled split chunk
            // In this case, we need to force a second iteration to perform tree shaking
            let should_continue_for_macros = iteration == 1 && 
                is_split_chunk && 
                has_macro_processing_config(config);
                
            if should_continue_for_macros {
                // Continue to next iteration
            } else {
                // No more modules to remove - convergence reached
                if iteration == 1 {
                    if is_split_chunk {
                        println!("Tree shaking: Split chunk detected - preserving all modules (no tree shaking)");
                    } else {
                        println!("Tree shaking: No unreachable modules found on first pass");
                    }
                } else {
                    println!("Tree shaking: Converged after {} iterations, removed {} total modules", 
                             iteration - 1, total_removed);
                }
                break;
            }
        }

        // Only remove modules if we have any to remove
        if !unreachable_modules.is_empty() {
            // Remove the unreachable modules
            
            total_removed += unreachable_modules.len();
            
            // Step 4: Remove unreachable modules from the AST
            let unreachable_set: FxHashSet<String> = unreachable_modules
                .into_iter()
                .map(|atom| atom.to_string())
                .collect();
            let mut module_remover = WebpackModuleRemover::new(unreachable_set);
            program.visit_mut_with(&mut module_remover);
        }
        
        // Continue to next iteration to see if more modules become unreachable
    }
    
    if total_removed > 0 {
        println!("Tree shaking: Total removed {} modules across all iterations", total_removed);
    }
}

/// AST visitor that removes specified webpack modules from __webpack_modules__ objects
struct WebpackModuleRemover {
    modules_to_remove: FxHashSet<String>,
}

impl WebpackModuleRemover {
    fn new(modules_to_remove: FxHashSet<String>) -> Self {
        Self { modules_to_remove }
    }

    /// Check if a property key matches a module ID that should be removed
    fn should_remove_property(&self, prop: &PropOrSpread) -> bool {
        if let PropOrSpread::Prop(prop) = prop {
            if let Prop::KeyValue(kv) = prop.as_ref() {
                let module_id = match &kv.key {
                    PropName::Num(num) => num.value.to_string().split('.').next().unwrap_or("").to_string(),
                    PropName::Str(s) => s.value.to_string(),
                    PropName::Ident(ident) => ident.sym.to_string(),
                    _ => return false,
                };
                return self.modules_to_remove.contains(&module_id);
            }
        }
        false
    }

    /// Remove modules from object literals in expressions
    fn remove_modules_from_expr(&mut self, expr: &mut Expr) {
        match expr {
            Expr::Object(obj) => {
                self.remove_modules_from_object(obj);
            }
            Expr::Paren(paren) => {
                if let Expr::Object(obj) = paren.expr.as_mut() {
                    self.remove_modules_from_object(obj);
                }
            }
            _ => {}
        }
    }

    /// Remove specified modules from an object literal
    fn remove_modules_from_object(&mut self, obj: &mut ObjectLit) {
        let before_count = obj.props.len();
        obj.props.retain(|prop| !self.should_remove_property(prop));
        let after_count = obj.props.len();
        let removed_count = before_count - after_count;
        if removed_count > 0 {
            // Modules were removed
        }
    }
    
    /// Process split chunk .push() arguments
    fn visit_mut_split_chunk_args(&mut self, args: &mut Vec<swc_ecma_ast::ExprOrSpread>) {
        // Split chunk format: .push([[chunk_ids], { modules }])
        if args.len() >= 1 {
            let swc_ecma_ast::ExprOrSpread { expr, .. } = &mut args[0];
            if let Expr::Array(array) = expr.as_mut() {
                // We expect 2 elements: [chunk_ids, modules_object]
                if array.elems.len() >= 2 {
                    if let Some(Some(swc_ecma_ast::ExprOrSpread { expr: modules_expr, .. })) = array.elems.get_mut(1) {
                        if let Expr::Object(obj) = modules_expr.as_mut() {
                            // Remove modules from the object
                            self.remove_modules_from_object(obj);
                        }
                    }
                }
            }
        }
    }
}

impl VisitMut for WebpackModuleRemover {
    /// Visit variable declarations to find and modify __webpack_modules__
    fn visit_mut_var_decl(&mut self, node: &mut VarDecl) {
        for declarator in &mut node.decls {
            if let Pat::Ident(ident) = &declarator.name {
                if ident.sym == "__webpack_modules__" {
                    if let Some(init) = &mut declarator.init {
                        self.remove_modules_from_expr(init);
                    }
                }
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }
    
    /// Visit call expressions to find split chunk .push() calls
    fn visit_mut_call_expr(&mut self, node: &mut swc_ecma_ast::CallExpr) {
        // Look for (self["webpackChunk..."] = ...).push([...])
        if let swc_ecma_ast::Callee::Expr(callee) = &node.callee {
            if let Expr::Member(member) = callee.as_ref() {
                // Check if this is a .push() call
                if let swc_ecma_ast::MemberProp::Ident(ident) = &member.prop {
                    if ident.sym == "push" {
                        // Check if this looks like a webpack chunk push
                        self.visit_mut_split_chunk_args(&mut node.args);
                    }
                }
            }
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }

    /// Visit assignment expressions to find and modify __webpack_modules__ and exports.modules
    fn visit_mut_assign_expr(&mut self, node: &mut swc_ecma_ast::AssignExpr) {
        match &node.left {
            // Handle __webpack_modules__ = { ... }
            swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Ident(ident)) => {
                if ident.sym == "__webpack_modules__" {
                    self.remove_modules_from_expr(&mut node.right);
                }
            }
            // Handle exports.modules = { ... }
            swc_ecma_ast::AssignTarget::Simple(swc_ecma_ast::SimpleAssignTarget::Member(member)) => {
                if let swc_ecma_ast::MemberProp::Ident(prop) = &member.prop {
                    if prop.sym == "modules" {
                        // Check if this is exports.modules
                        if let Expr::Ident(obj_ident) = member.obj.as_ref() {
                            if obj_ident.sym == "exports" {
                                self.remove_modules_from_expr(&mut node.right);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        // Continue visiting children
        node.visit_mut_children_with(self);
    }
}

/// Simple regex-based orphan removal for cases where AST parsing fails
/// This works on the generated code after macro processing
fn perform_simple_orphan_removal(program: &mut Program, cm: Lrc<SourceMap>, _comments: &SingleThreadedComments, config: &serde_json::Value) {
    // Emit the current AST to string for regex-based processing
    let current_code = {
        let mut buf = vec![];
        let wr = Box::new(text_writer::JsWriter::new(cm.clone(), "\n", &mut buf, None))
            as Box<dyn WriteJs>;
        let mut emitter = Emitter {
            cfg: codegen::Config::default().with_minify(false),
            comments: None,
            cm: cm.clone(),
            wr,
        };
        if emitter.emit_program(program).is_err() {
            return;
        }
        drop(emitter);
        
        match String::from_utf8(buf) {
            Ok(code) => code,
            Err(_) => return,
        }
    };
    
    // Check if this is a CommonJS chunk with exports.modules
    if !current_code.contains("exports.modules") {
        return;
    }
    
    // Check if there are macro processing directives that might create orphaned modules
    let has_macro_config = has_macro_processing_config(config);
    
    if has_macro_config {
        // Continue with the orphaned module detection logic below
    } else {
        return;
    }
    
    // Find all module IDs in exports.modules
    let module_pattern = regex::Regex::new(r#""([^"]+\.js)":\s*"#).unwrap();
    let mut all_modules: Vec<String> = Vec::new();
    
    for cap in module_pattern.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            all_modules.push(module_id.as_str().to_string());
        }
    }
    
    
    // Find all __webpack_require__ calls to see which modules are still referenced
    let require_pattern = regex::Regex::new(r#"__webpack_require__\s*\(\s*"([^"]+\.js)""#).unwrap();
    let mut referenced_modules: std::collections::HashSet<String> = std::collections::HashSet::new();
    
    for cap in require_pattern.captures_iter(&current_code) {
        if let Some(module_id) = cap.get(1) {
            referenced_modules.insert(module_id.as_str().to_string());
        }
    }
    
    
    // Find orphaned modules (modules that exist but are not referenced)
    let mut orphaned_modules: Vec<String> = Vec::new();
    for module_id in &all_modules {
        if !referenced_modules.contains(module_id) {
            orphaned_modules.push(module_id.clone());
        }
    }
    
    
    if orphaned_modules.is_empty() {
        return;
    }
    
    // Remove orphaned modules from the code using regex
    let mut modified_code = current_code.clone();
    let mut removed_count = 0;
    
    for module_id in &orphaned_modules {
        // Create a pattern to match the entire module entry
        // Pattern: "module_id": \n/*!...*/\n(function...}),
        let module_entry_pattern = format!(
            r#""{}":\s*\n/\*![^*]*\*+(?:[^/*][^*]*\*+)*/\s*\n\(function[^{{]*\{{.*?\n\}}\),"#,
            regex::escape(module_id)
        );
        
        let entry_re = match regex::Regex::new(&module_entry_pattern) {
            Ok(re) => re,
            Err(_) => continue,
        };
        
        if entry_re.is_match(&modified_code) {
            modified_code = entry_re.replace(&modified_code, "").to_string();
            removed_count += 1;
        }
    }
    
    if removed_count > 0 {
        
        // Parse the modified code back to AST
        let cm_new: Lrc<SourceMap> = Default::default();
        let fm = cm_new.new_source_file(FileName::Custom("optimized.js".to_string()).into(), modified_code);
        let comments_new = SingleThreadedComments::default();
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax::default()),
            StringInput::from(&*fm),
            Some(&comments_new),
        );
        
        if let Ok(new_program) = parser.parse_program() {
            *program = new_program;
        } else {
        }
    } else {
    }
}

/// Extract entry points from webpack require calls outside of module definitions
fn extract_entry_points_from_source(source: &str) -> Vec<Atom> {
    let mut entry_points = Vec::new();
    
    // Look for __webpack_require__ calls that are not inside module definitions
    // These are typically entry point calls like __webpack_require__(100);
    let require_regex = Regex::new(r"__webpack_require__\(([^)]+)\);").unwrap();
    
    for cap in require_regex.captures_iter(source) {
        if let Some(module_id) = cap.get(1) {
            let id_str = module_id.as_str().trim();
            // Remove quotes if it's a string literal
            let clean_id = if (id_str.starts_with('"') && id_str.ends_with('"')) || 
                             (id_str.starts_with('\'') && id_str.ends_with('\'')) {
                &id_str[1..id_str.len()-1]
            } else {
                id_str
            };
            
            // Skip __webpack_require__.d and __webpack_require__.r calls
            if let Some(full_match) = cap.get(0) {
                if !source[..full_match.start()].ends_with("__webpack_require__.d(") &&
                   !source[..full_match.start()].ends_with("__webpack_require__.r(") {
                    entry_points.push(Atom::from(clean_id));
                }
            }
        }
    }
    
    entry_points
}
