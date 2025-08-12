"use strict";
(self["webpackChunk_mf_react_remote"] = self["webpackChunk_mf_react_remote"] || []).push([["src_components_ChartWidget_jsx"], {
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

}]);
//# sourceMappingURL=src_components_ChartWidget_jsx.7eaccdad9c4d399f.js.map