(() => { // webpackBootstrap
"use strict";
var __webpack_modules__ = ({
// === ENTRY LEVEL MODULES (A, B, C) ===
"moduleA": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  A: () => (moduleA)
});
// HOISTED IMPORTS - always imported regardless of macros
var a1 = __webpack_require__("moduleA1");
var a2 = __webpack_require__("moduleA2");

// Feature A - conditionally USES A1 and A2 based on macro
function moduleA() {
    console.log('Module A: Feature A root');
    
    /* @common:if [condition="features.enableFeatureA"] */
    console.log('Feature A enabled, using A1 and A2');
    return {
        result: a1.processA1() + a2.processA2(),
        feature: 'A'
    };
    /* @common:endif */
    
    /* @common:if [condition="!features.enableFeatureA"] */
    console.log('Feature A disabled');
    return { result: 'Feature A disabled', feature: 'A' };
    /* @common:endif */
}


}),
"moduleB": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  B: () => (moduleB)
});
// HOISTED IMPORTS - always imported regardless of macros
var b1 = __webpack_require__("moduleB1");

// Feature B - conditionally USES B1 based on macro
function moduleB() {
    console.log('Module B: Feature B root');
    
    /* @common:if [condition="features.enableFeatureB"] */
    console.log('Feature B enabled, using B1');
    return {
        result: b1.processB1(),
        feature: 'B'
    };
    /* @common:endif */
    
    /* @common:if [condition="!features.enableFeatureB"] */
    console.log('Feature B disabled');
    return { result: 'Feature B disabled', feature: 'B' };
    /* @common:endif */
}


}),
"moduleC": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  C: () => (moduleC)
});
// HOISTED IMPORTS - always imported regardless of macros
var c1 = __webpack_require__("moduleC1");
var c2 = __webpack_require__("moduleC2");

// Feature C - conditionally USES C1 and C2 based on macro
function moduleC() {
    console.log('Module C: Feature C root');
    
    /* @common:if [condition="features.enableFeatureC"] */
    console.log('Feature C enabled, using C1 and C2');
    return {
        result: c1.processC1() + c2.processC2(),
        feature: 'C'
    };
    /* @common:endif */
    
    /* @common:if [condition="!features.enableFeatureC"] */
    console.log('Feature C disabled');
    return { result: 'Feature C disabled', feature: 'C' };
    /* @common:endif */
}


}),

// === LEVEL 1 MODULES (A1, A2, B1, C1, C2) ===
"moduleA1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processA1: () => (processA1)
});
// HOISTED IMPORTS - always imported regardless of macros
var a1_1 = __webpack_require__("moduleA1_1");
var a1_2 = __webpack_require__("moduleA1_2");

// A1 - always uses A1_1, conditionally uses A1_2
function processA1() {
    console.log('Processing A1');
    
    /* @common:if [condition="features.enableA1_2"] */
    console.log('A1_2 enabled in A1');
    return a1_1.deepProcessA1_1() + a1_2.deepProcessA1_2();
    /* @common:endif */
    
    /* @common:if [condition="!features.enableA1_2"] */
    console.log('A1_2 disabled in A1');
    return a1_1.deepProcessA1_1();
    /* @common:endif */
}


}),
"moduleA2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processA2: () => (processA2)
});
// HOISTED IMPORTS - always imported regardless of macros
var a2_1 = __webpack_require__("moduleA2_1");
var a2_2 = __webpack_require__("moduleA2_2");

// A2 - always uses A2_1, conditionally uses A2_2
function processA2() {
    console.log('Processing A2');
    
    /* @common:if [condition="features.enableA2_2"] */
    console.log('A2_2 enabled in A2');
    return a2_1.deepProcessA2_1() + a2_2.deepProcessA2_2();
    /* @common:endif */
    
    /* @common:if [condition="!features.enableA2_2"] */
    console.log('A2_2 disabled in A2');
    return a2_1.deepProcessA2_1();
    /* @common:endif */
}


}),
"moduleB1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processB1: () => (processB1)
});
// HOISTED IMPORTS - always imported regardless of macros
var b1_1 = __webpack_require__("moduleB1_1");
var b1_2 = __webpack_require__("moduleB1_2");

// B1 - conditionally uses B1_1 and B1_2 based on different conditions
function processB1() {
    console.log('Processing B1');
    var result = 0;
    
    /* @common:if [condition="features.enableB1_1"] */
    result += b1_1.deepProcessB1_1();
    console.log('B1_1 enabled in B1');
    /* @common:endif */
    
    /* @common:if [condition="features.enableB1_2"] */
    result += b1_2.deepProcessB1_2();
    console.log('B1_2 enabled in B1');
    /* @common:endif */
    
    return result || 100; // fallback value
}


}),
"moduleC1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processC1: () => (processC1)
});
// HOISTED IMPORTS - always imported regardless of macros
var shared = __webpack_require__("sharedDeepUtility");

