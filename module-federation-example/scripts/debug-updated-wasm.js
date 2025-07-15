#!/usr/bin/env node

async function debugUpdatedWasm() {
  try {
    console.log('Debugging updated WASM module...\n');
    
    // Try direct file import
    const wasmPath = require('path').resolve(__dirname, '../../crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    console.log('WASM path:', wasmPath);
    console.log('File exists:', require('fs').existsSync(wasmPath));
    
    if (require('fs').existsSync(wasmPath)) {
      try {
        console.log('\nTrying to import WASM module...');
        const wasmModule = await import(`file://${wasmPath}`);
        console.log('WASM module loaded successfully!');
        console.log('Available exports:', Object.keys(wasmModule));
        
        // Check if we need to initialize
        if (typeof wasmModule.default === 'function') {
          console.log('\nInitializing WASM...');
          await wasmModule.default();
          console.log('WASM initialized!');
        }
        
        // Check for optimize function
        if (wasmModule.optimize) {
          console.log('✅ optimize function found');
          console.log('optimize type:', typeof wasmModule.optimize);
        } else {
          console.log('❌ optimize function not found');
        }
        
        return wasmModule;
        
      } catch (importError) {
        console.error('Import error:', importError.message);
      }
    }
    
  } catch (error) {
    console.error('Debug error:', error.message);
  }
}

debugUpdatedWasm();