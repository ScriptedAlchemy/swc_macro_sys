# Webpack Chunk Tree Shaker

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![Tree Shaking](https://img.shields.io/badge/tree--shaking-30--70%25%20reduction-green.svg)](#performance)
[![Webpack](https://img.shields.io/badge/webpack-all%20formats-blue.svg)](#webpack-format-support)

A sophisticated Rust crate for safe, efficient tree shaking of webpack bundles. Performs intelligent module removal while maintaining bundle integrity and functionality.

## Features

- **Multiple Webpack Formats**: Supports CommonJS, JSONP, and WebpackModules formats
- **AST-Based Analysis**: Uses SWC for precise JavaScript parsing and dependency tracking
- **Safety First**: Comprehensive validation prevents breaking changes
- **High Performance**: 30-70% reduction in bundle size for typical applications
- **Production Ready**: Tested with real-world webpack bundles including lodash-es

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
webpack_chunk_tree_shaker = "0.1.0"
webpack_analyzer_v2 = "0.1.0"
```

Basic usage:

```rust
use webpack_chunk_tree_shaker::*;
use webpack_analyzer_v2::WebpackAnalyzer;

// Analyze webpack chunk
let analyzer = WebpackAnalyzer::new();
let chunk = analyzer.analyze_chunk(webpack_source)?;

// Perform tree shaking
let shaker = WebpackTreeShaker::new();
let result = shaker.shake_tree(&chunk, &["your-entry-module"])?;

println!("Reduced from {} to {} modules ({:.1}% reduction)", 
         result.stats.original_count,
         result.stats.final_count,
         result.stats.reduction_percentage);
```

## Performance

Real-world performance results from the test suite:

| Bundle Type | Module Reduction | Size Reduction |
|-------------|------------------|----------------|
| lodash-es Bundle | 30-70% | Variable |
| Feature Bundles | 33-57% | Variable |
| Large Applications | 30-70% | Variable |

## Safety & Validation

The tree shaker includes comprehensive safety mechanisms:

- **Pre-removal validation** - Checks chunk integrity before modifications
- **Impact analysis** - Simulates removal effects before making changes
- **Dependency tracking** - Maintains module relationship graphs
- **Post-removal validation** - Ensures optimized chunk is still functional

## Webpack Format Support

| Format | Description | Example Use Case |
|--------|-------------|------------------|
| **CommonJS** | `exports.modules = {...}` | Server-side rendering |
| **JSONP** | `webpackChunk.push([...])` | Dynamic chunk loading |
| **WebpackModules** | `var __webpack_modules__ = ({...})` | Standard webpack bundles |

## API Overview

### Core Types

- **`WebpackTreeShaker`** - Main tree shaking interface
- **`TreeShakingOptions`** - Configuration for tree shaking behavior
- **`TreeShakingResult`** - Comprehensive results with statistics
- **`ChunkOptimizer`** - Multiple optimization strategies
- **`TreeShakingValidator`** - Safety and integrity validation

### Key Methods

```rust
// Remove specific modules
fn remove_modules(&self, chunk: &WebpackChunk, modules: &[String]) -> Result<TreeShakingResult>

// Tree shake from entry points
fn shake_tree(&self, chunk: &WebpackChunk, entries: &[String]) -> Result<TreeShakingResult>

// Find unused modules
fn find_unused_modules(&self, chunk: &WebpackChunk, entries: &[String]) -> Result<Vec<String>>
```

## Documentation

- **[TECHNICAL_GUIDE.md](TECHNICAL_GUIDE.md)** - Comprehensive technical documentation with algorithms, architecture, and advanced usage
- **[API Reference](https://docs.rs/webpack_chunk_tree_shaker)** - Complete API documentation

## Examples

See the `tests/` directory for comprehensive examples:

- `integration_tests.rs` - Basic usage patterns
- `real_world_chunks_test.rs` - Production webpack bundle optimization
- `removal_cascade_test.rs` - Advanced dependency chain removal

## Integration with webpack_analyzer_v2

This crate works seamlessly with [webpack_analyzer_v2](../webpack_analyzer_v2/) which provides:

- AST-based webpack chunk parsing
- Module dependency graph construction
- Support for multiple webpack formats
- Type-safe module representation

## Contributing

We welcome contributions! Please ensure:

1. All tests pass: `cargo test`
2. Code follows the project style
3. New features include comprehensive tests
4. Documentation is updated for API changes

## License

Licensed under the MIT License. See [LICENSE](../../LICENSE) for details.

## Acknowledgments

- Built with [SWC](https://swc.rs/) for fast JavaScript parsing
- Inspired by webpack's tree shaking capabilities
- Designed for the Rust ecosystem's safety and performance standards