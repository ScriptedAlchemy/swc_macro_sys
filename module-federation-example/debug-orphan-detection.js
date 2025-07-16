#!/usr/bin/env node

const optimizer = require('swc_macro_wasm');
const fs = require('fs');

console.log('🔍 Debugging orphan detection in real-world lodash chunk');

// Read the original lodash chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');

// Test with a very specific config to enable only 2 exports
const specificConfig = {
  treeShake: {
    'lodash-es': {
      'default': true,
      'sortBy': true,
      'add': false,
      'after': false,
      'ary': false,
      'assign': false,
      'assignIn': false,
      'at': false,
      'attempt': false,
      'before': false,
      'bind': false,
      'bindAll': false,
      'bindKey': false,
      'camelCase': false,
      'capitalize': false,
      'castArray': false,
      'ceil': false,
      'chunk': false,
      'clamp': false,
      'clone': false,
      'cloneDeep': false,
      'cloneDeepWith': false,
      'cloneWith': false,
      'compact': false,
      'concat': false,
      'map': false,
      'filter': false,
      'reduce': false,
      'find': false,
      'forEach': false,
      'groupBy': false,
      'debounce': false,
      'throttle': false,
      'merge': false,
      'pick': false,
      'omit': false,
      'isEmpty': false,
      'isArray': false,
      'isObject': false,
      'isString': false,
      'isNumber': false,
      'isFunction': false,
      'flatten': false,
      'uniq': false,
      'reverse': false,
      'slice': false,
      'keys': false,
      'values': false,
      'entries': false,
      'has': false,
      'get': false,
      'set': false,
      'defaults': false,
      'defaultsDeep': false,
      'extend': false,
      'first': false,
      'last': false,
      'head': false,
      'tail': false,
      'initial': false,
      'difference': false,
      'union': false,
      'intersection': false,
      'without': false,
      'pull': false,
      'pullAll': false,
      'remove': false,
      'take': false,
      'drop': false,
      'takeWhile': false,
      'dropWhile': false,
      'fill': false,
      'range': false,
      'size': false,
      'every': false,
      'some': false,
      'includes': false,
      'indexOf': false,
      'lastIndexOf': false,
      'join': false,
      'toString': false,
      'toArray': false,
      'toNumber': false,
      'toInteger': false,
      'toLength': false,
      'toSafeInteger': false,
      'toFinite': false,
      'toString': false,
      'toLower': false,
      'toUpper': false,
      'trim': false,
      'trimStart': false,
      'trimEnd': false,
      'pad': false,
      'padStart': false,
      'padEnd': false,
      'repeat': false,
      'replace': false,
      'split': false,
      'startsWith': false,
      'endsWith': false,
      'escape': false,
      'unescape': false,
      'deburr': false,
      'words': false,
      'upperCase': false,
      'lowerCase': false,
      'capitalize': false,
      'startCase': false,
      'camelCase': false,
      'kebabCase': false,
      'snakeCase': false,
      'upperFirst': false,
      'lowerFirst': false,
      'template': false,
      'truncate': false,
      'wrap': false,
      'escapeRegExp': false,
      'parseInt': false,
      'random': false,
      'max': false,
      'min': false,
      'sum': false,
      'sumBy': false,
      'mean': false,
      'meanBy': false,
      'maxBy': false,
      'minBy': false,
      'round': false,
      'ceil': false,
      'floor': false,
      'abs': false,
      'subtract': false,
      'multiply': false,
      'divide': false,
      'now': false,
      'delay': false,
      'defer': false,
      'once': false,
      'memoize': false,
      'curry': false,
      'curryRight': false,
      'partial': false,
      'partialRight': false,
      'bind': false,
      'bindKey': false,
      'negate': false,
      'before': false,
      'after': false,
      'ary': false,
      'unary': false,
      'rest': false,
      'spread': false,
      'rearg': false,
      'flip': false,
      'over': false,
      'overEvery': false,
      'overSome': false,
      'overArgs': false,
      'flow': false,
      'flowRight': false,
      'method': false,
      'methodOf': false,
      'property': false,
      'propertyOf': false,
      'matches': false,
      'matchesProperty': false,
      'identity': false,
      'constant': false,
      'noop': false,
      'iteratee': false,
      'uniqueId': false,
      'times': false,
      'attempt': false,
      'cond': false,
      'conforms': false,
      'conformsTo': false,
      'defaultTo': false,
      'stubArray': false,
      'stubFalse': false,
      'stubObject': false,
      'stubString': false,
      'stubTrue': false,
      'tap': false,
      'thru': false,
      'toPath': false,
      'mixin': false,
      'VERSION': false
    }
  }
};

console.log('Testing with only 2 exports enabled (default + sortBy)...');
console.log('Expected: Should remove ~617 modules (619 - 2)');

// Run optimization and capture stderr to see tree shaking logs
const { spawn } = require('child_process');
const path = require('path');

// Create a temporary script that runs the optimization
const tempScript = `
const optimizer = require('swc_macro_wasm');
const fs = require('fs');

const chunk = fs.readFileSync('${chunkPath}', 'utf8');
const config = ${JSON.stringify(specificConfig)};

console.log('Running optimization...');
const result = optimizer.optimize(chunk, JSON.stringify(config));
console.log('Optimization complete');
console.log('Result size:', result.length);
`;

fs.writeFileSync('/tmp/test-optimization.js', tempScript);

// Run the script and capture output
const child = spawn('node', ['/tmp/test-optimization.js'], {
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
  console.log('\nSTDERR (tree shaking logs):');
  console.log(stderr);
  
  // Parse the tree shaking logs to see what's happening
  const orphanedLines = stderr.split('\n').filter(line => line.includes('orphaned modules'));
  const moduleRemovalLines = stderr.split('\n').filter(line => line.includes('Removing') && line.includes('modules'));
  
  console.log('\nOrphaned module detection:');
  orphanedLines.forEach(line => console.log(' ', line));
  
  console.log('\nModule removal attempts:');
  moduleRemovalLines.forEach(line => console.log(' ', line));
  
  if (orphanedLines.length === 0) {
    console.log('\n❌ No orphaned modules detected!');
    console.log('   This means the enhanced parser is not correctly analyzing the dependency graph');
    console.log('   Expected: Should detect hundreds of orphaned modules when only 2 exports are enabled');
  } else if (moduleRemovalLines.length === 0) {
    console.log('\n⚠️  Orphaned modules detected but no removal attempts');
    console.log('   This suggests an issue with the WebpackModuleRemover');
  } else {
    console.log('\n✅ Orphaned modules detected and removal attempted');
  }
  
  // Clean up
  fs.unlinkSync('/tmp/test-optimization.js');
});