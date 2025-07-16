import fs from 'fs';
import path from 'path';
import { optimize } from 'swc_macro_wasm';

/**
 * Analyzes a webpack chunk and returns metrics
 */
export function analyzeChunk(chunkPath) {
  const content = fs.readFileSync(chunkPath, 'utf-8');
  const size = fs.statSync(chunkPath).size;
  
  // Count modules
  const moduleCount = (content.match(/".+\.js":\s*function/g) || []).length;
  
  // Find exports
  const exports = [];
  const exportMatches = content.matchAll(/exports\.(\w+)\s*=/g);
  for (const match of exportMatches) {
    exports.push(match[1]);
  }
  
  // Check format
  const isCommonJS = content.includes('exports.modules');
  const isWebpackBundle = content.includes('__webpack_modules__');
  
  return {
    size,
    sizeKB: (size / 1024).toFixed(2),
    moduleCount,
    exports,
    format: isCommonJS ? 'commonjs' : isWebpackBundle ? 'webpack' : 'unknown'
  };
}

/**
 * Converts share-usage.json to optimization config
 */
export function shareUsageToConfig(shareUsagePath) {
  const shareUsage = JSON.parse(fs.readFileSync(shareUsagePath, 'utf-8'));
  const config = {
    treeShake: {},
    entryModules: {}
  };
  
  Object.entries(shareUsage.consume_shared_modules).forEach(([lib, usage]) => {
    const treeShake = {};
    
    // Mark used exports as true
    usage.used_exports.forEach(exp => {
      treeShake[exp] = true;
    });
    
    // Mark unused exports as false
    usage.unused_exports.forEach(exp => {
      treeShake[exp] = false;
    });
    
    config.treeShake[lib] = treeShake;
    config.entryModules[lib] = usage.entry_module_id;
  });
  
  return config;
}

/**
 * Optimizes a chunk with the given config
 */
export function optimizeChunk(chunkPath, config) {
  const content = fs.readFileSync(chunkPath, 'utf-8');
  const configStr = typeof config === 'string' ? config : JSON.stringify(config);
  
  return optimize(content, configStr);
}

/**
 * Compares two chunks and returns metrics
 */
export function compareChunks(originalPath, optimizedPath) {
  const original = analyzeChunk(originalPath);
  const optimized = analyzeChunk(optimizedPath);
  
  const reduction = ((original.size - optimized.size) / original.size) * 100;
  const moduleReduction = ((original.moduleCount - optimized.moduleCount) / original.moduleCount) * 100;
  
  return {
    original,
    optimized,
    reduction: reduction.toFixed(2),
    moduleReduction: moduleReduction.toFixed(2),
    sizeSaved: original.size - optimized.size,
    sizeSavedKB: ((original.size - optimized.size) / 1024).toFixed(2)
  };
}

/**
 * Merges usage data from multiple apps
 */
export function mergeUsageData(...shareUsagePaths) {
  const mergedUsage = {};
  
  shareUsagePaths.forEach(usagePath => {
    const usage = JSON.parse(fs.readFileSync(usagePath, 'utf-8'));
    
    Object.entries(usage.consume_shared_modules).forEach(([lib, data]) => {
      if (!mergedUsage[lib]) {
        mergedUsage[lib] = {
          used_exports: new Set(),
          unused_exports: new Set(data.unused_exports),
          entry_module_id: data.entry_module_id
        };
      }
      
      // Add used exports
      data.used_exports.forEach(exp => {
        mergedUsage[lib].used_exports.add(exp);
        mergedUsage[lib].unused_exports.delete(exp);
      });
    });
  });
  
  // Convert sets back to arrays
  Object.keys(mergedUsage).forEach(lib => {
    mergedUsage[lib].used_exports = Array.from(mergedUsage[lib].used_exports);
    mergedUsage[lib].unused_exports = Array.from(mergedUsage[lib].unused_exports);
  });
  
  return mergedUsage;
}