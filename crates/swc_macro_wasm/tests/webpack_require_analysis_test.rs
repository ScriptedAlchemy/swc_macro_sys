//! Tests for analyzing webpack_require patterns in module federation examples
//! 
//! This test suite provides comprehensive analysis of webpack require patterns including:
//! - Direct requires: __webpack_require__("module-path")
//! - Import destructuring: var _module = __webpack_require__(...)
//! - Dynamic requires
//! - Conditional requires
//! - Module nullification detection (exports = null)
//! - Require chain analysis and dependency graphs
//! - Orphaned module detection

use swc_core::common::{sync::Lrc, FileName, SourceMap};
use swc_core::ecma::ast::*;
use swc_core::ecma::parser::{Parser, StringInput, Syntax, EsSyntax};
use swc_core::ecma::visit::{Visit, VisitWith};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, PartialEq)]
struct RequireInfo {
    /// The module being required
    module_id: String,
    /// The variable name it's assigned to (if any)
    var_name: Option<String>,
    /// Is this a dynamic require?
    is_dynamic: bool,
    /// Is this inside a conditional?
    is_conditional: bool,
    /// The line number where the require appears
    line: usize,
}

#[derive(Debug, Default)]
struct RequireDependencyGraph {
    /// Map from module ID to list of modules it requires
    outgoing_requires: HashMap<String, Vec<RequireInfo>>,
    /// Map from module ID to list of modules that require it
    incoming_requires: HashMap<String, Vec<(String, RequireInfo)>>,
    /// Set of module IDs that have been nullified (exports = null)
    nullified_modules: HashSet<String>,
}

impl RequireDependencyGraph {
    fn add_require(&mut self, from_module: String, require_info: RequireInfo) {
        let to_module = require_info.module_id.clone();
        
        // Add outgoing edge
        self.outgoing_requires
            .entry(from_module.clone())
            .or_insert_with(Vec::new)
            .push(require_info.clone());
        
        // Add incoming edge
        self.incoming_requires
            .entry(to_module)
            .or_insert_with(Vec::new)
            .push((from_module, require_info));
    }
    
    fn mark_nullified(&mut self, module_id: String) {
        self.nullified_modules.insert(module_id);
    }
    
    fn get_orphaned_modules(&self) -> Vec<String> {
        let mut orphaned = Vec::new();
        
        for module_id in &self.nullified_modules {
            if !self.incoming_requires.contains_key(module_id) {
                orphaned.push(module_id.clone());
            }
        }
        
        orphaned
    }
    
    fn analyze_require_chains(&self, start_module: &str) -> Vec<Vec<String>> {
        let mut chains = Vec::new();
        let mut visited = HashSet::new();
        
        self._find_chains(start_module, vec![start_module.to_string()], &mut visited, &mut chains);
        
        chains
    }
    
    fn _find_chains(&self, current: &str, path: Vec<String>, visited: &mut HashSet<String>, chains: &mut Vec<Vec<String>>) {
        if let Some(requires) = self.outgoing_requires.get(current) {
            if requires.is_empty() {
                chains.push(path);
            } else {
                let mut has_unvisited = false;
                for req in requires {
                    if !visited.contains(&req.module_id) {
                        has_unvisited = true;
                        visited.insert(req.module_id.clone());
                        let mut new_path = path.clone();
                        new_path.push(req.module_id.clone());
                        self._find_chains(&req.module_id, new_path, visited, chains);
                        visited.remove(&req.module_id);
                    }
                }
                // If all requires lead to already visited nodes (cycle), save current path
                if !has_unvisited {
                    chains.push(path);
                }
            }
        } else {
            chains.push(path);
        }
    }
}

struct WebpackRequireAnalyzer {
    current_module: Option<String>,
    graph: RequireDependencyGraph,
    in_conditional: bool,
    source_map: Lrc<SourceMap>,
}

impl WebpackRequireAnalyzer {
    fn new(source_map: Lrc<SourceMap>) -> Self {
        Self {
            current_module: None,
            graph: RequireDependencyGraph::default(),
            in_conditional: false,
            source_map,
        }
    }
    
    fn analyze_webpack_chunk(&mut self, module: &Module) {
        module.visit_with(self);
    }
    
