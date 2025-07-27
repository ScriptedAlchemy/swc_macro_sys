#!/usr/bin/env node

import fs from 'fs';
import path from 'path';
import { fileURLToPath } from 'url';

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

// Import the SWC macro WASM optimizer
async function loadOptimizer() {
  try {
    // Load from workspace dependency (requires --experimental-wasm-modules flag)
    const swcMacro = await import('swc_macro_wasm');
    console.log('Available functions:', Object.keys(swcMacro).filter(k => typeof swcMacro[k] === 'function'));
    return swcMacro;
  } catch (error) {
    console.error('Failed to load SWC macro optimizer:', error.message);
    console.log('Please ensure:');
    console.log('1. WASM package is built: cd crates/swc_macro_wasm && wasm-pack build --release');
    console.log('2. Script is run with: node --experimental-wasm-modules');
    process.exit(1);
  }
}

/**
 * Read and parse share-usage.json files from dist directories
 */
function readShareUsageFiles() {
  const hostJsonPath = path.resolve(__dirname, '../host/dist/share-usage.json');
  const remoteJsonPath = path.resolve(__dirname, '../remote/dist/share-usage.json');
  
  const files = [];
  
  if (fs.existsSync(hostJsonPath)) {
    const hostData = JSON.parse(fs.readFileSync(hostJsonPath, 'utf8'));
    files.push({ name: 'host', data: hostData, path: hostJsonPath });
  }
  
  if (fs.existsSync(remoteJsonPath)) {
    const remoteData = JSON.parse(fs.readFileSync(remoteJsonPath, 'utf8'));
    files.push({ name: 'remote', data: remoteData, path: remoteJsonPath });
  }
  
  if (files.length === 0) {
    throw new Error('No share-usage.json files found. Please run build first.');
  }
  
  return files;
}

/**
 * Merge usage data from multiple apps into combined tree-shake flags
 * The new schema already contains treeShake structure with boolean values
 */
function mergeUsageData(files) {
  const mergedTreeShake = {};
  const entryModules = {};
  
  files.forEach(({ name, data }) => {
    // Handle new structure where treeShake is at top level
    if (data.treeShake) {
      Object.entries(data.treeShake).forEach(([moduleKey, moduleExports]) => {
        if (!mergedTreeShake[moduleKey]) {
          mergedTreeShake[moduleKey] = {};
        }
        
        // Merge export usage - if any app uses an export, mark it as true
        Object.entries(moduleExports).forEach(([exportName, isUsed]) => {
          // Skip chunk_characteristics metadata
          if (exportName === 'chunk_characteristics') {
            // Extract entry module ID if present
            if (moduleExports.chunk_characteristics?.entry_module_id) {
              entryModules[moduleKey] = moduleExports.chunk_characteristics.entry_module_id;
            }
            return;
          }
          
          // If this export is already marked as used, keep it as true
          // Otherwise, use the current app's value
          if (mergedTreeShake[moduleKey][exportName] !== true) {
            mergedTreeShake[moduleKey][exportName] = isUsed;
          }
        });
      });
    }
  });
  
  return {
    treeShake: mergedTreeShake,
    entryModules: entryModules,
    metadata: {
      timestamp: new Date().toISOString(),
      apps: files.map(f => f.name),
      modules: Object.keys(mergedTreeShake)
    }
  };
}

/**
 * Find shared library chunk files in dist directories
 */
function findSharedChunks() {
  const chunks = [];
  const distDirs = ['../host/dist', '../remote/dist'];
  const sharedLibraries = ['lodash-es', 'ramda', 'date-fns'];
  
  distDirs.forEach(distDir => {
    const fullPath = path.resolve(__dirname, distDir);
    if (!fs.existsSync(fullPath)) return;
    
    const files = fs.readdirSync(fullPath);
    files.forEach(file => {
      // Check if file contains any of our shared libraries
      const matchedLibrary = sharedLibraries.find(lib => file.includes(lib));
      if (matchedLibrary && file.endsWith('.js') && !file.endsWith('.map') && !file.endsWith('.original') && !file.endsWith('.optimized.js')) {
        chunks.push({
          path: path.join(fullPath, file),
          mapPath: path.join(fullPath, file + '.map'),
          app: distDir.includes('host') ? 'host' : 'remote',
          filename: file,
          library: matchedLibrary
        });
      }
    });
  });
  
  return chunks;
}

/**
 * Optimize a shared library chunk using SWC macro with tree-shake flags
 */
