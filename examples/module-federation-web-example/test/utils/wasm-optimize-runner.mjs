import fs from 'fs';
import { optimize } from 'swc_macro_wasm';

// Get command line arguments
const [, , chunkPath, configJson] = process.argv;

if (!chunkPath || !configJson) {
  console.error('Usage: wasm-optimize-runner.mjs <chunkPath> <configJson>');
  process.exit(1);
}

try {
  // Read the chunk file
  const content = fs.readFileSync(chunkPath, 'utf-8');
  
  // Run optimization
  const result = optimize(content, configJson);
  
  // Output the result
  console.log(result);
} catch (error) {
  console.error('Error running optimization:', error);
  process.exit(1);
}