use thiserror::Error;
use webpack_analyzer_v2::ModuleId;

/// Result type for tree shaking operations
pub type Result<T> = std::result::Result<T, TreeShakingError>;

/// Errors that can occur during tree shaking operations
#[derive(Error, Debug)]
pub enum TreeShakingError {
    #[error("Module not found: {module_id}")]
    ModuleNotFound { module_id: ModuleId },

    #[error("Cannot remove module {module_id}: would break {dependent_count} dependent modules")]
    UnsafeRemoval { 
        module_id: ModuleId, 
        dependent_count: usize 
    },

    #[error("Circular dependency detected involving module: {module_id}")]
    CircularDependency { module_id: ModuleId },

    #[error("Cannot remove entry module: {module_id}")]
    EntryModuleRemoval { module_id: ModuleId },

    #[error("Invalid chunk format: {format}")]
    InvalidFormat { format: String },

    #[error("Analysis error: {source}")]
    AnalysisError { 
        #[from]
        source: Box<dyn std::error::Error + Send + Sync> 
    },

    #[error("Validation failed: {reason}")]
    ValidationFailed { reason: String },

    #[error("Module {module_id} has unresolved dependencies: {dependencies:?}")]
    UnresolvedDependencies { 
        module_id: ModuleId, 
        dependencies: Vec<ModuleId> 
    },

    #[error("Tree shaking would result in empty chunk")]
    EmptyChunk,

    #[error("IO error: {source}")]
    IoError {
        #[from]
        source: std::io::Error,
    },

    #[error("Serialization error: {source}")]
    SerializationError {
        #[from]
        source: serde_json::Error,
    },
}

impl TreeShakingError {
    pub fn module_not_found(module_id: impl Into<ModuleId>) -> Self {
        Self::ModuleNotFound { 
            module_id: module_id.into() 
        }
    }

    pub fn unsafe_removal(module_id: impl Into<ModuleId>, dependent_count: usize) -> Self {
        Self::UnsafeRemoval { 
            module_id: module_id.into(), 
            dependent_count 
        }
    }

    pub fn circular_dependency(module_id: impl Into<ModuleId>) -> Self {
        Self::CircularDependency { 
            module_id: module_id.into() 
        }
    }

    

    pub fn validation_failed(reason: impl Into<String>) -> Self {
        Self::ValidationFailed { 
            reason: reason.into() 
        }
    }
}