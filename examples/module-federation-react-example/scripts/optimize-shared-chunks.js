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
function mergeUsageData(files, targetApp) {
  const mergedTreeShake = {};

  // OR merge export usage across all apps
  files.forEach(({ data }) => {
    if (!data.treeShake) return;
    Object.entries(data.treeShake).forEach(([moduleKey, moduleExports]) => {
      if (!mergedTreeShake[moduleKey]) {
        mergedTreeShake[moduleKey] = {};
      }
      Object.entries(moduleExports).forEach(([exportName, isUsed]) => {
        if (exportName === 'chunk_characteristics') return; // skip metadata
        if (mergedTreeShake[moduleKey][exportName] !== true) {
          mergedTreeShake[moduleKey][exportName] = Boolean(isUsed);
        }
      });
    });
  });

  // Populate entry modules
  // - If targetApp is provided, use its chunk_characteristics exclusively
  // - Otherwise, fall back to the first available entry_module_id across apps (prefers order of files)
  const entryModules = {};
  if (targetApp) {
    const target = files.find(f => f.name === targetApp);
    if (target?.data?.treeShake) {
      Object.entries(target.data.treeShake).forEach(([moduleKey, moduleExports]) => {
        const entryId = moduleExports?.chunk_characteristics?.entry_module_id;
        if (entryId) {
          entryModules[moduleKey] = entryId;
        }
      });
    }
  } else {
    files.forEach(({ data }) => {
      if (!data?.treeShake) return;
      Object.entries(data.treeShake).forEach(([moduleKey, moduleExports]) => {
        const entryId = moduleExports?.chunk_characteristics?.entry_module_id;
        if (entryId && !entryModules[moduleKey]) {
          entryModules[moduleKey] = entryId;
        }
      });
    });
  }

  return {
    treeShake: mergedTreeShake,
    entryModules,
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
function findSharedChunks(files) {
  const chunks = [];
  const distDirs = {
    host: path.resolve(__dirname, '../host/dist'),
    remote: path.resolve(__dirname, '../remote/dist')
  };

  files.forEach(({ name: app, data }) => {
    const distDir = distDirs[app];
    if (!distDir || !fs.existsSync(distDir)) return;

    if (!data.treeShake) return;

    Object.entries(data.treeShake).forEach(([library, moduleData]) => {
      const chunkFiles = moduleData?.chunk_characteristics?.chunk_files;
      if (!Array.isArray(chunkFiles)) return;

      chunkFiles.forEach(chunkFile => {
        if (typeof chunkFile !== 'string' || !chunkFile.endsWith('.js')) return;

        const fullPath = path.join(distDir, chunkFile);
        if (!fs.existsSync(fullPath)) return;

        // Skip maps, originals, optimized
        if (chunkFile.endsWith('.map') || chunkFile.endsWith('.original') || chunkFile.endsWith('.optimized.js')) return;

        chunks.push({
          path: fullPath,
          mapPath: fullPath + '.map',
          app,
          filename: chunkFile,
          library
        });
      });
    });
  });

  return chunks;
}

/**
 * Optimize a shared library chunk using SWC macro with tree-shake flags
 */
async function optimizeChunk(chunkPath, library, treeShakeConfig, entryModules, optimizer, chunkCharacteristics) {
  console.log(`Optimizing chunk: ${path.basename(chunkPath)}`);
  
  try {
    // Enforce: require chunk characteristics and entry module for this library
    const entryIdForLibrary = entryModules?.[library];
    const hasChunkCharacteristics = Boolean(chunkCharacteristics?.entry_module_id);
    if (!entryIdForLibrary || !hasChunkCharacteristics) {
      console.log(
        `Skipping ${path.basename(chunkPath)} for '${library}' - missing ${
          !entryIdForLibrary ? 'entry module id' : 'chunk characteristics'
        }`
      );
      return null;
    }

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
    
    // Attach per-app chunk characteristics for this library only
    const treeShakeWithMeta = { ...treeShakeConfig };
    if (chunkCharacteristics) {
      treeShakeWithMeta[library] = {
        ...(treeShakeWithMeta[library] || {}),
        chunk_characteristics: chunkCharacteristics
      };
    }

    const config = {
      treeShake: treeShakeWithMeta,  // Pass the full treeShake config
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
    
    // Precompute merged tree-shake flags once (OR across apps)
    console.log('Merging usage data (exports only)...');
    const mergedFlags = mergeUsageData(files).treeShake;
    console.log(`✅ Prepared merged export flags for modules: ${Object.keys(mergedFlags).join(', ')}\n`);
    
    // Find shared library chunks
    console.log('Finding shared library chunks...');
    const chunks = findSharedChunks(files);

    // Group by unique path and collect libraries per chunk
    const uniqueChunks = new Map();
    chunks.forEach(chunk => {
      const key = chunk.path;
      if (!uniqueChunks.has(key)) {
        uniqueChunks.set(key, { ...chunk, libraries: [] });
      }
      uniqueChunks.get(key).libraries.push(chunk.library);
    });
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
    
    // Optimize each unique chunk
    console.log('Optimizing chunks...');
    const results = [];
    
    for (const uniqueChunk of uniqueChunks.values()) {
      // Build per-app config dynamically: OR-merged exports + app-specific entry modules
      const { entryModules } = mergeUsageData(files, uniqueChunk.app);
      
      // Collect chunkCharacteristics - use first library's as representative
      const appData = files.find(f => f.name === uniqueChunk.app)?.data;
      const firstLibrary = uniqueChunk.libraries[0];
      const chunkCharacteristics = appData?.treeShake?.[firstLibrary]?.chunk_characteristics;

      // Merge treeShake configs for all libraries in this chunk
      const mergedLibraryConfig = {};
      uniqueChunk.libraries.forEach(lib => {
        if (mergedFlags[lib]) {
          Object.entries(mergedFlags[lib]).forEach(([exportName, shouldKeep]) => {
            if (shouldKeep) {
              mergedLibraryConfig[exportName] = true;
            }
          });
        }
      });

      // For treeShakeWithMeta, include all libraries' flags
      const treeShakeWithMeta = { ...mergedFlags };
      if (chunkCharacteristics) {
        // Attach to first library
        treeShakeWithMeta[firstLibrary] = {
          ...(treeShakeWithMeta[firstLibrary] || {}),
          chunk_characteristics: chunkCharacteristics
        };
      }

      const result = await optimizeChunk(
        uniqueChunk.path,
        uniqueChunk.libraries.join(', '),  // For logging
        treeShakeWithMeta,
        entryModules,
        optimizer,
        chunkCharacteristics
      );
      if (result) {
        results.push({
          app: uniqueChunk.app,
          filename: uniqueChunk.filename,
          library: uniqueChunk.libraries.join(', '),
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