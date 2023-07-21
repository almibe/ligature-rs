// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Identifier;
use wander::{preludes::common, run, WanderValue};

#[test]
fn run_wander_true() {
    let input = "true";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_integer() {
    let input = "-100";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Int(-100));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_string() {
    let input = "\"Hello world\"";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::String(String::from("Hello world")));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_identifier() {
    let expected_identifier = Identifier::new("hello").unwrap();
    let input = "<hello>";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Identifier(expected_identifier));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding() {
    let input = "let x = true";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding_and_reference() {
    let input = "let x = true x";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn run_native_function() {
    let input = "not(true)";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Boolean(false));
    assert_eq!(res, expected);
}

#[test]
fn run_nested_function_calls() {
    let input = "not(not(false))";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Boolean(false));
    assert_eq!(res, expected);
}

#[test]
fn run_scope() {
    let input = "let x = {true 5 6} {x}";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Int(6));
    assert_eq!(res, expected);
}

#[test]
fn run_conditional() {
    let input = "if true if not(true) 5 else 6 else 7";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Int(6));
    assert_eq!(res, expected);
}

#[test]
fn run_list() {
    let input = "[1 [2] []]";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::List(vec![
        WanderValue::Int(1),
        WanderValue::List(vec![WanderValue::Int(2)]),
        WanderValue::List(vec![]),
    ]));
    assert_eq!(res, expected);
}

#[test]
fn run_lambda() {
    let input = "let id = { x -> x } id(5)";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::Int(5));
    assert_eq!(res, expected);
}

#[test]
fn run_lambda_with_function() {
    let input = "let id = { x -> x } id([not(true) not(false)])";
    let res = run(input, &mut common());
    let expected = Ok(WanderValue::List(vec![
        WanderValue::Boolean(false),
        WanderValue::Boolean(true),
    ]));
    assert_eq!(res, expected);
}
