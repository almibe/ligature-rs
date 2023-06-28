// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Identifier;
use wander::{run, WanderValue};

#[test]
fn run_wander_true() {
    let input = "true";
    let res = run(input);
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_integer() {
    let input = "-100";
    let res = run(input);
    let expected = Ok(WanderValue::Int(-100));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_string() {
    let input = "\"Hello world\"";
    let res = run(input);
    let expected = Ok(WanderValue::String(String::from("Hello world")));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_identifier() {
    let expected_identifier = Identifier::new("hello").unwrap();
    let input = "<hello>";
    let res = run(input);
    let expected = Ok(WanderValue::Identifier(expected_identifier));
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding() {
    let input = "let x = true";
    let res = run(input);
    let expected = Ok(WanderValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn run_wander_let_binding_and_reference() {
    let input = "let x = true x";
    let res = run(input);
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}

//#[test]
fn run_native_function() {
    let input = "not(false)";
    let res = run(input);
}
