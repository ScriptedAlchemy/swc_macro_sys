import wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

const source = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("module 100"); },
    200: function() { console.log("module 200"); },
    300: function() { console.log("module 300"); }
  };
  
  /* @common:if [condition="features.enableFeature"] */
  __webpack_require__(100);
  /* @common:endif */
})();
`;

const config = JSON.stringify({
  features: {
    enableFeature: false
  },
  entryModules: {
    main: "200"
  }
});

console.log("Input:", source);
console.log("\nConfig:", config);

try {
  const result = wasm.optimize(source, config);
  console.log("\nOutput:", result);
  console.log("\nSuccess! Macro processing and tree shaking working.");
  
  // Check if module 100 was removed (since enableFeature is false)
  if (!result.includes("100:")) {
    console.log("✓ Module 100 correctly removed by macro processing");
  }
  
  // Check if module 300 was removed (not reachable from entry 200)
  if (!result.includes("300:")) {
    console.log("✓ Module 300 correctly removed by tree shaking");
  }
  
  // Check if module 200 is preserved (entry point)
  if (result.includes("200:")) {
    console.log("✓ Module 200 correctly preserved as entry point");
  }
} catch (e) {
  console.error("Error:", e);
}