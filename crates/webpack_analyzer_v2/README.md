# Webpack Analyzer V2

A comprehensive Rust-based static analysis tool for webpack bundles, designed for advanced tree shaking and bundle optimization. Built with SWC (Speedy Web Compiler) for robust AST parsing.

## Features

- **Format Detection**: Automatic identification of JSONP vs CommonJS webpack chunks
- **AST-Based Parsing**: Robust JavaScript/TypeScript analysis without regex dependencies
- **Dependency Graph**: Bidirectional module relationship mapping
- **Impact Analysis**: Orphan detection and removal impact simulation
- **Module Federation Support**: Optimized for micro-frontend architectures

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
webpack_analyzer_v2 = "0.1.0"
```

Basic usage:

```rust
use webpack_analyzer_v2::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let chunk_source = std::fs::read_to_string("vendor-chunk.js")?;
    
    // Create analyzer
    let analyzer = WebpackAnalyzer::new();
    
    // Analyze chunk
    let chunk = analyzer.analyze_chunk(&chunk_source)?;
    
    println!("Found {} modules", chunk.module_count());
    println!("Chunk type: {:?}", chunk.chunk_type);
    
    // Build dependency graph
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    println!("Total dependencies: {}", graph.total_dependencies());
    
    Ok(())
}
```

## Architecture

The analyzer follows a layered architecture:

1. **Input Layer**: Raw webpack chunk source code
2. **Detection Layer**: Chunk format identification (JSONP/CommonJS)
3. **Parsing Layer**: SWC-based AST parsing and module extraction
4. **Analysis Layer**: Dependency graph construction and relationship mapping
5. **Output Layer**: Structured data for tree shaking optimization

## Supported Formats

### CommonJS Chunks

```javascript
"use strict";
exports.ids = ["chunk-name"];
exports.modules = {
    "./src/module-a.js": function(module, exports, __webpack_require__) {
        const b = __webpack_require__("./src/module-b.js");
        module.exports = { processB: b.process };
    }
};
```

### JSONP Chunks

```javascript
(self["webpackChunkapp"] = self["webpackChunkapp"] || []).push([
    ["chunk-name"], 
    {
        "./src/module-a.js": function(module, exports, __webpack_require__) {
            const b = __webpack_require__("./src/module-b.js");
            module.exports = { processB: b.process };
        }
    }
]);
```

## Core Components

### WebpackAnalyzer

Main entry point for chunk analysis:

```rust
let analyzer = WebpackAnalyzer::new();
let chunk = analyzer.analyze_chunk(source)?;
let chunk_type = analyzer.detect_chunk_type(source)?;
```

### WebpackChunk

Represents a parsed webpack chunk:

```rust
pub struct WebpackChunk {
    pub chunk_type: ChunkType,
    pub modules: FxHashMap<ModuleId, WebpackModule>,
    pub source: String,
}
```

### WebpackModule

Individual module within a chunk:

```rust
pub struct WebpackModule {
    pub id: ModuleId,
    pub source: String,
    pub dependencies: HashSet<ModuleId>,
    pub dependents: HashSet<ModuleId>,
}
```

### DependencyGraph

Manages module relationships:

```rust
let mut graph = DependencyGraph::new();
graph.add_module(module);

let reachable = graph.get_reachable_modules(&module_id);
let impact = graph.simulate_module_removal(&module_id);
```

## Advanced Usage

### Tree Shaking Integration

```rust
use webpack_analyzer_v2::*;
use std::collections::HashSet;

fn optimize_chunk(chunk_source: &str, used_modules: &HashSet<ModuleId>) -> Result<String, Box<dyn std::error::Error>> {
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(chunk_source)?;
    
    // Build dependency graph
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    // Find reachable modules from used modules
    let mut reachable = HashSet::new();
    for used_module in used_modules {
        reachable.extend(graph.get_reachable_modules(used_module));
    }
    
    // Filter to only reachable modules
    let optimized_modules: Vec<_> = chunk.modules
        .iter()
        .filter(|(id, _)| reachable.contains(id))
        .collect();
    
    println!("Optimized from {} to {} modules", 
             chunk.module_count(), 
             optimized_modules.len());
    
    // Rebuild chunk (implementation depends on your needs)
    Ok(String::new()) // Placeholder
}
```

### Impact Analysis

```rust
use webpack_analyzer_v2::*;

fn analyze_module_removal(chunk_source: &str, module_to_remove: &str) -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(chunk_source)?;
    
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    let impact = graph.simulate_module_removal(&module_to_remove.to_string());
    
    println!("Removing module: {}", impact.removed_module);
    println!("Broken modules: {}", impact.broken_modules.len());
    println!("Potentially orphaned: {}", impact.potentially_orphaned.len());
    
    if impact.has_breaking_changes() {
        println!("⚠️  Warning: Removal would break other modules");
    }
    
    Ok(())
}
```

### Module Federation Analysis

```rust
use webpack_analyzer_v2::*;

