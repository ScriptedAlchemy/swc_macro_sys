use std::collections::{HashSet, HashMap};
use rustc_hash::FxHashMap;
use webpack_analyzer_v2::*;
use crate::{Result, TreeShakingError};

/// Validates tree shaking operations and chunk integrity
pub struct TreeShakingValidator {
    /// Whether to perform strict validation
    strict_mode: bool,
    /// Whether to validate dependency integrity
    validate_dependencies: bool,
    /// Whether to validate module references
    validate_references: bool,
}

/// Validation result with detailed feedback
#[derive(Debug, Clone)]
pub struct ValidationResult {
    /// Whether validation passed
    pub is_valid: bool,
    /// Validation errors found
    pub errors: Vec<ValidationError>,
    /// Validation warnings
    pub warnings: Vec<ValidationWarning>,
    /// Validation statistics
    pub stats: ValidationStats,
}

/// Validation error types
#[derive(Debug, Clone)]
pub enum ValidationError {
    /// Missing dependency
    MissingDependency {
        module_id: ModuleId,
        dependency_id: ModuleId,
    },
    /// Broken reference
    BrokenReference {
        from_module: ModuleId,
        to_module: ModuleId,
        reference_type: ReferenceType,
    },
    /// Circular dependency
    CircularDependency {
        cycle: Vec<ModuleId>,
    },
    /// Orphaned module
    OrphanedModule {
        module_id: ModuleId,
    },
    /// Invalid module structure
    InvalidModule {
        module_id: ModuleId,
        reason: String,
    },
}

/// Validation warning types
#[derive(Debug, Clone)]
pub enum ValidationWarning {
    /// Potentially unused module
    PotentiallyUnused {
        module_id: ModuleId,
        reason: String,
    },
    /// Large module
    LargeModule {
        module_id: ModuleId,
        size: usize,
    },
    /// Suspicious dependency
    SuspiciousDependency {
        from_module: ModuleId,
        to_module: ModuleId,
        reason: String,
    },
}

/// Reference types for validation
#[derive(Debug, Clone)]
pub enum ReferenceType {
    RequireCall,
    ImportStatement,
    DynamicImport,
    WeakReference,
}

/// Validation statistics
#[derive(Debug, Clone)]
pub struct ValidationStats {
    /// Total modules validated
    pub total_modules: usize,
    /// Total dependencies validated
    pub total_dependencies: usize,
    /// Errors found
    pub error_count: usize,
    /// Warnings found
    pub warning_count: usize,
    /// Validation time in milliseconds
    pub validation_time_ms: u128,
}

impl TreeShakingValidator {
    /// Create a new validator with default settings
    pub fn new() -> Self {
        Self {
            strict_mode: false,
            validate_dependencies: true,
            validate_references: true,
        }
    }

    /// Create a validator with custom settings
    pub fn with_settings(
        strict_mode: bool,
        validate_dependencies: bool,
        validate_references: bool,
    ) -> Self {
        Self {
            strict_mode,
            validate_dependencies,
            validate_references,
        }
    }

    /// Validate a chunk before tree shaking
    pub fn validate_before_shaking(&self, chunk: &WebpackChunk) -> Result<ValidationResult> {
        let start_time = std::time::Instant::now();
        
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Build dependency graph for validation
        let mut graph = DependencyGraph::new();
        for module in chunk.modules.values() {
            graph.add_module(module.clone());
        }
        
        // Validate chunk structure
        self.validate_chunk_structure(chunk, &mut errors, &mut warnings)?;
        
        // Validate dependencies
        if self.validate_dependencies {
            self.validate_dependencies_integrity(&graph, &mut errors, &mut warnings)?;
        }
        
        // Validate references
        if self.validate_references {
            self.validate_module_references(chunk, &mut errors, &mut warnings)?;
        }
        
        // Check for circular dependencies
        self.detect_circular_dependencies(&graph, &mut errors, &mut warnings)?;
        
        // Check for orphaned modules
        self.detect_orphaned_modules(&graph, &mut errors, &mut warnings)?;
        
        let validation_time = start_time.elapsed();
        
        let stats = ValidationStats {
            total_modules: chunk.module_count(),
            total_dependencies: graph.total_dependencies(),
            error_count: errors.len(),
            warning_count: warnings.len(),
            validation_time_ms: validation_time.as_millis(),
        };
        
        Ok(ValidationResult {
            is_valid: errors.is_empty() || !self.strict_mode,
            errors,
            warnings,
            stats,
        })
    }

