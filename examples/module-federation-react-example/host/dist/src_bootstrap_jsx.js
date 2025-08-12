"use strict";
(self["webpackChunk_mf_react_host"] = self["webpackChunk_mf_react_host"] || []).push([["src_bootstrap_jsx"], {
"./src/App.jsx": 
/*!*********************!*\
  !*** ./src/App.jsx ***!
  \*********************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! react-router-dom */ "webpack/sharing/consume/default/react-router-dom/react-router-dom");
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react_router_dom__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _components_AppHeader__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/AppHeader */ "./src/components/AppHeader.jsx");
/* ESM import */var _components_AppSidebar__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./components/AppSidebar */ "./src/components/AppSidebar.jsx");
/* ESM import */var _pages_Dashboard__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./pages/Dashboard */ "./src/pages/Dashboard.jsx");
/* ESM import */var _pages_Analytics__WEBPACK_IMPORTED_MODULE_6__ = __webpack_require__(/*! ./pages/Analytics */ "./src/pages/Analytics.jsx");
/* ESM import */var _pages_Users__WEBPACK_IMPORTED_MODULE_7__ = __webpack_require__(/*! ./pages/Users */ "./src/pages/Users.jsx");
/* ESM import */var _pages_Settings__WEBPACK_IMPORTED_MODULE_8__ = __webpack_require__(/*! ./pages/Settings */ "./src/pages/Settings.jsx");
/* ESM import */var _pages_RemoteShowcase__WEBPACK_IMPORTED_MODULE_9__ = __webpack_require__(/*! ./pages/RemoteShowcase */ "./src/pages/RemoteShowcase.jsx");
var _this = undefined;










var Content = antd__WEBPACK_IMPORTED_MODULE_2__.Layout.Content;
var LoadingFallback = function() {
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "loading-container",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 15,
            columnNumber: 3
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Spin, {
        size: "large",
        tip: "Loading...",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 16,
            columnNumber: 5
        },
        __self: _this
    }));
};
function App() {
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.BrowserRouter, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 22,
            columnNumber: 5
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Layout, {
        className: "app-layout",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 23,
            columnNumber: 7
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_AppSidebar__WEBPACK_IMPORTED_MODULE_4__["default"], {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 24,
            columnNumber: 9
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.Layout, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 25,
            columnNumber: 9
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_AppHeader__WEBPACK_IMPORTED_MODULE_3__["default"], {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 26,
            columnNumber: 11
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Content, {
        className: "app-content",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 27,
            columnNumber: 11
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react__WEBPACK_IMPORTED_MODULE_0__.Suspense, {
        fallback: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(LoadingFallback, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 28,
                columnNumber: 33
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 28,
            columnNumber: 13
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Routes, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 29,
            columnNumber: 15
        },
        __self: this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Navigate, {
            to: "/dashboard",
            replace: true,
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 30,
                columnNumber: 42
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 30,
            columnNumber: 17
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/dashboard",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Dashboard__WEBPACK_IMPORTED_MODULE_5__["default"], {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 31,
                columnNumber: 51
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 31,
            columnNumber: 17
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/analytics",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Analytics__WEBPACK_IMPORTED_MODULE_6__["default"], {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 32,
                columnNumber: 51
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 32,
            columnNumber: 17
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/users",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Users__WEBPACK_IMPORTED_MODULE_7__["default"], {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 33,
                columnNumber: 47
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 33,
            columnNumber: 17
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/remote-components",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_RemoteShowcase__WEBPACK_IMPORTED_MODULE_9__["default"], {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 34,
                columnNumber: 59
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 34,
            columnNumber: 17
        },
        __self: this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_router_dom__WEBPACK_IMPORTED_MODULE_1__.Route, {
        path: "/settings",
        element: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_pages_Settings__WEBPACK_IMPORTED_MODULE_8__["default"], {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
                lineNumber: 35,
                columnNumber: 50
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/App.jsx",
            lineNumber: 35,
            columnNumber: 17
        },
        __self: this
    })))))));
}
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (App);


}),
"./src/bootstrap.jsx": 
/*!***************************!*\
  !*** ./src/bootstrap.jsx ***!
  \***************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_dom_client__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! react-dom/client */ "../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var _store__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./store */ "./src/store/index.js");
/* ESM import */var _App__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./App */ "./src/App.jsx");






