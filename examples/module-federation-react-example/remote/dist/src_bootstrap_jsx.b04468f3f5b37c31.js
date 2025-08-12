"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_bootstrap_jsx"], {
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
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var _components_UserCard__WEBPACK_IMPORTED_MODULE_2__ = __webpack_require__(/*! ./components/UserCard */ "./src/components/UserCard.jsx");
/* ESM import */var _components_DataTable__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./components/DataTable */ "./src/components/DataTable.jsx");
/* ESM import */var _components_ChartWidget__WEBPACK_IMPORTED_MODULE_4__ = __webpack_require__(/*! ./components/ChartWidget */ "./src/components/ChartWidget.jsx");
/* ESM import */var _components_FormBuilder__WEBPACK_IMPORTED_MODULE_5__ = __webpack_require__(/*! ./components/FormBuilder */ "./src/components/FormBuilder.jsx");
var _this = undefined;






var Title = antd__WEBPACK_IMPORTED_MODULE_1__.Typography.Title;
var App = function() {
    var handleFormSubmit = function(values) {
        console.log('Form submitted:', values);
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 16,
            columnNumber: 5
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 1,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 17,
            columnNumber: 7
        },
        __self: _this
    }, "Remote Components Library"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Alert, {
        message: "Module Federation Remote App",
        description: "This app exposes React components that can be consumed by other applications. All components below are available for remote consumption.",
        type: "info",
        showIcon: true,
        style: {
            marginBottom: 32
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 19,
            columnNumber: 7
        },
        __self: _this
    }), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_1__.Space, {
        direction: "vertical",
        size: "large",
        style: {
            width: '100%'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 27,
            columnNumber: 7
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 28,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 29,
            columnNumber: 11
        },
        __self: _this
    }, "UserCard Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_UserCard__WEBPACK_IMPORTED_MODULE_2__["default"], {
        user: {
            name: 'Jane Doe',
            email: 'jane.doe@example.com',
            avatar: 'https://api.dicebear.com/7.x/avataaars/svg?seed=Jane',
            role: 'Product Manager',
            department: 'Product',
            joinDate: '2021-06-15'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 30,
            columnNumber: 11
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 42,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 43,
            columnNumber: 11
        },
        __self: _this
    }, "DataTable Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_DataTable__WEBPACK_IMPORTED_MODULE_3__["default"], {
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 44,
            columnNumber: 11
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 47,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 48,
            columnNumber: 11
        },
        __self: _this
    }, "ChartWidget Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_ChartWidget__WEBPACK_IMPORTED_MODULE_4__["default"], {
        type: "bar",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 49,
            columnNumber: 11
        },
        __self: _this
    })), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        className: "component-demo",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 52,
            columnNumber: 9
        },
        __self: _this
    }, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(Title, {
        level: 3,
        className: "component-title",
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 53,
            columnNumber: 11
        },
        __self: _this
    }, "FormBuilder Component"), /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_components_FormBuilder__WEBPACK_IMPORTED_MODULE_5__["default"], {
        fields: [
            {
                type: 'text',
                name: 'username',
                label: 'Username',
                required: true
            },
            {
                type: 'email',
                name: 'email',
                label: 'Email',
                required: true
            },
            {
                type: 'password',
                name: 'password',
                label: 'Password',
                required: true
            },
            {
                type: 'select',
                name: 'role',
                label: 'Role',
                options: [
                    'Admin',
                    'User',
                    'Guest'
                ],
                required: true
            }
        ],
        onSubmit: handleFormSubmit,
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/App.jsx",
            lineNumber: 54,
            columnNumber: 11
        },
        __self: _this
    }))));
};
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
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! antd */ "webpack/sharing/consume/default/antd/antd");
/* ESM import */var antd__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(antd__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var _App__WEBPACK_IMPORTED_MODULE_3__ = __webpack_require__(/*! ./App */ "./src/App.jsx");




var root = react_dom_client__WEBPACK_IMPORTED_MODULE_1__.createRoot(document.getElementById('root'));
root.render(/*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement((react__WEBPACK_IMPORTED_MODULE_0___default().StrictMode), {
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/bootstrap.jsx",
        lineNumber: 9,
        columnNumber: 3
    },
    __self: undefined
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(antd__WEBPACK_IMPORTED_MODULE_2__.ConfigProvider, {
    theme: {
        token: {
            colorPrimary: '#1890ff',
            borderRadius: 6
        }
    },
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/bootstrap.jsx",
        lineNumber: 10,
        columnNumber: 5
    },
    __self: undefined
}, /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(_App__WEBPACK_IMPORTED_MODULE_3__["default"], {
    __source: {
        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/bootstrap.jsx",
        lineNumber: 18,
        columnNumber: 7
    },
    __self: undefined
}))));


}),
"./src/components/ChartWidget.jsx": 
/*!****************************************!*\
  !*** ./src/components/ChartWidget.jsx ***!
  \****************************************/
(function (__unused_webpack_module, __webpack_exports__, __webpack_require__) {
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  "default": () => (__WEBPACK_DEFAULT_EXPORT__)
});
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0__ = /* #__PURE__ */ __webpack_require__(/*! react */ "webpack/sharing/consume/default/react/react");
/* ESM import */var react__WEBPACK_IMPORTED_MODULE_0___default = /*#__PURE__*/__webpack_require__.n(react__WEBPACK_IMPORTED_MODULE_0__);
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__ = /* #__PURE__ */ __webpack_require__(/*! react-chartjs-2 */ "webpack/sharing/consume/default/react-chartjs-2/react-chartjs-2");
/* ESM import */var react_chartjs_2__WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__);
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_2__ = /* #__PURE__ */ __webpack_require__(/*! chart.js */ "webpack/sharing/consume/default/chart.js/chart.js");
/* ESM import */var chart_js__WEBPACK_IMPORTED_MODULE_2___default = /*#__PURE__*/__webpack_require__.n(chart_js__WEBPACK_IMPORTED_MODULE_2__);
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3__ = /* #__PURE__ */ __webpack_require__(/*! lodash-es */ "webpack/sharing/consume/default/lodash-es/lodash-es");
/* ESM import */var lodash_es__WEBPACK_IMPORTED_MODULE_3___default = /*#__PURE__*/__webpack_require__.n(lodash_es__WEBPACK_IMPORTED_MODULE_3__);
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
var _this = undefined;




chart_js__WEBPACK_IMPORTED_MODULE_2__.Chart.register(chart_js__WEBPACK_IMPORTED_MODULE_2__.CategoryScale, chart_js__WEBPACK_IMPORTED_MODULE_2__.LinearScale, chart_js__WEBPACK_IMPORTED_MODULE_2__.PointElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.LineElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.BarElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.ArcElement, chart_js__WEBPACK_IMPORTED_MODULE_2__.Title, chart_js__WEBPACK_IMPORTED_MODULE_2__.Tooltip, chart_js__WEBPACK_IMPORTED_MODULE_2__.Legend);
var ChartWidget = function(param) {
    var _param_type = param.type, type = _param_type === void 0 ? 'line' : _param_type, propData = param.data, propOptions = param.options, _param_height = param.height, height = _param_height === void 0 ? 300 : _param_height;
    var defaultData = (0,react__WEBPACK_IMPORTED_MODULE_0__.useMemo)(function() {
        var labels = [
            'January',
            'February',
            'March',
            'April',
            'May',
            'June',
            'July'
        ];
        var dataset1 = labels.map(function() {
            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 100);
        });
        var dataset2 = labels.map(function() {
            return (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 100);
        });
        switch(type){
            case 'pie':
            case 'doughnut':
                return {
                    labels: [
                        'Red',
                        'Blue',
                        'Yellow',
                        'Green',
                        'Purple'
                    ],
                    datasets: [
                        {
                            data: [
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50),
                                (0,lodash_es__WEBPACK_IMPORTED_MODULE_3__.random)(10, 50)
                            ],
                            backgroundColor: [
                                'rgba(255, 99, 132, 0.6)',
                                'rgba(54, 162, 235, 0.6)',
                                'rgba(255, 206, 86, 0.6)',
                                'rgba(75, 192, 192, 0.6)',
                                'rgba(153, 102, 255, 0.6)'
                            ],
                            borderColor: [
                                'rgba(255, 99, 132, 1)',
                                'rgba(54, 162, 235, 1)',
                                'rgba(255, 206, 86, 1)',
                                'rgba(75, 192, 192, 1)',
                                'rgba(153, 102, 255, 1)'
                            ],
                            borderWidth: 1
                        }
                    ]
                };
            default:
                return {
                    labels: labels,
                    datasets: [
                        {
                            label: 'Dataset 1',
                            data: dataset1,
                            borderColor: 'rgb(255, 99, 132)',
                            backgroundColor: 'rgba(255, 99, 132, 0.5)',
                            tension: 0.1
                        },
                        {
                            label: 'Dataset 2',
                            data: dataset2,
                            borderColor: 'rgb(53, 162, 235)',
                            backgroundColor: 'rgba(53, 162, 235, 0.5)',
                            tension: 0.1
                        }
                    ]
                };
        }
    }, [
        type
    ]);
    var defaultOptions = {
        responsive: true,
        maintainAspectRatio: false,
        plugins: {
            legend: {
                position: 'top'
            },
            title: {
                display: true,
                text: "Sample ".concat(type.charAt(0).toUpperCase() + type.slice(1), " Chart")
            }
        }
    };
    var chartData = propData || defaultData;
    var chartOptions = propOptions || defaultOptions;
    var renderChart = function() {
        var props = {
            data: chartData,
            options: chartOptions,
            height: height
        };
        switch(type){
            case 'line':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Line, _object_spread_props(_object_spread({}, props), {
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
                        lineNumber: 110,
                        columnNumber: 16
                    },
                    __self: _this
                }));
            case 'bar':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Bar, _object_spread_props(_object_spread({}, props), {
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
                        lineNumber: 112,
                        columnNumber: 16
                    },
                    __self: _this
                }));
            case 'pie':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Pie, _object_spread_props(_object_spread({}, props), {
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
                        lineNumber: 114,
                        columnNumber: 16
                    },
                    __self: _this
                }));
            case 'doughnut':
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Doughnut, _object_spread_props(_object_spread({}, props), {
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
                        lineNumber: 116,
                        columnNumber: 16
                    },
                    __self: _this
                }));
            default:
                return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement(react_chartjs_2__WEBPACK_IMPORTED_MODULE_1__.Line, _object_spread_props(_object_spread({}, props), {
                    __source: {
                        fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
                        lineNumber: 118,
                        columnNumber: 16
                    },
                    __self: _this
                }));
        }
    };
    return /*#__PURE__*/ react__WEBPACK_IMPORTED_MODULE_0___default().createElement("div", {
        style: {
            height: height,
            position: 'relative'
        },
        __source: {
            fileName: "/Users/zackjackson/swc_macro_sys/examples/module-federation-react-example/remote/src/components/ChartWidget.jsx",
            lineNumber: 123,
            columnNumber: 5
        },
        __self: _this
    }, renderChart());
};
/* ESM default export */ const __WEBPACK_DEFAULT_EXPORT__ = (ChartWidget);


}),
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
//# sourceMappingURL=src_bootstrap_jsx.b04468f3f5b37c31.js.map