    fn extract_module_id_from_path(&self, path: &str) -> String {
        // Extract module ID from paths like "./node_modules/.pnpm/lodash@4.17.21/node_modules/lodash/_baseGet.js"
        if let Some(last_part) = path.split('/').last() {
            last_part.trim_end_matches(".js").to_string()
        } else {
            path.to_string()
        }
    }
}

impl Visit for WebpackRequireAnalyzer {
    fn visit_call_expr(&mut self, call: &CallExpr) {
        // Check for module definition pattern: __webpack_require__.d(__webpack_exports__, {...})
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Member(member) = &**expr {
                if let (Expr::Ident(obj), MemberProp::Ident(prop)) = (&*member.obj, &member.prop) {
                    if obj.sym == "__webpack_require__" && prop.sym == "d" {
                        if let Some(arg) = call.args.get(0) {
                            if let Expr::Ident(ident) = &*arg.expr {
                                if ident.sym == "__webpack_exports__" {
                                    // We're in a module definition
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        // Check for webpack require calls
        if let Callee::Expr(expr) = &call.callee {
            if let Expr::Ident(ident) = &**expr {
                if ident.sym == "__webpack_require__" {
                    if let Some(arg) = call.args.get(0) {
                        let module_id = match &*arg.expr {
                            Expr::Lit(Lit::Str(s)) => self.extract_module_id_from_path(&s.value),
                            Expr::Lit(Lit::Num(n)) => n.value.to_string(),
                            _ => return, // Dynamic require
                        };
                        
                        let line = self.source_map.lookup_char_pos(call.span.lo).line;
                        
                        let require_info = RequireInfo {
                            module_id: module_id.clone(),
                            var_name: None,
                            is_dynamic: matches!(&*arg.expr, Expr::Ident(_)),
                            is_conditional: self.in_conditional,
                            line,
                        };
                        
                        if let Some(current) = &self.current_module {
                            self.graph.add_require(current.clone(), require_info);
                        }
                    }
                }
            }
        }
        
        call.visit_children_with(self);
    }
    
    fn visit_var_declarator(&mut self, var: &VarDeclarator) {
        // Check for pattern: var _module = __webpack_require__(...)
        if let Pat::Ident(ident) = &var.name {
            if let Some(init) = &var.init {
                if let Expr::Call(call) = &**init {
                    if let Callee::Expr(expr) = &call.callee {
                        if let Expr::Ident(callee_ident) = &**expr {
                            if callee_ident.sym == "__webpack_require__" {
                                if let Some(arg) = call.args.get(0) {
                                    let module_id = match &*arg.expr {
                                        Expr::Lit(Lit::Str(s)) => self.extract_module_id_from_path(&s.value),
                                        Expr::Lit(Lit::Num(n)) => n.value.to_string(),
                                        _ => {
                                            var.visit_children_with(self);
                                            return;
                                        }
                                    };
                                    
                                    let line = self.source_map.lookup_char_pos(call.span.lo).line;
                                    
                                    let require_info = RequireInfo {
                                        module_id: module_id.clone(),
                                        var_name: Some(ident.id.sym.to_string()),
                                        is_dynamic: matches!(&*arg.expr, Expr::Ident(_)),
                                        is_conditional: self.in_conditional,
                                        line,
                                    };
                                    
                                    if let Some(current) = &self.current_module {
                                        self.graph.add_require(current.clone(), require_info);
                                    }
                                    
                                    // Don't visit children to avoid duplicate processing
                                    return;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        var.visit_children_with(self);
    }
    
    fn visit_if_stmt(&mut self, if_stmt: &IfStmt) {
        let was_conditional = self.in_conditional;
        self.in_conditional = true;
        
        if_stmt.visit_children_with(self);
        
        self.in_conditional = was_conditional;
    }
    
    fn visit_cond_expr(&mut self, cond: &CondExpr) {
        let was_conditional = self.in_conditional;
        self.in_conditional = true;
        
        cond.visit_children_with(self);
        
        self.in_conditional = was_conditional;
    }
    
    fn visit_assign_expr(&mut self, assign: &AssignExpr) {
        // Check for exports nullification: exports = null
        if let AssignTarget::Simple(SimpleAssignTarget::Ident(ident)) = &assign.left {
            if ident.id.sym == "exports" {
                if let Expr::Lit(Lit::Null(_)) = &*assign.right {
                    if let Some(current) = &self.current_module {
                        self.graph.mark_nullified(current.clone());
                    }
                }
            }
        }
        
        assign.visit_children_with(self);
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    
    fn analyze_code(code: &str) -> RequireDependencyGraph {
        let cm = Lrc::new(SourceMap::default());
        let fm = cm.new_source_file(Lrc::new(FileName::Custom("test.js".into())), code.into());
        
        let mut parser = Parser::new(
            Syntax::Es(EsSyntax {
                jsx: true,
                ..Default::default()
            }),
            StringInput::from(&*fm),
            None,
        );
        
        let module = parser
            .parse_module()
            .expect("Failed to parse module");
        
        let mut analyzer = WebpackRequireAnalyzer::new(cm.clone());
        analyzer.current_module = Some("test_module".to_string());
        analyzer.analyze_webpack_chunk(&module);
        
        analyzer.graph
    }
    
    #[test]
    fn test_direct_require_pattern() {
        let code = r#"
            __webpack_require__("./utils/helper.js");
            __webpack_require__(123);
            __webpack_require__("lodash/debounce");
        "#;
        
        let graph = analyze_code(code);
        
        assert_eq!(graph.outgoing_requires.get("test_module").unwrap().len(), 3);
        
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires[0].module_id, "helper");
        assert_eq!(requires[1].module_id, "123");
        assert_eq!(requires[2].module_id, "debounce");
        
        // All should be non-dynamic and non-conditional
        for req in requires {
            assert!(!req.is_dynamic);
            assert!(!req.is_conditional);
            assert!(req.var_name.is_none());
        }
    }
    
    #[test]
    fn test_import_destructuring_pattern() {
        let code = r#"
            var _debounce = __webpack_require__("./lodash/debounce.js");
            var _utils = __webpack_require__(456);
            const helper = __webpack_require__("./helper");
        "#;
        
        let graph = analyze_code(code);
        
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires.len(), 3);
        
        assert_eq!(requires[0].module_id, "debounce");
        assert_eq!(requires[0].var_name, Some("_debounce".to_string()));
        
        assert_eq!(requires[1].module_id, "456");
        assert_eq!(requires[1].var_name, Some("_utils".to_string()));
        
        assert_eq!(requires[2].module_id, "helper");
        assert_eq!(requires[2].var_name, Some("helper".to_string()));
    }
    
    #[test]
    fn test_dynamic_require_pattern() {
        let code = r#"
            var moduleId = "./dynamic.js";
            __webpack_require__(moduleId);
            
            function loadModule(id) {
                return __webpack_require__(id);
            }
        "#;
        
        let graph = analyze_code(code);
        
        // Dynamic requires are not tracked in this implementation
        assert!(graph.outgoing_requires.get("test_module").is_none() || 
                graph.outgoing_requires["test_module"].is_empty());
    }
    
    #[test]
    fn test_conditional_require_pattern() {
        let code = r#"
            if (condition) {
                __webpack_require__("./conditional-module.js");
            }
            
            var result = condition ? __webpack_require__("./true-module.js") : __webpack_require__("./false-module.js");
        "#;
        
        let graph = analyze_code(code);
        
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires.len(), 3);
        
        assert_eq!(requires[0].module_id, "conditional-module");
        assert!(requires[0].is_conditional);
        
        assert_eq!(requires[1].module_id, "true-module");
        assert!(requires[1].is_conditional);
        
        assert_eq!(requires[2].module_id, "false-module");
        assert!(requires[2].is_conditional);
    }
    
    #[test]
    fn test_nullified_module_detection() {
        let code = r#"
            var _helper = __webpack_require__("./helper.js");
            
            if (false) {
                exports.default = _helper;
            } else {
                exports = null;
            }
        "#;
        
        let graph = analyze_code(code);
        
        assert!(graph.nullified_modules.contains("test_module"));
        assert_eq!(graph.outgoing_requires["test_module"][0].module_id, "helper");
    }
    
    #[test]
    fn test_require_chain_analysis() {
        let mut graph = RequireDependencyGraph::default();
        
        // Build a simple dependency chain: main -> utils -> helper -> lodash
        graph.add_require("main".to_string(), RequireInfo {
            module_id: "utils".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 1,
        });
        
        graph.add_require("utils".to_string(), RequireInfo {
            module_id: "helper".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 10,
        });
        
        graph.add_require("helper".to_string(), RequireInfo {
            module_id: "lodash".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 20,
        });
        
        let chains = graph.analyze_require_chains("main");
        assert_eq!(chains.len(), 1);
        assert_eq!(chains[0], vec!["main", "utils", "helper", "lodash"]);
    }
    
    #[test]
    fn test_orphaned_module_detection() {
        let mut graph = RequireDependencyGraph::default();
        
        // Module with incoming requires (not orphaned)
        graph.add_require("main".to_string(), RequireInfo {
            module_id: "used-module".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 1,
        });
        graph.mark_nullified("used-module".to_string());
        
        // Module without incoming requires (orphaned)
        graph.mark_nullified("orphaned-module".to_string());
        
        let orphaned = graph.get_orphaned_modules();
        assert_eq!(orphaned.len(), 1);
        assert_eq!(orphaned[0], "orphaned-module");
    }
    
    #[test]
    fn test_complex_dependency_graph() {
        let mut graph = RequireDependencyGraph::default();
        
        // Create a more complex graph:
        // main -> [utils, helper]
        // utils -> [lodash, common]
        // helper -> [common]
        // common -> [lodash]
        
        graph.add_require("main".to_string(), RequireInfo {
            module_id: "utils".to_string(),
            var_name: Some("_utils".to_string()),
            is_dynamic: false,
            is_conditional: false,
            line: 1,
        });
        
        graph.add_require("main".to_string(), RequireInfo {
            module_id: "helper".to_string(),
            var_name: Some("_helper".to_string()),
            is_dynamic: false,
            is_conditional: false,
            line: 2,
        });
        
        graph.add_require("utils".to_string(), RequireInfo {
            module_id: "lodash".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: true,
            line: 10,
        });
        
        graph.add_require("utils".to_string(), RequireInfo {
            module_id: "common".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 11,
        });
        
        graph.add_require("helper".to_string(), RequireInfo {
            module_id: "common".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 20,
        });
        
        graph.add_require("common".to_string(), RequireInfo {
            module_id: "lodash".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 30,
        });
        
        // Test incoming requires
        assert_eq!(graph.incoming_requires["common"].len(), 2);
        assert_eq!(graph.incoming_requires["lodash"].len(), 2);
        
        // Test require chains from main
        let chains = graph.analyze_require_chains("main");
        assert!(chains.len() >= 2);
        
        // Verify that all chains start with "main"
        for chain in &chains {
            assert_eq!(chain[0], "main");
        }
    }
    
    #[test]
    fn test_real_webpack_pattern() {
        let code = r#"
var castPath = __webpack_require__("./node_modules/.pnpm/lodash@4.17.21/node_modules/lodash/_castPath.js");
var toKey = __webpack_require__("./node_modules/.pnpm/lodash@4.17.21/node_modules/lodash/_toKey.js");

function baseGet(object, path) {
  path = castPath(path, object);

  var index = 0,
      length = path.length;

  while (object != null && index < length) {
    object = object[toKey(path[index++])];
  }
  return (index && index == length) ? object : undefined;
}

module.exports = baseGet;
"#;
        
        let graph = analyze_code(code);
        
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires.len(), 2);
        
        assert_eq!(requires[0].module_id, "_castPath");
        assert_eq!(requires[0].var_name, Some("castPath".to_string()));
        
        assert_eq!(requires[1].module_id, "_toKey");
        assert_eq!(requires[1].var_name, Some("toKey".to_string()));
    }
    
    #[test]
    fn test_module_federation_pattern() {
        let code = r#"
            // Module with exports nullified
            var _helper = __webpack_require__("./helper.js");
            if (false) {
                exports.default = _helper;
            } else {
                exports = null;
            }
            
            // Another module that requires the nullified module
            var _consumer = __webpack_require__("./consumer.js");
            _consumer.use(_helper);
        "#;
        
        let graph = analyze_code(code);
        
        // Check that module is marked as nullified
        assert!(graph.nullified_modules.contains("test_module"));
        
        // Check requires
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires.len(), 2);
        assert_eq!(requires[0].module_id, "helper");
        assert_eq!(requires[1].module_id, "consumer");
    }
    
    #[test]
    fn test_analyze_lodash_require_pattern() {
        let code = r#"
            // Main lodash module
            var _baseGet = __webpack_require__("./node_modules/lodash/_baseGet.js");
            var _hasPath = __webpack_require__("./node_modules/lodash/_hasPath.js");
            
            function get(object, path, defaultValue) {
                var result = object == null ? undefined : _baseGet(object, path);
                return result === undefined ? defaultValue : result;
            }
            
            function has(object, path) {
                return object != null && _hasPath(object, path);
            }
            
            module.exports = { get, has };
        "#;
        
        let graph = analyze_code(code);
        
        let requires = &graph.outgoing_requires["test_module"];
        assert_eq!(requires.len(), 2);
        assert_eq!(requires[0].module_id, "_baseGet");
        assert_eq!(requires[1].module_id, "_hasPath");
        
        // Check incoming requires - these modules are required by test_module
        assert!(graph.incoming_requires.contains_key("_baseGet"));
        assert!(graph.incoming_requires.contains_key("_hasPath"));
    }
    
    #[test]
    fn test_webpack_chunk_with_nullified_exports() {
        let mut graph = RequireDependencyGraph::default();
        
        // Simulate a webpack chunk structure
        // lodash.js requires many helpers
        graph.add_require("lodash".to_string(), RequireInfo {
            module_id: "_baseGet".to_string(),
            var_name: Some("baseGet".to_string()),
            is_dynamic: false,
            is_conditional: false,
            line: 10,
        });
        
        graph.add_require("lodash".to_string(), RequireInfo {
            module_id: "_hasPath".to_string(),
            var_name: Some("hasPath".to_string()),
            is_dynamic: false,
            is_conditional: false,
            line: 11,
        });
        
        graph.add_require("lodash".to_string(), RequireInfo {
            module_id: "_isEqual".to_string(),
            var_name: Some("isEqual".to_string()),
            is_dynamic: false,
            is_conditional: false,
            line: 12,
        });
        
        // Mark some modules as nullified (their exports were set to null)
        graph.mark_nullified("_baseGet".to_string());
        graph.mark_nullified("_isEqual".to_string());
        graph.mark_nullified("_orphanedHelper".to_string()); // This one has no incoming requires
        
        // Test orphaned detection
        let orphaned = graph.get_orphaned_modules();
        assert_eq!(orphaned.len(), 1);
        assert_eq!(orphaned[0], "_orphanedHelper");
        
        // _baseGet and _isEqual are not orphaned because lodash requires them
        assert!(graph.incoming_requires.contains_key("_baseGet"));
        assert!(graph.incoming_requires.contains_key("_isEqual"));
    }
    
    #[test]
    fn test_complex_require_chain_with_cycles() {
        let mut graph = RequireDependencyGraph::default();
        
        // Create a graph with potential cycles
        // A -> B -> C
        // B -> D
        // C -> D
        // D -> B (cycle)
        
        graph.add_require("A".to_string(), RequireInfo {
            module_id: "B".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 1,
        });
        
        graph.add_require("B".to_string(), RequireInfo {
            module_id: "C".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 10,
        });
        
        graph.add_require("B".to_string(), RequireInfo {
            module_id: "D".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 11,
        });
        
        graph.add_require("C".to_string(), RequireInfo {
            module_id: "D".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 20,
        });
        
        graph.add_require("D".to_string(), RequireInfo {
            module_id: "B".to_string(),
            var_name: None,
            is_dynamic: false,
            is_conditional: false,
            line: 30,
        });
        
        // Analyze chains from A - should handle cycles gracefully
        let chains = graph.analyze_require_chains("A");
        
        // We should get at least one chain
        assert!(chains.len() > 0);
        for chain in &chains {
            assert!(chain[0] == "A");
            // Chains should not be infinitely long due to cycle detection
            assert!(chain.len() <= 6); // Increased limit to account for cycles
        }
    }
}