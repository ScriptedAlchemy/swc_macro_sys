use std::collections::HashSet;
use webpack_analyzer_v2::*;
use crate::{Result, TreeShakingError};

/// Main tree shaking implementation that works with webpack_analyzer_v2
pub struct WebpackTreeShaker {
    options: TreeShakingOptions,
}

/// Configuration options for tree shaking
#[derive(Debug, Clone)]
pub struct TreeShakingOptions {
    /// Whether to preserve entry modules (modules with no dependencies)
    pub preserve_entry_modules: bool,
    /// Whether to perform aggressive tree shaking (remove modules with circular deps)
    pub aggressive_mode: bool,
    /// Whether to validate chunk integrity after tree shaking
    pub validate_integrity: bool,
    /// Maximum number of modules to remove in one operation
    pub max_removals: Option<usize>,
    /// Whether to preserve webpack runtime modules
    pub preserve_runtime: bool,
}

impl Default for TreeShakingOptions {
    fn default() -> Self {
        Self {
            preserve_entry_modules: true,
            aggressive_mode: false,
            validate_integrity: true,
            max_removals: None,
            preserve_runtime: true,
        }
    }
}

/// Result of a tree shaking operation
#[derive(Debug, Clone)]
pub struct TreeShakingResult {
    /// The optimized chunk with removed modules
    pub optimized_chunk: WebpackChunk,
    /// Modules that were removed
    pub removed_modules: Vec<ModuleId>,
    /// Modules that were preserved
    pub preserved_modules: Vec<ModuleId>,
    /// Impact analysis of the removal
    pub impact: ModuleRemovalImpact,
    /// Statistics about the optimization
    pub stats: TreeShakingStats,
}

/// Statistics about tree shaking operation
#[derive(Debug, Clone)]
pub struct TreeShakingStats {
    /// Original module count
    pub original_count: usize,
    /// Final module count after optimization
    pub final_count: usize,
    /// Number of modules removed
    pub removed_count: usize,
    /// Reduction percentage
    pub reduction_percentage: f64,
    /// Original chunk size estimate (in characters)
    pub original_size: usize,
    /// Final chunk size estimate (in characters)
    pub final_size: usize,
    /// Size reduction percentage
    pub size_reduction_percentage: f64,
}

impl WebpackTreeShaker {
    /// Create a new tree shaker with default options
    pub fn new() -> Self {
        Self {
            options: TreeShakingOptions::default(),
        }
    }

    /// Create a new tree shaker with custom options
    pub fn with_options(options: TreeShakingOptions) -> Self {
        Self { options }
    }

    /// Remove specific modules from a chunk
    pub fn remove_modules(
        &self,
        chunk: &WebpackChunk,
        modules_to_remove: &[impl AsRef<str>],
    ) -> Result<TreeShakingResult> {
        let module_ids: Vec<ModuleId> = modules_to_remove
            .iter()
            .map(|m| ModuleId::from(m.as_ref()))
            .collect();

        // Build dependency graph
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }

        // Validate removal safety
        self.validate_removal_safety(&graph, &module_ids)?;

        // Perform the removal
        let removed_modules = self.perform_removal(chunk, &graph, &module_ids)?;

