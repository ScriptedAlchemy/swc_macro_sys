#!/usr/bin/env node

const fs = require('fs');

console.log('🔍 Testing AST parsing on real-world chunk');

// Read the real-world chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

console.log('Chunk size:', chunk.length);

// Test with very minimal config to see if any modules are detected
const minimalConfig = {
  treeShake: {
    'lodash-es': {
      'default': true,
      'sortBy': true
    }
  }
};

console.log('🧪 Testing optimization with minimal config...');

// Capture stderr to see webpack_graph logs
const { spawn } = require('child_process');
const path = require('path');

// Create a temporary script that runs the optimization
const tempScript = `
const optimizer = require('swc_macro_wasm');
const fs = require('fs');

const chunk = fs.readFileSync('${chunkPath}', 'utf8');
const config = ${JSON.stringify(minimalConfig)};

console.log('Running optimization...');
const result = optimizer.optimize(chunk, JSON.stringify(config));
console.log('Result size:', result.length);
console.log('Original modules:', (chunk.match(/\\.js\\":/g) || []).length);
console.log('Optimized modules:', (result.match(/\\.js\\":/g) || []).length);
`;

fs.writeFileSync('/tmp/test-ast-parsing.js', tempScript);

// Run the script and capture output
const child = spawn('node', ['/tmp/test-ast-parsing.js'], {
  stdio: ['pipe', 'pipe', 'pipe'],
  cwd: __dirname
});

let stdout = '';
let stderr = '';

child.stdout.on('data', (data) => {
  stdout += data.toString();
});

child.stderr.on('data', (data) => {
  stderr += data.toString();
});

child.on('close', (code) => {
  console.log('STDOUT:', stdout);
  console.log('\nSTDERR (debug logs):');
  console.log(stderr);
  
  // Analyze the logs
  const astVisitorLines = stderr.split('\n').filter(line => line.includes('AST visitor found'));
  const regexExtractionLines = stderr.split('\n').filter(line => line.includes('Regex extraction found'));
  const webpackGraphLines = stderr.split('\n').filter(line => line.includes('[webpack_graph]'));
  
  console.log('\n📊 ANALYSIS:');
  console.log('AST visitor results:', astVisitorLines.length > 0 ? astVisitorLines[0] : 'No AST visitor results');
  console.log('Regex extraction results:', regexExtractionLines.length > 0 ? regexExtractionLines[0] : 'No regex extraction results');
  
  console.log('\nAll webpack_graph logs:');
  webpackGraphLines.forEach(line => console.log(' ', line));
  
  // Check if any modules are being detected
  const foundModulesLine = stderr.split('\n').find(line => line.includes('Split chunk detected with'));
  if (foundModulesLine) {
    console.log('\n✅ Modules detected:', foundModulesLine);
  } else {
    console.log('\n❌ No modules detected in real-world chunk');
  }
  
  // Check if tree shaking is running
  const treeShakingLines = stderr.split('\n').filter(line => line.includes('webpack_tree_shaking'));
  if (treeShakingLines.length > 0) {
    console.log('\n🌲 Tree shaking is running:');
    treeShakingLines.forEach(line => console.log(' ', line));
  } else {
    console.log('\n⚠️  Tree shaking is NOT running');
  }
  
  // Clean up
  fs.unlinkSync('/tmp/test-ast-parsing.js');
});