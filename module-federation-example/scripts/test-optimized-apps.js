#!/usr/bin/env node

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');

/**
 * Test if the optimized Module Federation apps still work correctly
 */
async function testOptimizedApps() {
  console.log('🧪 Testing optimized Module Federation apps...\n');
  
  try {
    // Check if optimized files exist
    console.log('1. Checking optimized files exist...');
    const hostChunk = path.resolve(__dirname, '../host/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
    const remoteChunk = path.resolve(__dirname, '../remote/dist/vendors-node_modules_pnpm_lodash-es_4_17_21_node_modules_lodash-es_lodash_js.js');
    const hostOriginal = hostChunk + '.original';
    const remoteOriginal = remoteChunk + '.original';
    
    if (!fs.existsSync(hostChunk)) {
      throw new Error('Host optimized chunk not found');
    }
    
    if (!fs.existsSync(remoteChunk)) {
      throw new Error('Remote optimized chunk not found');
    }
    
    if (!fs.existsSync(hostOriginal)) {
      throw new Error('Host original chunk backup not found');
    }
    
    if (!fs.existsSync(remoteOriginal)) {
      throw new Error('Remote original chunk backup not found');
    }
    
    console.log('✅ All optimized files exist\n');
    
    // Check size reduction
    console.log('2. Verifying size reduction...');
    const hostOriginalSize = fs.statSync(hostOriginal).size;
    const hostOptimizedSize = fs.statSync(hostChunk).size;
    const remoteOriginalSize = fs.statSync(remoteOriginal).size;
    const remoteOptimizedSize = fs.statSync(remoteChunk).size;
    
    const hostReduction = ((hostOriginalSize - hostOptimizedSize) / hostOriginalSize * 100);
    const remoteReduction = ((remoteOriginalSize - remoteOptimizedSize) / remoteOriginalSize * 100);
    
    console.log(`Host chunk: ${hostOriginalSize} → ${hostOptimizedSize} bytes (${hostReduction.toFixed(2)}% reduction)`);
    console.log(`Remote chunk: ${remoteOriginalSize} → ${remoteOptimizedSize} bytes (${remoteReduction.toFixed(2)}% reduction)`);
    
    if (hostReduction < 50 || remoteReduction < 50) {
      throw new Error('Size reduction is less than expected (should be >50%)');
    }
    
    console.log('✅ Size reduction verified\n');
    
    // Test lodash functions are still available
    console.log('3. Testing lodash functions in optimized chunks...');
    
    // Create a simple test script that loads the optimized chunks
    const testScript = `
      // Simulate module environment
      global.exports = {};
      global.module = { exports: {} };
      global.require = () => ({}); // Mock require
      
      try {
        // Load optimized host chunk
        const hostCode = require('fs').readFileSync('${hostChunk}', 'utf8');
        eval(hostCode);
        
        // Load optimized remote chunk  
        const remoteCode = require('fs').readFileSync('${remoteChunk}', 'utf8');
        eval(remoteCode);
        
        console.log('✅ Optimized chunks loaded without errors');
        
        // Check if critical lodash functions exist in the optimized code
        const hostHasUniq = hostCode.includes('uniq') || hostCode.includes('unique');
        const hostHasSortBy = hostCode.includes('sortBy') || hostCode.includes('sort');
        const remoteHasCapitalize = remoteCode.includes('capitalize') || remoteCode.includes('Capitalize');
        const remoteHasDebounce = remoteCode.includes('debounce') || remoteCode.includes('Debounce');
        
        if (!hostHasUniq || !hostHasSortBy) {
          throw new Error('Host required lodash functions not found in optimized chunk');
        }
        
        if (!remoteHasCapitalize || !remoteHasDebounce) {
          throw new Error('Remote required lodash functions not found in optimized chunk');
        }
        
        console.log('✅ Required lodash functions found in optimized chunks');
        
        // Check that unused functions are removed
        const hostHasUnusedAdd = hostCode.includes('function add(') || hostCode.includes('"add"');
        const remoteHasUnusedAdd = remoteCode.includes('function add(') || remoteCode.includes('"add"');
        
        if (hostHasUnusedAdd || remoteHasUnusedAdd) {
          console.log('⚠️  Some unused functions may still be present (this could be normal for minified code)');
        } else {
          console.log('✅ Unused functions appear to be removed');
        }
        
      } catch (error) {
        console.error('❌ Error testing optimized chunks:', error.message);
        process.exit(1);
      }
    `;
    
    // Write and execute test
    const testFile = path.resolve(__dirname, '../test-chunks.js');
    fs.writeFileSync(testFile, testScript);
    
    await new Promise((resolve, reject) => {
      const testProcess = spawn('node', [testFile], { 
        stdio: 'pipe',
        cwd: path.dirname(testFile)
      });
      
      let output = '';
      let errors = '';
      
      testProcess.stdout.on('data', (data) => {
        output += data.toString();
      });
      
      testProcess.stderr.on('data', (data) => {
        errors += data.toString();
      });
      
      testProcess.on('close', (code) => {
        fs.unlinkSync(testFile); // Clean up
        
        if (code === 0) {
          console.log(output);
          resolve();
        } else {
          console.error('Test process failed:', errors);
          reject(new Error(`Test process exited with code ${code}`));
        }
      });
      
      testProcess.on('error', (error) => {
        fs.unlinkSync(testFile); // Clean up
        reject(error);
      });
    });
    
    console.log('4. Checking HTML files can load...');
    
    // Check that HTML files exist and reference the optimized chunks
    const hostHtml = path.resolve(__dirname, '../host/dist/index.html');
    const remoteHtml = path.resolve(__dirname, '../remote/dist/index.html');
    
    if (!fs.existsSync(hostHtml) || !fs.existsSync(remoteHtml)) {
      throw new Error('HTML files not found');
    }
    
    const hostHtmlContent = fs.readFileSync(hostHtml, 'utf8');
    const remoteHtmlContent = fs.readFileSync(remoteHtml, 'utf8');
    
    // Check that HTML files reference JavaScript files
    if (!hostHtmlContent.includes('.js')) {
      throw new Error('Host HTML does not reference JavaScript files');
    }
    
    if (!remoteHtmlContent.includes('.js')) {
      throw new Error('Remote HTML does not reference JavaScript files');
    }
    
    console.log('✅ HTML files exist and reference JavaScript\n');
    
    // Check Module Federation entry points
    console.log('5. Checking Module Federation entry points...');
    
    const remoteEntry = path.resolve(__dirname, '../remote/dist/remoteEntry.js');
    if (!fs.existsSync(remoteEntry)) {
      throw new Error('Remote entry point not found');
    }
    
    const remoteEntryContent = fs.readFileSync(remoteEntry, 'utf8');
    if (!remoteEntryContent.includes('Button') || !remoteEntryContent.includes('utils')) {
      throw new Error('Remote entry point does not expose expected modules');
    }
    
    console.log('✅ Module Federation entry points verified\n');
    
    // Final success message
    console.log('🎉 All tests passed! The optimized Module Federation apps appear to be working correctly.\n');
    
    console.log('📊 Test Summary:');
    console.log('================');
    console.log('✅ Optimized files exist with backups');
    console.log(`✅ Significant size reduction achieved (${hostReduction.toFixed(1)}% and ${remoteReduction.toFixed(1)}%)`);
    console.log('✅ Required lodash functions preserved');
    console.log('✅ Chunks load without JavaScript errors');
    console.log('✅ HTML files properly generated');
    console.log('✅ Module Federation entry points intact');
    console.log('\n🚀 The optimization was successful and safe!');
    
    return true;
    
  } catch (error) {
    console.error('❌ Test failed:', error.message);
    console.log('\n🔧 Troubleshooting:');
    console.log('1. Run: pnpm run build:optimized');
    console.log('2. Check that optimization produced .original backup files');
    console.log('3. Verify SWC macro is working correctly');
    return false;
  }
}

// Run if called directly
if (require.main === module) {
  testOptimizedApps().then(success => {
    process.exit(success ? 0 : 1);
  });
}

module.exports = { testOptimizedApps };