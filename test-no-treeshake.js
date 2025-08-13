#!/usr/bin/env node

import { optimize } from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test with simple JavaScript without tree shaking
const simpleCode = `
function hello() {
  console.log("Hello world");
}

hello();
`;

// Create a config without tree shaking
const config = {};

console.log('Testing without tree shaking...');
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