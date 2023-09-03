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

#[test]
fn tokenize_name() {
    let input = "hello123";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Name(String::from("hello123"))]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_names_keywords_and_symbols() {
    let input = "let x = 5";
    let res = tokenize(input);
    let expected = Ok(vec![
        Token::Let,
        Token::Name(String::from("x")),
        Token::EqualSign,
        Token::Int(5),
    ]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_function_call() {
    let input = "not(false)";
    let res = tokenize(input);
    let expected = Ok(vec![
        Token::Name(String::from("not")),
        Token::OpenParen,
        Token::Boolean(false),
        Token::CloseParen,
    ]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_function_call_with_forward() {
    let input = "false >> not()";
    let res = tokenize(input);
    let expected = Ok(vec![
        Token::Boolean(false),
        Token::Forward,
        Token::Name(String::from("not")),
        Token::OpenParen,
        Token::CloseParen,
    ]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_comment() {
    let input = "--hello";
    let res = tokenize(input);
    let expected = Ok(vec![]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_complex_comment() {
    let input = "-- <<<>>> () {} }{ )( ><";
    let res = tokenize(input);
    let expected = Ok(vec![]);
    assert_eq!(res, expected);
}

#[test]
fn multiline_comment() {
    let input = "-- <<<>>> () {} }{ )( ><\n5--five\n--comment";
    let res = tokenize(input);
    let expected = Ok(vec![Token::Int(5)]);
    assert_eq!(res, expected);
}
