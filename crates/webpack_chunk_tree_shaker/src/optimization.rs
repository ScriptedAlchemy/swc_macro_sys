use std::collections::HashMap;
use webpack_analyzer_v2::*;
use crate::{Result, WebpackTreeShaker, TreeShakingOptions};

/// Advanced optimization strategies for webpack chunks
pub struct ChunkOptimizer {
    tree_shaker: WebpackTreeShaker,
}

/// Optimization strategy configuration
#[derive(Debug, Clone)]
pub struct OptimizationStrategy {
    /// Remove unused modules
    pub remove_unused: bool,
    /// Optimize for production
    pub production_mode: bool,
}

impl Default for OptimizationStrategy {
    fn default() -> Self {
        Self {
            remove_unused: true,
            production_mode: false,
        }
    }
}

/// Result of optimization with detailed analysis
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// The optimized chunk
    pub optimized_chunk: WebpackChunk,
    /// Applied optimizations
    pub applied_optimizations: Vec<OptimizationType>,
    /// Modules removed by each optimization
    pub removed_by_optimization: HashMap<OptimizationType, Vec<ModuleId>>,
    /// Overall statistics
    pub stats: OptimizationStats,
}

/// Types of optimizations that can be applied
#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub enum OptimizationType {
    UnusedModuleRemoval,
}

/// Statistics about the optimization process
#[derive(Debug, Clone)]
pub struct OptimizationStats {
    /// Original module count
    pub original_modules: usize,
    /// Final module count
    pub final_modules: usize,
    /// Total modules removed
    pub total_removed: usize,
    /// Modules removed by each optimization type
    pub removed_by_type: HashMap<OptimizationType, usize>,
    /// Size reduction
    pub size_reduction: f64,
    /// Optimization time in milliseconds
    pub optimization_time_ms: u128,
}

impl ChunkOptimizer {
    /// Create a new chunk optimizer
    pub fn new() -> Self {
        Self {
            tree_shaker: WebpackTreeShaker::new(),
        }
    }

    /// Create optimizer with custom tree shaker options
    pub fn with_tree_shaker_options(options: TreeShakingOptions) -> Self {
        Self {
            tree_shaker: WebpackTreeShaker::with_options(options),
        }
    }

    /// Optimize a chunk using the specified strategy and explicit entry modules
    pub fn optimize_chunk_with_entries(
        &self,
        chunk: &WebpackChunk,
        strategy: &OptimizationStrategy,
        entry_modules: &[ModuleId],
    ) -> Result<OptimizationResult> {
        let start_time = std::time::Instant::now();
        
        let mut working_chunk = chunk.clone();
        let mut applied_optimizations = Vec::new();
        let mut removed_by_optimization = HashMap::new();
        
        // Apply optimizations in order
        if strategy.remove_unused {
            let (optimized, removed) = self.remove_unused_modules(&working_chunk, entry_modules)?;
            working_chunk = optimized;
            applied_optimizations.push(OptimizationType::UnusedModuleRemoval);
            removed_by_optimization.insert(OptimizationType::UnusedModuleRemoval, removed);
        }
        
        // Only unused module removal is supported now
        
        let optimization_time = start_time.elapsed();
        
        // Calculate statistics
        let stats = self.calculate_optimization_stats(
            chunk,
            &working_chunk,
            &removed_by_optimization,
            optimization_time.as_millis(),
        );
        
        Ok(OptimizationResult {
            optimized_chunk: working_chunk,
            applied_optimizations,
            removed_by_optimization,
            stats,
        })
    }

    /// Backward-compatible wrapper: optimize without explicit entries (no unused removal applied)
    pub fn optimize_chunk(
        &self,
        chunk: &WebpackChunk,
        strategy: &OptimizationStrategy,
    ) -> Result<OptimizationResult> {
        self.optimize_chunk_with_entries(chunk, strategy, &[])
    }

    /// Remove unused modules using provided entry modules
    fn remove_unused_modules(
        &self,
        chunk: &WebpackChunk,
        entry_modules: &[ModuleId],
    ) -> Result<(WebpackChunk, Vec<ModuleId>)> {
        if entry_modules.is_empty() {
            // No explicit entry modules; preserve all modules
            return Ok((chunk.clone(), Vec::new()));
        }

        let result = self.tree_shaker.shake_tree(chunk, entry_modules)?;

        Ok((result.optimized_chunk, result.removed_modules))
    }

    // No-exports and debug-only removal have been removed to focus on unused removal only

    // Entry-point heuristics removed; explicit entries must be provided externally

    /// Calculate optimization statistics
    fn calculate_optimization_stats(
        &self,
        original_chunk: &WebpackChunk,
        optimized_chunk: &WebpackChunk,
        removed_by_optimization: &HashMap<OptimizationType, Vec<ModuleId>>,
        optimization_time_ms: u128,
    ) -> OptimizationStats {
        let original_modules = original_chunk.module_count();
        let final_modules = optimized_chunk.module_count();
        let total_removed = original_modules - final_modules;
        
        let removed_by_type = removed_by_optimization
            .iter()
            .map(|(opt_type, modules)| (opt_type.clone(), modules.len()))
            .collect();
        
        // Calculate size reduction
        let original_size = original_chunk.modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();
        
        let final_size = optimized_chunk.modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();
        
        let size_reduction = if original_size > 0 {
            ((original_size - final_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };
        
        OptimizationStats {
            original_modules,
            final_modules,
            total_removed,
            removed_by_type,
            size_reduction,
            optimization_time_ms,
        }
    }
}

impl Default for ChunkOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

impl OptimizationResult {
    /// Get a summary of the optimization results
    pub fn summary(&self) -> String {
        format!(
            "Optimization removed {} modules ({:.1}% reduction) in {}ms",
            self.stats.total_removed,
            self.stats.size_reduction,
            self.stats.optimization_time_ms
        )
    }

    /// Check if optimization was successful
    pub fn was_successful(&self) -> bool {
        self.stats.total_removed > 0
    }

    /// Get details about each optimization type
    pub fn optimization_details(&self) -> Vec<String> {
        let mut details = Vec::new();
        
        for optimization_type in &self.applied_optimizations {
            if let Some(removed_count) = self.stats.removed_by_type.get(optimization_type) {
                let detail = match optimization_type {
                    OptimizationType::UnusedModuleRemoval => {
                        format!("Unused module removal: {} modules", removed_count)
                    }
                };
                details.push(detail);
            }
        }
        
        details
    }
}