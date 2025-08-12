"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_components_UserCard_jsx"], {
"./src/components/UserCard.jsx": 
/*!*************************************!*\
  !*** ./src/components/UserCard.jsx ***!
  \*************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! dayjs */ "webpack/sharing/consume/default/dayjs/dayjs");
/* ESM import */var dayjs__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(dayjs__WEBPACK_IMPORTED_MODULE_4__);
var _this = undefined;





var Text = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Text, Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var UserCard = function(param) {
    var user = param.user;
    var _ref = user || {}, _ref_name = _ref.name, name = _ref_name === void 0 ? 'Unknown User' : _ref_name, _ref_email = _ref.email, email = _ref_email === void 0 ? 'no-email@example.com' : _ref_email, avatar = _ref.avatar, _ref_role = _ref.role, role = _ref_role === void 0 ? 'User' : _ref_role, _ref_department = _ref.department, department = _ref_department === void 0 ? 'General' : _ref_department, _ref_joinDate = _ref.joinDate, joinDate = _ref_joinDate === void 0 ? new Date().toISOString() : _ref_joinDate;
    var formatJoinDate = dayjs__WEBPACK_IMPORTED_MODULE_4___default()(joinDate).format('MMMM D, YYYY');
    var yearsOfService = dayjs__WEBPACK_IMPORTED_MODULE_4___default()().diff(dayjs__WEBPACK_IMPORTED_MODULE_4___default()(joinDate), 'year');
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        style: {
            width: '100%',
            maxWidth: 400
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 23,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "middle",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 24,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        align: "middle",
        gutter: 16,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 25,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 26,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Avatar, {
        size: 80,
        src: avatar,
        icon: !avatar && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
                lineNumber: 30,
                columnNumber: 32
            }
        }),
        style: {
            backgroundColor: '#1890ff'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 27,
            columnNumber: 13
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 34,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 4,
        style: {
            margin: 0
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 35,
            columnNumber: 13
        },
        __self: _this
    }, (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(name)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
        color: "blue",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 36,
            columnNumber: 13
        },
        __self: _this
    }, role))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 40,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 41,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.MailOutlined, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 42,
            columnNumber: 13
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 43,
            columnNumber: 13
        },
        __self: _this
    }, email)), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 46,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.TeamOutlined, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 47,
            columnNumber: 13
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 48,
            columnNumber: 13
        },
        __self: _this
    }, (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.capitalize)(department), " Department")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 51,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.CalendarOutlined, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 52,
            columnNumber: 13
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 53,
            columnNumber: 13
        },
        __self: _this
    }, "Joined ", formatJoinDate))), yearsOfService > 0 && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            marginTop: 12,
            padding: '8px 12px',
            background: '#f0f2f5',
            borderRadius: 4
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 58,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        type: "secondary",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/UserCard.jsx",
            lineNumber: 64,
            columnNumber: 13
        },
        __self: _this
    }, yearsOfService, " ", yearsOfService === 1 ? 'year' : 'years', " with the company"))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (UserCard);


}),

}]);
//# sourceMappingURL=src_components_UserCard_jsx.5f545daeb0baf443.js.map