    /// Validate a chunk after tree shaking
    pub fn validate_after_shaking(
        &self,
        original_chunk: &WebpackChunk,
        optimized_chunk: &WebpackChunk,
        removed_modules: &[ModuleId],
    ) -> Result<ValidationResult> {
        let start_time = std::time::Instant::now();
        
        let mut errors = Vec::new();
        let mut warnings = Vec::new();
        
        // Validate that removal was safe
        self.validate_removal_safety(
            original_chunk,
            optimized_chunk,
            removed_modules,
            &mut errors,
            &mut warnings,
        )?;
        
        // Validate optimized chunk integrity
        let chunk_validation = self.validate_before_shaking(optimized_chunk)?;
        errors.extend(chunk_validation.errors);
        warnings.extend(chunk_validation.warnings);
        
        // Validate that no required modules were removed
        self.validate_no_required_modules_removed(
            original_chunk,
            removed_modules,
            &mut errors,
            &mut warnings,
        )?;
        
        let validation_time = start_time.elapsed();
        
        let stats = ValidationStats {
            total_modules: optimized_chunk.module_count(),
            total_dependencies: chunk_validation.stats.total_dependencies,
            error_count: errors.len(),
            warning_count: warnings.len(),
            validation_time_ms: validation_time.as_millis(),
        };
        
        Ok(ValidationResult {
            is_valid: errors.is_empty() || !self.strict_mode,
            errors,
            warnings,
            stats,
        })
    }

    /// Validate basic chunk structure
    fn validate_chunk_structure(
        &self,
        chunk: &WebpackChunk,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        // Check if chunk has modules
        if chunk.modules.is_empty() {
            return Err(TreeShakingError::validation_failed("Chunk has no modules"));
        }
        
        // Validate each module
        for (module_id, module) in &chunk.modules {
            // Check module ID consistency
            if module.id != *module_id {
                errors.push(ValidationError::InvalidModule {
                    module_id: module_id.clone(),
                    reason: "Module ID mismatch".to_string(),
                });
            }
            
            // Check module source
            if module.source.is_empty() {
                errors.push(ValidationError::InvalidModule {
                    module_id: module_id.clone(),
                    reason: "Empty module source".to_string(),
                });
            }
            
            // Check for large modules
            if module.source.len() > 50000 {
                warnings.push(ValidationWarning::LargeModule {
                    module_id: module_id.clone(),
                    size: module.source.len(),
                });
            }
        }
        
        Ok(())
    }

    /// Validate dependency integrity
    fn validate_dependencies_integrity(
        &self,
        graph: &DependencyGraph,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        for (module_id, module) in &graph.modules {
            // Check that all dependencies exist
            for dep_id in &module.dependencies {
                if !graph.modules.contains_key(dep_id) {
                    errors.push(ValidationError::MissingDependency {
                        module_id: module_id.clone(),
                        dependency_id: dep_id.clone(),
                    });
                }
            }
            
            // Check that all dependents exist
            for dep_id in &module.dependents {
                if !graph.modules.contains_key(dep_id) {
                    errors.push(ValidationError::MissingDependency {
                        module_id: dep_id.clone(),
                        dependency_id: module_id.clone(),
                    });
                }
            }
            
            // Check for suspicious dependencies
            for dep_id in &module.dependencies {
                if module_id == dep_id {
                    warnings.push(ValidationWarning::SuspiciousDependency {
                        from_module: module_id.clone(),
                        to_module: dep_id.clone(),
                        reason: "Self-dependency".to_string(),
                    });
                }
            }
        }
        
        Ok(())
    }

