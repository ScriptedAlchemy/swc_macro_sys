use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;
use crate::module::{ModuleId, WebpackModule};

/// Represents the different types of webpack chunks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkType {
    /// JSONP format: (self["webpackChunk..."] = ...).push([[...], {...}])
    JSONP,
    /// CommonJS format: exports.modules = {...}
    CommonJS,
}

/// Represents a webpack chunk with its modules
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebpackChunk {
    /// The type of chunk format
    pub chunk_type: ChunkType,
    /// The modules contained in this chunk
    pub modules: FxHashMap<ModuleId, WebpackModule>,
    /// Original source code for debugging
    pub source: String,
}

impl WebpackChunk {
    /// Create a new webpack chunk
    pub fn new(chunk_type: ChunkType, source: String) -> Self {
        Self {
            chunk_type,
            modules: FxHashMap::default(),
            source,
        }
    }

    /// Add a module to this chunk
    pub fn add_module(&mut self, module_id: ModuleId, module: WebpackModule) {
        self.modules.insert(module_id, module);
    }

    /// Get all module IDs in this chunk
    pub fn get_module_ids(&self) -> Vec<ModuleId> {
        self.modules.keys().cloned().collect()
    }

    /// Get a module by ID
    pub fn get_module(&self, module_id: &ModuleId) -> Option<&WebpackModule> {
        self.modules.get(module_id)
    }

    /// Get the number of modules in this chunk
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }
}