use rustc_hash::FxHashMap;
use webpack_analyzer_v2::*;
use crate::{Result, TreeShakingError};

/// Handles reconstruction of webpack chunks after tree shaking
pub struct ChunkReconstructor {
    /// Whether to preserve original formatting
    _preserve_formatting: bool,
    /// Whether to minify the output
    minify: bool,
}

impl ChunkReconstructor {
    /// Create a new chunk reconstructor with default settings
    pub fn new() -> Self {
        Self {
            _preserve_formatting: true,
            minify: false,
        }
    }

    /// Create a reconstructor with custom settings
    pub fn with_options(preserve_formatting: bool, minify: bool) -> Self {
        Self {
            _preserve_formatting: preserve_formatting,
            minify,
        }
    }

    /// Reconstruct a webpack chunk from optimized modules
    pub fn reconstruct_chunk(
        &self,
        chunk: &WebpackChunk,
        preserved_modules: &FxHashMap<ModuleId, WebpackModule>,
    ) -> Result<String> {
        match chunk.chunk_type {
            ChunkType::CommonJSAsync | ChunkType::CommonJSSync => self.reconstruct_commonjs_chunk(preserved_modules),
            ChunkType::JSONP => self.reconstruct_jsonp_chunk(preserved_modules),
            ChunkType::WebpackModules => self.reconstruct_webpack_modules_chunk(preserved_modules),
            ChunkType::ESModules => {
                // For now, treat ES modules similar to webpack modules
                self.reconstruct_webpack_modules_chunk(preserved_modules)
            },
            ChunkType::Unknown => {
                Err(TreeShakingError::reconstruction_failed("Cannot reconstruct chunk of unknown type").into())
            },
        }
    }

    /// Reconstruct a CommonJS format chunk
    fn reconstruct_commonjs_chunk(
        &self,
        modules: &FxHashMap<ModuleId, WebpackModule>,
    ) -> Result<String> {
        let mut output = String::new();
        
        // Add header
        output.push_str("\"use strict\";\n");
        
        // Add exports.ids (we'll use a default chunk name)
        let chunk_ids = self.extract_chunk_ids(modules);
        output.push_str(&format!("exports.ids = {:?};\n", chunk_ids));
        
        // Add exports.modules
        output.push_str("exports.modules = {\n");
        
        // Add each module
        let mut module_entries = Vec::new();
        for (module_id, module) in modules {
            let module_entry = self.format_module_entry(module_id, module)?;
            module_entries.push(module_entry);
        }
        
        // Join modules with commas
        output.push_str(&module_entries.join(",\n"));
        
        // Close exports.modules
        output.push_str("\n};\n");
        
        Ok(output)
    }

    /// Reconstruct a JSONP format chunk
    fn reconstruct_jsonp_chunk(
        &self,
        modules: &FxHashMap<ModuleId, WebpackModule>,
    ) -> Result<String> {
        let mut output = String::new();
        
        // Add JSONP header
        let chunk_ids = self.extract_chunk_ids(modules);
        output.push_str(&format!(
            "(self[\"webpackChunkapp\"] = self[\"webpackChunkapp\"] || []).push([\n"
        ));
        
        // Add chunk IDs
        output.push_str(&format!("    {:?},\n", chunk_ids));
        
        // Add modules object
        output.push_str("    {\n");
        
        // Add each module
        let mut module_entries = Vec::new();
        for (module_id, module) in modules {
            let module_entry = self.format_module_entry(module_id, module)?;
            module_entries.push(format!("        {}", module_entry));
        }
        
        // Join modules with commas
        output.push_str(&module_entries.join(",\n"));
        
        // Close modules object and JSONP
        output.push_str("\n    }\n]);\n");
        
        Ok(output)
    }

    /// Reconstruct a WebpackModules format chunk
    fn reconstruct_webpack_modules_chunk(
        &self,
        modules: &FxHashMap<ModuleId, WebpackModule>,
    ) -> Result<String> {
        let mut output = String::new();
        
        // Add webpack bootstrap start
        output.push_str("(() => { // webpackBootstrap\n");
        output.push_str("\"use strict\";\n");
        
        // Add webpack modules object
        output.push_str("var __webpack_modules__ = ({\n");
        
        // Add each module
        let mut module_entries = Vec::new();
        for (module_id, module) in modules {
            let module_entry = self.format_module_entry(module_id, module)?;
            module_entries.push(format!("    {}", module_entry));
        }
        
        // Join modules with commas
        output.push_str(&module_entries.join(",\n"));
        
        // Close modules object
        output.push_str("\n});\n\n");
        
        // Add webpack runtime (simplified)
        output.push_str("// The module cache\n");
        output.push_str("var __webpack_module_cache__ = {};\n\n");
        
        output.push_str("// The require function\n");
        output.push_str("function __webpack_require__(moduleId) {\n");
        output.push_str("    var cachedModule = __webpack_module_cache__[moduleId];\n");
        output.push_str("    if (cachedModule !== undefined) {\n");
        output.push_str("        return cachedModule.exports;\n");
        output.push_str("    }\n");
        output.push_str("    var module = (__webpack_module_cache__[moduleId] = {\n");
        output.push_str("        exports: {}\n");
        output.push_str("    });\n");
        output.push_str("    __webpack_modules__[moduleId](module, module.exports, __webpack_require__);\n");
        output.push_str("    return module.exports;\n");
        output.push_str("}\n\n");
        
        // Close webpack bootstrap
        output.push_str("})();\n");
        
        Ok(output)
    }