var root = react_dom_client__WEBPACK_IMPORTED_MODULE_1__.createRoot(document.getElementById('root'));
root.render(/*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement((react__WEBPACK_IMPORTED_MODULE_0___default().StrictMode), {
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/bootstrap.jsx",
        lineNumber: 11,
        columnNumber: 3
    },
    __self: undefined
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_redux__WEBPACK_IMPORTED_MODULE_2__.Provider, {
    store: _store__WEBPACK_IMPORTED_MODULE_4__.store,
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/bootstrap.jsx",
        lineNumber: 12,
        columnNumber: 5
    },
    __self: undefined
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_3__.ConfigProvider, {
    theme: {
        token: {
            colorPrimary: '#1890ff',
            borderRadius: 6
        }
    },
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/bootstrap.jsx",
        lineNumber: 13,
        columnNumber: 7
    },
    __self: undefined
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_App__WEBPACK_IMPORTED_MODULE_5__["default"], {
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/bootstrap.jsx",
        lineNumber: 21,
        columnNumber: 9
    },
    __self: undefined
})))));


}),
"./src/components/AppHeader.jsx": 
/*!**************************************!*\
  !*** ./src/components/AppHeader.jsx ***!
  \**************************************/
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
var _this = undefined;



var Header = antd__WEBPACK_IMPORTED_MODULE_1__.Layout.Header;
var AppHeader = function() {
    var userMenuItems = [
        {
            key: 'profile',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
                    lineNumber: 11,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Profile'
        },
        {
            key: 'settings',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SettingOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
                    lineNumber: 16,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Settings'
        },
        {
            type: 'divider'
        },
        {
            key: 'logout',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.LogoutOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
                    lineNumber: 24,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Logout'
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Header, {
        className: "app-header",
        style: {
            padding: '0 24px'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 30,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            flex: 1
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 31,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("h2", {
        style: {
            margin: 0
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 32,
            columnNumber: 9
        },
        __self: _this
    }, "Module Federation React Demo")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        size: "large",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 34,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Badge, {
        count: 5,
        size: "small",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 35,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.BellOutlined, {
        style: {
            fontSize: 18,
            cursor: 'pointer'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 36,
            columnNumber: 11
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Dropdown, {
        menu: {
            items: userMenuItems
        },
        placement: "bottomRight",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 38,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Avatar, {
        style: {
            cursor: 'pointer',
            backgroundColor: '#1890ff'
        },
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
                lineNumber: 41,
                columnNumber: 19
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppHeader.jsx",
            lineNumber: 39,
            columnNumber: 11
        },
        __self: _this
    }))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (AppHeader);


}),
"./src/components/AppSidebar.jsx": 
/*!***************************************!*\
  !*** ./src/components/AppSidebar.jsx ***!
  \***************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-router-dom */ "webpack/sharing/consume/default/react-router-dom/react-router-dom");
/* ESM import */var react_router_dom__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_router_dom__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__);
var _this = undefined;




var Sider = antd__WEBPACK_IMPORTED_MODULE_1__.Layout.Sider;
var AppSidebar = function() {
    var navigate = (0,react_router_dom__WEBPACK_IMPORTED_MODULE_2__.useNavigate)();
    var location = (0,react_router_dom__WEBPACK_IMPORTED_MODULE_2__.useLocation)();
    var menuItems = [
        {
            key: '/dashboard',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.DashboardOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
                    lineNumber: 21,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Dashboard'
        },
        {
            key: '/analytics',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.BarChartOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
                    lineNumber: 26,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Analytics'
        },
        {
            key: '/users',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.UserOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
                    lineNumber: 31,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Users'
        },
        {
            key: '/remote-components',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.AppstoreOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
                    lineNumber: 36,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Remote Components'
        },
        {
            key: '/settings',
            icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_3__.SettingOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
                    lineNumber: 41,
                    columnNumber: 13
                },
                __self: _this
            }),
            label: 'Settings'
        }
    ];
    var handleMenuClick = function(param) {
        var key = param.key;
        navigate(key);
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Sider, {
        width: 250,
        theme: "dark",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
            lineNumber: 51,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 64,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            borderBottom: '1px solid rgba(255, 255, 255, 0.1)'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
            lineNumber: 52,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("h3", {
        style: {
            color: '#fff',
            margin: 0
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
            lineNumber: 59,
            columnNumber: 9
        },
        __self: _this
    }, "MF React App")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Menu, {
        theme: "dark",
        mode: "inline",
        selectedKeys: [
            location.pathname
        ],
        items: menuItems,
        onClick: handleMenuClick,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/components/AppSidebar.jsx",
            lineNumber: 61,
            columnNumber: 7
        },
        __self: _this
    }));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (AppSidebar);


}),
"./src/pages/Analytics.jsx": 
/*!*********************************!*\
  !*** ./src/pages/Analytics.jsx ***!
  \*********************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../store/slices/analyticsSlice */ "./src/store/slices/analyticsSlice.js");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! react-chartjs-2 */ "webpack/sharing/consume/default/react-chartjs-2/react-chartjs-2");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__);
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_5__ = /* #__PURE__ */ __webpack_require__(/*! chart.js */ "webpack/sharing/consume/default/chart.js/chart.js");
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_5___default = /*#__PURE__*/__webpack_require__.n(chart_js__WEBPACK_IMPORTED_MODULE_5__);
var _this = undefined;






