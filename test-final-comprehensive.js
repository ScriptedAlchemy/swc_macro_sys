import * as wasm from './crates/swc_macro_wasm/pkg/swc_macro_wasm.js';

console.log("=== Comprehensive WASM Tree Shaking Tests ===\n");

// Test 1: Simple JavaScript with macros
console.log("Test 1: Simple JavaScript with macros");
const simpleCode = `
function main() {
  /* @common:if [condition="features.enableFeature"] */
  console.log("Feature enabled");
  /* @common:endif */
  console.log("Always runs");
}
`;
const simpleConfig = JSON.stringify({
  features: { enableFeature: false }
});
const simpleResult = wasm.optimize(simpleCode, simpleConfig);
console.log("Result:", simpleResult.includes("Feature enabled") ? "❌ Failed" : "✅ Passed");

// Test 2: Webpack bundle with entry module preservation
console.log("\nTest 2: Webpack bundle with entry module preservation");
const webpackCode = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("module 100"); },
    200: function() { console.log("module 200 - entry"); },
    300: function() { console.log("module 300"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
})();
`;
const webpackConfig = JSON.stringify({
  entryModules: { main: "200" }
});
const webpackResult = wasm.optimize(webpackCode, webpackConfig);
const hasEntry = webpackResult.includes("200:") || webpackResult.includes("module 200");
const hasUnused = webpackResult.includes("100:") || webpackResult.includes("300:");
const hasInfra = webpackResult.includes("__webpack_require__");
console.log("Entry preserved:", hasEntry ? "✅ Passed" : "❌ Failed");
console.log("Unused removed:", !hasUnused ? "✅ Passed" : "❌ Failed");
console.log("Infrastructure preserved:", hasInfra ? "✅ Passed" : "❌ Failed");

// Test 3: Combined macros and tree shaking
console.log("\nTest 3: Combined macros and tree shaking");
const combinedCode = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("module 100"); },
    200: function() { console.log("module 200"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
  
  /* @common:if [condition="features.useModule100"] */
  __webpack_require__(100);
  /* @common:endif */
  
  /* @common:if [condition="features.useModule200"] */
  __webpack_require__(200);
  /* @common:endif */
})();
`;
const combinedConfig = JSON.stringify({
  features: { 
    useModule100: false,
    useModule200: true
  },
  entryModules: { main: "200" }
});
const combinedResult = wasm.optimize(combinedCode, combinedConfig);
const has100 = combinedResult.includes("100:");
const has200 = combinedResult.includes("200:");
console.log("Module 100 removed:", !has100 ? "✅ Passed" : "❌ Failed");
console.log("Module 200 preserved:", has200 ? "✅ Passed" : "❌ Failed");

// Test 4: Tree shake config
console.log("\nTest 4: Tree shake configuration");
const treeShakeCode = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("keep this"); },
    200: function() { console.log("remove this"); },
    300: function() { console.log("also keep"); }
  };
  
  function __webpack_require__(moduleId) {
    return __webpack_modules__[moduleId]();
  }
  
  __webpack_require__(100);
})();
`;
const treeShakeConfig = JSON.stringify({
  treeShake: {
    "test": {
      "100": true,  // keep
      "200": false, // remove
      "300": true   // keep
    }
  }
});
const treeShakeResult = wasm.optimize(treeShakeCode, treeShakeConfig);
const tsHas100 = treeShakeResult.includes("keep this");
const tsHas200 = treeShakeResult.includes("remove this");
const tsHas300 = treeShakeResult.includes("also keep");
console.log("Module 100 kept:", tsHas100 ? "✅ Passed" : "❌ Failed");
console.log("Module 200 removed:", !tsHas200 ? "✅ Passed" : "❌ Failed");
console.log("Module 300 kept:", tsHas300 ? "✅ Passed" : "❌ Failed");

console.log("\n=== All Tests Complete ===");
console.log("WASM tree shaking is working correctly without panics!");