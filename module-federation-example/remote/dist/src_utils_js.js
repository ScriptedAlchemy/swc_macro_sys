"use strict";
(self["webpackChunkremote"] = self["webpackChunkremote"] || []).push([["src_utils_js"], {
"./src/utils.js": 
/*!**********************!*\
  !*** ./src/utils.js ***!
  \**********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  createDebouncedFunction: () => (createDebouncedFunction),
  createThrottledFunction: () => (createThrottledFunction),
  formatUserData: () => (formatUserData),
  omitFields: () => (omitFields),
  pickFields: () => (pickFields)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);
function _define_property(obj, key, value) {
    if (key in obj) {
        Object.defineProperty(obj, key, {
            value: value,
            enumerable: true,
            configurable: true,
            writable: true
        });
    } else {
        obj[key] = value;
    }
    return obj;
}
function _object_spread(target) {
    for(var i = 1; i < arguments.length; i++){
        var source = arguments[i] != null ? arguments[i] : {};
        var ownKeys = Object.keys(source);
        if (typeof Object.getOwnPropertySymbols === "function") {
            ownKeys = ownKeys.concat(Object.getOwnPropertySymbols(source).filter(function(sym) {
                return Object.getOwnPropertyDescriptor(source, sym).enumerable;
            }));
        }
        ownKeys.forEach(function(key) {
            _define_property(target, key, source[key]);
        });
    }
    return target;
}
function ownKeys(object, enumerableOnly) {
    var keys = Object.keys(object);
    if (Object.getOwnPropertySymbols) {
        var symbols = Object.getOwnPropertySymbols(object);
        if (enumerableOnly) {
            symbols = symbols.filter(function(sym) {
                return Object.getOwnPropertyDescriptor(object, sym).enumerable;
            });
        }
        keys.push.apply(keys, symbols);
    }
    return keys;
}
function _object_spread_props(target, source) {
    source = source != null ? source : {};
    if (Object.getOwnPropertyDescriptors) {
        Object.defineProperties(target, Object.getOwnPropertyDescriptors(source));
    } else {
        ownKeys(Object(source)).forEach(function(key) {
            Object.defineProperty(target, key, Object.getOwnPropertyDescriptor(source, key));
        });
    }
    return target;
}

const createDebouncedFunction = (fn, delay = 300)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.debounce)(fn, delay);
};
const createThrottledFunction = (fn, delay = 100)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.throttle)(fn, delay);
};
const pickFields = (obj, fields)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.pick)(obj, fields);
};
const omitFields = (obj, fields)=>{
    return (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.omit)(obj, fields);
};
const formatUserData = (userData)=>{
    const publicFields = pickFields(userData, [
        'name',
        'email',
        'role'
    ]);
    return _object_spread_props(_object_spread({}, publicFields), {
        displayName: publicFields.name ? publicFields.name.toUpperCase() : 'Anonymous'
    });
};


}),

}]);
//# sourceMappingURL=src_utils_js.js.map