chart_js__WEBPACK_IMPORTED_MODULE_5__.Chart.register(chart_js__WEBPACK_IMPORTED_MODULE_5__.CategoryScale, chart_js__WEBPACK_IMPORTED_MODULE_5__.LinearScale, chart_js__WEBPACK_IMPORTED_MODULE_5__.PointElement, chart_js__WEBPACK_IMPORTED_MODULE_5__.LineElement, chart_js__WEBPACK_IMPORTED_MODULE_5__.Title, chart_js__WEBPACK_IMPORTED_MODULE_5__.Tooltip, chart_js__WEBPACK_IMPORTED_MODULE_5__.Legend, chart_js__WEBPACK_IMPORTED_MODULE_5__.ArcElement);
var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Analytics = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useSelector)(function(state) {
        return state.analytics;
    }), data = _useSelector.data, loading = _useSelector.loading, timeRange = _useSelector.timeRange;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__.fetchAnalyticsData)());
    }, [
        dispatch,
        timeRange
    ]);
    var handleTimeRangeChange = function(value) {
        dispatch((0,_store_slices_analyticsSlice__WEBPACK_IMPORTED_MODULE_3__.setTimeRange)(value));
    };
    var chartOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                position: 'top'
            }
        }
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 54,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        justify: "space-between",
        align: "middle",
        style: {
            marginBottom: 24
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 55,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 56,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 57,
            columnNumber: 11
        },
        __self: _this
    }, "Analytics")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 59,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        value: timeRange,
        onChange: handleTimeRangeChange,
        style: {
            width: 200
        },
        options: [
            {
                value: '7days',
                label: 'Last 7 days'
            },
            {
                value: '30days',
                label: 'Last 30 days'
            },
            {
                value: '6months',
                label: 'Last 6 months'
            },
            {
                value: '1year',
                label: 'Last year'
            }
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 60,
            columnNumber: 11
        },
        __self: _this
    }))), loading ? /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "loading-container",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 75,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Spin, {
        size: "large",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 76,
            columnNumber: 11
        },
        __self: _this
    })) : /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 79,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 12,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 80,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Revenue Trend",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 81,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 82,
            columnNumber: 15
        },
        __self: _this
    }, (data === null || data === void 0 ? void 0 : data.revenue) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Line, {
        data: data.revenue,
        options: chartOptions,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 84,
            columnNumber: 19
        },
        __self: _this
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 12,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 89,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "User Growth",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 90,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 91,
            columnNumber: 15
        },
        __self: _this
    }, (data === null || data === void 0 ? void 0 : data.userGrowth) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Line, {
        data: data.userGrowth,
        options: chartOptions,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 93,
            columnNumber: 19
        },
        __self: _this
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 98,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Device Categories",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 99,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: 300
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 100,
            columnNumber: 15
        },
        __self: _this
    }, (data === null || data === void 0 ? void 0 : data.categories) && /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_4__.Doughnut, {
        data: data.categories,
        options: chartOptions,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 102,
            columnNumber: 19
        },
        __self: _this
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 107,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Key Metrics",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 108,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "large",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 109,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: 16,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 110,
            columnNumber: 17
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 111,
            columnNumber: 19
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Page Views",
        value: "1,234,567",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 112,
            columnNumber: 21
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 114,
            columnNumber: 19
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Unique Visitors",
        value: "456,789",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 115,
            columnNumber: 21
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        span: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 117,
            columnNumber: 19
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Avg. Session",
        value: "5m 32s",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Analytics.jsx",
            lineNumber: 118,
            columnNumber: 21
        },
        __self: _this
    }))))))));
};
// Import Statistic component

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Analytics);


}),
"./src/pages/Dashboard.jsx": 
/*!*********************************!*\
  !*** ./src/pages/Dashboard.jsx ***!
  \*********************************/
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
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_3__);
/* ESM import */var _store_slices_dashboardSlice__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ../store/slices/dashboardSlice */ "./src/store/slices/dashboardSlice.js");
var _this = undefined;





