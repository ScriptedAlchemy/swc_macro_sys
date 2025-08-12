use serde::{Deserialize, Serialize};
use rustc_hash::FxHashMap;
use crate::module::{ModuleId, WebpackModule};

/// Represents chunk characteristics from webpack/rspack build metadata
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChunkCharacteristics {
    pub is_runtime_chunk: bool,
    pub has_runtime: bool,
    pub is_entrypoint: bool,
    pub can_be_initial: bool,
    pub is_only_initial: bool,
    pub chunk_format: String,
    pub chunk_loading_type: Option<String>,
    pub runtime_names: Vec<String>,
    pub entry_name: Option<String>,
    pub has_async_chunks: bool,
    pub chunk_files: Vec<String>,
    pub is_shared_chunk: bool,
    pub shared_modules: Vec<serde_json::Value>,
}

/// Represents the different types of webpack chunks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ChunkType {
    /// JSONP format: (self["webpackChunk..."] = ...).push([[...], {...}])
    /// Used for web browser chunks with "jsonp" chunk_format
    JSONP,
    /// CommonJS async format: exports.modules = {...}
    /// Used for node chunks with "async-node" chunk_format  
    CommonJSAsync,
    /// CommonJS sync format: exports.modules = {...}
    /// Used for node chunks with "require" chunk_format
    CommonJSSync,
    /// Webpack modules format: var __webpack_modules__ = ({...})
    /// Used for main entry chunks or when chunk_characteristics unavailable
    WebpackModules,
    /// ES modules format for modern bundlers
    ESModules,
    /// Unknown chunk type when detection fails
    Unknown,
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
    /// Chunk characteristics from webpack/rspack build metadata (optional)
    pub characteristics: Option<ChunkCharacteristics>,
}

impl WebpackChunk {
    /// Create a new webpack chunk
    pub fn new(chunk_type: ChunkType, source: String) -> Self {
        Self {
            chunk_type,
            modules: FxHashMap::default(),
            source,
            characteristics: None,
        }
    }

    /// Create a new webpack chunk with characteristics
    pub fn new_with_characteristics(chunk_type: ChunkType, source: String, characteristics: ChunkCharacteristics) -> Self {
        Self {
            chunk_type,
            modules: FxHashMap::default(),
            source,
            characteristics: Some(characteristics),
        }
    }

    /// Set chunk characteristics
    pub fn set_characteristics(&mut self, characteristics: ChunkCharacteristics) {
        self.characteristics = Some(characteristics);
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

impl ChunkCharacteristics {
    /// Determine the chunk type based on chunk characteristics
    pub fn determine_chunk_type(&self) -> ChunkType {
        match self.chunk_format.as_str() {
            "jsonp" => ChunkType::JSONP,
            "async-node" => ChunkType::CommonJSAsync,
            "require" => ChunkType::CommonJSSync,
            "webpack" => ChunkType::WebpackModules,
            "module" => ChunkType::ESModules,
            _ => {
                // Fall back to heuristics for unknown formats
                if self.is_entrypoint || self.has_runtime {
                    ChunkType::WebpackModules
                } else {
                    ChunkType::Unknown
                }
            }
        }
    }

    /// Check if this chunk is likely a vendor chunk based on characteristics
    pub fn is_vendor_chunk(&self) -> bool {
        // Vendor chunks are typically:
        // - Not runtime chunks
        // - Not entry points  
        // - Can be loaded initially (at startup)
        // - May have vendor-related naming patterns
        !self.is_runtime_chunk 
            && !self.is_entrypoint 
            && self.can_be_initial
    }

    /// Check if this is a runtime chunk
    pub fn is_runtime(&self) -> bool {
        self.is_runtime_chunk || self.has_runtime
    }

    /// Get the chunk file names
    pub fn get_chunk_files(&self) -> &Vec<String> {
        &self.chunk_files
    }
}