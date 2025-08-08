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
  // The new schema already has the correct structure, so just return it directly
  return JSON.parse(fs.readFileSync(shareUsagePath, 'utf-8'));
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
  const mergedConfig = {
    treeShake: {}
  };
  
  shareUsagePaths.forEach(usagePath => {
    const usage = JSON.parse(fs.readFileSync(usagePath, 'utf-8'));
    
    Object.entries(usage.treeShake).forEach(([lib, exports]) => {
      if (!mergedConfig.treeShake[lib]) {
        // Clone the exports object for this library
        mergedConfig.treeShake[lib] = { ...exports };
      } else {
        // Merge exports: if any app uses an export, mark it as true
        Object.entries(exports).forEach(([exportName, isUsed]) => {
          if (exportName === 'chunk_characteristics') {
            // Keep the chunk_characteristics from the first occurrence
            if (!mergedConfig.treeShake[lib].chunk_characteristics) {
              mergedConfig.treeShake[lib].chunk_characteristics = isUsed;
            }
          } else if (isUsed === true) {
            // If any app uses this export, mark it as used
            mergedConfig.treeShake[lib][exportName] = true;
          }
        });
      }
    });
  });
  
  return mergedConfig;
}