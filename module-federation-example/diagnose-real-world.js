#!/usr/bin/env node

const optimizer = require('swc_macro_wasm');
const fs = require('fs');

console.log('🔍 Diagnosing real-world lodash chunk module structure');

// Read the original lodash chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

// Look for the lodash.js module structure
const lodashModuleStart = chunk.indexOf('lodash-es/lodash.js');
if (lodashModuleStart !== -1) {
  console.log('Found lodash.js module at position:', lodashModuleStart);
  
  // Get the section around lodash.js
  const section = chunk.substring(lodashModuleStart - 500, lodashModuleStart + 3000);
  console.log('\nLodash.js module section (first 1000 chars):');
  console.log(section.substring(0, 1000));
  
  // Look for export definitions
  const exportDefs = section.match(/\w+:\s*\(\)\s*=>\s*\([^)]+\)/g);
  if (exportDefs) {
    console.log('\nExport definitions found:', exportDefs.length);
    console.log('Sample exports:', exportDefs.slice(0, 5));
  }
  
  // Look for require statements
  const requireCalls = section.match(/__webpack_require__\([^)]+\)/g);
  if (requireCalls) {
    console.log('\nWebpack require calls found:', requireCalls.length);
    console.log('Sample requires:', requireCalls.slice(0, 5));
  }
}

// Check if there are any macro conditions
const macroConditions = chunk.match(/@common:if\s+\[[^\]]+\]/g);
console.log('\nMacro conditions found:', macroConditions ? macroConditions.length : 0);

if (macroConditions) {
  console.log('Sample macro conditions:', macroConditions.slice(0, 3));
}

// Check if the macro conditions are being processed correctly
const enabledOnly = {
  treeShake: {
    'lodash-es': {
      'default': true,
      'sortBy': true,
      // Everything else false
    }
  }
};

// First, let's add verbose logging by creating a wrapper optimizer
let verboseOptimizer = {
  optimize: function(code, config) {
    console.log('\n🔧 Starting optimization with config:', JSON.parse(config));
    
    const result = optimizer.optimize(code, config);
    
    console.log('\n📊 Optimization complete');
    console.log('Input size:', code.length);
    console.log('Output size:', result.length);
    console.log('Reduction:', ((code.length - result.length) / code.length * 100).toFixed(1) + '%');
    
    return result;
  }
};

console.log('\n🧪 Testing with sortBy + default enabled...');
const optimized = verboseOptimizer.optimize(chunk, JSON.stringify(enabledOnly));

// Check if the optimization is working at all
const originalExports = chunk.match(/\w+:\s*\(\)\s*=>\s*\([^)]+\)/g);
const optimizedExports = optimized.match(/\w+:\s*\(\)\s*=>\s*\([^)]+\)/g);

console.log('\nOriginal exports found:', originalExports ? originalExports.length : 0);
console.log('Optimized exports found:', optimizedExports ? optimizedExports.length : 0);

// Check if any exports are being nullified
const nullExports = optimized.match(/\w+:\s*\(\)\s*=>\s*null/g);
console.log('Null exports found:', nullExports ? nullExports.length : 0);

// Check if any modules are being removed
const originalModuleCount = (chunk.match(/\"[^\"]+\.js\":/g) || []).length;
const optimizedModuleCount = (optimized.match(/\"[^\"]+\.js\":/g) || []).length;

console.log('\nModule analysis:');
console.log('Original modules:', originalModuleCount);
console.log('Optimized modules:', optimizedModuleCount);
console.log('Modules removed:', originalModuleCount - optimizedModuleCount);

// Check if any __webpack_require__ calls are removed
const originalRequires = (chunk.match(/__webpack_require__\([^)]+\)/g) || []).length;
const optimizedRequires = (optimized.match(/__webpack_require__\([^)]+\)/g) || []).length;

console.log('\nRequire call analysis:');
console.log('Original requires:', originalRequires);
console.log('Optimized requires:', optimizedRequires);
console.log('Requires removed:', originalRequires - optimizedRequires);

// If exports are being nullified but modules aren't being removed, 
// it means the parser isn't detecting orphaned modules
if (nullExports && nullExports.length > 0 && originalModuleCount === optimizedModuleCount) {
  console.log('\n⚠️  ISSUE DETECTED:');
  console.log('   - Exports are being nullified (macro processing working)');
  console.log('   - But modules are not being removed (orphaned detection not working)');
  console.log('   - This suggests the enhanced parser may not be detecting orphaned modules correctly');
  console.log('   - Need to investigate the dependency graph construction');
}

// Let's also check if the CommonJS format is being handled
const hasExportsModules = optimized.includes('exports.modules');
console.log('\nCommonJS format detected:', hasExportsModules);

if (hasExportsModules) {
  console.log('✅ CommonJS format is being processed');
} else {
  console.log('❌ CommonJS format may not be detected correctly');
}