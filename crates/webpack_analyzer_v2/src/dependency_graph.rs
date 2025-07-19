use rustc_hash::FxHashMap;
use std::collections::{HashSet, VecDeque};
use swc_core::atoms::Atom;
use crate::module::{ModuleId, WebpackModule};

/// Represents a dependency graph for webpack modules
#[derive(Debug, Clone)]
pub struct DependencyGraph {
    /// All modules in the graph
    pub modules: FxHashMap<ModuleId, WebpackModule>,
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}

impl DependencyGraph {
    /// Create a new dependency graph
    pub fn new() -> Self {
        Self {
            modules: FxHashMap::default(),
        }
    }

    /// Add a module to the graph
    pub fn add_module(&mut self, module: WebpackModule) {
        self.modules.insert(module.id.clone(), module);
    }

    /// Add a dependency relationship between two modules
    pub fn add_dependency(&mut self, from_module: &ModuleId, to_module: &ModuleId) {
        // Add dependency to the from_module
        if let Some(module) = self.modules.get_mut(from_module) {
            module.add_dependency(to_module.clone());
        }

        // Add dependent to the to_module
        if let Some(module) = self.modules.get_mut(to_module) {
            module.add_dependent(from_module.clone());
        }
    }

    /// Get all modules that are reachable from a given module
    pub fn get_reachable_modules(&self, start_module: &ModuleId) -> HashSet<ModuleId> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(start_module.clone());
        visited.insert(start_module.clone());

        while let Some(current) = queue.pop_front() {
            if let Some(module) = self.modules.get(&current) {
                for dependency in &module.dependencies {
                    if !visited.contains(dependency) {
                        visited.insert(dependency.clone());
                        queue.push_back(dependency.clone());
                    }
                }
            }
        }

        visited
    }

    /// Get all modules that are reachable from multiple start modules
    pub fn get_reachable_from_multiple(&self, start_modules: &[ModuleId]) -> HashSet<ModuleId> {
        let mut reachable = HashSet::new();
        
        for start_module in start_modules {
            let module_reachable = self.get_reachable_modules(start_module);
            reachable.extend(module_reachable);
        }

        reachable
    }

    /// Find orphaned modules (modules that are not reachable from any entry point)
    pub fn find_orphaned_modules(&self, entry_points: &[ModuleId]) -> Vec<ModuleId> {
        let reachable = self.get_reachable_from_multiple(entry_points);
        
        self.modules
            .keys()
            .filter(|module_id| !reachable.contains(*module_id))
            .cloned()
            .collect()
    }

    /// Find potential entry points (modules with no dependents)
    pub fn find_potential_entry_points(&self) -> Vec<ModuleId> {
        self.modules
            .values()
            .filter(|module| !module.has_dependents())
            .map(|module| module.id.clone())
            .collect()
    }

    /// Find modules that match a pattern (e.g., main modules)
    pub fn find_modules_by_pattern(&self, pattern: impl Fn(&ModuleId) -> bool) -> Vec<ModuleId> {
        self.modules
            .keys()
            .filter(|module_id| pattern(module_id))
            .cloned()
            .collect()
    }

    /// Get module count
    pub fn module_count(&self) -> usize {
        self.modules.len()
    }

    /// Get total dependency count
    pub fn total_dependencies(&self) -> usize {
        self.modules.values().map(|m| m.dependencies.len()).sum()
    }

    /// Print debug information about the graph
    pub fn print_debug_info(&self) {
        println!("Dependency Graph Debug Info:");
        println!("Total modules: {}", self.module_count());
        println!("Total dependencies: {}", self.total_dependencies());
        
        for (module_id, module) in &self.modules {
            println!("  Module '{}': deps={:?}, dependents={:?}", 
                     module_id, 
                     module.dependencies.len(), 
                     module.dependents.len());
        }
    }

    /// Simulate removing a module and return what would be impacted
    pub fn simulate_module_removal(&self, module_to_remove: &ModuleId) -> ModuleRemovalImpact {
        let mut impact = ModuleRemovalImpact::new(module_to_remove.clone());
        
        // Find all modules that depend on this module (would be broken)
        if let Some(module) = self.modules.get(module_to_remove) {
            for dependent in &module.dependents {
                impact.broken_modules.push(dependent.clone());
            }
        }
        
        // Find all modules that this module depends on (potentially orphaned)
        if let Some(module) = self.modules.get(module_to_remove) {
            for dependency in &module.dependencies {
                // Check if this dependency would become orphaned
                if let Some(dep_module) = self.modules.get(dependency) {
                    // Count how many things depend on this dependency
                    let remaining_dependents = dep_module.dependents.len() - 
                        if dep_module.dependents.contains(module_to_remove) { 1 } else { 0 };
                    
                    if remaining_dependents == 0 {
                        impact.potentially_orphaned.push(dependency.clone());
                    }
                }
            }
        }
        
        impact
    }

    /// Simulate removing multiple modules and return cumulative impact
    pub fn simulate_multiple_module_removal(&self, modules_to_remove: &[ModuleId]) -> ModuleRemovalImpact {
        let mut cumulative_impact = ModuleRemovalImpact::new(Atom::from("multiple"));
        
        for module_id in modules_to_remove {
            let impact = self.simulate_module_removal(module_id);
            cumulative_impact.broken_modules.extend(impact.broken_modules);
            cumulative_impact.potentially_orphaned.extend(impact.potentially_orphaned);
        }
        
        // Remove duplicates
        cumulative_impact.broken_modules.sort();
        cumulative_impact.broken_modules.dedup();
        cumulative_impact.potentially_orphaned.sort();
        cumulative_impact.potentially_orphaned.dedup();
        
        cumulative_impact
    }
}

/// Represents the impact of removing a module
#[derive(Debug, Clone)]
pub struct ModuleRemovalImpact {
    /// The module that was removed
    pub removed_module: ModuleId,
    /// Modules that would be broken (they depend on the removed module)
    pub broken_modules: Vec<ModuleId>,
    /// Modules that would potentially become orphaned (no longer have dependents)
    pub potentially_orphaned: Vec<ModuleId>,
}

impl ModuleRemovalImpact {
    pub fn new(removed_module: ModuleId) -> Self {
        Self {
            removed_module,
            broken_modules: Vec::new(),
            potentially_orphaned: Vec::new(),
        }
    }
    
    pub fn print_summary(&self) {
        println!("Impact of removing '{}':", self.removed_module);
        println!("  - Broken modules: {}", self.broken_modules.len());
        for module in &self.broken_modules {
            let short_name = module.split('/').next_back().unwrap_or(module);
            println!("    * {short_name}");
        }
        println!("  - Potentially orphaned: {}", self.potentially_orphaned.len());
        for module in &self.potentially_orphaned {
            let short_name = module.split('/').next_back().unwrap_or(module);
            println!("    * {short_name}");
        }
    }
}