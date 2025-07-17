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
 */
function mergeUsageData(files) {
  const combined = {};
  const entryModules = {};
  
  files.forEach(({ name, data }) => {
    if (!data.consume_shared_modules) return;
    
    Object.entries(data.consume_shared_modules).forEach(([moduleKey, moduleData]) => {
      if (!combined[moduleKey]) {
        combined[moduleKey] = {
          used_exports: new Set(),
          unused_exports: new Set(moduleData.unused_exports || []),
          apps: []
        };
      }
      
      // Add used exports from this app
      (moduleData.used_exports || []).forEach(exportName => {
        combined[moduleKey].used_exports.add(exportName);
        combined[moduleKey].unused_exports.delete(exportName);
      });
      
      // Store entry module ID if present
      if (moduleData.entry_module_id) {
        entryModules[moduleKey] = moduleData.entry_module_id;
      }
      
      combined[moduleKey].apps.push(name);
    });
  });
  
  // Convert to tree-shake format
  const treeShakeConfig = {};
  
  Object.entries(combined).forEach(([moduleKey, data]) => {
    treeShakeConfig[moduleKey] = {};
    
    // Mark used exports as true (keep), unused as false (remove)
    data.used_exports.forEach(exportName => {
      treeShakeConfig[moduleKey][exportName] = true;
    });
    
    data.unused_exports.forEach(exportName => {
      treeShakeConfig[moduleKey][exportName] = false;
    });
  });
  
  return {
    treeShake: treeShakeConfig,
    entryModules: entryModules,
    metadata: {
      timestamp: new Date().toISOString(),
      apps: files.map(f => f.name),
      modules: Object.keys(treeShakeConfig)
    }
  };
}

/**
 * Find lodash chunk files in dist directories
 */
function findLodashChunks() {
  const chunks = [];
  const distDirs = ['../host/dist', '../remote/dist'];
  
  distDirs.forEach(distDir => {
    const fullPath = path.resolve(__dirname, distDir);
    if (!fs.existsSync(fullPath)) return;
    
    const files = fs.readdirSync(fullPath);
    files.forEach(file => {
      if (file.includes('lodash-es') && file.endsWith('.js') && !file.endsWith('.map')) {
        chunks.push({
          path: path.join(fullPath, file),
          mapPath: path.join(fullPath, file + '.map'),
          app: distDir.includes('host') ? 'host' : 'remote',
          filename: file
        });
      }
    });
  });
  
  return chunks;
}

/**
 * Optimize a lodash chunk using SWC macro with tree-shake flags
 */
async function optimizeChunk(chunkPath, treeShakeConfig, entryModules, optimizer) {
  console.log(`Optimizing chunk: ${path.basename(chunkPath)}`);
  
  try {
    const sourceCode = fs.readFileSync(chunkPath, 'utf8');
    
    // Create optimization config for lodash-es
    const config = {
      treeShake: {
        'lodash-es': treeShakeConfig['lodash-es'] || {}
      },
      entryModules: entryModules || {}
    };
    
    const configJson = JSON.stringify(config);
    console.log(`Tree-shake config for lodash-es:`, Object.keys(config.treeShake['lodash-es']).length, 'exports configured');
    console.log(`Entry modules:`, JSON.stringify(config.entryModules, null, 2));
    console.log('Config JSON preview:', configJson.substring(0, 200) + '...');
    console.log('Sample flags:', JSON.stringify(Object.fromEntries(Object.entries(config.treeShake['lodash-es']).slice(0, 5)), null, 2));
    
    // Run SWC macro optimization
    const result = optimizer.optimize(sourceCode, configJson);
    
    if (result && typeof result === 'string' && result !== sourceCode) {
      // Backup original
      fs.writeFileSync(chunkPath + '.original', sourceCode);
      
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
    
    // Find lodash chunks
    console.log('Finding lodash chunks...');
    const chunks = findLodashChunks();
    console.log(`✅ Found ${chunks.length} lodash chunks\n`);
    
    if (chunks.length === 0) {
      console.log('No lodash chunks found to optimize.');
      return;
    }
    
    // Optimize each chunk
    console.log('Optimizing chunks...');
    const results = [];
    
    for (const chunk of chunks) {
      const result = await optimizeChunk(chunk.path, mergedConfig.treeShake, mergedConfig.entryModules, optimizer);
      if (result) {
        results.push({
          app: chunk.app,
          filename: chunk.filename,
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
      
      results.forEach(result => {
        console.log(`${result.app}/${result.filename}:`);
        console.log(`  Size reduction: ${result.reduction.toFixed(2)}% (${result.original_size} → ${result.optimized_size} bytes)`);
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