// C1 - always uses shared deep utility
function processC1() {
    console.log('Processing C1');
    return shared.processSharedDeep('C1');
}


}),
"moduleC2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processC2: () => (processC2)
});
// C2 - isolated module with no dependencies (leaf)
function processC2() {
    console.log('Processing C2 (isolated leaf)');
    return 42;
}


}),

// === LEVEL 2 MODULES (A1_1, A1_2, A2_1, A2_2, B1_1, B1_2) ===
"moduleA1_1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessA1_1: () => (deepProcessA1_1)
});
// HOISTED IMPORTS - always imported regardless of macros
var shared = __webpack_require__("sharedDeepUtility");

// A1_1 - conditionally uses shared deep utility
function deepProcessA1_1() {
    console.log('Deep processing A1_1');
    
    /* @common:if [condition="features.enableSharedDeep"] */
    console.log('Shared deep utility enabled in A1_1');
    return shared.processSharedDeep('A1_1');
    /* @common:endif */
    
    /* @common:if [condition="!features.enableSharedDeep"] */
    console.log('Shared deep utility disabled in A1_1');
    return 10;
    /* @common:endif */
}


}),
"moduleA1_2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessA1_2: () => (deepProcessA1_2)
});
// A1_2 - isolated processing (leaf)
function deepProcessA1_2() {
    console.log('Deep processing A1_2 (leaf)');
    return 12;
}


}),
"moduleA2_1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessA2_1: () => (deepProcessA2_1)
});
// HOISTED IMPORTS - always imported regardless of macros
var shared = __webpack_require__("sharedDeepUtility");

// A2_1 - conditionally uses shared deep utility
function deepProcessA2_1() {
    console.log('Deep processing A2_1');
    
    /* @common:if [condition="features.enableSharedDeep"] */
    console.log('Shared deep utility enabled in A2_1');
    return shared.processSharedDeep('A2_1');
    /* @common:endif */
    
    /* @common:if [condition="!features.enableSharedDeep"] */
    console.log('Shared deep utility disabled in A2_1');
    return 21;
    /* @common:endif */
}


}),
"moduleA2_2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessA2_2: () => (deepProcessA2_2)
});
// A2_2 - isolated processing (leaf)
function deepProcessA2_2() {
    console.log('Deep processing A2_2 (leaf)');
    return 22;
}


}),
"moduleB1_1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessB1_1: () => (deepProcessB1_1)
});
// HOISTED IMPORTS - always imported regardless of macros
var shared = __webpack_require__("sharedDeepUtility");

// B1_1 - conditionally uses shared deep utility
function deepProcessB1_1() {
    console.log('Deep processing B1_1');
    
    /* @common:if [condition="features.enableSharedDeep"] */
    console.log('Shared deep utility enabled in B1_1');
    return shared.processSharedDeep('B1_1');
    /* @common:endif */
    
    /* @common:if [condition="!features.enableSharedDeep"] */
    console.log('Shared deep utility disabled in B1_1');
    return 111;
    /* @common:endif */
}


}),
"moduleB1_2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  deepProcessB1_2: () => (deepProcessB1_2)
});
// HOISTED IMPORTS - always imported regardless of macros
var b1_2_deep = __webpack_require__("moduleB1_2_Deep");

// B1_2 - conditionally uses separate deep utility chain
function deepProcessB1_2() {
    console.log('Deep processing B1_2');
    
    /* @common:if [condition="features.enableB1_2Deep"] */
    console.log('B1_2 deep chain enabled');
    return b1_2_deep.processB1_2_Deep();
    /* @common:endif */
    
    /* @common:if [condition="!features.enableB1_2Deep"] */
    console.log('B1_2 deep chain disabled');
    return 120;
    /* @common:endif */
}


}),

// === SHARED DEEP UTILITY (heavily used across multiple chains) ===
"sharedDeepUtility": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processSharedDeep: () => (processSharedDeep)
});
// HOISTED IMPORTS - always imported regardless of macros
var deepUtil1 = __webpack_require__("deepUtility1");
var deepUtil2 = __webpack_require__("deepUtility2");

// Shared deep utility - conditionally uses deep utilities
function processSharedDeep(caller) {
    console.log('Shared deep utility called by:', caller);
    var result = 0;
    
    /* @common:if [condition="features.enableDeepUtil1"] */
    result += deepUtil1.processDeepUtil1();
    console.log('Deep utility 1 enabled in shared deep');
    /* @common:endif */
    
    /* @common:if [condition="features.enableDeepUtil2"] */
    result += deepUtil2.processDeepUtil2();
    console.log('Deep utility 2 enabled in shared deep');
    /* @common:endif */
    
    return result + caller.length; // base calculation + caller identifier
}


}),

// === DEEP UTILITIES (Level 3+ dependencies) ===
"deepUtility1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processDeepUtil1: () => (processDeepUtil1)
});
// HOISTED IMPORTS - always imported regardless of macros
var leaf1 = __webpack_require__("leafUtility1");

