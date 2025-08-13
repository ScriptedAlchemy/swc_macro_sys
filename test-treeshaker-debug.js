#!/usr/bin/env node

import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';
import fs from 'fs';

// Read a real webpack chunk
const chunk = fs.readFileSync('./examples/module-federation-example/host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js', 'utf8');

// Create a config with explicit entry modules
const config = {
  treeShake: {
    'lodash-es': {
      debounce: true,
      throttle: true,
      pick: true,
      omit: true,
      capitalize: true,
      uniq: true,
      sortBy: true,
      groupBy: true,
      'default': true,
      chunk_characteristics: {
        entry_module_id: "../../../node_modules/.pnpm/lodash-es@4.17.21/node_modules/lodash-es/lodash.js"
      }
    }
  }
};

console.log('Chunk size:', chunk.length);
console.log('Config:', JSON.stringify(config, null, 2));

try {
  console.log('Calling optimize...');
  const result = optimize(chunk, JSON.stringify(config));
  console.log('Success! Optimized size:', result.length);
  console.log('Reduction:', ((chunk.length - result.length) / chunk.length * 100).toFixed(2) + '%');
} catch (e) {
  console.error('Error:', e.message);
  console.error('Stack:', e.stack);
}