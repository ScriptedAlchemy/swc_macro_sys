#!/usr/bin/env node

const optimizer = require('swc_macro_wasm');
const fs = require('fs');

console.log('🧪 Testing minimal lodash configuration for maximum reduction');

// Read the original lodash chunk
const chunkPath = 'host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js.original';
const chunk = fs.readFileSync(chunkPath, 'utf8');
const originalSize = chunk.length;

console.log(`Original lodash chunk: ${(originalSize / 1024).toFixed(1)} KB`);

// Test with absolute minimal config - only default and sortBy
const minimalConfig = {
  treeShake: {
    'lodash-es': {
      'default': true,
      'sortBy': true,
      // Everything else disabled
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
      'cond': false,
      'conforms': false,
      'conformsTo': false,
      'constant': false,
      'countBy': false,
      'create': false,
      'curry': false,
      'curryRight': false,
      'debounce': false,
      'deburr': false,
      'defaultTo': false,
      'defaults': false,
      'defaultsDeep': false,
      'defer': false,
      'delay': false,
      'difference': false,
      'differenceBy': false,
      'differenceWith': false,
      'divide': false,
      'drop': false,
      'dropRight': false,
      'dropRightWhile': false,
      'dropWhile': false,
      'each': false,
      'eachRight': false,
      'endsWith': false,
      'entries': false,
      'entriesIn': false,
      'eq': false,
      'escape': false,
      'escapeRegExp': false,
      'every': false,
      'extend': false,
      'extendWith': false,
      'fill': false,
      'filter': false,
      'find': false,
      'findIndex': false,
      'findKey': false,
      'findLast': false,
      'findLastIndex': false,
      'findLastKey': false,
      'first': false,
      'flatMap': false,
      'flatMapDeep': false,
      'flatMapDepth': false,
      'flatten': false,
      'flattenDeep': false,
      'flattenDepth': false,
      'flip': false,
      'floor': false,
      'flow': false,
      'flowRight': false,
      'forEach': false,
      'forEachRight': false,
      'forIn': false,
      'forInRight': false,
      'forOwn': false,
      'forOwnRight': false,
      'fromPairs': false,
      'functions': false,
      'functionsIn': false,
      'get': false,
      'groupBy': false,
      'gt': false,
      'gte': false,
      'has': false,
      'hasIn': false,
      'head': false,
      'identity': false,
      'includes': false,
      'indexOf': false,
      'initial': false,
      'inRange': false,
      'intersection': false,
      'intersectionBy': false,
      'intersectionWith': false,
      'invert': false,
      'invertBy': false,
      'invoke': false,
      'invokeMap': false,
      'isArguments': false,
      'isArray': false,
      'isArrayBuffer': false,
      'isArrayLike': false,
      'isArrayLikeObject': false,
      'isBoolean': false,
      'isBuffer': false,
      'isDate': false,
      'isElement': false,
      'isEmpty': false,
      'isEqual': false,
      'isEqualWith': false,
      'isError': false,
      'isFinite': false,
      'isFunction': false,
      'isInteger': false,
      'isLength': false,
      'isMap': false,
      'isMatch': false,
      'isMatchWith': false,
      'isNaN': false,
      'isNative': false,
      'isNil': false,
      'isNull': false,
      'isNumber': false,
      'isObject': false,
      'isObjectLike': false,
      'isPlainObject': false,
      'isRegExp': false,
      'isSafeInteger': false,
      'isSet': false,
      'isString': false,
      'isSymbol': false,
      'isTypedArray': false,
      'isUndefined': false,
      'isWeakMap': false,
      'isWeakSet': false,
      'iteratee': false,
      'join': false,
      'kebabCase': false,
      'keyBy': false,
      'keys': false,
      'keysIn': false,
      'last': false,
      'lastIndexOf': false,
      'lowerCase': false,
      'lowerFirst': false,
      'lt': false,
      'lte': false,
      'map': false,
      'mapKeys': false,
      'mapValues': false,
      'matches': false,
      'matchesProperty': false,
      'max': false,
      'maxBy': false,
      'mean': false,
      'meanBy': false,
      'memoize': false,
      'merge': false,
      'mergeWith': false,
      'method': false,
      'methodOf': false,
      'min': false,
      'minBy': false,
      'mixin': false,
      'multiply': false,
      'negate': false,
      'noop': false,
      'now': false,
      'nth': false,
      'nthArg': false,
      'omit': false,
      'omitBy': false,
      'once': false,
      'orderBy': false,
      'over': false,
      'overArgs': false,
      'overEvery': false,
      'overSome': false,
      'pad': false,
      'padEnd': false,
      'padStart': false,
      'parseInt': false,
      'partial': false,
      'partialRight': false,
      'partition': false,
      'pick': false,
      'pickBy': false,
      'property': false,
      'propertyOf': false,
      'pull': false,
      'pullAll': false,
      'pullAllBy': false,
      'pullAllWith': false,
      'pullAt': false,
      'random': false,
      'range': false,
      'rangeRight': false,
      'rearg': false,
      'reduce': false,
      'reduceRight': false,
      'reject': false,
      'remove': false,
      'repeat': false,
      'replace': false,
      'rest': false,
      'result': false,
      'reverse': false,
      'round': false,
      'sample': false,
      'sampleSize': false,
      'set': false,
      'setWith': false,
      'shuffle': false,
      'size': false,
      'slice': false,
      'snakeCase': false,
      'some': false,
      'sortedIndex': false,
      'sortedIndexBy': false,
      'sortedIndexOf': false,
      'sortedLastIndex': false,
      'sortedLastIndexBy': false,
      'sortedLastIndexOf': false,
      'sortedUniq': false,
      'sortedUniqBy': false,
      'split': false,
      'spread': false,
      'startCase': false,
      'startsWith': false,
      'subtract': false,
      'sum': false,
      'sumBy': false,
      'tail': false,
      'take': false,
      'takeRight': false,
      'takeRightWhile': false,
      'takeWhile': false,
      'tap': false,
      'template': false,
      'throttle': false,
      'thru': false,
      'times': false,
      'toArray': false,
      'toFinite': false,
      'toInteger': false,
      'toLength': false,
      'toLower': false,
      'toNumber': false,
      'toPairs': false,
      'toPairsIn': false,
      'toPath': false,
      'toPlainObject': false,
      'toSafeInteger': false,
      'toString': false,
      'toUpper': false,
      'transform': false,
      'trim': false,
      'trimEnd': false,
      'trimStart': false,
      'truncate': false,
      'unary': false,
      'unescape': false,
      'union': false,
      'unionBy': false,
      'unionWith': false,
      'uniq': false,
      'uniqBy': false,
      'uniqWith': false,
      'uniqueId': false,
      'unset': false,
      'unzip': false,
      'unzipWith': false,
      'update': false,
      'updateWith': false,
      'upperCase': false,
      'upperFirst': false,
      'values': false,
      'valuesIn': false,
      'without': false,
      'words': false,
      'wrap': false,
      'xor': false,
      'xorBy': false,
      'xorWith': false,
      'zip': false,
      'zipObject': false,
      'zipObjectDeep': false,
      'zipWith': false
    }
  }
};

