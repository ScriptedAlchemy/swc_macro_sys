//! # Webpack Chunk Tree Shaker
//!
//! A high-performance tree shaking implementation for webpack chunks that works with
//! webpack_analyzer_v2 to remove unused modules and optimize bundle size.
//!
//! ## Features
//!
//! - **Module Removal**: Remove specific modules by ID
//! - **Tree Shaking**: Remove unused modules based on dependency analysis
//! - **Module Filtering**: Filter out unused modules from chunks
//! - **Format Agnostic**: Works with all webpack chunk formats (CommonJS/JSONP/WebpackModules)
//! - **Safety Checks**: Validates removal safety before applying changes
//!
//! ## Usage
//!
//! ```rust,no_run
//! use webpack_analyzer_v2::chunk::ChunkCharacteristics;
//!
//! # fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
//! # let chunk_source = "/* webpack chunk */";
//! // Analyze chunk
//! let analyzer = webpack_chunk_tree_shaker::WebpackAnalyzer::new();
//! let chunk_characteristics = ChunkCharacteristics {
//!     is_runtime_chunk: false,
//!     has_runtime: false,
//!     is_entrypoint: false,
//!     can_be_initial: true,
//!     is_only_initial: false,
//!     chunk_format: "require".to_string(),
//!     chunk_loading_type: None,
//!     runtime_names: vec![],
//!     entry_name: None,
//!     has_async_chunks: false,
//!     chunk_files: vec![],
//!     is_shared_chunk: false,
//!     shared_modules: vec![],
//! };
//! let chunk = analyzer.analyze_chunk(chunk_source, chunk_characteristics)
//!     .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
//!
//! // Create tree shaker
//! let shaker = webpack_chunk_tree_shaker::WebpackTreeShaker::new();
//!
//! // Remove specific modules
//! let result = shaker.remove_modules(&chunk, &["unused-module-1", "unused-module-2"])
//!     .map_err(|e| Box::<dyn std::error::Error>::from(e.to_string()))?;
//!
//! // Get optimized chunk
//! let optimized_chunk = result.optimized_chunk;
//! # Ok(())
//! # }
//! ```

pub mod error;
pub mod shaker;
pub mod optimization;
pub mod validation;

pub use error::*;
pub use shaker::*;
pub use optimization::*;
pub use validation::*;

// Re-export analyzer types for convenience
pub use webpack_analyzer_v2::{
    WebpackAnalyzer, WebpackChunk, WebpackModule, DependencyGraph, 
    ModuleRemovalImpact, ModuleId, ChunkType
};