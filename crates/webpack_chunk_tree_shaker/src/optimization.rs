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
    /// Remove modules with no exports
    pub remove_no_exports: bool,
    /// Remove duplicate modules
    pub remove_duplicates: bool,
    /// Remove debug-only modules
    pub remove_debug_modules: bool,
    /// Optimize for production
    pub production_mode: bool,
}

impl Default for OptimizationStrategy {
    fn default() -> Self {
        Self {
            remove_unused: true,
            remove_no_exports: true,
            remove_duplicates: true,
            remove_debug_modules: true,
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
    NoExportsRemoval,
    DuplicateRemoval,
    DebugModuleRemoval,
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

    /// Optimize a chunk using the specified strategy
    pub fn optimize_chunk(
        &self,
        chunk: &WebpackChunk,
        strategy: &OptimizationStrategy,
    ) -> Result<OptimizationResult> {
        let start_time = std::time::Instant::now();
        
        let mut working_chunk = chunk.clone();
        let mut applied_optimizations = Vec::new();
        let mut removed_by_optimization = HashMap::new();
        
        // Apply optimizations in order
        if strategy.remove_unused {
            let (optimized, removed) = self.remove_unused_modules(&working_chunk)?;
            working_chunk = optimized;
            applied_optimizations.push(OptimizationType::UnusedModuleRemoval);
            removed_by_optimization.insert(OptimizationType::UnusedModuleRemoval, removed);
        }
        
        if strategy.remove_no_exports {
            let (optimized, removed) = self.remove_no_export_modules(&working_chunk)?;
            working_chunk = optimized;
            applied_optimizations.push(OptimizationType::NoExportsRemoval);
            removed_by_optimization.insert(OptimizationType::NoExportsRemoval, removed);
        }
        
        if strategy.remove_duplicates {
            let (optimized, removed) = self.remove_duplicate_modules(&working_chunk)?;
            working_chunk = optimized;
            applied_optimizations.push(OptimizationType::DuplicateRemoval);
            removed_by_optimization.insert(OptimizationType::DuplicateRemoval, removed);
        }
        
        if strategy.remove_debug_modules {
            let (optimized, removed) = self.remove_debug_modules(&working_chunk)?;
            working_chunk = optimized;
            applied_optimizations.push(OptimizationType::DebugModuleRemoval);
            removed_by_optimization.insert(OptimizationType::DebugModuleRemoval, removed);
        }
        
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

    /// Remove unused modules using entry point analysis
    fn remove_unused_modules(&self, chunk: &WebpackChunk) -> Result<(WebpackChunk, Vec<ModuleId>)> {
        // Find potential entry points (modules with no or minimal dependencies)
        let entry_modules = self.find_entry_points(chunk);
        
        if entry_modules.is_empty() {
            // If no clear entry points, preserve all modules
            return Ok((chunk.clone(), Vec::new()));
        }
        
        // Use tree shaker to remove unused modules
        let result = self.tree_shaker.shake_tree(chunk, &entry_modules)?;
        
        Ok((result.optimized_chunk, result.removed_modules))
    }

    /// Remove modules that don't export anything
    fn remove_no_export_modules(&self, chunk: &WebpackChunk) -> Result<(WebpackChunk, Vec<ModuleId>)> {
        let mut modules_to_remove = Vec::new();
        
        for (module_id, module) in &chunk.modules {
            if self.has_no_exports(&module.source) {
                modules_to_remove.push(module_id.clone());
            }
        }
        
        if modules_to_remove.is_empty() {
            return Ok((chunk.clone(), Vec::new()));
        }
        
        let result = self.tree_shaker.remove_modules(chunk, &modules_to_remove)?;
        
        Ok((result.optimized_chunk, result.removed_modules))
    }

    /// Remove duplicate modules (same source code)
    fn remove_duplicate_modules(&self, chunk: &WebpackChunk) -> Result<(WebpackChunk, Vec<ModuleId>)> {
        let mut source_to_modules: HashMap<String, Vec<ModuleId>> = HashMap::new();
        
        // Group modules by source code
        for (module_id, module) in &chunk.modules {
            source_to_modules
                .entry(module.source.clone())
                .or_default()
                .push(module_id.clone());
        }
        
        // Find duplicates (keep first occurrence)
        let mut modules_to_remove = Vec::new();
        for (_, module_ids) in source_to_modules {
            if module_ids.len() > 1 {
                // Keep the first one, remove the rest
                modules_to_remove.extend(module_ids.into_iter().skip(1));
            }
        }
        
        if modules_to_remove.is_empty() {
            return Ok((chunk.clone(), Vec::new()));
        }
        
        let result = self.tree_shaker.remove_modules(chunk, &modules_to_remove)?;
        
        Ok((result.optimized_chunk, result.removed_modules))
    }

    /// Remove debug-only modules
    fn remove_debug_modules(&self, chunk: &WebpackChunk) -> Result<(WebpackChunk, Vec<ModuleId>)> {
        let mut modules_to_remove = Vec::new();
        
        for (module_id, module) in &chunk.modules {
            if self.is_debug_module(module_id, &module.source) {
                modules_to_remove.push(module_id.clone());
            }
        }
        
        if modules_to_remove.is_empty() {
            return Ok((chunk.clone(), Vec::new()));
        }
        
        let result = self.tree_shaker.remove_modules(chunk, &modules_to_remove)?;
        
        Ok((result.optimized_chunk, result.removed_modules))
    }

    /// Find potential entry points in the chunk
    fn find_entry_points(&self, chunk: &WebpackChunk) -> Vec<ModuleId> {
        let mut entry_points = Vec::new();
        
        // Build dependency graph
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }
        
        // Find modules that are likely entry points
        for (module_id, module) in &chunk.modules {
            // Consider a module an entry point if:
            // 1. It has no dependencies, OR
            // 2. It has many dependents (popular module), OR
            // 3. It matches common entry point patterns
            
            if module.dependencies.is_empty() ||
               module.dependents.len() > 3 ||
               self.matches_entry_point_pattern(module_id) {
                entry_points.push(module_id.clone());
            }
        }
        
        // If no entry points found, use modules with most dependents
        if entry_points.is_empty() {
            let mut modules_with_dependents: Vec<_> = chunk.modules
                .iter()
                .filter(|(_, module)| !module.dependents.is_empty())
                .collect();
            
            modules_with_dependents.sort_by(|a, b| 
                b.1.dependents.len().cmp(&a.1.dependents.len())
            );
            
            if let Some((module_id, _)) = modules_with_dependents.first() {
                entry_points.push((*module_id).clone());
            }
        }
        
        entry_points
    }

    /// Check if a module has no exports
    fn has_no_exports(&self, source: &str) -> bool {
        // Simple heuristic: check for common export patterns
        !source.contains("module.exports") &&
        !source.contains("exports.") &&
        !source.contains("export ") &&
        !source.contains("__webpack_require__.d(")
    }

    /// Check if a module is debug-only
    fn is_debug_module(&self, module_id: &str, source: &str) -> bool {
        // Check module ID patterns
        if module_id.contains("debug") ||
           module_id.contains("test") ||
           module_id.contains("spec") ||
           module_id.contains("__test__") ||
           module_id.contains("__debug__") {
            return true;
        }
        
        // Check source code patterns
        if source.contains("console.log") ||
           source.contains("console.debug") ||
           source.contains("debugger;") ||
           source.contains("__DEV__") {
            return true;
        }
        
        false
    }

    /// Check if a module ID matches common entry point patterns
    fn matches_entry_point_pattern(&self, module_id: &ModuleId) -> bool {
        let id_str = module_id.as_str();
        id_str.contains("index") ||
        id_str.contains("main") ||
        id_str.contains("entry") ||
        id_str.contains("bootstrap") ||
        id_str.ends_with("/index.js") ||
        id_str.ends_with("/main.js")
    }

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
                    OptimizationType::NoExportsRemoval => {
                        format!("No exports removal: {} modules", removed_count)
                    }
                    OptimizationType::DuplicateRemoval => {
                        format!("Duplicate removal: {} modules", removed_count)
                    }
                    OptimizationType::DebugModuleRemoval => {
                        format!("Debug module removal: {} modules", removed_count)
                    }
                };
                details.push(detail);
            }
        }
        
        details
    }
}