#!/usr/bin/env node

const fs = require('fs');

console.log('🔍 Debugging AST parsing issue');

// Read the real-world chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

console.log('Chunk size:', chunk.length);

// Check the basic structure
console.log('\n📋 Basic structure check:');
console.log('Has exports.modules:', chunk.includes('exports.modules'));
console.log('Has function declarations:', chunk.includes('function('));

// Check if we can find the exports.modules assignment
const exportsModulesIndex = chunk.indexOf('exports.modules = {');
if (exportsModulesIndex !== -1) {
  console.log('Found exports.modules assignment at position:', exportsModulesIndex);
  
  // Look at the structure around it
  const context = chunk.substring(exportsModulesIndex, exportsModulesIndex + 500);
  console.log('\nContext around exports.modules:');
  console.log(context);
  
  // Try to count the modules manually
  const modulePattern = /\"[^\"]+\.js\":/g;
  const moduleMatches = chunk.match(modulePattern);
  console.log('\nModule count by regex:', moduleMatches ? moduleMatches.length : 0);
  
  if (moduleMatches) {
    console.log('First 5 modules:');
    moduleMatches.slice(0, 5).forEach(match => {
      console.log(' ', match);
    });
  }
} else {
  console.log('❌ exports.modules assignment not found');
}

// Test with a simple chunk to see if AST visitor works
const simpleChunk = `
"use strict";
exports.modules = {
  "test1.js": function() { console.log("test1"); },
  "test2.js": function() { console.log("test2"); }
};
`;

console.log('\n🧪 Testing with simple chunk:');
const simpleConfig = {
  treeShake: {
    'test': {
      'test1': true,
      'test2': false
    }
  }
};

try {
  const optimizer = require('swc_macro_wasm');
  const simpleResult = optimizer.optimize(simpleChunk, JSON.stringify(simpleConfig));
  console.log('Simple chunk result size:', simpleResult.length);
  console.log('Simple chunk modules:', (simpleResult.match(/test\d+\.js/g) || []).length);
  console.log('Simple chunk result:', simpleResult);
} catch (error) {
  console.error('Error with simple chunk:', error.message);
}

// The issue might be that the real-world chunk is too large or has syntax issues
// Let's try with just the first part of the real-world chunk
console.log('\n🔍 Testing with first part of real-world chunk:');
const firstPart = chunk.substring(0, 10000); // First 10KB
console.log('First part size:', firstPart.length);
console.log('First part contains exports.modules:', firstPart.includes('exports.modules'));

try {
  const optimizer = require('swc_macro_wasm');
  const partialResult = optimizer.optimize(firstPart, JSON.stringify({
    treeShake: {
      'lodash-es': {
        'default': true
      }
    }
  }));
  console.log('Partial result size:', partialResult.length);
} catch (error) {
  console.error('Error with partial chunk:', error.message);
}

console.log('\n💡 HYPOTHESIS:');
console.log('The issue might be:');
console.log('1. The real-world chunk is too large for the AST parser');
console.log('2. The chunk has syntax that the SWC parser cannot handle');
console.log('3. The AST visitor is not correctly visiting the exports.modules assignment');
console.log('4. The webpack_graph parser is failing silently');