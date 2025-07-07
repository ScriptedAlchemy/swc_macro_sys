"use strict";
(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["shared_utils_js"], {
"./shared/config.js": 
/*!**************************!*\
  !*** ./shared/config.js ***!
  \**************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  EH: () => (DEFAULT_TIMEOUT)
});
// Shared configuration module
const API_ENDPOINTS = (/* unused pure expression or super */ null && ({
	users: "/api/users",
	posts: "/api/posts",
	auth: "/api/auth"
}));

const DEFAULT_TIMEOUT = 5000;
const MAX_RETRIES = 3;

const getApiUrl = endpoint => {
	return `${process.env.API_BASE_URL || "http://localhost:3000"}${endpoint}`;
};

/* unused ESM default export */ var __WEBPACK_DEFAULT_EXPORT__ = ((/* unused pure expression or super */ null && ({
	API_ENDPOINTS,
	DEFAULT_TIMEOUT,
	MAX_RETRIES,
	getApiUrl
})));


}),
"./shared/nested-utils.js": 
/*!********************************!*\
  !*** ./shared/nested-utils.js ***!
  \********************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.d(__webpack_exports__, {
  I8: () => (deepClone),
  Ox: () => (generateId),
  oH: () => (validateEmail)
});
// Nested utility functions to test PURE annotations
const validateEmail = email => {
	const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
	return emailRegex.test(email);
};

const generateId = () => {
	return Math.random().toString(36).substr(2, 9);
};

const deepClone = obj => {
	if (obj === null || typeof obj !== "object") return obj;
	if (obj instanceof Date) return new Date(obj.getTime());
	if (Array.isArray(obj)) return obj.map(item => deepClone(item));
	if (typeof obj === "object") {
		const copy = {};
		for (const key of Object.keys(obj)) {
			copy[key] = deepClone(obj[key]);
		}
		return copy;
	}
};

const sortBy = (array, key) => {
	return array.sort((a, b) => {
		if (a[key] < b[key]) return -1;
		if (a[key] > b[key]) return 1;
		return 0;
	});
};

/* unused ESM default export */ var __WEBPACK_DEFAULT_EXPORT__ = ((/* unused pure expression or super */ null && ({
	validateEmail,
	generateId,
	deepClone,
	sortBy
})));


}),
"./shared/utils.js": 
/*!*************************!*\
  !*** ./shared/utils.js ***!
  \*************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  capitalize: () => (/* @common:if [condition="treeShake.utility-lib.capitalize"] */ capitalize /* @common:endif */),
  debounce: () => (/* @common:if [condition="treeShake.utility-lib.debounce"] */ debounce /* @common:endif */),
  deepClone: () => (/* @common:if [condition="treeShake.utility-lib.deepClone"] */ /* reexport safe */ _nested_utils_js__WEBPACK_IMPORTED_MODULE_1__.I8 /* @common:endif */),
  "default": () => (/* @common:if [condition="treeShake.utility-lib.default"] */ __WEBPACK_DEFAULT_EXPORT__ /* @common:endif */),
  formatDate: () => (/* @common:if [condition="treeShake.utility-lib.formatDate"] */ formatDate /* @common:endif */),
  generateId: () => (/* @common:if [condition="treeShake.utility-lib.generateId"] */ /* reexport safe */ _nested_utils_js__WEBPACK_IMPORTED_MODULE_1__.Ox /* @common:endif */),
  processWithHelper: () => (/* @common:if [condition="treeShake.utility-lib.processWithHelper"] */ processWithHelper /* @common:endif */),
  validateEmail: () => (/* @common:if [condition="treeShake.utility-lib.validateEmail"] */ /* reexport safe */ _nested_utils_js__WEBPACK_IMPORTED_MODULE_1__.oH /* @common:endif */)
});
/* ESM import */var _config_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./config.js */ "./shared/config.js");
/* ESM import */var _nested_utils_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./nested-utils.js */ "./shared/nested-utils.js");

// Shared utility functions


// Import CommonJS helper to test PURE annotations for CommonJS requires
const cjsHelper = require("./cjs-helper.js");

const formatDate = date => {
	return new Intl.DateTimeFormat("en-US").format(date);
};

const capitalize = str => {
	return str.charAt(0).toUpperCase() + str.slice(1);
};

// Use CommonJS helper function to test CommonJS integration
const processWithHelper = input => {
	return cjsHelper.helperFunction(input);
};

// Re-export nested utilities


const debounce = (func, wait) => {
	let timeout;
	return function executedFunction(...args) {
		const later = () => {
			clearTimeout(timeout);
			func(...args);
		};
		clearTimeout(timeout);
		timeout = setTimeout(later, wait);
	};
};

/* @common:if [condition="treeShake.utility-lib.default"] */ /* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
	formatDate,
	capitalize,
	debounce
}) /* @common:endif */;


}),

}]);