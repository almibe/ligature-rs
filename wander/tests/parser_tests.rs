// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::lexer::Token;
use wander::parser::{parse, Element};

#[test]
fn parse_booleans() {
    let input = vec![
        Token::Boolean(true),
        Token::Boolean(false),
        Token::Boolean(true),
    ];
    let res = parse(input);
    let expected = Ok(vec![
        Element::Boolean(true),
        Element::Boolean(false),
        Element::Boolean(true),
    ]);
    assert_eq!(res, expected);
}

#[test]
fn parse_integers() {
    let input = vec![Token::Int(0), Token::Int(-100), Token::Int(4200)];
    let res = parse(input);
    let expected = Ok(vec![
        Element::Int(0),
        Element::Int(-100),
        Element::Int(4200),
    ]);
    assert_eq!(res, expected);
}

#[test]
fn parse_strings() {
    let input = vec![
        Token::String(String::from("Hello")),
        Token::String(String::from("This is a test")),
    ];
    let res = parse(input);
    let expected = Ok(vec![
        Element::String(String::from("Hello")),
        Element::String(String::from("This is a test")),
    ]);
    assert_eq!(res, expected);
}

#[test]
fn parse_name() {
    let input = vec![Token::Name(String::from("test"))];
    let expected = Ok(vec![Element::Name(String::from("test"))]);
    let res = parse(input);
    assert_eq!(res, expected);
}

#[test]
fn parse_let_binding() {
    let input = vec![
        Token::Let,
        Token::Name(String::from("x")),
        Token::EqualSign,
        Token::Int(5),
    ];
    let res = parse(input);
    let expected = Ok(vec![Element::Let(
        String::from("x"),
        Box::new(Element::Int(5)),
    )]);
    assert_eq!(res, expected)
}

#[test]
fn parse_function_call() {
    let input = vec![
        Token::Name(String::from("test")),
        Token::OpenParen,
        Token::Boolean(false),
        Token::CloseParen,
    ];
    let res = parse(input);
    let expected = Ok(vec![Element::FunctionCall(
        String::from("test"),
        vec![Element::Boolean(false)],
    )]);
    assert_eq!(res, expected);
}

#[test]
fn parse_empty_scope() {
    let input = vec![
        Token::OpenBrace,
        Token::CloseBrace,
    ];
    let res = parse(input);
    let expected = Ok(vec![Element::Scope(vec![])]);
    assert_eq!(res, expected);
}

#[test]
fn parse_nested_scopes() {
    let input = vec![
        Token::OpenBrace,
        Token::Int(5),
        Token::OpenBrace,
        Token::Boolean(false),
        Token::CloseBrace,
        Token::CloseBrace,
    ];
    let res = parse(input);
    let expected = Ok(vec![
        Element::Scope(vec![
            Element::Int(5),
            Element::Scope(vec![
                Element::Boolean(false)
            ])
        ])
    ]);
    assert_eq!(res, expected);
}
