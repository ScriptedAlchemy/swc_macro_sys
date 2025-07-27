"use strict";
exports.ids = ["src_functionalUtils_js"];
exports.modules = {
"./src/functionalUtils.js": 
/*!********************************!*\
  !*** ./src/functionalUtils.js ***!
  \********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  composeTransforms: () => (composeTransforms),
  createCurriedFunction: () => (createCurriedFunction),
  filterData: () => (filterData),
  getNestedProperty: () => (getNestedProperty),
  getProperty: () => (getProperty),
  mapData: () => (mapData),
  pipeTransforms: () => (pipeTransforms),
  processUserData: () => (processUserData),
  reduceData: () => (reduceData),
  sumValues: () => (sumValues)
});
/* ESM import */var ramda__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! ramda */ "webpack/sharing/consume/default/ramda/ramda");
/* ESM import */var ramda__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(ramda__WEBPACK_IMPORTED_MODULE_0__);
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

// Export functions that use only a subset of ramda
const composeTransforms = (...fns)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.compose)(...fns);
const pipeTransforms = (...fns)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.pipe)(...fns);
const createCurriedFunction = (fn)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.curry)(fn);
const mapData = (fn, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.map)(fn, data);
const filterData = (predicate, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.filter)(predicate, data);
const reduceData = (reducer, initial, data)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.reduce)(reducer, initial, data);
const getProperty = (propName, obj)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.prop)(propName, obj);
const getNestedProperty = (pathArray, obj)=>(0,ramda__WEBPACK_IMPORTED_MODULE_0__.path)(pathArray, obj);
// Example usage functions
const processUserData = (0,ramda__WEBPACK_IMPORTED_MODULE_0__.pipe)((0,ramda__WEBPACK_IMPORTED_MODULE_0__.filter)((user)=>user.active), (0,ramda__WEBPACK_IMPORTED_MODULE_0__.map)((user)=>_object_spread_props(_object_spread({}, user), {
        displayName: user.name.toUpperCase()
    })));
const sumValues = (0,ramda__WEBPACK_IMPORTED_MODULE_0__.reduce)((acc, val)=>acc + val, 0);


}),

};
;