    /// Format a single module entry
    fn format_module_entry(
        &self,
        module_id: &ModuleId,
        module: &WebpackModule,
    ) -> Result<String> {
        let source = if self.minify {
            self.minify_module_source(&module.source)
        } else {
            module.source.clone()
        };
        
        // Format as: "module_id": function_source
        Ok(format!("{}: {}", 
            serde_json::to_string(module_id)
                .map_err(|e| TreeShakingError::SerializationError { source: e })?,
            source
        ))
    }

    /// Extract chunk IDs from modules (heuristic approach)
    fn extract_chunk_ids(&self, modules: &FxHashMap<ModuleId, WebpackModule>) -> Vec<String> {
        // For now, we'll generate a default chunk ID based on content
        // In a real implementation, you might want to preserve the original chunk ID
        let module_count = modules.len();
        vec![format!("optimized-chunk-{}-modules", module_count)]
    }

    /// Minify module source code (basic implementation)
    fn minify_module_source(&self, source: &str) -> String {
        if !self.minify {
            return source.to_string();
        }
        
        // Basic minification: remove extra whitespace and comments
        let mut minified = String::new();
        let mut in_string = false;
        let mut string_char = '"';
        let mut escape_next = false;
        
        let chars: Vec<char> = source.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            if escape_next {
                minified.push(ch);
                escape_next = false;
                i += 1;
                continue;
            }
            
            if ch == '\\' && in_string {
                escape_next = true;
                minified.push(ch);
                i += 1;
                continue;
            }
            
            if (ch == '"' || ch == '\'') && !in_string {
                in_string = true;
                string_char = ch;
                minified.push(ch);
            } else if ch == string_char && in_string {
                in_string = false;
                minified.push(ch);
            } else if in_string {
                minified.push(ch);
            } else {
                // Not in string, apply minification rules
                match ch {
                    ' ' | '\t' | '\n' | '\r' => {
                        // Only add space if needed (between identifiers)
                        if !minified.is_empty() && 
                           minified.chars().last().unwrap().is_alphanumeric() &&
                           i + 1 < chars.len() &&
                           chars[i + 1].is_alphanumeric() {
                            minified.push(' ');
                        }
                    }
                    '/' if i + 1 < chars.len() && chars[i + 1] == '/' => {
                        // Skip single-line comment
                        i += 2;
                        while i < chars.len() && chars[i] != '\n' {
                            i += 1;
                        }
                        continue;
                    }
                    '/' if i + 1 < chars.len() && chars[i + 1] == '*' => {
                        // Skip multi-line comment
                        i += 2;
                        while i + 1 < chars.len() && !(chars[i] == '*' && chars[i + 1] == '/') {
                            i += 1;
                        }
                        if i + 1 < chars.len() {
                            i += 2;
                        }
                        continue;
                    }
                    _ => minified.push(ch),
                }
            }
            
            i += 1;
        }
        
        minified
    }
}

impl Default for ChunkReconstructor {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for chunk reconstruction
pub mod utils {
    use super::*;
    
    /// Estimate the size reduction from tree shaking
    pub fn estimate_size_reduction(
        original_chunk: &WebpackChunk,
        optimized_modules: &FxHashMap<ModuleId, WebpackModule>,
    ) -> (usize, usize, f64) {
        let original_size = original_chunk.modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();
        
        let optimized_size = optimized_modules
            .values()
            .map(|m| m.source.len())
            .sum::<usize>();
        
        let reduction_percentage = if original_size > 0 {
            ((original_size - optimized_size) as f64 / original_size as f64) * 100.0
        } else {
            0.0
        };
        
        (original_size, optimized_size, reduction_percentage)
    }
    
    /// Validate that a reconstructed chunk is valid
    pub fn validate_reconstructed_chunk(
        chunk_source: &str,
        expected_module_count: usize,
    ) -> Result<bool> {
        // Basic validation - check if it looks like a valid webpack chunk
        let has_proper_structure = if chunk_source.contains("exports.modules") {
            // CommonJS format
            chunk_source.contains("\"use strict\";") &&
            chunk_source.contains("exports.ids = ") &&
            chunk_source.contains("exports.modules = {")
        } else if chunk_source.contains("webpackChunk") {
            // JSONP format
            chunk_source.contains(".push([") &&
            chunk_source.contains("]);")
        } else {
            false
        };
        
        if !has_proper_structure {
            return Err(TreeShakingError::validation_failed(
                "Reconstructed chunk doesn't have proper webpack structure"
            ));
        }
        
        // Count modules (approximate)
        let module_count = chunk_source.matches("function(").count();
        if module_count != expected_module_count {
            return Err(TreeShakingError::validation_failed(
                format!("Expected {} modules, found {}", expected_module_count, module_count)
            ));
        }
        
        Ok(true)
    }
    
    /// Extract module IDs from a reconstructed chunk (for debugging)
    pub fn extract_module_ids_from_chunk(chunk_source: &str) -> Vec<String> {
        let mut module_ids = Vec::new();
        
        // Use regex to find module IDs
        let module_pattern = regex::Regex::new(r#""([^"]+)"\s*:"#).unwrap();
        
        for cap in module_pattern.captures_iter(chunk_source) {
            if let Some(module_id) = cap.get(1) {
                module_ids.push(module_id.as_str().to_string());
            }
        }
        
        module_ids
    }
}