(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["cjs-modules_pure-cjs-helper_js"], {
"./cjs-modules/pure-cjs-helper.js": 
/*!****************************************!*\
  !*** ./cjs-modules/pure-cjs-helper.js ***!
  \****************************************/
(function (module, exports) {
// Pure CommonJS module with only require() usage - no ES6 imports
const crypto = {
	// Simulated crypto for browser
	createHash: () => ({
		update: () => ({ digest: () => "mock-hash" })
	})
};

// Pure CommonJS export patterns
/* @common:if [condition="treeShake.cjs-pure-helper.generateId"] */ exports.generateId = function () {
	return `id_${Math.random().toString(36).substr(2, 9)}`;
} /* @common:endif */;

/* @common:if [condition="treeShake.cjs-pure-helper.hashString"] */ exports.hashString = function (input) {
	return crypto.createHash("md5").update(input).digest("hex");
} /* @common:endif */;

/* @common:if [condition="treeShake.cjs-pure-helper.validateInput"] */ exports.validateInput = function (input) {
	return input && typeof input === "string" && input.trim().length > 0;
} /* @common:endif */;

/* @common:if [condition="treeShake.cjs-pure-helper.processData"] */ exports.processData = function (data) {
	if (!Array.isArray(data)) {
		return null;
	}
	return data.map(item => ({
		id: this.generateId(),
		hash: this.hashString(String(item)),
		valid: this.validateInput(String(item))
	}));
} /* @common:endif */;

// Utility object
/* @common:if [condition="treeShake.cjs-pure-helper.helpers"] */ exports.helpers = {
	timestamp: () => Date.now(),
	random: () => Math.random(),
	formatNumber: num => num.toLocaleString()
} /* @common:endif */;

// Constants
/* @common:if [condition="treeShake.cjs-pure-helper.CONSTANTS"] */ exports.CONSTANTS = {
	MAX_LENGTH: 100,
	MIN_LENGTH: 1,
	DEFAULT_PREFIX: "cjs_",
	SUPPORTED_TYPES: ["string", "number", "boolean"]
} /* @common:endif */;

// Class export
class DataValidator {
	constructor(options = {}) {
		this.options = {
			strict: false,
			allowEmpty: false,
			...options
		};
	}

	validate(data) {
		if (!data && !this.options.allowEmpty) {
			return false;
		}
		return this.options.strict ? this.strictValidate(data) : true;
	}

	strictValidate(data) {
		return exports.CONSTANTS.SUPPORTED_TYPES.includes(typeof data);
	}
}

/* @common:if [condition="treeShake.cjs-pure-helper.DataValidator"] */ exports.DataValidator = DataValidator /* @common:endif */;

// Factory function
/* @common:if [condition="treeShake.cjs-pure-helper.createValidator"] */ exports.createValidator = function (options) {
	return new DataValidator(options);
} /* @common:endif */;

// This module will NOT be imported via ES6 - only via require()
/* @common:if [condition="treeShake.cjs-pure-helper.info"] */ module.exports.info = {
	name: "pure-cjs-helper",
	version: "1.0.0",
	type: "pure-commonjs",
	description: "CommonJS module accessed only via require()"
} /* @common:endif */;


}),

}]);