var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Dashboard = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_3__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_3__.useSelector)(function(state) {
        return state.dashboard;
    }), stats = _useSelector.stats, loading = _useSelector.loading;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_dashboardSlice__WEBPACK_IMPORTED_MODULE_4__.fetchDashboardStats)());
    }, [
        dispatch
    ]);
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 18,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 19,
            columnNumber: 7
        },
        __self: _this
    }, "Dashboard"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ],
        className: "dashboard-stats",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 20,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 21,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 22,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Total Users",
        value: (stats === null || stats === void 0 ? void 0 : stats.totalUsers) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 26,
                columnNumber: 23
            }
        }),
        valueStyle: {
            color: '#3f8600'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 23,
            columnNumber: 13
        },
        __self: _this
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 31,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 32,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Active Users",
        value: (stats === null || stats === void 0 ? void 0 : stats.activeUsers) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.UserOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 36,
                columnNumber: 23
            }
        }),
        suffix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
            style: {
                fontSize: 14,
                color: '#3f8600'
            },
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 38,
                columnNumber: 17
            }
        }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowUpOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 39,
                columnNumber: 19
            }
        }), " 8%"),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 33,
            columnNumber: 13
        },
        __self: _this
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 45,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 46,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Revenue",
        value: (stats === null || stats === void 0 ? void 0 : stats.revenue) || 0,
        prefix: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.DollarOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 50,
                columnNumber: 23
            }
        }),
        precision: 2,
        valueStyle: {
            color: '#1890ff'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 47,
            columnNumber: 13
        },
        __self: _this
    }))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        sm: 12,
        lg: 6,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 56,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 57,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Statistic, {
        title: "Growth",
        value: (stats === null || stats === void 0 ? void 0 : stats.growth) || 0,
        suffix: "%",
        valueStyle: {
            color: (stats === null || stats === void 0 ? void 0 : stats.growth) > 0 ? '#3f8600' : '#cf1322'
        },
        prefix: (stats === null || stats === void 0 ? void 0 : stats.growth) > 0 ? /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowUpOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 63,
                columnNumber: 43
            }
        }) : /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.ArrowDownOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
                lineNumber: 63,
                columnNumber: 65
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 58,
            columnNumber: 13
        },
        __self: _this
    })))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 69,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 70,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Recent Activity",
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 71,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 72,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 73,
            columnNumber: 15
        },
        __self: _this
    }, "User John Doe completed a purchase - $125.00"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 74,
            columnNumber: 15
        },
        __self: _this
    }, "New user registration: jane.smith@example.com"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 75,
            columnNumber: 15
        },
        __self: _this
    }, "System maintenance scheduled for next week"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 76,
            columnNumber: 15
        },
        __self: _this
    }, "Revenue target achieved for Q4")))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 80,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Quick Stats",
        loading: loading,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 81,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 82,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 83,
            columnNumber: 15
        },
        __self: _this
    }, "Conversion Rate: 3.2%"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 84,
            columnNumber: 15
        },
        __self: _this
    }, "Avg. Session Duration: 5m 32s"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 85,
            columnNumber: 15
        },
        __self: _this
    }, "Bounce Rate: 42%"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Dashboard.jsx",
            lineNumber: 86,
            columnNumber: 15
        },
        __self: _this
    }, "Page Views: 125,432"))))));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Dashboard);


}),
"./src/pages/RemoteShowcase.jsx": 
/*!**************************************!*\
  !*** ./src/pages/RemoteShowcase.jsx ***!
  \**************************************/
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
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
}
function _iterable_to_array_limit(arr, i) {
    var _i = arr == null ? null : typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"];
    if (_i == null) return;
    var _arr = [];
    var _n = true;
    var _d = false;
    var _s, _e;
    try {
        for(_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true){
            _arr.push(_s.value);
            if (i && _arr.length === i) break;
        }
    } catch (err) {
        _d = true;
        _e = err;
    } finally{
        try {
            if (!_n && _i["return"] != null) _i["return"]();
        } finally{
            if (_d) throw _e;
        }
    }
    return _arr;
}
function _non_iterable_rest() {
    throw new TypeError("Invalid attempt to destructure non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}
var _this = undefined;



var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
// Lazy load remote components
var UserCard = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_UserCard").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/UserCard */ "webpack/container/remote/remote/UserCard", 23));
});
var DataTable = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_DataTable").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/DataTable */ "webpack/container/remote/remote/DataTable", 23));
});
var ChartWidget = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_ChartWidget").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/ChartWidget */ "webpack/container/remote/remote/ChartWidget", 23));
});
var FormBuilder = /*#__PURE__*/ (0,react__WEBPACK_IMPORTED_MODULE_0__.lazy)(function() {
    return __webpack_require__.e(/*! import() */ "webpack_container_remote_remote_FormBuilder").then(__webpack_require__.t.bind(__webpack_require__, /*! remote/FormBuilder */ "webpack/container/remote/remote/FormBuilder", 23));
});
var RemoteComponentWrapper = function(param) {
    var children = param.children, title = param.title;
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: title,
        className: "remote-component-wrapper",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 14,
            columnNumber: 3
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react__WEBPACK_IMPORTED_MODULE_0__.Suspense, {
        fallback: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
            style: {
                textAlign: 'center',
                padding: 40
            },
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                lineNumber: 16,
                columnNumber: 7
            }
        }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Spin, {
            size: "large",
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                lineNumber: 17,
                columnNumber: 9
            }
        }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("p", {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                lineNumber: 18,
                columnNumber: 9
            }
        }, "Loading remote component...")),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 15,
            columnNumber: 5
        },
        __self: _this
    }, children));
};
var RemoteShowcase = function() {
    var _useState = _sliced_to_array((0,react__WEBPACK_IMPORTED_MODULE_0__.useState)('1'), 2), activeTab = _useState[0], setActiveTab = _useState[1];
    var tabItems = [
        {
            key: '1',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 33,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.AppstoreOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 34,
                    columnNumber: 11
                },
                __self: _this
            }), "User Card"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "User Profile Card Component",
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 39,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(UserCard, {
                user: {
                    name: 'John Doe',
                    email: 'john.doe@example.com',
                    avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=John',
                    role: 'Senior Developer',
                    department: 'Engineering',
                    joinDate: '2022-01-15'
                },
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 40,
                    columnNumber: 11
                },
                __self: _this
            }))
        },
        {
            key: '2',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 56,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.TableOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 57,
                    columnNumber: 11
                },
                __self: _this
            }), "Data Table"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Advanced Data Table Component",
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 62,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(DataTable, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 63,
                    columnNumber: 11
                },
                __self: _this
            }))
        },
        {
            key: '3',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 70,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.BarChartOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 71,
                    columnNumber: 11
                },
                __self: _this
            }), "Charts"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Chart Widgets",
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 76,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(ChartWidget, {
                type: "line",
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 77,
                    columnNumber: 11
                },
                __self: _this
            }))
        },
        {
            key: '4',
            label: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 84,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.FormOutlined, {
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 85,
                    columnNumber: 11
                },
                __self: _this
            }), "Form Builder"),
            children: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(RemoteComponentWrapper, {
                title: "Dynamic Form Builder",
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 90,
                    columnNumber: 9
                },
                __self: _this
            }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(FormBuilder, {
                fields: [
                    {
                        type: 'text',
                        name: 'firstName',
                        label: 'First Name',
                        required: true
                    },
                    {
                        type: 'text',
                        name: 'lastName',
                        label: 'Last Name',
                        required: true
                    },
                    {
                        type: 'email',
                        name: 'email',
                        label: 'Email',
                        required: true
                    },
                    {
                        type: 'select',
                        name: 'department',
                        label: 'Department',
                        options: [
                            'Engineering',
                            'Sales',
                            'Marketing',
                            'HR'
                        ]
                    }
                ],
                onSubmit: function(values) {
                    return console.log('Form submitted:', values);
                },
                __source: {
                    fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
                    lineNumber: 91,
                    columnNumber: 11
                },
                __self: _this
            }))
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 107,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 108,
            columnNumber: 7
        },
        __self: _this
    }, "Remote Components Showcase"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Alert, {
        message: "Module Federation in Action",
        description: "These components are loaded dynamically from a remote application running on port 3002. They share React, Ant Design, and other dependencies with the host application.",
        type: "info",
        showIcon: true,
        style: {
            marginBottom: 24
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 110,
            columnNumber: 7
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tabs, {
        activeKey: activeTab,
        onChange: setActiveTab,
        items: tabItems,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/RemoteShowcase.jsx",
            lineNumber: 118,
            columnNumber: 7
        },
        __self: _this
    }));
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (RemoteShowcase);


}),
"./src/pages/Settings.jsx": 
/*!********************************!*\
  !*** ./src/pages/Settings.jsx ***!
  \********************************/
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
function _array_like_to_array(arr, len) {
    if (len == null || len > arr.length) len = arr.length;
    for(var i = 0, arr2 = new Array(len); i < len; i++)arr2[i] = arr[i];
    return arr2;
}
function _array_with_holes(arr) {
    if (Array.isArray(arr)) return arr;
}
function _iterable_to_array_limit(arr, i) {
    var _i = arr == null ? null : typeof Symbol !== "undefined" && arr[Symbol.iterator] || arr["@@iterator"];
    if (_i == null) return;
    var _arr = [];
    var _n = true;
    var _d = false;
    var _s, _e;
    try {
        for(_i = _i.call(arr); !(_n = (_s = _i.next()).done); _n = true){
            _arr.push(_s.value);
            if (i && _arr.length === i) break;
        }
    } catch (err) {
        _d = true;
        _e = err;
    } finally{
        try {
            if (!_n && _i["return"] != null) _i["return"]();
        } finally{
            if (_d) throw _e;
        }
    }
    return _arr;
}
function _non_iterable_rest() {
    throw new TypeError("Invalid attempt to destructure non-iterable instance.\\nIn order to be iterable, non-array objects must have a [Symbol.iterator]() method.");
}
function _sliced_to_array(arr, i) {
    return _array_with_holes(arr) || _iterable_to_array_limit(arr, i) || _unsupported_iterable_to_array(arr, i) || _non_iterable_rest();
}
function _unsupported_iterable_to_array(o, minLen) {
    if (!o) return;
    if (typeof o === "string") return _array_like_to_array(o, minLen);
    var n = Object.prototype.toString.call(o).slice(8, -1);
    if (n === "Object" && o.constructor) n = o.constructor.name;
    if (n === "Map" || n === "Set") return Array.from(n);
    if (n === "Arguments" || /^(?:Ui|I)nt(?:8|16|32)(?:Clamped)?Array$/.test(n)) return _array_like_to_array(o, minLen);
}
var _this = undefined;



