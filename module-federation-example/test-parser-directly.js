#!/usr/bin/env node

const fs = require('fs');

console.log('🧪 Testing parser directly on real-world chunk');

// Read the real-world chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

// Let's examine the actual format more carefully
console.log('Chunk size:', chunk.length);

// Look for the exports.modules pattern
const exportsIndex = chunk.indexOf('exports.modules = {');
if (exportsIndex !== -1) {
  console.log('Found exports.modules at position:', exportsIndex);
  
  // Extract a small sample of the modules section
  const modulesSample = chunk.substring(exportsIndex + 19, exportsIndex + 2000);
  console.log('\nFirst 1000 chars of modules section:');
  console.log(modulesSample.substring(0, 1000));
  
  // Look for module patterns with the exact real-world format
  // The real format appears to be: "module_id": \n/*!...*/\n(function...
  const realWorldPattern = /"([^"]+)":\s*\n\/\*![^*]*\*+(?:[^/*][^*]*\*+)*\/\s*\n\(function/g;
  const matches = [...modulesSample.matchAll(realWorldPattern)];
  
  console.log('\nReal-world pattern matches found:', matches.length);
  if (matches.length > 0) {
    console.log('Sample matches:');
    matches.slice(0, 3).forEach((match, i) => {
      console.log(`${i + 1}. ${match[1]}`);
    });
  }
  
  // Also try a simpler pattern that might match
  const simplePattern = /"([^"]+)":\s*\n/g;
  const simpleMatches = [...modulesSample.matchAll(simplePattern)];
  console.log('\nSimple pattern matches:', simpleMatches.length);
  
  if (simpleMatches.length > 0) {
    console.log('Sample simple matches:');
    simpleMatches.slice(0, 5).forEach((match, i) => {
      console.log(`${i + 1}. ${match[1]}`);
    });
  }
  
  // Check if there are lodash-es specific modules
  const lodashModules = simpleMatches.filter(match => match[1].includes('lodash-es'));
  console.log('\nLodash-es modules found:', lodashModules.length);
  
  if (lodashModules.length > 0) {
    console.log('Sample lodash-es modules:');
    lodashModules.slice(0, 5).forEach((match, i) => {
      console.log(`${i + 1}. ${match[1]}`);
    });
  }
}

// The issue might be that the parser expects a specific format
// Let's check if the issue is in the dependency analysis
console.log('\n🔍 Checking dependency analysis...');

// Look for the main lodash.js module
const lodashMainIndex = chunk.indexOf('lodash-es/lodash.js');
if (lodashMainIndex !== -1) {
  console.log('Found main lodash.js module');
  
  // Extract the section around the main module
  const mainModuleSection = chunk.substring(lodashMainIndex - 500, lodashMainIndex + 5000);
  
  // Look for export definitions
  const exportDefs = mainModuleSection.match(/(\w+):\s*\(\)\s*=>\s*\([^)]*\)/g);
  if (exportDefs) {
    console.log('Export definitions found:', exportDefs.length);
    
    // Look for macro conditions in these exports
    const macroExports = exportDefs.filter(exp => exp.includes('@common:if'));
    console.log('Exports with macro conditions:', macroExports.length);
    
    if (macroExports.length > 0) {
      console.log('Sample macro exports:');
      macroExports.slice(0, 3).forEach((exp, i) => {
        console.log(`${i + 1}. ${exp.substring(0, 100)}...`);
      });
    }
  }
  
  // Look for require statements in the main module
  const requireCalls = mainModuleSection.match(/__webpack_require__\([^)]+\)/g);
  if (requireCalls) {
    console.log('Require calls found:', requireCalls.length);
    
    // Look for macro-conditional requires
    const macroRequires = requireCalls.filter(req => {
      const index = mainModuleSection.indexOf(req);
      const beforeReq = mainModuleSection.substring(Math.max(0, index - 100), index);
      return beforeReq.includes('@common:if');
    });
    console.log('Macro-conditional requires:', macroRequires.length);
  }
}

console.log('\n💡 HYPOTHESIS:');
console.log('The issue might be that:');
console.log('1. The parser is correctly parsing the chunk');
console.log('2. But the dependency analysis is not correctly identifying orphaned modules');
console.log('3. This could be because all modules are being considered reachable');
console.log('4. Or the main lodash.js module is including too many dependencies');
console.log('\nNext step: Check if the enhanced parser is correctly analyzing dependencies');