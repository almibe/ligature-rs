// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;
use wander::preludes::common;
use wander::{lexer::Token, run, ScriptValue, TokenTransformer, WanderError};

struct EmptyTransformer {}
impl TokenTransformer for EmptyTransformer {
    fn transform(&self, _input: &[Token]) -> Result<Vec<Token>, WanderError> {
        Ok(vec![])
    }
}

struct NothingTransformer {}
impl TokenTransformer for NothingTransformer {
    fn transform(&self, _input: &[Token]) -> Result<Vec<Token>, WanderError> {
        Ok([Token::Nothing].to_vec())
    }
}

struct UpperCaseTransformer {}
impl TokenTransformer for UpperCaseTransformer {
    fn transform(&self, input: &[Token]) -> Result<Vec<Token>, WanderError> {
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
fn empty_transformer_no_input_test() {
    let input = "Empty.empty``";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "Empty".to_owned(),
        "empty".to_owned(),
        Rc::new(EmptyTransformer {}),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_no_input_test() {
    let input = "None.none``";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "None".to_owned(),
        "none".to_owned(),
        Rc::new(NothingTransformer {}),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_none() {
    let input = "None.none`this (will) >>  [be ] {ignored}} `";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "None".to_owned(),
        "none".to_owned(),
        Rc::new(NothingTransformer {}),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_upper() {
    let input = "Case.upper`\"test\"`";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "Case".to_owned(),
        "upper".to_owned(),
        Rc::new(UpperCaseTransformer {}),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(ScriptValue::String("TEST".to_owned()));
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_graph_empty() {
    let input = "Graph.graph``";
    let res = run(input, &mut common());
    let expected = run("Graph.graph([])", &mut common());
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_graph() {
    let input = "Graph.graph`<a> <b> <c>`";
    let res = run(input, &mut common());
    let expected = run("Graph.graph([(<a> <b> <c>)])", &mut common());
    assert_eq!(res, expected);
}