var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title, Text = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Text;
var Settings = function() {
    var _Form_useForm = _sliced_to_array(antd__WEBPACK_IMPORTED_MODULE_1__.Form.useForm(), 1), form = _Form_useForm[0];
    var handleSubmit = function(values) {
        console.log('Settings saved:', values);
    // Here you would typically save to backend/state
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 16,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 17,
            columnNumber: 7
        },
        __self: _this
    }, "Settings"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        gutter: [
            16,
            16
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 19,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 16,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 20,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "General Settings",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 21,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form, {
        form: form,
        layout: "vertical",
        onFinish: handleSubmit,
        initialValues: {
            appName: 'Module Federation React Demo',
            language: 'en',
            timezone: 'UTC',
            notifications: true,
            darkMode: false
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 22,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Application Name",
        name: "appName",
        rules: [
            {
                required: true,
                message: 'Please enter application name'
            }
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 34,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Input, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 39,
            columnNumber: 17
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Language",
        name: "language",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 42,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 43,
            columnNumber: 17
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "en",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 44,
            columnNumber: 19
        },
        __self: _this
    }, "English"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "es",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 45,
            columnNumber: 19
        },
        __self: _this
    }, "Spanish"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "fr",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 46,
            columnNumber: 19
        },
        __self: _this
    }, "French"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "de",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 47,
            columnNumber: 19
        },
        __self: _this
    }, "German"))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Timezone",
        name: "timezone",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 51,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 52,
            columnNumber: 17
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "UTC",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 53,
            columnNumber: 19
        },
        __self: _this
    }, "UTC"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "EST",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 54,
            columnNumber: 19
        },
        __self: _this
    }, "Eastern Time"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "PST",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 55,
            columnNumber: 19
        },
        __self: _this
    }, "Pacific Time"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select.Option, {
        value: "CET",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 56,
            columnNumber: 19
        },
        __self: _this
    }, "Central European Time"))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Divider, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 60,
            columnNumber: 15
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Email Notifications",
        name: "notifications",
        valuePropName: "checked",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 62,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Switch, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 63,
            columnNumber: 17
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        label: "Dark Mode",
        name: "darkMode",
        valuePropName: "checked",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 66,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Switch, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 67,
            columnNumber: 17
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Form.Item, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 70,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        type: "primary",
        htmlType: "submit",
        icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_2__.SaveOutlined, {
            __source: {
                fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
                lineNumber: 71,
                columnNumber: 64
            }
        }),
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 71,
            columnNumber: 17
        },
        __self: _this
    }, "Save Settings"))))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        xs: 24,
        lg: 8,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 79,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Application Info",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 80,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 81,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 82,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 83,
            columnNumber: 17
        },
        __self: _this
    }, "Version:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 84,
            columnNumber: 17
        },
        __self: _this
    }, " 1.0.0")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 86,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 87,
            columnNumber: 17
        },
        __self: _this
    }, "Environment:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 88,
            columnNumber: 17
        },
        __self: _this
    }, " Development")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 90,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 91,
            columnNumber: 17
        },
        __self: _this
    }, "API Endpoint:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 92,
            columnNumber: 17
        },
        __self: _this
    }, " http://localhost:3000/api")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 94,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 95,
            columnNumber: 17
        },
        __self: _this
    }, "Build Date:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 96,
            columnNumber: 17
        },
        __self: _this
    }, " ", new Date().toLocaleDateString())))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Card, {
        title: "Module Federation Info",
        style: {
            marginTop: 16
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 101,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 102,
            columnNumber: 13
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 103,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 104,
            columnNumber: 17
        },
        __self: _this
    }, "Host URL:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 105,
            columnNumber: 17
        },
        __self: _this
    }, " http://localhost:3001")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 107,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 108,
            columnNumber: 17
        },
        __self: _this
    }, "Remote URL:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 109,
            columnNumber: 17
        },
        __self: _this
    }, " http://localhost:3002")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 111,
            columnNumber: 15
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        strong: true,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 112,
            columnNumber: 17
        },
        __self: _this
    }, "Shared Libs:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Text, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Settings.jsx",
            lineNumber: 113,
            columnNumber: 17
        },
        __self: _this
    }, " React, Ant Design, Redux")))))));
};
// Import Row and Col

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Settings);


}),
"./src/pages/Users.jsx": 
/*!*****************************!*\
  !*** ./src/pages/Users.jsx ***!
  \*****************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! react-redux */ "webpack/sharing/consume/default/react-redux/react-redux");
