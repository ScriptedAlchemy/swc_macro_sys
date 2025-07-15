#!/usr/bin/env node

const { readShareUsageFiles, mergeUsageData } = require('./optimize-shared-chunks.js');

async function testMerge() {
  try {
    console.log('🧪 Testing usage data merge...\n');
    
    // Read the actual JSON files
    const files = readShareUsageFiles();
    console.log('Share usage files found:');
    files.forEach(file => {
      console.log(`  ${file.name}: ${file.data.metadata?.total_modules || 'unknown'} modules`);
      
      if (file.data.consume_shared_modules) {
        Object.entries(file.data.consume_shared_modules).forEach(([moduleKey, moduleData]) => {
          console.log(`    ${moduleKey}: ${moduleData.used_exports?.length || 0} used, ${moduleData.unused_exports?.length || 0} unused`);
        });
      }
    });
    
    console.log('\nMerging data...');
    const merged = mergeUsageData(files);
    
    console.log('\n📋 Merged Tree-Shake Configuration:');
    console.log('=====================================');
    
    Object.entries(merged.treeShake).forEach(([moduleKey, exports]) => {
      const usedCount = Object.values(exports).filter(Boolean).length;
      const unusedCount = Object.values(exports).filter(v => !v).length;
      
      console.log(`\n${moduleKey}:`);
      console.log(`  Total exports configured: ${Object.keys(exports).length}`);
      console.log(`  Keep (used): ${usedCount}`);
      console.log(`  Remove (unused): ${unusedCount}`);
      console.log(`  Tree-shake ratio: ${(unusedCount / Object.keys(exports).length * 100).toFixed(1)}% removable`);
      
      // Show first few used/unused for verification
      const used = Object.entries(exports).filter(([, keep]) => keep).slice(0, 5);
      const unused = Object.entries(exports).filter(([, keep]) => !keep).slice(0, 5);
      
      if (used.length > 0) {
        console.log(`  Used exports (showing ${used.length}): ${used.map(([name]) => name).join(', ')}`);
      }
      
      if (unused.length > 0) {
        console.log(`  Unused exports (showing ${unused.length}): ${unused.map(([name]) => name).join(', ')}`);
      }
    });
    
    console.log('\n✅ Merge test completed successfully!');
    
    return merged;
    
  } catch (error) {
    console.error('❌ Merge test failed:', error.message);
    return null;
  }
}

if (require.main === module) {
  testMerge();
}

module.exports = { testMerge };