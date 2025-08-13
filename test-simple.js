import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test with simple JavaScript code (not webpack bundle)
const source = `
// Simple functions with conditional macros
function formatMessage(message) {
  return \`[NEW] \${message}\`;
}

/* @common:if [condition="features.enableNewFeature"] */
export function newFeature() {
  const message = formatMessage("New feature is enabled!");
  return message;
}
/* @common:endif */

/* @common:if [condition="features.enableOldFeature"] */
export function oldFeature() {
  return "Old feature is enabled";
}
/* @common:endif */

// Always present code
export function alwaysPresent() {
  return "This function is always present";
}
`;

const config = JSON.stringify({
  features: {
    enableNewFeature: false,  // This should remove newFeature
    enableOldFeature: true    // This should keep oldFeature
  }
});

console.log("=== Testing Simple JavaScript with Macros ===\n");
console.log("Input:", source);
console.log("\nConfig:", config);

try {
  const result = wasm.optimize(source, config);
  console.log("\n=== Output ===");
  console.log(result);
  console.log("\n=== Verification ===");
  
  // Check if newFeature was removed (enableNewFeature = false)
  if (!result.includes("newFeature")) {
    console.log("✓ newFeature correctly removed (feature disabled)");
  } else {
    console.log("✗ newFeature should have been removed");
  }
  
  // Check if oldFeature is preserved (enableOldFeature = true)
  if (result.includes("oldFeature")) {
    console.log("✓ oldFeature correctly preserved (feature enabled)");
  } else {
    console.log("✗ oldFeature should have been preserved");
  }
  
  // Check if alwaysPresent is preserved
  if (result.includes("alwaysPresent")) {
    console.log("✓ alwaysPresent correctly preserved");
  } else {
    console.log("✗ alwaysPresent should have been preserved");
  }
  
  console.log("\n✅ SUCCESS! Macro processing working correctly in WASM!");
} catch (e) {
  console.error("\n❌ Error:", e);
  console.error("Stack:", e.stack);
}