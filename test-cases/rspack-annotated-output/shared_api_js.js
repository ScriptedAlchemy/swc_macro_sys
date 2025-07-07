"use strict";
(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["shared_api_js"], {
"./shared/api.js": 
/*!***********************!*\
  !*** ./shared/api.js ***!
  \***********************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  ApiClient: () => (/* @common:if [condition="treeShake.api-lib.ApiClient"] */ ApiClient /* @common:endif */),
  createApiClient: () => (/* @common:if [condition="treeShake.api-lib.createApiClient"] */ createApiClient /* @common:endif */),
  "default": () => (/* @common:if [condition="treeShake.api-lib.default"] */ __WEBPACK_DEFAULT_EXPORT__ /* @common:endif */),
  fetchWithTimeout: () => (/* @common:if [condition="treeShake.api-lib.fetchWithTimeout"] */ fetchWithTimeout /* @common:endif */)
});
/* ESM import */var _config_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(/*! ./config.js */ "./shared/config.js");
/* ESM import */var _nested_utils_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./nested-utils.js */ "./shared/nested-utils.js");

// Shared API utilities


const fetchWithTimeout = async (
	url,
	options = {},
	timeout = _config_js__WEBPACK_IMPORTED_MODULE_0__/* .DEFAULT_TIMEOUT */.EH
) => {
	const controller = new AbortController();
	const timeoutId = setTimeout(() => controller.abort(), timeout);

	try {
		const response = await fetch(url, {
			...options,
			signal: controller.signal
		});
		clearTimeout(timeoutId);
		return response;
	} catch (error) {
		clearTimeout(timeoutId);
		throw error;
	}
};

class ApiClient {
	constructor(baseUrl, headers = {}) {
		this.baseUrl = baseUrl;
		this.headers = headers;
		this.sessionId = (0,_nested_utils_js__WEBPACK_IMPORTED_MODULE_1__/* .generateId */.Ox)(); // Use imported function
	}

	async get(endpoint) {
		return fetchWithTimeout(`${this.baseUrl}${endpoint}`, {
			headers: this.headers
		});
	}

	async post(endpoint, data) {
		return fetchWithTimeout(`${this.baseUrl}${endpoint}`, {
			method: "POST",
			headers: {
				"Content-Type": "application/json",
				...this.headers
			},
			body: JSON.stringify(data)
		});
	}
}

const createApiClient = (baseUrl, headers) => {
	return new ApiClient(baseUrl, headers);
};

/* @common:if [condition="treeShake.api-lib.default"] */ /* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
	fetchWithTimeout,
	ApiClient,
	createApiClient
}) /* @common:endif */;


}),
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

}]);