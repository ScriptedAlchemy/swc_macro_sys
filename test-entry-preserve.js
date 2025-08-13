import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test that entry modules are preserved even without explicit require calls
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
  
  /* @common:if [condition="features.enableFeature"] */
  __webpack_require__(100);
  /* @common:endif */
})();
`;

const config = JSON.stringify({
  features: {
    enableFeature: false  // This removes the require(100) call
  },
  entryModules: {
    main: "200"  // Module 200 is our entry point
  }
});

console.log("=== Testing Entry Module Preservation ===\n");
console.log("Input:", source);
console.log("\nConfig:", config);
console.log("\nExpected: Module 200 should be preserved as entry, 100 and 300 removed\n");

try {
  const result = wasm.optimize(source, config);
  console.log("=== Output ===");
  console.log(result);
  console.log("\n=== Analysis ===");
  
  if (!result.includes("100:") && !result.includes("100: function")) {
    console.log("✓ Module 100 correctly removed");
  } else {
    console.log("✗ Module 100 should have been removed");
  }
  
  if (result.includes("200:") || result.includes("200: function")) {
    console.log("✓ Module 200 preserved (entry point)");
  } else {
    console.log("✗ Module 200 missing (should be kept as entry)");
  }
  
  if (!result.includes("300:") && !result.includes("300: function")) {
    console.log("✓ Module 300 correctly removed");
  } else {
    console.log("✗ Module 300 should have been removed");
  }
  
  if (result.includes("__webpack_modules__")) {
    console.log("✓ Webpack infrastructure preserved");
  } else {
    console.log("✗ Webpack infrastructure removed by DCE");
  }
} catch (e) {
  console.error("\n❌ Error:", e);
  console.error("Stack:", e.stack);
}