#!/usr/bin/env node

const fs = require('fs');

console.log('🔍 Debugging why tree shaking isn\'t running on real-world chunk');

// Read the original lodash chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

console.log('Chunk size:', chunk.length);

// Check for webpack bundle patterns
const hasWebpackModules = chunk.includes('__webpack_modules__');
const hasExportsModules = chunk.includes('exports.modules');
const hasWebpackRequire = chunk.includes('__webpack_require__');

console.log('\nWebpack bundle detection:');
console.log('- Has __webpack_modules__:', hasWebpackModules);
console.log('- Has exports.modules:', hasExportsModules);
console.log('- Has __webpack_require__:', hasWebpackRequire);

// If it has exports.modules, it should be detected as a split chunk
if (hasExportsModules) {
  console.log('\n✅ Should be detected as CommonJS split chunk');
  
  // Check the structure around exports.modules
  const exportsIndex = chunk.indexOf('exports.modules');
  if (exportsIndex !== -1) {
    const context = chunk.substring(exportsIndex, exportsIndex + 200);
    console.log('exports.modules context:', context);
  }
} else {
  console.log('\n❌ Not detected as CommonJS split chunk');
}

// Check for module patterns
const modulePatterns = chunk.match(/\"[^\"]+\.js\":\s*function/g);
console.log('\nModule pattern count:', modulePatterns ? modulePatterns.length : 0);

if (modulePatterns) {
  console.log('Sample module patterns:');
  modulePatterns.slice(0, 5).forEach(pattern => {
    console.log(' ', pattern);
  });
}

// Check for macro conditions
const macroConditions = chunk.match(/@common:if\s+\[[^\]]+\]/g);
console.log('\nMacro conditions:', macroConditions ? macroConditions.length : 0);

// Let's check if the issue is in our test configuration
// Maybe the tree shaking isn't running because we're not providing all the exports
const allExports = [];
const exportMatches = chunk.match(/\"?(\w+)\"?:\s*\(\)\s*=>\s*\(/g);
if (exportMatches) {
  exportMatches.forEach(match => {
    const exportName = match.match(/\"?(\w+)\"?:/)[1];
    allExports.push(exportName);
  });
}

console.log('\nAll exports found:', allExports.length);
console.log('Sample exports:', allExports.slice(0, 10));

// Create a more complete configuration
const completeConfig = {
  treeShake: {
    'lodash-es': {}
  }
};

// Add all exports as false except the ones we want
allExports.forEach(exportName => {
  completeConfig.treeShake['lodash-es'][exportName] = false;
});

// Enable only the ones we want
completeConfig.treeShake['lodash-es']['default'] = true;
completeConfig.treeShake['lodash-es']['sortBy'] = true;

console.log('\nCreated complete config with', Object.keys(completeConfig.treeShake['lodash-es']).length, 'exports');

// Test this configuration
const optimizer = require('swc_macro_wasm');
console.log('\n🧪 Testing with complete configuration...');

const result = optimizer.optimize(chunk, JSON.stringify(completeConfig));
const reduction = ((chunk.length - result.length) / chunk.length * 100).toFixed(1);

console.log('Result size:', result.length);
console.log('Reduction:', reduction + '%');

// Check if any modules were removed
const originalModules = (chunk.match(/\"[^\"]+\.js\":/g) || []).length;
const optimizedModules = (result.match(/\"[^\"]+\.js\":/g) || []).length;

console.log('Original modules:', originalModules);
console.log('Optimized modules:', optimizedModules);
console.log('Modules removed:', originalModules - optimizedModules);

if (originalModules === optimizedModules) {
  console.log('\n❌ ISSUE: No modules were removed despite comprehensive configuration');
  console.log('   This suggests the tree shaking is not running on this chunk');
  console.log('   Possible reasons:');
  console.log('   1. The chunk format is not being recognized as a webpack bundle');
  console.log('   2. The parser is failing to parse the chunk');
  console.log('   3. The tree shaking logic is not being triggered');
} else {
  console.log('\n✅ SUCCESS: Modules were removed with complete configuration');
}