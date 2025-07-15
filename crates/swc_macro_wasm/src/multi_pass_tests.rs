//! Tests to identify scenarios that require multi-pass optimization
//! and understand the interaction between macro transforms, DCE, and webpack tree shaking

use crate::optimize::optimize;
use serde_json::json;

    /// Test case 1: Macro removal creates new unused variables that need additional DCE
    #[test]
    fn test_macro_removal_creates_unused_vars() {
        let source = r#"
        /*@:if condition="DEBUG"*/
        let debugVar = "debug info";
        let processedVar = debugVar + " processed";
        /*@:endif*/
        
        let normalVar = "production";
        console.log(normalVar);
        "#;

        let config = json!({
            "DEBUG": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 1 - After macro removal and single pass:");
        println!("{}", result);
        
        // Check if unused variables were properly cleaned up
        assert!(!result.contains("debugVar"));
        assert!(!result.contains("processedVar"));
        assert!(result.contains("normalVar"));
    }

    /// Test case 2: Chained dependencies where removing one creates another unused item
    #[test]
    fn test_chained_unused_dependencies() {
        let source = r#"
        function utilityFunction() {
            return "utility";
        }
        
        /*@:if condition="FEATURE_A"*/
        function featureA() {
            return utilityFunction();
        }
        /*@:endif*/
        
        /*@:if condition="FEATURE_B"*/
        function featureB() {
            return utilityFunction();
        }
        /*@:endif*/
        
        let mainValue = "main";
        "#;

        let config = json!({
            "FEATURE_A": false,
            "FEATURE_B": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 2 - After chained dependency removal:");
        println!("{}", result);
        
        // utilityFunction should be removed as it's no longer used
        assert!(!result.contains("utilityFunction"));
        assert!(!result.contains("featureA"));
        assert!(!result.contains("featureB"));
        assert!(result.contains("mainValue"));
    }

    /// Test case 3: Variable assignments that become no-ops after macro removal
    #[test]
    fn test_variable_assignments_after_macro_removal() {
        let source = r#"
        let config = {};
        
        /*@:if condition="ENABLE_FEATURE"*/
        config.feature = true;
        /*@:endif*/
        
        /*@:if condition="ENABLE_DEBUG"*/
        config.debug = true;
        /*@:endif*/
        
        // If both conditions are false, config remains empty object
        // and subsequent usage might be optimizable
        let finalConfig = config;
        "#;

        let config = json!({
            "ENABLE_FEATURE": false,
            "ENABLE_DEBUG": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 3 - After removing conditional assignments:");
        println!("{}", result);
        
        // Check if empty object optimizations occurred
        assert!(result.contains("config"));
    }

    /// Test case 4: Webpack module removal cascading effects
    #[test]
    fn test_webpack_module_cascading_removal() {
        let source = r#"
        var __webpack_modules__ = {
            "module1": function() {
                /*@:if condition="USE_MODULE1"*/
                return "module1 content";
                /*@:endif*/
            },
            "module2": function() {
                // Module2 depends on module1
                var mod1 = __webpack_modules__["module1"];
                return mod1 ? mod1() + " extended" : "fallback";
            },
            "module3": function() {
                // Module3 depends on module2
                var mod2 = __webpack_modules__["module2"];
                return mod2();
            }
        };
        
        // No entry points - this should remove all modules
        "#;

        let config = json!({
            "USE_MODULE1": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 4 - After webpack module cascading removal:");
        println!("{}", result);
        
        // All modules should be removed due to no entry points
        assert!(!result.contains("__webpack_modules__"));
    }

    /// Test case 5: Complex nested conditionals with interdependencies
    #[test]
    fn test_nested_conditionals_with_dependencies() {
        let source = r#"
        /*@:if condition="OUTER_FEATURE"*/
        let outerVar = "outer";
        
        /*@:if condition="INNER_FEATURE"*/
        let innerVar = outerVar + " inner";
        
        function processInner() {
            return innerVar.toUpperCase();
        }
        /*@:endif*/
        
        function processOuter() {
            /*@:if condition="INNER_FEATURE"*/
            return processInner();
            /*@:endif*/
            return outerVar;
        }
        /*@:endif*/
        
        let result = "default";
        /*@:if condition="OUTER_FEATURE"*/
        result = processOuter();
        /*@:endif*/
        "#;

        let config = json!({
            "OUTER_FEATURE": true,
            "INNER_FEATURE": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 5 - After nested conditional processing:");
        println!("{}", result);
        
        // Should keep outer but remove inner
        assert!(result.contains("outerVar"));
        assert!(!result.contains("innerVar"));
        assert!(!result.contains("processInner"));
        assert!(result.contains("processOuter"));
    }

    /// Test case 6: Function parameters and return values after macro removal
    #[test]
    fn test_function_parameters_after_macro_removal() {
        let source = r#"
        function processData(input, options) {
            let processed = input;
            
            /*@:if condition="ENABLE_VALIDATION"*/
            if (!options.skipValidation) {
                processed = validateInput(processed);
            }
            /*@:endif*/
            
            /*@:if condition="ENABLE_TRANSFORMATION"*/
            processed = transformInput(processed);
            /*@:endif*/
            
            return processed;
        }
        
        /*@:if condition="ENABLE_VALIDATION"*/
        function validateInput(input) {
            return input ? input : "default";
        }
        /*@:endif*/
        
        /*@:if condition="ENABLE_TRANSFORMATION"*/
        function transformInput(input) {
            return input.toString().toUpperCase();
        }
        /*@:endif*/
        "#;

        let config = json!({
            "ENABLE_VALIDATION": false,
            "ENABLE_TRANSFORMATION": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 6 - After function parameter optimization:");
        println!("{}", result);
        
        // Check if unused parameters are optimized
        assert!(!result.contains("validateInput"));
        assert!(!result.contains("transformInput"));
        assert!(result.contains("processData"));
    }

    /// Test case 7: Object property access after conditional removal
    #[test]
    fn test_object_property_access_optimization() {
        let source = r#"
        let config = {
            /*@:if condition="FEATURE_A"*/
            featureA: {
                enabled: true,
                value: "a"
            },
            /*@:endif*/
            /*@:if condition="FEATURE_B"*/
            featureB: {
                enabled: true,
                value: "b"
            },
            /*@:endif*/
            common: "shared"
        };
        
        let result = config.common;
        /*@:if condition="FEATURE_A"*/
        result += config.featureA.value;
        /*@:endif*/
        /*@:if condition="FEATURE_B"*/
        result += config.featureB.value;
        /*@:endif*/
        "#;

        let config = json!({
            "FEATURE_A": false,
            "FEATURE_B": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 7 - After object property access optimization:");
        println!("{}", result);
        
        // Check object simplification
        assert!(!result.contains("featureA"));
        assert!(!result.contains("featureB"));
        assert!(result.contains("common"));
    }

    /// Test case 8: Testing the actual DCE loop behavior
    #[test]
    fn test_dce_loop_convergence() {
        // This test examines the internal DCE loop in perform_dce
        let source = r#"
        function a() { return b(); }
        function b() { return c(); }
        function c() { return "unused chain"; }
        
        /*@:if condition="USE_CHAIN"*/
        let result = a();
        /*@:endif*/
        
        let main = "main code";
        "#;

        let config = json!({
            "USE_CHAIN": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 8 - DCE loop convergence test:");
        println!("{}", result);
        
        // All functions in the unused chain should be removed
        assert!(!result.contains("function a"));
        assert!(!result.contains("function b"));
        assert!(!result.contains("function c"));
        assert!(result.contains("main"));
    }

    /// Test case 9: Testing webpack tree shaking iteration behavior
    #[test]
    fn test_webpack_tree_shaking_iterations() {
        let source = r#"
        var __webpack_modules__ = {
            "1": function(module, exports, __webpack_require__) {
                /*@:if condition="USE_MODULE_1"*/
                exports.value = "module1";
                /*@:endif*/
            },
            "2": function(module, exports, __webpack_require__) {
                var mod1 = __webpack_require__("1");
                exports.value = mod1.value + " extended";
            },
            "3": function(module, exports, __webpack_require__) {
                var mod2 = __webpack_require__("2");
                exports.value = mod2.value + " final";
            }
        };
        
        // No entry points should cause all modules to be removed
        "#;

        let config = json!({
            "USE_MODULE_1": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 9 - Webpack tree shaking iterations:");
        println!("{}", result);
        
        // All modules should be removed
        assert!(!result.contains("__webpack_modules__"));
    }

    /// Test case 10: Testing multiple optimization passes interaction
    #[test]
    fn test_multi_pass_interaction() {
        let source = r#"
        // Setup with macros that create patterns requiring multiple passes
        /*@:if condition="STEP_1"*/
        let step1 = "first";
        /*@:endif*/
        
        /*@:if condition="STEP_2"*/
        let step2 = step1 + " second";
        /*@:endif*/
        
        /*@:if condition="STEP_3"*/
        let step3 = step2 + " third";
        /*@:endif*/
        
        // Final usage
        /*@:if condition="USE_RESULT"*/
        console.log(step3);
        /*@:endif*/
        
        let independent = "always here";
        "#;

        let config = json!({
            "STEP_1": true,
            "STEP_2": false,
            "STEP_3": false,
            "USE_RESULT": false
        });

        let result = optimize(source.to_string(), config);
        println!("Test 10 - Multi-pass interaction test:");
        println!("{}", result);
        
        // Only step1 should remain but be unused, so might get removed in additional passes
        assert!(result.contains("independent"));
        // step1 might or might not be removed depending on DCE effectiveness
        println!("Contains step1: {}", result.contains("step1"));
    }