/* ESM import */var react_redux__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(react_redux__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ../store/slices/usersSlice */ "./src/store/slices/usersSlice.js");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_4__ = /* #__PURE__ */ __webpack_require__(/*! @ant-design/icons */ "webpack/sharing/consume/default/@ant-design/icons/@ant-design/icons");
/* ESM import */var _ant_design_icons__WEBPACK_IMPORTED_MODULE_4___default = /*#__PURE__*/__webpack_require__.n(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__);
var _this = undefined;





var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var Users = function() {
    var dispatch = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useDispatch)();
    var _useSelector = (0,react_redux__WEBPACK_IMPORTED_MODULE_2__.useSelector)(function(state) {
        return state.users;
    }), list = _useSelector.list, loading = _useSelector.loading, sortField = _useSelector.sortField;
    (0,react__WEBPACK_IMPORTED_MODULE_0__.useEffect)(function() {
        dispatch((0,_store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__.fetchUsers)());
    }, [
        dispatch
    ]);
    var handleSortChange = function(value) {
        dispatch((0,_store_slices_usersSlice__WEBPACK_IMPORTED_MODULE_3__.sortUsers)(value));
    };
    var columns = [
        {
            title: 'Name',
            dataIndex: 'name',
            key: 'name'
        },
        {
            title: 'Email',
            dataIndex: 'email',
            key: 'email'
        },
        {
            title: 'Role',
            dataIndex: 'role',
            key: 'role',
            render: function(role) {
                var color = role === 'Admin' ? 'red' : role === 'Manager' ? 'blue' : 'green';
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
                    color: color,
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                        lineNumber: 38,
                        columnNumber: 16
                    },
                    __self: _this
                }, role);
            }
        },
        {
            title: 'Status',
            dataIndex: 'status',
            key: 'status',
            render: function(status) {
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Tag, {
                    color: status === 'active' ? 'green' : 'default',
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                        lineNumber: 46,
                        columnNumber: 9
                    },
                    __self: _this
                }, status.toUpperCase());
            }
        },
        {
            title: 'Actions',
            key: 'actions',
            render: function(_, record) {
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
                    size: "middle",
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                        lineNumber: 55,
                        columnNumber: 9
                    },
                    __self: _this
                }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    type: "link",
                    icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__.EditOutlined, {
                        __source: {
                            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                            lineNumber: 58,
                            columnNumber: 19
                        }
                    }),
                    onClick: function() {
                        return console.log('Edit', record);
                    },
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                        lineNumber: 56,
                        columnNumber: 11
                    },
                    __self: _this
                }, "Edit"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
                    type: "link",
                    danger: true,
                    icon: /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_ant_design_icons__WEBPACK_IMPORTED_MODULE_4__.DeleteOutlined, {
                        __source: {
                            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                            lineNumber: 66,
                            columnNumber: 19
                        }
                    }),
                    onClick: function() {
                        return console.log('Delete', record);
                    },
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
                        lineNumber: 63,
                        columnNumber: 11
                    },
                    __self: _this
                }, "Delete"));
            }
        }
    ];
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 77,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Row, {
        justify: "space-between",
        align: "middle",
        style: {
            marginBottom: 24
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 78,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 79,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 2,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 80,
            columnNumber: 11
        },
        __self: _this
    }, "Users")), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Col, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 82,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 83,
            columnNumber: 11
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("span", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 84,
            columnNumber: 13
        },
        __self: _this
    }, "Sort by:"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Select, {
        value: sortField,
        onChange: handleSortChange,
        style: {
            width: 150
        },
        options: [
            {
                value: 'name',
                label: 'Name'
            },
            {
                value: 'email',
                label: 'Email'
            },
            {
                value: 'role',
                label: 'Role'
            }
        ],
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 85,
            columnNumber: 13
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Button, {
        type: "primary",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 95,
            columnNumber: 13
        },
        __self: _this
    }, "Add User")))), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Table, {
        columns: columns,
        dataSource: list,
        loading: loading,
        rowKey: "id",
        pagination: {
            pageSize: 10,
            showTotal: function(total) {
                return "Total ".concat(total, " users");
            }
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/host/src/pages/Users.jsx",
            lineNumber: 100,
            columnNumber: 7
        },
        __self: _this
    }));
};
// Import Row and Col

/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (Users);


}),
"../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js": 
/*!**************************************************************************************************!*\
  !*** ../../../node_modules/.pnpm/react-dom@18.3.1_react@18.3.1/node_modules/react-dom/client.js ***!
  \**************************************************************************************************/
