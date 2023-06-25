// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Identifier;
use wander::lexer::{tokenize, Token};

#[test]
fn tokenize_boolean_true() {
    let input = "true";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Boolean(true)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_boolean_false() {
    let input = "false";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Boolean(false)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_booleans() {
    let input = "true false false";
    let res = tokenize(input);
    let expected = Ok(vec![
        Token::Boolean(true),
        Token::Boolean(false),
        Token::Boolean(false),
    ]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_integer() {
    let input = "123450";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Int(123450)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_integers() {
    let input = "0 -100 4200";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Int(0), Token::Int(-100), Token::Int(4200)]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_strings() {
    let input = "\"Hello world\"";
    let res = tokenize(input);
    let expected = Ok(vec![Token::String(String::from("Hello world"))]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_identifier() {
    let expected_identifier = Identifier::new("hello123").unwrap();
    let input = "<hello123>";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Identifier(expected_identifier)]);
    assert_eq!(res, expected);
}
