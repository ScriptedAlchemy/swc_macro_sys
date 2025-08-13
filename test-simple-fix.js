// Simple test to verify the fix approach
const code = `
(function() {
  var __webpack_modules__ = {
    100: function() { console.log("100"); },
    200: function() { console.log("200"); }
  };
  function __webpack_require__(id) { return __webpack_modules__[id](); }
  // If we add this, DCE will preserve the bundle:
  __webpack_require__("200"); 
})();
`;

console.log("If there's at least one __webpack_require__ call, DCE preserves the bundle.");
console.log("So we just need to ensure one exists when entry modules are configured.");