(function (__unused_webpack_module, exports, __webpack_require__) {

var m = __webpack_require__(/*! react-dom */ "webpack/sharing/consume/default/react-dom/react-dom");
if (false) {} else {
    var i = m.__SECRET_INTERNALS_DO_NOT_USE_OR_YOU_WILL_BE_FIRED;
    exports.createRoot = function(c, o) {
        i.usingClientEntryPoint = true;
        try {
            return m.createRoot(c, o);
        } finally{
            i.usingClientEntryPoint = false;
        }
    };
    exports.hydrateRoot = function(c, h, o) {
        i.usingClientEntryPoint = true;
        try {
            return m.hydrateRoot(c, h, o);
        } finally{
            i.usingClientEntryPoint = false;
        }
    };
}


}),
"./src/store/index.js": 
/*!****************************!*\
  !*** ./src/store/index.js ***!
  \****************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  store: () => (store)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var _slices_dashboardSlice_js__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(/*! ./slices/dashboardSlice.js */ "./src/store/slices/dashboardSlice.js");
/* ESM import */var _slices_usersSlice_js__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./slices/usersSlice.js */ "./src/store/slices/usersSlice.js");
/* ESM import */var _slices_analyticsSlice_js__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./slices/analyticsSlice.js */ "./src/store/slices/analyticsSlice.js");




var store = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.configureStore)({
    reducer: {
        dashboard: _slices_dashboardSlice_js__WEBPACK_IMPORTED_MODULE_1__["default"],
        users: _slices_usersSlice_js__WEBPACK_IMPORTED_MODULE_2__["default"],
        analytics: _slices_analyticsSlice_js__WEBPACK_IMPORTED_MODULE_3__["default"]
    }
});


}),
"./src/store/slices/analyticsSlice.js": 
/*!********************************************!*\
  !*** ./src/store/slices/analyticsSlice.js ***!
  \********************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchAnalyticsData: () => (fetchAnalyticsData),
  setTimeRange: () => (setTimeRange)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchAnalyticsData = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('analytics/fetchData', function() {
    return _async_to_generator(function() {
        var months, generateData;
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(600)
                    ];
                case 1:
                    _state.sent();
                    months = [
                        'Jan',
                        'Feb',
                        'Mar',
                        'Apr',
                        'May',
                        'Jun'
                    ];
                    generateData = function() {
                        return months.map(function() {
                            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(100, 500);
                        });
                    };
                    return [
                        2,
                        {
                            revenue: {
                                labels: months,
                                datasets: [
                                    {
                                        label: 'Revenue',
                                        data: generateData(),
                                        borderColor: 'rgb(75, 192, 192)',
                                        backgroundColor: 'rgba(75, 192, 192, 0.2)'
                                    }
                                ]
                            },
                            userGrowth: {
                                labels: months,
                                datasets: [
                                    {
                                        label: 'New Users',
                                        data: generateData(),
                                        borderColor: 'rgb(54, 162, 235)',
                                        backgroundColor: 'rgba(54, 162, 235, 0.2)'
                                    }
                                ]
                            },
                            categories: {
                                labels: [
                                    'Desktop',
                                    'Mobile',
                                    'Tablet'
                                ],
                                datasets: [
                                    {
                                        data: [
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(30, 50),
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(30, 50),
                                            (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.random)(10, 20)
                                        ],
                                        backgroundColor: [
                                            'rgba(255, 99, 132, 0.6)',
                                            'rgba(54, 162, 235, 0.6)',
                                            'rgba(255, 206, 86, 0.6)'
                                        ]
                                    }
                                ]
                            }
                        }
                    ];
            }
        });
    })();
});
var analyticsSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'analytics',
    initialState: {
        data: null,
        loading: false,
        error: null,
        timeRange: '6months'
    },
    reducers: {
        setTimeRange: function(state, action) {
            state.timeRange = action.payload;
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchAnalyticsData.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchAnalyticsData.fulfilled, function(state, action) {
            state.loading = false;
            state.data = action.payload;
        }).addCase(fetchAnalyticsData.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var setTimeRange = analyticsSlice.actions.setTimeRange;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (analyticsSlice.reducer);


}),
"./src/store/slices/dashboardSlice.js": 
/*!********************************************!*\
  !*** ./src/store/slices/dashboardSlice.js ***!
  \********************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchDashboardStats: () => (fetchDashboardStats),
  resetDashboard: () => (resetDashboard)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchDashboardStats = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('dashboard/fetchStats', function() {
    return _async_to_generator(function() {
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(1000)
                    ];
                case 1:
                    _state.sent();
                    return [
                        2,
                        {
                            totalUsers: 12543,
                            activeUsers: 8921,
                            revenue: 458320,
                            growth: 12.5
                        }
                    ];
            }
        });
    })();
});
var dashboardSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'dashboard',
    initialState: {
        stats: null,
        loading: false,
        error: null
    },
    reducers: {
        resetDashboard: function(state) {
            state.stats = null;
            state.loading = false;
            state.error = null;
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchDashboardStats.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchDashboardStats.fulfilled, function(state, action) {
            state.loading = false;
            state.stats = action.payload;
        }).addCase(fetchDashboardStats.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var resetDashboard = dashboardSlice.actions.resetDashboard;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (dashboardSlice.reducer);


}),
"./src/store/slices/usersSlice.js": 
/*!****************************************!*\
  !*** ./src/store/slices/usersSlice.js ***!
  \****************************************/
