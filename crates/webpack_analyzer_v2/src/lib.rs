pub mod chunk;
pub mod module;
pub mod dependency_graph;
pub mod analyzer;

pub use chunk::{ChunkType, WebpackChunk};
pub use module::{ModuleId, WebpackModule};
pub use dependency_graph::{DependencyGraph, ModuleRemovalImpact};
pub use analyzer::WebpackAnalyzer;

/// Result type for webpack analysis operations
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

#[cfg(test)]
mod tests;