        // Build result
        self.build_result(chunk, &graph, &removed_modules)
    }

    /// Perform aggressive tree shaking to remove all unused modules
    pub fn shake_tree(
        &self,
        chunk: &WebpackChunk,
        entry_modules: &[impl AsRef<str>],
    ) -> Result<TreeShakingResult> {
        let entry_ids: Vec<ModuleId> = entry_modules
            .iter()
            .map(|m| ModuleId::from(m.as_ref()))
            .collect();

        // Build dependency graph
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }

        // Find all reachable modules from entry points
        let mut reachable = HashSet::new();
        for entry_id in &entry_ids {
            reachable.extend(graph.get_reachable_modules(entry_id));
        }

        // Identify modules to remove (unreachable modules)
        let modules_to_remove: Vec<ModuleId> = chunk.modules
            .keys()
            .filter(|module_id| !reachable.contains(*module_id))
            .cloned()
            .collect();

        // Apply max removals limit if set
        let modules_to_remove = if let Some(max) = self.options.max_removals {
            modules_to_remove.into_iter().take(max).collect()
        } else {
            modules_to_remove
        };

        // Perform the removal
        let removed_modules = self.perform_removal(chunk, &graph, &modules_to_remove)?;

        // Build result
        self.build_result(chunk, &graph, &removed_modules)
    }

    /// Find unused modules based on dependency analysis
    pub fn find_unused_modules(
        &self,
        chunk: &WebpackChunk,
        entry_modules: &[impl AsRef<str>],
    ) -> Result<Vec<ModuleId>> {
        let entry_ids: Vec<ModuleId> = entry_modules
            .iter()
            .map(|m| ModuleId::from(m.as_ref()))
            .collect();

        // Build dependency graph
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }

        // Find all reachable modules from entry points
        let mut reachable = HashSet::new();
        for entry_id in &entry_ids {
            reachable.extend(graph.get_reachable_modules(entry_id));
        }

        // Return unreachable modules
        Ok(chunk.modules
            .keys()
            .filter(|module_id| !reachable.contains(*module_id))
            .cloned()
            .collect())
    }

    /// Validate that module removal is safe
    fn validate_removal_safety(
        &self,
        graph: &DependencyGraph,
        modules_to_remove: &[ModuleId],
    ) -> Result<()> {
        for module_id in modules_to_remove {
            // Check if module exists
            if !graph.modules.contains_key(module_id) {
                return Err(TreeShakingError::module_not_found(module_id.clone()));
            }

            // Check if it's an entry module and we should preserve it
            if self.options.preserve_entry_modules {
                if let Some(module) = graph.modules.get(module_id) {
                    if module.dependencies.is_empty() {
                        return Err(TreeShakingError::EntryModuleRemoval {
                            module_id: module_id.clone(),
                        });
                    }
                }
            }

            // Check if removal would break dependent modules
            let impact = graph.simulate_module_removal(module_id);
            if !impact.broken_modules.is_empty() && !self.options.aggressive_mode {
                return Err(TreeShakingError::unsafe_removal(
                    module_id.clone(),
                    impact.broken_modules.len(),
                ));
            }
        }

        Ok(())
    }

    /// Perform the actual module removal
    fn perform_removal(
        &self,
        chunk: &WebpackChunk,
        graph: &DependencyGraph,
        modules_to_remove: &[ModuleId],
    ) -> Result<Vec<ModuleId>> {
        let mut removed_modules = Vec::new();

        // Remove modules from the chunk
        for module_id in modules_to_remove {
            // Skip if module doesn't exist
            if !chunk.modules.contains_key(module_id) {
                continue;
            }

            // Skip runtime modules if preserve_runtime is enabled
            if self.options.preserve_runtime && self.is_runtime_module(module_id) {
                continue;
            }

            removed_modules.push(module_id.clone());
        }

        // Check if we would create an empty chunk
        if removed_modules.len() == chunk.modules.len() {
            return Err(TreeShakingError::EmptyChunk);
        }

        Ok(removed_modules)
    }

    /// Build the final result
    fn build_result(
        &self,
        original_chunk: &WebpackChunk,
        graph: &DependencyGraph,
        removed_modules: &[ModuleId],
    ) -> Result<TreeShakingResult> {
        // Create optimized chunk
        let mut optimized_chunk = WebpackChunk::new(
            original_chunk.chunk_type.clone(),
            original_chunk.source.clone(),
        );

        // Add remaining modules
        for (module_id, module) in &original_chunk.modules {
            if !removed_modules.contains(module_id) {
                optimized_chunk.add_module(module_id.clone(), module.clone());
            }
        }

        // Calculate impact (use first removed module as representative)
        let impact = if let Some(first_removed) = removed_modules.first() {
            graph.simulate_module_removal(first_removed)
        } else {
            ModuleRemovalImpact::new(ModuleId::from(""))
        };

        // Calculate statistics
        let stats = self.calculate_stats(original_chunk, &optimized_chunk);

        // Get preserved modules
        let preserved_modules: Vec<ModuleId> = optimized_chunk.modules.keys().cloned().collect();

        Ok(TreeShakingResult {
            optimized_chunk,
            removed_modules: removed_modules.to_vec(),
            preserved_modules,
            impact,
            stats,
        })
    }

    /// Calculate tree shaking statistics
    fn calculate_stats(
        &self,
        original_chunk: &WebpackChunk,
        optimized_chunk: &WebpackChunk,
    ) -> TreeShakingStats {
        let original_count = original_chunk.module_count();
        let final_count = optimized_chunk.module_count();
        let removed_count = original_count - final_count;
        
        let reduction_percentage = if original_count > 0 {
            (removed_count as f64 / original_count as f64) * 100.0
        } else {
            0.0
        };

        // Estimate sizes based on source code length
        let original_size = original_chunk.modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();

        let final_size = optimized_chunk.modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();

        let size_reduction_percentage = if original_size > 0 {
            ((original_size - final_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };

        TreeShakingStats {
            original_count,
            final_count,
            removed_count,
            reduction_percentage,
            original_size,
            final_size,
            size_reduction_percentage,
        }
    }

    /// Check if a module is a webpack runtime module
    fn is_runtime_module(&self, module_id: &ModuleId) -> bool {
        module_id.contains("webpack/runtime") || 
        module_id.contains("webpack/bootstrap") ||
        module_id.starts_with("webpack/")
    }
}

impl Default for WebpackTreeShaker {
    fn default() -> Self {
        Self::new()
    }
}

impl TreeShakingResult {
    /// Get the reduction percentage (0-100)
    pub fn reduction_percentage(&self) -> f64 {
        self.stats.reduction_percentage
    }

    /// Get the size reduction percentage (0-100)
    pub fn size_reduction_percentage(&self) -> f64 {
        self.stats.size_reduction_percentage
    }

    /// Check if the tree shaking was successful
    pub fn was_successful(&self) -> bool {
        self.stats.removed_count > 0
    }

    /// Get a summary of the optimization
    pub fn summary(&self) -> String {
        format!(
            "Tree shaking removed {} modules ({:.1}% reduction), size reduced by {:.1}%",
            self.stats.removed_count,
            self.stats.reduction_percentage,
            self.stats.size_reduction_percentage
        )
    }
}