(function (__unused_webpack___webpack_module__, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__),
  fetchUsers: () => (fetchUsers),
  sortUsers: () => (sortUsers),
  updateUserStatus: () => (updateUserStatus)
});
/* ESM import */var _reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! @reduxjs/toolkit */ "webpack/sharing/consume/default/@reduxjs/toolkit/@reduxjs/toolkit");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
function asyncGeneratorStep(gen, resolve, reject, _next, _throw, key, arg) {
    try {
        var info = gen[key](arg);
        var value = info.value;
    } catch (error) {
        reject(error);
        return;
    }
    if (info.done) {
        resolve(value);
    } else {
        Promise.resolve(value).then(_next, _throw);
    }
}
function _async_to_generator(fn) {
    return function() {
        var self = this, args = arguments;
        return new Promise(function(resolve, reject) {
            var gen = fn.apply(self, args);
            function _next(value) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "next", value);
            }
            function _throw(err) {
                asyncGeneratorStep(gen, resolve, reject, _next, _throw, "throw", err);
            }
            _next(undefined);
        });
    };
}
function _ts_generator(thisArg, body) {
    var f, y, t, _ = {
        label: 0,
        sent: function() {
            if (t[0] & 1) throw t[1];
            return t[1];
        },
        trys: [],
        ops: []
    }, g = Object.create((typeof Iterator === "function" ? Iterator : Object).prototype);
    return g.next = verb(0), g["throw"] = verb(1), g["return"] = verb(2), typeof Symbol === "function" && (g[Symbol.iterator] = function() {
        return this;
    }), g;
    function verb(n) {
        return function(v) {
            return step([
                n,
                v
            ]);
        };
    }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while(g && (g = 0, op[0] && (_ = 0)), _)try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [
                op[0] & 2,
                t.value
            ];
            switch(op[0]){
                case 0:
                case 1:
                    t = op;
                    break;
                case 4:
                    _.label++;
                    return {
                        value: op[1],
                        done: false
                    };
                case 5:
                    _.label++;
                    y = op[1];
                    op = [
                        0
                    ];
                    continue;
                case 7:
                    op = _.ops.pop();
                    _.trys.pop();
                    continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) {
                        _ = 0;
                        continue;
                    }
                    if (op[0] === 3 && (!t || op[1] > t[0] && op[1] < t[3])) {
                        _.label = op[1];
                        break;
                    }
                    if (op[0] === 6 && _.label < t[1]) {
                        _.label = t[1];
                        t = op;
                        break;
                    }
                    if (t && _.label < t[2]) {
                        _.label = t[2];
                        _.ops.push(op);
                        break;
                    }
                    if (t[2]) _.ops.pop();
                    _.trys.pop();
                    continue;
            }
            op = body.call(thisArg, _);
        } catch (e) {
            op = [
                6,
                e
            ];
            y = 0;
        } finally{
            f = t = 0;
        }
        if (op[0] & 5) throw op[1];
        return {
            value: op[0] ? op[1] : void 0,
            done: true
        };
    }
}


// Simulate API call
var fetchUsers = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createAsyncThunk)('users/fetchUsers', function() {
    return _async_to_generator(function() {
        return _ts_generator(this, function(_state) {
            switch(_state.label){
                case 0:
                    return [
                        4,
                        (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.delay)(800)
                    ];
                case 1:
                    _state.sent();
                    return [
                        2,
                        [
                            {
                                id: 1,
                                name: 'John Doe',
                                email: 'john@example.com',
                                role: 'Admin',
                                status: 'active'
                            },
                            {
                                id: 2,
                                name: 'Jane Smith',
                                email: 'jane@example.com',
                                role: 'User',
                                status: 'active'
                            },
                            {
                                id: 3,
                                name: 'Bob Johnson',
                                email: 'bob@example.com',
                                role: 'User',
                                status: 'inactive'
                            },
                            {
                                id: 4,
                                name: 'Alice Brown',
                                email: 'alice@example.com',
                                role: 'Manager',
                                status: 'active'
                            },
                            {
                                id: 5,
                                name: 'Charlie Wilson',
                                email: 'charlie@example.com',
                                role: 'User',
                                status: 'active'
                            }
                        ]
                    ];
            }
        });
    })();
});
var usersSlice = (0,_reduxjs_toolkit__WEBPACK_IMPORTED_MODULE_0__.createSlice)({
    name: 'users',
    initialState: {
        list: [],
        loading: false,
        error: null,
        sortField: 'name'
    },
    reducers: {
        sortUsers: function(state, action) {
            state.sortField = action.payload;
            state.list = (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.sortBy)(state.list, [
                action.payload
            ]);
        },
        updateUserStatus: function(state, action) {
            var user = state.list.find(function(u) {
                return u.id === action.payload.id;
            });
            if (user) {
                user.status = action.payload.status;
            }
        }
    },
    extraReducers: function(builder) {
        builder.addCase(fetchUsers.pending, function(state) {
            state.loading = true;
            state.error = null;
        }).addCase(fetchUsers.fulfilled, function(state, action) {
            state.loading = false;
            state.list = (0,lodash_es__WEBPACK_IMPORTED_MODULE_1__.sortBy)(action.payload, [
                state.sortField
            ]);
        }).addCase(fetchUsers.rejected, function(state, action) {
            state.loading = false;
            state.error = action.error.message;
        });
    }
});
var _usersSlice_actions = usersSlice.actions;
var sortUsers = _usersSlice_actions.sortUsers, updateUserStatus = _usersSlice_actions.updateUserStatus;
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (usersSlice.reducer);


}),

}]);
//# sourceMappingURL=src_bootstrap_jsx.js.map