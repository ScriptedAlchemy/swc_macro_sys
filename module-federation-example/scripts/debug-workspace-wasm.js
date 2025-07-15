#!/usr/bin/env node

async function debugWorkspaceWasm() {
  try {
    console.log('Debugging workspace WASM module...\n');
    
    // Try workspace dependency
    console.log('Trying workspace dependency import...');
    const wasmModule = await import('swc_macro_wasm');
    console.log('Workspace WASM module loaded successfully!');
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
      
      // Try a simple test
      console.log('\nTesting optimize function...');
      const testCode = 'console.log("test");';
      const testConfig = '{"treeShake":{"lodash-es":{"test":true}}}';
      
      try {
        const result = wasmModule.optimize(testCode, testConfig);
        console.log('Test call successful!');
        console.log('Result type:', typeof result);
        console.log('Result keys:', result ? Object.keys(result) : 'null/undefined');
      } catch (testError) {
        console.log('Test call failed:', testError.message);
      }
      
    } else {
      console.log('❌ optimize function not found');
    }
    
    return wasmModule;
    
  } catch (error) {
    console.error('Workspace debug error:', error.message);
  }
}

debugWorkspaceWasm();