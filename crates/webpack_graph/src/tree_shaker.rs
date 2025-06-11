use crate::graph::ModuleGraph;

/// Provides tree-shaking capabilities for a `ModuleGraph`.
///
/// This struct holds a mutable reference to a `ModuleGraph` and offers
/// methods to remove modules and perform tree-shaking to eliminate dead code.
pub struct TreeShaker<'a> {
    graph: &'a mut ModuleGraph,
}

impl<'a> TreeShaker<'a> {
    /// Creates a new `TreeShaker` instance for the given `ModuleGraph`.
    pub fn new(graph: &'a mut ModuleGraph) -> Self {
        Self { graph }
    }

    /// Removes a module by its ID.
    ///
    /// This is a low-level operation that disconnects the module from its
    /// dependencies and dependents, then removes it from the graph.
    ///
    /// Returns `true` if the module was found and removed, `false` otherwise.
    pub fn remove_module(&mut self, module_id: &str) -> bool {
        if let Some(removed_module) = self.graph.modules.remove(module_id) {
            // Disconnect from dependencies: for each module this one depended on,
            // remove this module from their list of dependents.
            for dep_id in &removed_module.dependencies {
                if let Some(dep_module) = self.graph.modules.get_mut(dep_id) {
                    dep_module.dependents.remove(module_id);
                }
            }

            // Disconnect from dependents: for each module that depended on this one,
            // remove this module from their list of dependencies.
            for dependent_id in &removed_module.dependents {
                if let Some(dependent_module) = self.graph.modules.get_mut(dependent_id) {
                    dependent_module.dependencies.remove(module_id);
                }
            }

            // If the removed module was an entry point, remove it from the list.
            self.graph.entry_points.retain(|id| id != module_id);

            true
        } else {
            false
        }
    }

    /// Performs tree-shaking by removing all modules that are not reachable
    /// from the graph's entry points.
    ///
    /// This is the primary method for eliminating dead code from the graph.
    ///
    /// Returns a `Vec<String>` of the removed module IDs.
    pub fn shake(&mut self) -> Vec<String> {
        let unreachable_ids = self.graph.get_unreachable_modules();
        for module_id in &unreachable_ids {
            self.remove_module(module_id);
        }
        unreachable_ids
    }
}



 