// Deep utility 1 - imports leaf utility 1
function processDeepUtil1() {
    console.log('Processing deep utility 1');
    return leaf1.processLeaf1();
}


}),
"deepUtility2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processDeepUtil2: () => (processDeepUtil2)
});
// HOISTED IMPORTS - always imported regardless of macros
var leaf2 = __webpack_require__("leafUtility2");

// Deep utility 2 - conditionally uses leaf utilities
function processDeepUtil2() {
    console.log('Processing deep utility 2');
    
    /* @common:if [condition="features.enableLeaf2"] */
    console.log('Leaf utility 2 enabled in deep util 2');
    return leaf2.processLeaf2();
    /* @common:endif */
    
    /* @common:if [condition="!features.enableLeaf2"] */
    console.log('Leaf utility 2 disabled in deep util 2');
    return 200;
    /* @common:endif */
}


}),

// === B1_2 SEPARATE DEEP CHAIN ===
"moduleB1_2_Deep": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processB1_2_Deep: () => (processB1_2_Deep)
});
// HOISTED IMPORTS - always imported regardless of macros
var leaf3 = __webpack_require__("leafUtility3");

// B1_2 Deep - separate deep chain
function processB1_2_Deep() {
    console.log('Processing B1_2 deep chain');
    return leaf3.processLeaf3();
}


}),

// === LEAF UTILITIES (deepest level) ===
"leafUtility1": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processLeaf1: () => (processLeaf1)
});
// Leaf utility 1 - terminal node
function processLeaf1() {
    console.log('Processing leaf utility 1 (terminal)');
    return 1001;
}


}),
"leafUtility2": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processLeaf2: () => (processLeaf2)
});
// Leaf utility 2 - terminal node
function processLeaf2() {
    console.log('Processing leaf utility 2 (terminal)');
    return 1002;
}


}),
"leafUtility3": (function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  processLeaf3: () => (processLeaf3)
});
// Leaf utility 3 - terminal node (B1_2 chain)
function processLeaf3() {
    console.log('Processing leaf utility 3 (terminal, B1_2 chain)');
    return 1003;
}


}),

});
/************************************************************************/
// The module cache
var __webpack_module_cache__ = {};

// The require function
/*#__NO_SIDE_EFFECTS__*/
function __webpack_require__(moduleId) {

// Check if module is in cache
var cachedModule = __webpack_module_cache__[moduleId];
if (cachedModule !== undefined) {
return cachedModule.exports;
}
// Create a new module (and put it into the cache)
var module = (__webpack_module_cache__[moduleId] = {
exports: {}
});
// Execute the module function
__webpack_modules__[moduleId](module, module.exports, __webpack_require__);

// Return the exports of the module
return module.exports;

}

/************************************************************************/
// webpack/runtime/define_property_getters
(() => {
__webpack_require__.d = (exports, definition) => {
	for(var key in definition) {
        if(__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
            Object.defineProperty(exports, key, { enumerable: true, get: definition[key] });
        }
    }
};
})();
// webpack/runtime/has_own_property
(() => {
__webpack_require__.o = (obj, prop) => (Object.prototype.hasOwnProperty.call(obj, prop))
})();
// webpack/runtime/rspack_version
(() => {
__webpack_require__.rv = () => ("1.3.12")
})();
// webpack/runtime/rspack_unique_id
(() => {
__webpack_require__.ruid = "bundler=rspack@1.3.12";

})();
/************************************************************************/
var __webpack_exports__ = {};
// This entry needs to be wrapped in an IIFE because it needs to be isolated against other modules in the chunk.
(() => {
/* ESM import */var _moduleA_ts__WEBPACK_IMPORTED_MODULE_0__ = /*#__PURE__*/ __webpack_require__("moduleA");
/* ESM import */var _moduleB_ts__WEBPACK_IMPORTED_MODULE_1__ = /*#__PURE__*/ __webpack_require__("moduleB");
/* ESM import */var _moduleC_ts__WEBPACK_IMPORTED_MODULE_2__ = /*#__PURE__*/ __webpack_require__("moduleC");
// main.js - Entry point WITH macros at top level

console.log('=== Deep Nested Macros with Top-Level Demo ===');
console.log('Entry point: Conditionally loading feature modules based on top-level macros');

// TOP-LEVEL MACROS - conditionally load A, B, C
/* @common:if [condition="features.enableTopLevelA"] */
console.log('Top-level A enabled, loading Feature A');
console.log('Feature A result:', (0,_moduleA_ts__WEBPACK_IMPORTED_MODULE_0__/* .moduleA */.A)());
/* @common:endif */

/* @common:if [condition="features.enableTopLevelB"] */
console.log('Top-level B enabled, loading Feature B');
console.log('Feature B result:', (0,_moduleB_ts__WEBPACK_IMPORTED_MODULE_1__/* .moduleB */.B)());
/* @common:endif */

/* @common:if [condition="features.enableTopLevelC"] */
console.log('Top-level C enabled, loading Feature C');
console.log('Feature C result:', (0,_moduleC_ts__WEBPACK_IMPORTED_MODULE_2__/* .moduleC */.C)());
/* @common:endif */

console.log('Main application completed - processed enabled features');

})();

})();