"use strict";
exports.ids = ["src_Button_js"];
exports.modules = {
"./src/Button.js": 
/*!***********************!*\
  !*** ./src/Button.js ***!
  \***********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (Button)
});
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_0__);

function Button({ text, onClick }) {
    return /*#__PURE__*/ React.createElement("button", {
        onClick: onClick,
        style: {
            padding: '10px 20px',
            backgroundColor: '#007acc',
            color: 'white',
            border: 'none',
            borderRadius: '4px',
            cursor: 'pointer',
            fontSize: '16px'
        }
    }, (0,lodash_es__WEBPACK_IMPORTED_MODULE_0__.capitalize)(text));
}


}),

};
;