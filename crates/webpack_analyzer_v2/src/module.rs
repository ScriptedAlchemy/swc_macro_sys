use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use swc_core::atoms::Atom;

/// Type alias for module identifiers
pub type ModuleId = Atom;

/// Represents a webpack module
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebpackModule {
    /// The module identifier
    pub id: ModuleId,
    /// The source code of the module
    pub source: String,
    /// Dependencies found in this module (webpack_require calls)
    pub dependencies: HashSet<ModuleId>,
    /// Modules that depend on this module
    pub dependents: HashSet<ModuleId>,
}

impl WebpackModule {
    /// Create a new webpack module
    pub fn new(id: ModuleId, source: String) -> Self {
        Self {
            id,
            source,
            dependencies: HashSet::new(),
            dependents: HashSet::new(),
        }
    }

    /// Add a dependency to this module
    pub fn add_dependency(&mut self, dependency: ModuleId) {
        self.dependencies.insert(dependency);
    }

    /// Add a dependent to this module
    pub fn add_dependent(&mut self, dependent: ModuleId) {
        self.dependents.insert(dependent);
    }

    /// Check if this module depends on another module
    pub fn depends_on(&self, module_id: &ModuleId) -> bool {
        self.dependencies.contains(module_id)
    }

    /// Check if this module is depended on by another module
    pub fn is_depended_on_by(&self, module_id: &ModuleId) -> bool {
        self.dependents.contains(module_id)
    }

    /// Get all dependencies
    pub fn get_dependencies(&self) -> Vec<ModuleId> {
        self.dependencies.iter().cloned().collect()
    }

    /// Get all dependents
    pub fn get_dependents(&self) -> Vec<ModuleId> {
        self.dependents.iter().cloned().collect()
    }

    /// Check if this module has any dependencies
    pub fn has_dependencies(&self) -> bool {
        !self.dependencies.is_empty()
    }

    /// Check if this module has any dependents
    pub fn has_dependents(&self) -> bool {
        !self.dependents.is_empty()
    }
}