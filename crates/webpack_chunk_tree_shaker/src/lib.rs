//! # Webpack Chunk Tree Shaker
//!
//! A high-performance tree shaking implementation for webpack chunks that works with
//! webpack_analyzer_v2 to remove unused modules and optimize bundle size.
//!
//! ## Features
//!
//! - **Module Removal**: Remove specific modules by ID
//! - **Tree Shaking**: Remove unused modules based on dependency analysis
//! - **Chunk Reconstruction**: Rebuild webpack chunks with optimized module set
//! - **Format Preservation**: Maintains original webpack chunk format (CommonJS/JSONP)
//! - **Safety Checks**: Validates removal safety before applying changes
//!
//! ## Usage
//!
//! ```rust
//! use webpack_chunk_tree_shaker::*;
//! use webpack_analyzer_v2::*;
//!
//! // Analyze chunk
//! let analyzer = WebpackAnalyzer::new();
//! let chunk = analyzer.analyze_chunk(chunk_source)?;
//!
//! // Create tree shaker
//! let shaker = WebpackTreeShaker::new();
//!
//! // Remove specific modules
//! let result = shaker.remove_modules(&chunk, &["unused-module-1", "unused-module-2"])?;
//!
//! // Get optimized chunk
//! let optimized_chunk = result.optimized_chunk;
//! ```

pub mod error;
pub mod shaker;
pub mod reconstruction;
pub mod optimization;
pub mod validation;

pub use error::*;
pub use shaker::*;
pub use reconstruction::*;
pub use optimization::*;
pub use validation::*;

// Re-export analyzer types for convenience
pub use webpack_analyzer_v2::{
    WebpackAnalyzer, WebpackChunk, WebpackModule, DependencyGraph, 
    ModuleRemovalImpact, ModuleId, ChunkType
};