import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test with webpack bundle - with entry point preserved
const source = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("module 100"); },
    200: function() { console.log("module 200"); },
    300: function() { console.log("module 300"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
  
  // This entry point should be preserved
  __webpack_require__(100);
})();
`;

const config = JSON.stringify({
  entryModules: {
    main: "100"  // Module 100 is our entry point
  },
  treeShake: {
    "test": {
      "100": true,  // Keep module 100
      "200": false, // Remove module 200
      "300": false  // Remove module 300
    }
  }
});

console.log("=== Testing Webpack Bundle ===\n");
console.log("Input:", source);
console.log("\nConfig:", config);

try {
  const result = wasm.optimize(source, config);
  console.log("\n=== Output ===");
  console.log(result);
  console.log("\n=== Analysis ===");
  
  // Check what happened to each module
  if (result.includes("100:")) {
    console.log("✓ Module 100 preserved (entry point)");
  } else {
    console.log("✗ Module 100 missing (should be kept as entry)");
  }
  
  if (!result.includes("200:")) {
    console.log("✓ Module 200 removed (not reachable)");
  } else {
    console.log("✗ Module 200 should have been removed");
  }
  
  if (!result.includes("300:")) {
    console.log("✓ Module 300 removed (not reachable)");
  } else {
    console.log("✗ Module 300 should have been removed");
  }
  
  // Check if webpack infrastructure is preserved
  if (result.includes("__webpack_modules__")) {
    console.log("✓ Webpack modules object preserved");
  } else {
    console.log("✗ Webpack modules object missing");
  }
  
  if (result.includes("__webpack_require__")) {
    console.log("✓ Webpack require function preserved");
  } else {
    console.log("✗ Webpack require function missing");
  }
} catch (e) {
  console.error("\n❌ Error:", e);
  console.error("Stack:", e.stack);
}