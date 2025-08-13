import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Simpler test - no macros, just entry module config
const source = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("module 100"); },
    200: function() { console.log("module 200 - entry"); },
    300: function() { console.log("module 300"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
  
  // No require calls initially
})();
`;

const config = JSON.stringify({
  entryModules: {
    main: "200"  // Module 200 is our entry point
  }
});

console.log("=== Debugging Entry Module Preservation ===\n");
console.log("Input:", source);
console.log("\nConfig:", config);

try {
  const result = wasm.optimize(source, config);
  console.log("\n=== Output ===");
  console.log(result);
  
  // Detailed analysis
  if (result.includes("__webpack_require__")) {
    console.log("\n✓ Found __webpack_require__");
    const matches = result.match(/__webpack_require__\((.*?)\)/g);
    if (matches) {
      console.log("  Calls:", matches);
    }
  }
  
  if (result.includes("__webpack_modules__")) {
    console.log("✓ Found __webpack_modules__");
  }
  
  if (result.includes('"200"') || result.includes("'200'") || result.includes("200:")) {
    console.log("✓ Module 200 referenced");
  }
  
} catch (e) {
  console.error("\n❌ Error:", e);
}