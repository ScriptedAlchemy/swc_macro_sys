#!/usr/bin/env node

import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test with a simple JavaScript file first
const simpleCode = `
function hello() {
  console.log("Hello world");
}

hello();
`;

// Create a config with tree shaking but no webpack modules
const config = {
  treeShake: {
    'test': {
      hello: true
    }
  }
};

console.log('Testing with simple code...');
console.log('Code:', simpleCode);
console.log('Config:', JSON.stringify(config, null, 2));

try {
  console.log('Calling optimize...');
  const result = optimize(simpleCode, JSON.stringify(config));
  console.log('Success! Result:', result);
} catch (e) {
  console.error('Error:', e.message);
  console.error('Stack:', e.stack);
}