// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, LigatureError, Value};
use wander::{preludes::common, run, write_identifier, write_value, ScriptValue};

#[test]
fn run_wander_true() {
    let input = "true";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_integer() {
    let input = "-100";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Int(-100));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_string() {
    let input = "\"Hello world\"";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::String(String::from("Hello world")));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_identifier() {
    let expected_identifier = Identifier::new("hello").unwrap();
    let input = "<hello>";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Identifier(expected_identifier));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding() {
    let input = "let x = true";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding_and_reference() {
    let input = "let x = true x";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn run_native_function() {
    let input = "not(true)";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Boolean(false));
    assert_eq!(res, expected);
}

#[test]
fn run_nested_function_calls() {
    let input = "not(not(false))";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Boolean(false));
    assert_eq!(res, expected);
}

#[test]
fn run_scope() {
    let input = "let x = {true 5 6} {x}";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Int(6));
    assert_eq!(res, expected);
}

#[test]
fn run_conditional() {
    let input = "if true if not(true) 5 else 6 else 7";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Int(6));
    assert_eq!(res, expected);
}

#[test]
fn run_list() {
    let input = "[1 [2] []]";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::List(vec![
        ScriptValue::Int(1),
        ScriptValue::List(vec![ScriptValue::Int(2)]),
        ScriptValue::List(vec![]),
    ]));
    assert_eq!(res, expected);
}

#[test]
fn run_tuple() {
    let input = "() (1 (2) ())";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Tuple(vec![
        ScriptValue::Int(1),
        ScriptValue::Tuple(vec![ScriptValue::Int(2)]),
        ScriptValue::Tuple(vec![]),
    ]));
    assert_eq!(res, expected);
}

#[test]
fn run_lambda() {
    let input = "let id = { x -> x } id(5)";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Int(5));
    assert_eq!(res, expected);
}

#[test]
fn run_lambda_with_function() {
    let input = "let id = { x -> x } id([not(true) not(false)])";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::List(vec![
        ScriptValue::Boolean(false),
        ScriptValue::Boolean(true),
    ]));
    assert_eq!(res, expected);
}

#[test]
fn forward_operator() {
    let input = "true >> not()";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Boolean(false));
    assert_eq!(res, expected);
}

#[test]
fn write_entities() -> Result<(), LigatureError> {
    let e = Identifier::new("test")?;
    assert_eq!(write_identifier(&e), "<test>".to_string());
    Ok(())
}

#[test]
fn write_string_literals() {
    assert_eq!(write_value(&Value::String("test".to_string())), "\"test\"");
}

#[test]
fn write_integer_literals() {
    assert_eq!(write_value(&Value::Integer(5)), "5");
}

// #[test]
// fn write_float_literals() {
//     assert_eq!(write_value(&Value::FloatLiteral(5.5)), "5.5");
//     assert_eq!(write_value(&Value::FloatLiteral(5f64)), "5.0");
// }

#[test]
fn write_bytes_literals() {
    assert_eq!(write_value(&Value::Bytes(vec![0, 255])), "0x00ff");
}
