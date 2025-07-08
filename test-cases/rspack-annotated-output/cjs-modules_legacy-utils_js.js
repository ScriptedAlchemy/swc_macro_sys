(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["cjs-modules_legacy-utils_js"], {
"./cjs-modules/legacy-utils.js": 
/*!*************************************!*\
  !*** ./cjs-modules/legacy-utils.js ***!
  \*************************************/
(function (module, exports) {
// CommonJS module with various export patterns (browser-compatible)
// Simulated path and fs modules for browser environment
const path = {
	normalize: p => p.replace(/[\/\\]+/g, "/").replace(/\/+$/, "") || "/",
	join: (...paths) =>
		paths
			.filter(Boolean)
			.join("/")
			.replace(/[\/\\]+/g, "/"),
	dirname: p => p.replace(/[\/\\][^\/\\]*$/, "") || "/",
	basename: p => p.split(/[\/\\]/).pop() || "",
	extname: p => {
		const m = p.match(/\.[^.\/\\]*$/);
		return m ? m[0] : "";
	},
	resolve: (...paths) =>
		`/${paths
			.filter(Boolean)
			.join("/")
			.replace(/[\/\\]+/g, "/")}`,
	isAbsolute: p => p.startsWith("/"),
	relative: (from, to) => to // Simplified for browser
};

const fs = {
	readFileSync: (path, encoding) => {
		// Simulated file reading for browser
		return `Simulated content of ${path}`;
	},
	existsSync: path => {
		// Simulated file existence check
		return true;
	}
};

// Named exports using exports object
/* @common:if [condition="treeShake.cjs-legacy-utils.formatPath"] */ exports.formatPath = function (filePath) {
	return path.normalize(filePath);
} /* @common:endif */;

/* @common:if [condition="treeShake.cjs-legacy-utils.readFileSync"] */ exports.readFileSync = function (filePath) {
	try {
		return fs.readFileSync(filePath, "utf8");
	} catch (error) {
		return `Error reading file: ${error.message}`;
	}
} /* @common:endif */;

// Object assignment to exports
/* @common:if [condition="treeShake.cjs-legacy-utils.constants"] */ exports.constants = {
	DEFAULT_ENCODING: "utf8",
	MAX_FILE_SIZE: 1024 * 1024,
	SUPPORTED_FORMATS: ["txt", "json", "js"]
} /* @common:endif */;

// Function assignment to exports
/* @common:if [condition="treeShake.cjs-legacy-utils.validateFile"] */ exports.validateFile = function (filePath) {
	const ext = path.extname(filePath).slice(1);
	return this.constants.SUPPORTED_FORMATS.includes(ext);
} /* @common:endif */;

// Class export
class FileManager {
	constructor(basePath = ".") {
		this.basePath = basePath;
	}

	resolve(filePath) {
		return path.resolve(this.basePath, filePath);
	}

	exists(filePath) {
		return fs.existsSync(this.resolve(filePath));
	}
}

/* @common:if [condition="treeShake.cjs-legacy-utils.FileManager"] */ exports.FileManager = FileManager /* @common:endif */;

// Default-style export using module.exports
const defaultUtils = {
	name: "legacy-utils",
	version: "1.0.0",
	type: "commonjs",

	// Methods
	join: (...paths) => path.join(...paths),
	dirname: filePath => path.dirname(filePath),
	basename: filePath => path.basename(filePath),

	// Utility functions
	isAbsolute: filePath => path.isAbsolute(filePath),
	relative: (from, to) => path.relative(from, to)
};

// Mixed pattern: both exports.* and module.exports
module.exports = defaultUtils;

// Additional exports after module.exports (CommonJS allows this)
/* @common:if [condition="treeShake.cjs-legacy-utils.formatPath"] */ module.exports.formatPath = exports.formatPath /* @common:endif */;
/* @common:if [condition="treeShake.cjs-legacy-utils.readFileSync"] */ module.exports.readFileSync = exports.readFileSync /* @common:endif */;
/* @common:if [condition="treeShake.cjs-legacy-utils.constants"] */ module.exports.constants = exports.constants /* @common:endif */;
/* @common:if [condition="treeShake.cjs-legacy-utils.validateFile"] */ module.exports.validateFile = exports.validateFile /* @common:endif */;
/* @common:if [condition="treeShake.cjs-legacy-utils.FileManager"] */ module.exports.FileManager = exports.FileManager /* @common:endif */;

// Circular reference test
/* @common:if [condition="treeShake.cjs-legacy-utils.getSelf"] */ module.exports.getSelf = function () {
	return module.exports;
} /* @common:endif */;


}),

}]);