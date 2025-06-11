use webpack_graph::{WebpackBundleParser, Result};

#[test]
fn test_debug_optimized_output_parsing() {
    // This test analyzes the actual optimized.js output to understand why tree shaking isn't working
    let optimized_content = r#"
(()=>{
    "use strict";
    var __webpack_modules__ = {
        418: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                V: ()=>dataProcessor
            });
            var dataProcessor = {
                processLargeDataset (data) {
                    console.log("Processing ".concat(data.length, " items - this is expensive data processing!"));
                    return data.map((item)=>({
                            id: item,
                            processed: true,
                            timestamp: Date.now(),
                            metadata: {
                                processed: true,
                                heavy: 'computation'
                            }
                        }));
                },
                aggregateData (datasets) {
                    console.log('Aggregating multiple datasets - heavy computation!');
                    return datasets.reduce((acc, dataset)=>acc.concat(dataset), []);
                },
                transformComplexData (input) {
                    console.log('Complex data transformation - should be tree-shaken if unused!');
                    return {
                        transformed: input,
                        complexity: 'high'
                    };
                }
            };
        },
        153: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                v: ()=>featureA
            });
            var _heavyMathUtils_ts__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__(78);
            var _dataProcessor_ts__WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__(418);
            function featureA() {
                console.log('FeatureA: Using heavy math utilities...');
                var result = _heavyMathUtils_ts__WEBPACK_IMPORTED_MODULE_0__.D.fibonacci(10);
                console.log('FeatureA: Processing complex data...');
                var processedData = _dataProcessor_ts__WEBPACK_IMPORTED_MODULE_1__.V.processLargeDataset([
                    1,
                    2,
                    3,
                    4,
                    5
                ]);
                return "FeatureA: Computed fibonacci(10)=".concat(result, ", processed ").concat(processedData.length, " items");
            }
        },
        78: function(__unused_webpack_module, __webpack_exports__, __webpack_require__) {
            __webpack_require__.d(__webpack_exports__, {
                D: ()=>heavyMathUtils
            });
            var heavyMathUtils = {
                fibonacci (n) {
                    console.log("Computing fibonacci(".concat(n, ") - this is expensive!"));
                    if (n <= 1) return n;
                    return this.fibonacci(n - 1) + this.fibonacci(n - 2);
                },
                primeFactors (n) {
                    console.log("Computing prime factors of ".concat(n, " - another heavy operation!"));
                    var factors = [];
                    for(var i = 2; i <= n; i++){
                        while(n % i === 0){
                            factors.push(i);
                            n /= i;
                        }
                    }
                    return factors;
                },
                matrixMultiply (a, b) {
                    console.log('Performing matrix multiplication - very expensive!');
                    return [
                        [
                            1,
                            2
                        ],
                        [
                            3,
                            4
                        ]
                    ];
                }
            };
        }
    };
    var __webpack_module_cache__ = {};
    function __webpack_require__(moduleId) {
        var cachedModule = __webpack_module_cache__[moduleId];
        if (cachedModule !== undefined) {
            return cachedModule.exports;
        }
        var module = __webpack_module_cache__[moduleId] = {
            exports: {}
        };
        __webpack_modules__[moduleId](module, module.exports, __webpack_require__);
        return module.exports;
    }
    (()=>{
        __webpack_require__.d = (exports, definition)=>{
            for(var key in definition){
                if (__webpack_require__.o(definition, key) && !__webpack_require__.o(exports, key)) {
                    Object.defineProperty(exports, key, {
                        enumerable: true,
                        get: definition[key]
                    });
                }
            }
        };
    })();
    (()=>{
        __webpack_require__.o = (obj, prop)=>Object.prototype.hasOwnProperty.call(obj, prop);
    })();
    (()=>{
        __webpack_require__.rv = ()=>"1.3.12";
    })();
    (()=>{
        __webpack_require__.ruid = "bundler=rspack@1.3.12";
    })();
    (()=>{
        console.log('=== Tree Shaking Demo ===');
        console.log('Main application started - base functionality always included');
    })();
})();
"#;

    let parser = WebpackBundleParser::new().expect("Failed to create parser");
    let graph = parser.parse_bundle(optimized_content).expect("Failed to parse optimized bundle");

    // Show all modules and their dependencies
    let mut modules: Vec<_> = graph.modules.iter().collect();
    modules.sort_by(|(id_a, _), (id_b, _)| {
        // Proper sorting: numeric IDs first (by value), then string IDs (alphabetically)
        match (id_a.parse::<u32>(), id_b.parse::<u32>()) {
            (Ok(a), Ok(b)) => a.cmp(&b),           // Both numeric: sort by value
            (Ok(_), Err(_)) => std::cmp::Ordering::Less,    // Numeric comes before string
            (Err(_), Ok(_)) => std::cmp::Ordering::Greater, // String comes after numeric
            (Err(_), Err(_)) => id_a.cmp(id_b),    // Both string: sort alphabetically
        }
    });
    
    // Reachability analysis
    let reachable = graph.get_reachable_modules();
    let mut unreachable = graph.get_unreachable_modules();
    unreachable.sort();
    
    // This should be the key test - with 0 entry points, ALL modules should be unreachable
    assert_eq!(graph.entry_points.len(), 0, "Should detect 0 entry points in optimized output");
    assert_eq!(unreachable.len(), graph.modules.len(), "All modules should be unreachable with 0 entry points");
} 