async function optimizeChunk(chunkPath, library, treeShakeConfig, entryModules, optimizer) {
  console.log(`Optimizing chunk: ${path.basename(chunkPath)}`);
  
  try {
    const sourceCode = fs.readFileSync(chunkPath, 'utf8');
    
    // Create optimization config for the library - only include exports marked as true
    const libraryConfig = {};
    if (treeShakeConfig[library]) {
      Object.entries(treeShakeConfig[library]).forEach(([exportName, shouldKeep]) => {
        if (shouldKeep === true) {
          libraryConfig[exportName] = true;
        }
      });
    }
    
    const config = {
      treeShake: treeShakeConfig,  // Pass the full treeShake config
      entryModules: entryModules || {}
    };
    
    const configJson = JSON.stringify(config);
    console.log(`Tree-shake config for ${library}:`, Object.keys(libraryConfig).length, 'exports to keep');
    console.log(`Entry modules:`, JSON.stringify(config.entryModules, null, 2));
    console.log('Exports to keep:', Object.keys(libraryConfig).join(', '));
    
    // Run SWC macro optimization
    const result = optimizer.optimize(sourceCode, configJson);
    
    if (result && typeof result === 'string' && result !== sourceCode) {
      // Backup original
      if (!fs.existsSync(chunkPath + '.original')) {
        fs.writeFileSync(chunkPath + '.original', sourceCode);
      }
      
      // Write optimized code
      fs.writeFileSync(chunkPath, result);
      
      console.log(`✅ Optimized ${path.basename(chunkPath)}`);
      console.log(`   Original size: ${sourceCode.length} bytes`);
      console.log(`   Optimized size: ${result.length} bytes`);
      console.log(`   Size reduction: ${((sourceCode.length - result.length) / sourceCode.length * 100).toFixed(2)}%`);
      
      return {
        original_size: sourceCode.length,
        optimized_size: result.length,
        reduction: ((sourceCode.length - result.length) / sourceCode.length * 100)
      };
    } else {
      console.log(`ℹ️  No optimization applied for ${path.basename(chunkPath)}`);
      console.log(`   Result type: ${typeof result}`);
      console.log(`   Result equals source: ${result === sourceCode}`);
      return null;
    }
  } catch (error) {
    console.error(`❌ Failed to optimize ${path.basename(chunkPath)}:`, error.message);
    return null;
  }
}

/**
 * Main optimization function
 */
async function main() {
  console.log('🚀 Starting Module Federation chunk optimization...\n');
  
  try {
    // Load SWC macro optimizer
    console.log('Loading SWC macro optimizer...');
    const optimizer = await loadOptimizer();
    console.log('✅ SWC macro optimizer loaded\n');
    
    // Read share usage files
    console.log('Reading share-usage.json files...');
    const files = readShareUsageFiles();
    console.log(`✅ Found ${files.length} share-usage files: ${files.map(f => f.name).join(', ')}\n`);
    
    // Merge usage data
    console.log('Merging usage data...');
    const mergedConfig = mergeUsageData(files);
    console.log(`✅ Merged usage data for modules: ${mergedConfig.metadata.modules.join(', ')}\n`);
    
    // Save merged config for reference
    const configPath = path.resolve(__dirname, '../dist/merged-tree-shake-config.json');
    fs.mkdirSync(path.dirname(configPath), { recursive: true });
    fs.writeFileSync(configPath, JSON.stringify(mergedConfig, null, 2));
    console.log(`✅ Saved merged config to: ${path.relative(process.cwd(), configPath)}\n`);
    
    // Find shared library chunks
    console.log('Finding shared library chunks...');
    const chunks = findSharedChunks();
    console.log(`✅ Found ${chunks.length} shared library chunks\n`);
    
    if (chunks.length === 0) {
      console.log('No shared library chunks found to optimize.');
      return;
    }
    
    // Group chunks by library
    const chunksByLibrary = chunks.reduce((acc, chunk) => {
      if (!acc[chunk.library]) acc[chunk.library] = [];
      acc[chunk.library].push(chunk);
      return acc;
    }, {});
    
    console.log('Chunks by library:');
    Object.entries(chunksByLibrary).forEach(([lib, libChunks]) => {
      console.log(`  ${lib}: ${libChunks.length} chunks`);
    });
    console.log();
    
    // Optimize each chunk
    console.log('Optimizing chunks...');
    const results = [];
    
    for (const chunk of chunks) {
      const result = await optimizeChunk(chunk.path, chunk.library, mergedConfig.treeShake, mergedConfig.entryModules, optimizer);
      if (result) {
        results.push({
          app: chunk.app,
          filename: chunk.filename,
          library: chunk.library,
          ...result
        });
      }
    }
    
    // Summary
    console.log('\n📊 Optimization Summary:');
    console.log('========================');
    
    if (results.length > 0) {
      const totalOriginal = results.reduce((sum, r) => sum + r.original_size, 0);
      const totalOptimized = results.reduce((sum, r) => sum + r.optimized_size, 0);
      const totalReduction = ((totalOriginal - totalOptimized) / totalOriginal * 100);
      
      // Group results by library for better display
      const resultsByLibrary = results.reduce((acc, result) => {
        if (!acc[result.library]) acc[result.library] = [];
        acc[result.library].push(result);
        return acc;
      }, {});
      
      Object.entries(resultsByLibrary).forEach(([library, libResults]) => {
        console.log(`\n${library}:`);
        libResults.forEach(result => {
          console.log(`  ${result.app}/${result.filename}:`);
          console.log(`    Size reduction: ${result.reduction.toFixed(2)}% (${result.original_size.toLocaleString()} → ${result.optimized_size.toLocaleString()} bytes)`);
        });
      });
      
      console.log(`\nTotal size reduction: ${totalReduction.toFixed(2)}% (${totalOriginal} → ${totalOptimized} bytes)`);
      console.log(`Total bytes saved: ${totalOriginal - totalOptimized}`);
    } else {
      console.log('No chunks were optimized.');
    }
    
    console.log('\n✅ Module Federation optimization complete!');
    
  } catch (error) {
    console.error('❌ Optimization failed:', error.message);
    process.exit(1);
  }
}

// Run if called directly
if (import.meta.url === `file://${process.argv[1]}`) {
  main().catch(console.error);
}

export { main, mergeUsageData, readShareUsageFiles };