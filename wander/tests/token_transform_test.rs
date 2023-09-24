// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::rc::Rc;
use wander::preludes::common;
use wander::{lexer::Token, run, TokenTransformer, WanderError, WanderValue};

fn empty_transform(_input: &[Token]) -> Result<Vec<Token>, WanderError> {
    Ok(vec![])
}

fn nothing_transform(_input: &[Token]) -> Result<Vec<Token>, WanderError> {
    Ok([Token::Nothing].to_vec())
}

fn upper_case_transform(input: &[Token]) -> Result<Vec<Token>, WanderError> {
    if let Some(Token::String(value)) = input.get(0) {
        let t = value.clone().to_ascii_uppercase();
        let t = Token::String(t);
        Ok(vec![t])
    } else {
        panic!()
    }
}

#[test]
fn empty_transformer_no_input_test() {
    let input = "Empty.empty``";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "Empty".to_owned(),
        "empty".to_owned(),
        Rc::new(empty_transform),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(WanderValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_no_input_test() {
    let input = "None.none``";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "None".to_owned(),
        "none".to_owned(),
        Rc::new(nothing_transform),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(WanderValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_none() {
    let input = "None.none`this (will) >>  [be ] {ignored}} `";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "None".to_owned(),
        "none".to_owned(),
        Rc::new(nothing_transform),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(WanderValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn token_transformer_upper() {
    let input = "Case.upper`\"test\"`";
    let mut bindings = common();
    bindings.bind_token_transformer(
        "Case".to_owned(),
        "upper".to_owned(),
        Rc::new(upper_case_transform),
    );
    let res = run(input, &mut bindings);
    let expected = Ok(WanderValue::String("TEST".to_owned()));
    assert_eq!(res, expected);
}
