import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

// Test to trace the issue
const source = `
(function() {
  var __webpack_modules__ = {
    200: function() { console.log("module 200 - entry"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
})();
`;

// First test: no config (should not modify)
console.log("=== Test 1: No config ===");
const result1 = wasm.optimize(source, "{}");
console.log("Output:", result1);

// Second test: with entry module config
console.log("\n=== Test 2: With entry module ===");
const config = JSON.stringify({
  entryModules: {
    main: "200"
  }
});
const result2 = wasm.optimize(source, config);
console.log("Output:", result2);

// Third test: with tree shake config
console.log("\n=== Test 3: With tree shake ===");
const config3 = JSON.stringify({
  treeShake: {
    "test": {
      "200": true  // Keep module 200
    }
  }
});
const result3 = wasm.optimize(source, config3);
console.log("Output:", result3);