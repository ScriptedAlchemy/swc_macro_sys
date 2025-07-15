//! Debug tests specifically for macro transformer behavior

use crate::optimize::optimize;
use serde_json::json;

/// Test to see exactly how macro transformer handles different constructs
#[test]
fn test_macro_transformer_statement_removal() {
    let source = r#"
        let before = "before";
        
        /*@:if condition="REMOVE_STMT"*/
        let toRemove = "should be gone";
        /*@:endif*/
        
        let after = "after";
        "#;

    let config = json!({
        "REMOVE_STMT": false
    });

    let result = optimize(source.to_string(), config.clone());
    println!("MACRO TRANSFORMER STATEMENT REMOVAL:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("'{}'", result);
    
    // Let's see what exactly remains character by character
    println!("\nCharacter analysis:");
    for (i, ch) in result.chars().enumerate() {
        if !ch.is_whitespace() {
            println!("  {}: '{}'", i, ch);
        }
    }
}

/// Test macro transformer with expression removal
#[test]
fn test_macro_transformer_expression_removal() {
    let source = r#"
        let result = /*@:if condition="INCLUDE_EXPR"*/ getValue() /*@:endif*/;
        "#;

    let config = json!({
        "INCLUDE_EXPR": false
    });

    let result = optimize(source.to_string(), config.clone());
    println!("MACRO TRANSFORMER EXPRESSION REMOVAL:");
    println!("Input source:");
    println!("{}", source);
    println!("\nAfter optimization:");
    println!("'{}'", result);
}

/// Test the specific failing case with detailed analysis
#[test]
fn test_failing_case_step_by_step() {
    // Step 1: Apply only macro transformation
    let source = r#"function a() { return b(); }
function b() { return c(); }
function c() { return "unused chain"; }

/*@:if condition="USE_CHAIN"*/
let result = a();
/*@:endif*/

let main = "main code";"#;

    let config = json!({
        "USE_CHAIN": false
    });

    println!("STEP BY STEP ANALYSIS:");
    println!("Original source:");
    println!("{}", source);
    
    let result = optimize(source.to_string(), config.clone());
    println!("\nAfter full optimization:");
    println!("'{}'", result);
    
    // Let's also test with a simpler version
    let simple_source = r#"/*@:if condition="USE_CHAIN"*/
let result = a();
/*@:endif*/"#;
    
    let simple_result = optimize(simple_source.to_string(), config.clone());
    println!("\nSimple case result:");
    println!("'{}'", simple_result);
}

/// Test to see if the issue is with let statements specifically
#[test]
fn test_let_statement_macro_removal() {
    let source = r#"
        /*@:if condition="TEST"*/
        let x = someFunction();
        /*@:endif*/
        "#;

    let config = json!({
        "TEST": false
    });

    let result = optimize(source.to_string(), config.clone());
    println!("LET STATEMENT MACRO REMOVAL:");
    println!("Input: {}", source);
    println!("Output: '{}'", result);
}

/// Test with different statement types
#[test]
fn test_different_statement_types() {
    let source = r#"
        /*@:if condition="TEST"*/
        console.log("test");
        /*@:endif*/
        
        /*@:if condition="TEST"*/
        let x = 5;
        /*@:endif*/
        
        /*@:if condition="TEST"*/
        functionCall();
        /*@:endif*/
        "#;

    let config = json!({
        "TEST": false
    });

    let result = optimize(source.to_string(), config.clone());
    println!("DIFFERENT STATEMENT TYPES:");
    println!("Input: {}", source);
    println!("Output: '{}'", result);
}