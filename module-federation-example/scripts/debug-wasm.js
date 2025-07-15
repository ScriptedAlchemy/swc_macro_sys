#!/usr/bin/env node

async function debugWasm() {
  try {
    const swcMacro = await import('swc_macro_wasm');
    console.log('Available WASM functions:');
    console.log(Object.getOwnPropertyNames(swcMacro));
    
    // Try some common function names
    const possibleNames = [
      'optimize_code_with_config',
      'optimize_code',
      'optimizeCode',
      'optimize',
      'transform',
      'process'
    ];
    
    console.log('\nChecking for optimization functions:');
    possibleNames.forEach(name => {
      if (typeof swcMacro[name] === 'function') {
        console.log(`✅ ${name}: function`);
      } else if (swcMacro[name] !== undefined) {
        console.log(`⚠️ ${name}: ${typeof swcMacro[name]}`);
      } else {
        console.log(`❌ ${name}: not found`);
      }
    });
    
  } catch (error) {
    console.error('Failed to load WASM module:', error.message);
  }
}

debugWasm();