console.log('\n🧪 Testing with minimal config (only default + sortBy)...');
const minimalOptimized = optimizer.optimize(chunk, JSON.stringify(minimalConfig));
const minimalSize = minimalOptimized.length;
const minimalReduction = ((originalSize - minimalSize) / originalSize * 100).toFixed(1);

console.log(`Minimal optimized size: ${(minimalSize / 1024).toFixed(1)} KB`);
console.log(`Minimal reduction: ${minimalReduction}%`);
console.log(`Bytes saved: ${((originalSize - minimalSize) / 1024).toFixed(1)} KB`);

// Count modules
const originalModules = (chunk.match(/\"[^\"]+\.js\":/g) || []).length;
const minimalModules = (minimalOptimized.match(/\"[^\"]+\.js\":/g) || []).length;

console.log(`\nModule count: ${originalModules} → ${minimalModules}`);
console.log(`Modules removed: ${originalModules - minimalModules}`);

// Test with only default export
const defaultOnlyConfig = {
  treeShake: {
    'lodash-es': {
      'default': true,
      'sortBy': false,
      // (all other exports remain false as above)
    }
  }
};

// Copy the false values from minimal config
Object.keys(minimalConfig.treeShake['lodash-es']).forEach(key => {
  if (key !== 'default' && key !== 'sortBy') {
    defaultOnlyConfig.treeShake['lodash-es'][key] = false;
  }
});

console.log('\n🧪 Testing with default-only config...');
const defaultOptimized = optimizer.optimize(chunk, JSON.stringify(defaultOnlyConfig));
const defaultSize = defaultOptimized.length;
const defaultReduction = ((originalSize - defaultSize) / originalSize * 100).toFixed(1);

console.log(`Default-only optimized size: ${(defaultSize / 1024).toFixed(1)} KB`);
console.log(`Default-only reduction: ${defaultReduction}%`);
console.log(`Bytes saved: ${((originalSize - defaultSize) / 1024).toFixed(1)} KB`);

const defaultModules = (defaultOptimized.match(/\"[^\"]+\.js\":/g) || []).length;
console.log(`Default-only modules: ${originalModules} → ${defaultModules}`);
console.log(`Default-only modules removed: ${originalModules - defaultModules}`);

console.log('\n📊 SUMMARY:');
console.log(`Original: ${(originalSize / 1024).toFixed(1)} KB (${originalModules} modules)`);
console.log(`Minimal (default + sortBy): ${(minimalSize / 1024).toFixed(1)} KB (${minimalModules} modules) - ${minimalReduction}% reduction`);
console.log(`Default-only: ${(defaultSize / 1024).toFixed(1)} KB (${defaultModules} modules) - ${defaultReduction}% reduction`);

if (parseFloat(defaultReduction) > 80) {
  console.log('\n🎉 Achieved high reduction rate! Enhanced parser is working effectively.');
} else if (parseFloat(defaultReduction) > 60) {
  console.log('\n✅ Good reduction rate achieved.');
} else {
  console.log('\n⚠️  Reduction rate could be improved. May need further optimization.');
}