fn analyze_shared_chunk(chunk_source: &str) -> Result<(), Box<dyn std::error::Error>> {
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(chunk_source)?;
    
    // Identify shared modules (typically have specific patterns)
    let shared_modules: Vec<_> = chunk.modules
        .iter()
        .filter(|(id, _)| id.contains("webpack/sharing/"))
        .collect();
    
    println!("Total modules: {}", chunk.module_count());
    println!("Shared modules: {}", shared_modules.len());
    
    // Analyze dependency patterns
    let mut graph = DependencyGraph::new();
    for module in chunk.modules.values() {
        graph.add_module(module.clone());
    }
    
    println!("Total dependencies: {}", graph.total_dependencies());
    
    Ok(())
}
```

## Performance Characteristics

### Complexity Analysis

| Operation | Time Complexity | Space Complexity |
|-----------|----------------|------------------|
| Chunk Parsing | O(n) | O(n) |
| Module Extraction | O(n) | O(n) |
| Dependency Building | O(n × m) | O(n × m) |
| Impact Analysis | O(n + e) | O(n) |

Where:
- **n** = number of modules
- **m** = average dependencies per module  
- **e** = total edges in dependency graph

### Benchmarks

Real-world performance with lodash-es chunk:
- **Modules**: 619 modules analyzed
- **Dependencies**: 1,644 relationships mapped
- **Parse time**: ~180ms
- **Memory usage**: ~45MB

## Testing

Run the test suite:

```bash
cargo test
```

Run with output to see detailed analysis:

```bash
cargo test -- --nocapture
```

Test specific components:

```bash
# Test chunk type detection
cargo test test_chunk_type_detection

# Test real-world vendor chunks
cargo test test_host_vendor_chunk

# Test source chunks
cargo test test_source_utils_chunk
```

## API Reference

### Types

```rust
pub type ModuleId = String;
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[derive(Debug, Clone, PartialEq)]
pub enum ChunkType {
    JSONP,
    CommonJS,
}
```

### WebpackAnalyzer

```rust
impl WebpackAnalyzer {
    pub fn new() -> Self;
    pub fn analyze_chunk(&self, source: &str) -> Result<WebpackChunk>;
    pub fn detect_chunk_type(&self, source: &str) -> Result<ChunkType>;
}
```

### WebpackChunk

```rust
impl WebpackChunk {
    pub fn new(chunk_type: ChunkType, source: String) -> Self;
    pub fn module_count(&self) -> usize;
    pub fn get_module(&self, id: &ModuleId) -> Option<&WebpackModule>;
    pub fn add_module(&mut self, module: WebpackModule);
}
```

### DependencyGraph

```rust
impl DependencyGraph {
    pub fn new() -> Self;
    pub fn add_module(&mut self, module: WebpackModule);
    pub fn get_reachable_modules(&self, start: &ModuleId) -> HashSet<ModuleId>;
    pub fn simulate_module_removal(&self, module: &ModuleId) -> ModuleRemovalImpact;
    pub fn total_dependencies(&self) -> usize;
    pub fn find_entry_modules(&self) -> Vec<ModuleId>;
    pub fn find_leaf_modules(&self) -> Vec<ModuleId>;
}
```

### ModuleRemovalImpact

```rust
pub struct ModuleRemovalImpact {
    pub removed_module: ModuleId,
    pub broken_modules: HashSet<ModuleId>,
    pub potentially_orphaned: HashSet<ModuleId>,
}

impl ModuleRemovalImpact {
    pub fn new(removed_module: ModuleId) -> Self;
    pub fn total_affected(&self) -> usize;
    pub fn has_breaking_changes(&self) -> bool;
}
```

## Error Handling

The analyzer provides comprehensive error handling:

```rust
match analyzer.analyze_chunk(source) {
    Ok(chunk) => {
        // Process chunk
    }
    Err(e) => {
        eprintln!("Analysis failed: {}", e);
        // Handle specific error types
    }
}
```

Common error scenarios:
- **Parse errors**: Invalid JavaScript/TypeScript syntax
- **Format errors**: Unknown chunk format
- **Memory errors**: Chunks too large for available memory

## Limitations

### Current Limitations

- **Runtime chunks**: Bootstrap and webpack runtime code not supported
- **Entry chunks**: Application entry points excluded from analysis
- **Dynamic imports**: `import()` statements not yet supported
- **Circular dependencies**: Detection only, no resolution

### Memory Constraints

- Large chunks (600+ modules) require significant memory (~45MB)
- Complex dependency graphs may impact performance
- Consider chunking analysis for very large bundles

## Integration Examples

### With Build Tools

```rust
// webpack.config.js integration
const { optimize_chunk } = require('./webpack-analyzer-bridge');

module.exports = {
    plugins: [
        new webpack.optimize.ModuleConcatenationPlugin(),
        {
            apply: (compiler) => {
                compiler.hooks.emit.tapAsync('WebpackAnalyzerV2', (compilation, callback) => {
                    for (const [name, chunk] of compilation.chunks) {
                        if (name.includes('vendor')) {
                            const optimized = optimize_chunk(chunk.source);
                            // Replace with optimized version
                        }
                    }
                    callback();
                });
            }
        }
    ]
};
```

### With Module Federation

```rust
// Analyze shared dependencies
fn analyze_federation_chunk(chunk_source: &str) -> FederationAnalysis {
    let analyzer = WebpackAnalyzer::new();
    let chunk = analyzer.analyze_chunk(chunk_source).unwrap();
    
    let shared_modules = chunk.modules
        .iter()
        .filter(|(id, _)| id.contains("webpack/sharing/"))
        .count();
    
    FederationAnalysis {
        total_modules: chunk.module_count(),
        shared_modules,
        optimization_potential: calculate_optimization_potential(&chunk),
    }
}
```

## Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Make your changes
4. Add tests for new functionality
5. Ensure all tests pass (`cargo test`)
6. Commit your changes (`git commit -m 'Add amazing feature'`)
7. Push to the branch (`git push origin feature/amazing-feature`)
8. Open a Pull Request

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Changelog

### v0.1.0
- Initial release
- Support for CommonJS and JSONP chunk formats
- Complete dependency graph analysis
- Module removal impact simulation
- Comprehensive test suite with real-world validation

## Support

For questions, issues, or contributions:

- **Issues**: Report bugs and feature requests
- **Discussions**: Ask questions and share ideas
- **Documentation**: Comprehensive API docs and examples

---

Built with ❤️ for the webpack optimization community.