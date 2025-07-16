#!/usr/bin/env node

const fs = require('fs');

// Test the macro processing with the lodash chunk
const lodash_chunk = `
    "use strict";
    exports.ids = ["vendors-lodash"];
    exports.modules = {
        "../../node_modules/lodash-es/lodash.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                sortBy: () => (/* @common:if [condition="treeShake.lodash-es.sortBy"] */ /* reexport safe */ _sortBy_js__WEBPACK_IMPORTED_MODULE_0__["default"] /* @common:endif */),
                map: () => (/* @common:if [condition="treeShake.lodash-es.map"] */ /* reexport safe */ _map_js__WEBPACK_IMPORTED_MODULE_1__["default"] /* @common:endif */),
                filter: () => (/* @common:if [condition="treeShake.lodash-es.filter"] */ /* reexport safe */ _filter_js__WEBPACK_IMPORTED_MODULE_2__["default"] /* @common:endif */),
                reduce: () => (/* @common:if [condition="treeShake.lodash-es.reduce"] */ /* reexport safe */ _reduce_js__WEBPACK_IMPORTED_MODULE_3__["default"] /* @common:endif */),
                "default": () => (/* @common:if [condition="treeShake.lodash-es.default"] */ /* reexport safe */ _lodash_default_js__WEBPACK_IMPORTED_MODULE_4__["default"] /* @common:endif */)
            });
            /* @common:if [condition="treeShake.lodash-es.sortBy"] */
            var _sortBy_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("../../node_modules/lodash-es/sortBy.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.map"] */
            var _map_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("../../node_modules/lodash-es/map.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.filter"] */
            var _filter_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__("../../node_modules/lodash-es/filter.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.reduce"] */
            var _reduce_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__("../../node_modules/lodash-es/reduce.js");
            /* @common:endif */
            /* @common:if [condition="treeShake.lodash-es.default"] */
            var _lodash_default_js__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__("../../node_modules/lodash-es/lodash.default.js");
            /* @common:endif */
        },
        "../../node_modules/lodash-es/sortBy.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.sortBy"] */
            function sortBy(collection, iteratee) {
                return collection.sort();
            }
            const __WEBPACK_DEFAULT_EXPORT__ = sortBy;
            /* @common:endif */
        },
        "../../node_modules/lodash-es/lodash.default.js": function(__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
            __webpack_require__.r(__webpack_exports__);
            __webpack_require__.d(__webpack_exports__, {
                "default": () => __WEBPACK_DEFAULT_EXPORT__
            });
            /* @common:if [condition="treeShake.lodash-es.default"] */
            function defaultFunc() {
                return "default lodash function";
            }
            const __WEBPACK_DEFAULT_EXPORT__ = defaultFunc;
            /* @common:endif */
        }
    };
`;

console.log('🧪 Testing macro processing with different configurations...');

// Test with sortBy + default enabled
const config1 = {
    treeShake: {
        'lodash-es': {
            'sortBy': true,
            'default': true,
            'map': false,
            'filter': false,
            'reduce': false
        }
    }
};

// Test with only default enabled
const config2 = {
    treeShake: {
        'lodash-es': {
            'sortBy': false,
            'default': true,
            'map': false,
            'filter': false,
            'reduce': false
        }
    }
};

try {
    const optimizer = require('./crates/swc_macro_wasm/pkg/swc_macro_wasm.js');
    
    console.log('\n=== Config 1: sortBy + default ===');
    const result1 = optimizer.optimize(lodash_chunk, JSON.stringify(config1));
    console.log('Result 1 length:', result1.length);
    console.log('Result 1 modules:', (result1.match(/.js":/g) || []).length);
    
    // Check what dependencies remain
    const deps1 = result1.match(/__webpack_require__\("([^"]+)"\)/g) || [];
    console.log('Dependencies in result 1:', deps1.length);
    deps1.forEach(dep => console.log('  -', dep));
    
    console.log('\n=== Config 2: default only ===');
    const result2 = optimizer.optimize(lodash_chunk, JSON.stringify(config2));
    console.log('Result 2 length:', result2.length);
    console.log('Result 2 modules:', (result2.match(/.js":/g) || []).length);
    
    // Check what dependencies remain
    const deps2 = result2.match(/__webpack_require__\("([^"]+)"\)/g) || [];
    console.log('Dependencies in result 2:', deps2.length);
    deps2.forEach(dep => console.log('  -', dep));
    
    console.log('\n=== Analysis ===');
    console.log('The tree shaker should preserve modules that have __webpack_require__ calls pointing to them');
    console.log('Config 1 should preserve: lodash.js, sortBy.js, lodash.default.js');
    console.log('Config 2 should preserve: lodash.js, lodash.default.js');
    
} catch (error) {
    console.error('Error:', error.message);
}