    /// Validate module references
    fn validate_module_references(
        &self,
        chunk: &WebpackChunk,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        for (module_id, module) in &chunk.modules {
            // Look for webpack_require calls in source
            let require_calls = self.extract_require_calls(&module.source);
            
            for required_module in require_calls {
                if !chunk.modules.contains_key(&required_module) {
                    errors.push(ValidationError::BrokenReference {
                        from_module: module_id.clone(),
                        to_module: required_module,
                        reference_type: ReferenceType::RequireCall,
                    });
                }
            }
        }
        
        Ok(())
    }

    /// Detect circular dependencies
    fn detect_circular_dependencies(
        &self,
        graph: &DependencyGraph,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();
        let mut path = Vec::new();
        
        for module_id in graph.modules.keys() {
            if !visited.contains(module_id) {
                if let Some(cycle) = self.detect_cycle_from_module(
                    graph,
                    module_id,
                    &mut visited,
                    &mut recursion_stack,
                    &mut path,
                ) {
                    errors.push(ValidationError::CircularDependency { cycle });
                }
            }
        }
        
        Ok(())
    }

    /// Detect orphaned modules
    fn detect_orphaned_modules(
        &self,
        graph: &DependencyGraph,
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        for (module_id, module) in &graph.modules {
            // A module is orphaned if it has no dependents and is not an entry point
            if module.dependents.is_empty() && !self.is_entry_point(module_id) {
                warnings.push(ValidationWarning::PotentiallyUnused {
                    module_id: module_id.clone(),
                    reason: "No dependents, might be unused".to_string(),
                });
            }
        }
        
        Ok(())
    }

    /// Validate removal safety
    fn validate_removal_safety(
        &self,
        original_chunk: &WebpackChunk,
        optimized_chunk: &WebpackChunk,
        removed_modules: &[ModuleId],
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        // Check that removed modules actually existed
        for module_id in removed_modules {
            if !original_chunk.modules.contains_key(module_id) {
                errors.push(ValidationError::InvalidModule {
                    module_id: module_id.clone(),
                    reason: "Removed module didn't exist in original chunk".to_string(),
                });
            }
        }
        
        // Check that removed modules aren't referenced by remaining modules
        for (module_id, module) in &optimized_chunk.modules {
            let require_calls = self.extract_require_calls(&module.source);
            
            for required_module in require_calls {
                if removed_modules.contains(&required_module) {
                    errors.push(ValidationError::BrokenReference {
                        from_module: module_id.clone(),
                        to_module: required_module,
                        reference_type: ReferenceType::RequireCall,
                    });
                }
            }
        }
        
        Ok(())
    }

    /// Validate that no required modules were removed
    fn validate_no_required_modules_removed(
        &self,
        original_chunk: &WebpackChunk,
        removed_modules: &[ModuleId],
        errors: &mut Vec<ValidationError>,
        warnings: &mut Vec<ValidationWarning>,
    ) -> Result<()> {
        // Build dependency graph from original chunk
        let mut graph = DependencyGraph::new();
        for module in original_chunk.modules.values() {
            graph.add_module(module.clone());
        }
        
        // Check each removed module
        for module_id in removed_modules {
            if let Some(module) = graph.modules.get(module_id) {
                // If the module has dependents, it might be required
                if !module.dependents.is_empty() {
                    let impact = graph.simulate_module_removal(module_id);
                    if !impact.broken_modules.is_empty() {
                        errors.push(ValidationError::InvalidModule {
                            module_id: module_id.clone(),
                            reason: format!("Removed module has {} dependents", module.dependents.len()),
                        });
                    }
                }
            }
        }
        
        Ok(())
    }

