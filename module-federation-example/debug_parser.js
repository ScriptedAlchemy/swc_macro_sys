#!/usr/bin/env node

const optimizer = require('swc_macro_wasm');
const fs = require('fs');

console.log('🔍 DEBUGGING THE ENHANCED PARSER ISSUE');
console.log('='.repeat(50));

// Let's examine the exact structure of the lodash chunk
const chunk = fs.readFileSync('host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original', 'utf8');

// Look at the main lodash.js module specifically
const lodashModuleRegex = /"[^"]*lodash\.js":\s*function[^}]+\}/g;
const lodashModuleMatch = chunk.match(lodashModuleRegex);

if (lodashModuleMatch) {
  console.log('Found lodash.js module, first 500 chars:');
  console.log(lodashModuleMatch[0].substring(0, 500));
} else {
  console.log('❌ No lodash.js module found');
}

// Let's look for the exact export pattern in lodash.js
const lodashJsStart = chunk.indexOf('lodash.js');
if (lodashJsStart !== -1) {
  const lodashSection = chunk.substring(lodashJsStart, lodashJsStart + 2000);
  console.log('\nLodash.js section (first 1000 chars):');
  console.log(lodashSection.substring(0, 1000));
  
  // Look for the specific export patterns
  const exportPatterns = lodashSection.match(/\w+: \(\) => \([^)]+\)/g);
  if (exportPatterns) {
    console.log('\nExport patterns found:', exportPatterns.length);
    console.log('First 5 patterns:', exportPatterns.slice(0, 5));
  }
}

// Check if the macro conditions are properly present
const macroCount = (chunk.match(/@common:if/g) || []).length;
console.log('\nMacro conditions found:', macroCount);

// Test with a minimal webpack_require detection
const requireMatches = chunk.match(/__webpack_require__\([^)]+\)/g);
console.log('Direct webpack_require calls found:', requireMatches ? requireMatches.length : 0);

// Check if the issue is in the CommonJS format parsing
const isCommonJS = chunk.includes('exports.modules');
console.log('Is CommonJS format:', isCommonJS);

if (isCommonJS) {
  // For CommonJS, modules are inside exports.modules = { ... }
  const exportsModulesMatch = chunk.match(/exports\.modules\s*=\s*\{([\s\S]*?)\}\s*;/);
  if (exportsModulesMatch) {
    console.log('\nCommonJS exports.modules structure detected');
    console.log('Modules section length:', exportsModulesMatch[1].length);
    
    // Count modules in the CommonJS format
    const cjsModules = exportsModulesMatch[1].match(/"[^"]+":\s*function/g);
    console.log('CJS modules found:', cjsModules ? cjsModules.length : 0);
  }
}

console.log('\n🎯 ISSUE DIAGNOSIS:');
console.log('The enhanced parser may not be correctly handling the CommonJS split chunk format.');
console.log('Need to investigate why orphaned modules are not being detected.');

// Let's test with a more aggressive config to see if ANY modules get removed
console.log('\n🧪 TESTING AGGRESSIVE CONFIGURATION:');
const aggressiveConfig = {
  treeShake: {
    'lodash-es': {
      'default': true,
      // Disable EVERYTHING else - be very explicit
      'sortBy': false,
      'map': false,
      'filter': false,
      'reduce': false,
      'find': false,
      'forEach': false,
      'groupBy': false,
      'debounce': false,
      'throttle': false,
      'cloneDeep': false,
      'merge': false,
      'pick': false,
      'omit': false,
      'capitalize': false,
      'flatten': false,
      'isEmpty': false,
      'isArray': false,
      'isObject': false,
      'isString': false,
      'isNumber': false,
      'isFunction': false,
      'chunk': false,
      'compact': false,
      'concat': false,
      'difference': false,
      'drop': false,
      'fill': false,
      'first': false,
      'head': false,
      'indexOf': false,
      'initial': false,
      'intersection': false,
      'join': false,
      'last': false,
      'lastIndexOf': false,
      'nth': false,
      'pull': false,
      'remove': false,
      'reverse': false,
      'slice': false,
      'tail': false,
      'take': false,
      'union': false,
      'uniq': false,
      'without': false,
      'zip': false,
      'zipObject': false,
      'zipWith': false,
      'add': false,
      'ceil': false,
      'divide': false,
      'floor': false,
      'max': false,
      'min': false,
      'multiply': false,
      'round': false,
      'subtract': false,
      'sum': false,
      'sumBy': false,
      'mean': false,
      'assign': false,
      'assignIn': false,
      'at': false,
      'create': false,
      'defaults': false,
      'defaultsDeep': false,
      'entries': false,
      'entriesIn': false,
      'extend': false,
      'extendWith': false,
      'findKey': false,
      'findLastKey': false,
      'forIn': false,
      'forInRight': false,
      'forOwn': false,
      'forOwnRight': false,
      'functions': false,
      'functionsIn': false,
      'get': false,
      'has': false,
      'hasIn': false,
      'invert': false,
      'invertBy': false,
      'invoke': false,
      'keys': false,
      'keysIn': false,
      'mapKeys': false,
      'mapValues': false,
      'pickBy': false,
      'result': false,
      'set': false,
      'setWith': false,
      'toPairs': false,
      'toPairsIn': false,
      'transform': false,
      'unset': false,
      'update': false,
      'updateWith': false,
      'values': false,
      'valuesIn': false
    }
  }
};

const aggressiveOptimized = optimizer.optimize(chunk, JSON.stringify(aggressiveConfig));
const aggressiveReduction = ((chunk.length - aggressiveOptimized.length) / chunk.length * 100).toFixed(1);
console.log('Aggressive optimization reduction:', aggressiveReduction, '%');

const aggressiveModules = (aggressiveOptimized.match(/"[^"]+\.js":/g) || []).length;
console.log('Aggressive modules remaining:', aggressiveModules);

// Let's also check if there's any tree shaking happening at all
console.log('\n🔍 CHECKING FOR ANY TREE SHAKING:');
const noTreeShakeConfig = {};
const noTreeShakeOptimized = optimizer.optimize(chunk, JSON.stringify(noTreeShakeConfig));
const noTreeShakeReduction = ((chunk.length - noTreeShakeOptimized.length) / chunk.length * 100).toFixed(1);
console.log('No tree-shake config reduction:', noTreeShakeReduction, '%');

if (parseFloat(noTreeShakeReduction) > 0) {
  console.log('✅ Base optimization is working');
} else {
  console.log('❌ Even base optimization is not working');
}

// Check what's different between original and optimized
console.log('\n🔄 DIFFERENCE ANALYSIS:');
console.log('Original size:', chunk.length);
console.log('Optimized size:', aggressiveOptimized.length);
console.log('Difference:', chunk.length - aggressiveOptimized.length, 'bytes');

// Check if the issue is that the main lodash.js module is keeping everything alive
const lodashJsPattern = /"[^"]*lodash\.js":\s*function[^}]+}/;
const lodashJsInOptimized = aggressiveOptimized.match(lodashJsPattern);
if (lodashJsInOptimized) {
  console.log('\nLodash.js in optimized (first 300 chars):');
  console.log(lodashJsInOptimized[0].substring(0, 300));
}