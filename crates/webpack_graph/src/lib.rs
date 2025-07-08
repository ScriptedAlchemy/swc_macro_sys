pub mod error;
pub mod graph;
pub mod parser;
pub mod tree_shaker;

pub use error::WebpackGraphError;
pub use graph::{ModuleGraph, ModuleNode};
pub use parser::WebpackBundleParser;
pub use tree_shaker::TreeShaker;

/// Result type for webpack graph operations
pub type Result<T> = std::result::Result<T, WebpackGraphError>;  