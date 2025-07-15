//! Debug tests to identify exactly where multi-pass optimization is needed
//! These tests will examine the step-by-step process

use crate::optimize::optimize;
use serde_json::json;

/// Test the exact case that fails - DCE not removing unused function chain after macro removal
#[test]
fn test_debug_dce_failure_case() {
    // This is the exact case from the failing test
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
    println!("DEBUG DCE FAILURE CASE:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    // Let's see what exactly remains
    println!("\nAnalysis:");
    println!("Contains 'function a': {}", result.contains("function a"));
    println!("Contains 'function b': {}", result.contains("function b"));
    println!("Contains 'function c': {}", result.contains("function c"));
    println!("Contains 'let result': {}", result.contains("let result"));
    println!("Contains 'a()': {}", result.contains("a()"));
    println!("Contains 'main': {}", result.contains("main"));
    
    // The issue: functions a, b, c should be removed because the call to a() is removed
    // But DCE isn't detecting that they're unused after macro removal
}

/// Test a simpler case to isolate the DCE behavior
#[test]
fn test_simple_unused_function_dce() {
    let source = r#"
        function unusedFunction() {
            return "I am unused";
        }
        
        let main = "main code";
        console.log(main);
        "#;

    let config = json!({});

    let result = optimize(source.to_string(), config);
    println!("SIMPLE UNUSED FUNCTION TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'unusedFunction': {}", result.contains("unusedFunction"));
    
    // This should remove the unused function - if it doesn't, DCE isn't working
}

/// Test what happens with the chain after MANUAL removal of the call
#[test]
fn test_manual_call_removal() {
    let source = r#"
        function a() { return b(); }
        function b() { return c(); }
        function c() { return "unused chain"; }
        
        // No call to a() at all
        
        let main = "main code";
        "#;

    let config = json!({});

    let result = optimize(source.to_string(), config);
    println!("MANUAL CALL REMOVAL TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'function a': {}", result.contains("function a"));
    println!("Contains 'function b': {}", result.contains("function b"));
    println!("Contains 'function c': {}", result.contains("function c"));
}

/// Test macro processing in isolation
#[test]
fn test_macro_processing_only() {
    let source = r#"
        /*@:if condition="REMOVE_ME"*/
        let shouldBeRemoved = "this should be gone";
        /*@:endif*/
        
        /*@:if condition="KEEP_ME"*/
        let shouldStay = "this should remain";
        /*@:endif*/
        
        let always = "always here";
        "#;

    let config = json!({
        "REMOVE_ME": false,
        "KEEP_ME": true
    });

    let result = optimize(source.to_string(), config);
    println!("MACRO PROCESSING ONLY TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'shouldBeRemoved': {}", result.contains("shouldBeRemoved"));
    println!("Contains 'shouldStay': {}", result.contains("shouldStay"));
    println!("Contains 'always': {}", result.contains("always"));
}

/// Test webpack tree shaking in isolation with no macros
#[test]
fn test_webpack_only_no_macros() {
    let source = r#"
        var __webpack_modules__ = {
            "1": function() {
                return "module 1";
            },
            "2": function() {
                return "module 2 - unreachable";
            }
        };
        
        // No entry points - should remove all modules
        "#;

    let config = json!({});

    let result = optimize(source.to_string(), config);
    println!("WEBPACK ONLY (NO MACROS) TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains '__webpack_modules__': {}", result.contains("__webpack_modules__"));
}

/// Test the interaction between macros and webpack modules
#[test]
fn test_macro_webpack_interaction() {
    let source = r#"
        /*@:if condition="INCLUDE_WEBPACK"*/
        var __webpack_modules__ = {
            "1": function() {
                return "conditional webpack module";
            }
        };
        /*@:endif*/
        
        let other = "other code";
        "#;

    let config = json!({
        "INCLUDE_WEBPACK": false
    });

    let result = optimize(source.to_string(), config);
    println!("MACRO-WEBPACK INTERACTION TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains '__webpack_modules__': {}", result.contains("__webpack_modules__"));
    println!("Contains 'other': {}", result.contains("other"));
}

/// Test variable usage after macro removal
#[test]
fn test_variable_usage_after_macro_removal() {
    let source = r#"
        let sharedVar = "shared";
        
        /*@:if condition="USE_SHARED"*/
        console.log(sharedVar);
        /*@:endif*/
        
        let independentVar = "independent";
        console.log(independentVar);
        "#;

    let config = json!({
        "USE_SHARED": false
    });

    let result = optimize(source.to_string(), config);
    println!("VARIABLE USAGE AFTER MACRO REMOVAL TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'sharedVar': {}", result.contains("sharedVar"));
    println!("Contains 'independentVar': {}", result.contains("independentVar"));
    
    // sharedVar should be removed because its only usage is in a removed macro block
}

/// Test what happens with nested function calls after macro removal
#[test]
fn test_nested_function_calls_after_macro() {
    let source = r#"
        function level1() {
            return level2();
        }
        
        function level2() {
            return level3();
        }
        
        function level3() {
            return "deep result";
        }
        
        /*@:if condition="USE_DEEP_CALL"*/
        let result = level1();
        /*@:endif*/
        "#;

    let config = json!({
        "USE_DEEP_CALL": false
    });

    let result = optimize(source.to_string(), config);
    println!("NESTED FUNCTION CALLS AFTER MACRO TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'level1': {}", result.contains("level1"));
    println!("Contains 'level2': {}", result.contains("level2"));
    println!("Contains 'level3': {}", result.contains("level3"));
    
    // All three functions should be removed since the entry point call is removed
}

/// Test two-step dependency removal
#[test]
fn test_two_step_dependency_removal() {
    let source = r#"
        function utilityA() {
            return "utility A";
        }
        
        function utilityB() {
            return "utility B";
        }
        
        /*@:if condition="FEATURE_1"*/
        function feature1() {
            return utilityA();
        }
        /*@:endif*/
        
        /*@:if condition="FEATURE_2"*/  
        function feature2() {
            return utilityB();
        }
        /*@:endif*/
        
        // If both features are disabled, both utilities should be removable
        let main = "main";
        "#;

    let config = json!({
        "FEATURE_1": false,
        "FEATURE_2": false
    });

    let result = optimize(source.to_string(), config);
    println!("TWO-STEP DEPENDENCY REMOVAL TEST:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("{}", result);
    
    println!("Contains 'utilityA': {}", result.contains("utilityA"));
    println!("Contains 'utilityB': {}", result.contains("utilityB"));
    println!("Contains 'feature1': {}", result.contains("feature1"));
    println!("Contains 'feature2': {}", result.contains("feature2"));
    
    // This tests if utilities are removed after their consumers are removed
}