    /// Extract require calls from module source
    fn extract_require_calls(&self, source: &str) -> Vec<ModuleId> {
        let mut require_calls = Vec::new();
        
        // Simple regex-based extraction (in real implementation, use AST)
        let require_pattern = regex::Regex::new(r#"__webpack_require__\s*\(\s*["']([^"']+)["']\s*\)"#).unwrap();
        
        for cap in require_pattern.captures_iter(source) {
            if let Some(module_id) = cap.get(1) {
                require_calls.push(module_id.as_str().to_string());
            }
        }
        
        require_calls
    }

    /// Detect cycle from a specific module
    fn detect_cycle_from_module(
        &self,
        graph: &DependencyGraph,
        module_id: &ModuleId,
        visited: &mut HashSet<ModuleId>,
        recursion_stack: &mut HashSet<ModuleId>,
        path: &mut Vec<ModuleId>,
    ) -> Option<Vec<ModuleId>> {
        visited.insert(module_id.clone());
        recursion_stack.insert(module_id.clone());
        path.push(module_id.clone());
        
        if let Some(module) = graph.modules.get(module_id) {
            for dep_id in &module.dependencies {
                if !visited.contains(dep_id) {
                    if let Some(cycle) = self.detect_cycle_from_module(
                        graph,
                        dep_id,
                        visited,
                        recursion_stack,
                        path,
                    ) {
                        return Some(cycle);
                    }
                } else if recursion_stack.contains(dep_id) {
                    // Found a cycle
                    let cycle_start = path.iter().position(|id| id == dep_id).unwrap();
                    let mut cycle = path[cycle_start..].to_vec();
                    cycle.push(dep_id.clone());
                    return Some(cycle);
                }
            }
        }
        
        recursion_stack.remove(module_id);
        path.pop();
        None
    }

    /// Check if a module is an entry point
    fn is_entry_point(&self, module_id: &ModuleId) -> bool {
        module_id.contains("index") ||
        module_id.contains("main") ||
        module_id.contains("entry") ||
        module_id.contains("bootstrap")
    }
}

impl Default for TreeShakingValidator {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    /// Check if validation passed
    pub fn is_valid(&self) -> bool {
        self.is_valid
    }

    /// Get error summary
    pub fn error_summary(&self) -> String {
        if self.errors.is_empty() {
            "No errors found".to_string()
        } else {
            format!("{} errors found", self.errors.len())
        }
    }

    /// Get warning summary
    pub fn warning_summary(&self) -> String {
        if self.warnings.is_empty() {
            "No warnings found".to_string()
        } else {
            format!("{} warnings found", self.warnings.len())
        }
    }

    /// Get detailed report
    pub fn detailed_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("Validation Report\n"));
        report.push_str(&format!("================\n"));
        report.push_str(&format!("Status: {}\n", if self.is_valid { "PASSED" } else { "FAILED" }));
        report.push_str(&format!("Modules: {}\n", self.stats.total_modules));
        report.push_str(&format!("Dependencies: {}\n", self.stats.total_dependencies));
        report.push_str(&format!("Errors: {}\n", self.stats.error_count));
        report.push_str(&format!("Warnings: {}\n", self.stats.warning_count));
        report.push_str(&format!("Validation time: {}ms\n\n", self.stats.validation_time_ms));
        
        if !self.errors.is_empty() {
            report.push_str("Errors:\n");
            for (i, error) in self.errors.iter().enumerate() {
                report.push_str(&format!("  {}. {:?}\n", i + 1, error));
            }
            report.push('\n');
        }
        
        if !self.warnings.is_empty() {
            report.push_str("Warnings:\n");
            for (i, warning) in self.warnings.iter().enumerate() {
                report.push_str(&format!("  {}. {:?}\n", i + 1, warning));
            }
        }
        
        report
    }
}