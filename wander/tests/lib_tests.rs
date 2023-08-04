// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;

use ligature::Identifier;
use wander::{preludes::common, run, ScriptValue, TokenTransformer, lexer::Token};

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

struct NothingTransformer {}
impl TokenTransformer for NothingTransformer {
    fn transform(&self, _input: &[wander::lexer::Token]) -> Result<Vec<Token>, ligature::LigatureError> {
        Ok([Token::Nothing].to_vec())
    }
}

struct UpperCaseTransformer {}
impl TokenTransformer for UpperCaseTransformer {
    fn transform(&self, input: &[wander::lexer::Token]) -> Result<Vec<Token>, ligature::LigatureError> {
        if let Some(Token::String(value)) = input.get(0) {
            let t = value.clone().to_ascii_uppercase();
            let t = Token::String(t);
            Ok(vec![t])
        } else {
            panic!()
        }
    }
}

#[test]
fn token_transformer_no_input_test() {
    let input = "none``";
    let mut bindings = common();
    bindings.bind_token_transformer("none".to_owned(), Rc::new(NothingTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_test() {
    let input = "none`this (will) >>  [be ] {ignored}} `";
    let mut bindings = common();
    bindings.bind_token_transformer("none".to_owned(), Rc::new(NothingTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_test2() {
    let input = "upper`\"test\"`";
    let mut bindings = common();
    bindings.bind_token_transformer("upper".to_owned(), Rc::new(UpperCaseTransformer {}));
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::String("TEST".to_owned()));
    assert_eq!(res, expected);
}
