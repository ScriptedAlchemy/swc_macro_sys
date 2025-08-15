// Simple Node.js test to verify lodash functions work correctly
// This test directly imports and executes the processItems function

import { processItems } from '../host/src/bootstrap.js';

// Sample test data
const testItems = [
  { name: 'Charlie', age: 30, category: 'user' },
  { name: 'Alice', age: 25, category: 'admin' },
  { name: 'Bob', age: 35, category: 'user' },
  { name: 'Alice', age: 25, category: 'admin' }, // duplicate
  { name: 'David', age: 28, category: 'user' }
];

console.log('=== Simple Node.js Test for Lodash Functions ===');
console.log('Original items:', testItems);

try {
  // Test the processItems function which uses lodash sortBy and uniq
  const result = processItems(testItems);
  
  console.log('\nProcessed items (sorted and deduplicated):', result);
  
  // Verify the results
  const expectedLength = 4; // Should remove one duplicate Alice
  const isCorrectLength = result.length === expectedLength;
  const isSorted = result[0].name === 'Alice' && result[1].name === 'Bob';
  
  console.log('\n=== Test Results ===');
  console.log('✓ Length check:', isCorrectLength ? 'PASS' : 'FAIL', `(expected: ${expectedLength}, got: ${result.length})`);
  console.log('✓ Sort check:', isSorted ? 'PASS' : 'FAIL', '(should start with Alice, Bob)');
  console.log('✓ Overall test:', (isCorrectLength && isSorted) ? 'PASS' : 'FAIL');
  
  if (isCorrectLength && isSorted) {
    console.log('\n🎉 All tests passed! Lodash functions are working correctly.');
  } else {
    console.log('\n❌ Some tests failed. Check the lodash optimization.');
  }
  
} catch (error) {
  console.error('\n❌ Test failed with error:', error.message);
  console.error('Stack trace:', error.stack);
}