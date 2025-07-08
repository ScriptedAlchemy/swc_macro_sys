"use strict";
(self["webpackChunkrspack_basic_example"] = self["webpackChunkrspack_basic_example"] || []).push([["shared_components_js"], {
"./shared/components.js": 
/*!******************************!*\
  !*** ./shared/components.js ***!
  \******************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  Button: () => (/* @common:if [condition="treeShake.component-lib.Button"] */ Button /* @common:endif */),
  Modal: () => (/* @common:if [condition="treeShake.component-lib.Modal"] */ Modal /* @common:endif */),
  Tooltip: () => (/* @common:if [condition="treeShake.component-lib.Tooltip"] */ Tooltip /* @common:endif */),
  createAlert: () => (/* @common:if [condition="treeShake.component-lib.createAlert"] */ createAlert /* @common:endif */),
  createCard: () => (/* @common:if [condition="treeShake.component-lib.createCard"] */ createCard /* @common:endif */),
  "default": () => (/* @common:if [condition="treeShake.component-lib.default"] */ __WEBPACK_DEFAULT_EXPORT__ /* @common:endif */)
});
// Shared component library - testing various export scenarios

// Used export (imported in index.js)
class Button {
	constructor(text, onClick) {
		this.element = document.createElement("button");
		this.element.textContent = text;
		this.element.addEventListener("click", onClick);
	}

	render() {
		return this.element;
	}
}

// Used export (imported in index.js)
class Modal {
	constructor(title, content) {
		this.title = title;
		this.content = content;
		this.isOpen = false;
	}

	open() {
		this.isOpen = true;
		console.log(`Modal "${this.title}" opened`);
	}

	close() {
		this.isOpen = false;
		console.log(`Modal "${this.title}" closed`);
	}
}

// Unused export (not imported anywhere)
const createCard = (title, description) => {
	return {
		title,
		description,
		render() {
			return `<div class="card"><h3>${title}</h3><p>${description}</p></div>`;
		}
	};
};

// Additional unused exports for testing
class Tooltip {
	constructor(element, text) {
		this.element = element;
		this.text = text;
	}

	show() {
		console.log(`Showing tooltip: ${this.text}`);
	}
}

const createAlert = (message, type = "info") => {
	return {
		message,
		type,
		show() {
			console.log(`Alert (${type}): ${message}`);
		}
	};
};

// Default export (not imported but defined)
/* @common:if [condition="treeShake.component-lib.default"] */ /* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = ({
	Button,
	Modal,
	createCard,
	Tooltip,
	createAlert
}) /* @common:endif */;


}),

}]);