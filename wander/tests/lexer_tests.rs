// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2/.

use ligature::Element;
use wander::lexer::{tokenize_and_filter, Token};

#[test]
fn tokenize_true() {
    let input = "true";
    let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.clone()).collect();
    let expected = vec![Token::Element(Element("true".to_owned()))];
    assert_eq!(res, expected);
}

#[test]
fn tokenize_booleans() {
    let input = "true false false";
    let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.clone()).collect();
    let expected = vec![
        Token::Element(Element("true".to_owned())),
        Token::Element(Element("false".to_owned())),
        Token::Element(Element("false".to_string())),
    ];
    assert_eq!(res, expected);
}

#[test]
fn tokenize_integer() {
    let input = "123450";
    let res: Vec<Token> = tokenize_and_filter(input).iter().map(|t| t.first().unwrap().clone()).collect();
    let expected = vec![Token::Element(Element("123450".to_owned()))];
    assert_eq!(res, expected);
}

#[test]
fn tokenize_integers() {
    let input = "0 -100 4200";
    let res: Vec<_> = tokenize_and_filter(input).unwrap().iter().map(|t| t.clone()).collect();
    let expected = vec![Token::Element(Element("0".to_owned())), Token::Element(Element("-100".to_owned())), Token::Element(Element("4200".to_owned()))];
    assert_eq!(res, expected);
}

#[test]
fn tokenize_strings() {
    let input = "\"Hello, world\"";
    let res = tokenize_and_filter(input).unwrap().first().unwrap().clone();
    let expected = Token::String("Hello, world".to_owned());
    assert_eq!(res, expected);
}

#[test]
fn tokenize_strings_with_quotes() {
    let input = "\"\\\"Hello, world\\\"\"";
    let res = tokenize_and_filter(input).unwrap().first().unwrap().clone();
    let expected = Token::String(String::from("\\\"Hello, world\\\""));
    assert_eq!(res, expected);
}

#[test]
fn tokenize_name() {
    let input = "hello123";
    let res = tokenize_and_filter(input).unwrap().first().unwrap().clone();
    let expected = Token::Element(Element("hello123".to_owned()));
    assert_eq!(res, expected);
}

#[test]
fn tokenize_function_call() {
    let input = "not false";
    let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.clone()).collect();
    let expected = vec![
        Token::Element(Element("not".to_owned())),
        Token::Element(Element("false".to_owned())),
    ];
    assert_eq!(res, expected);
}

// #[test]
// fn tokenize_function_call_with_pipe() {
//     let input = "false | not";
//     let res = tokenize_and_filter(input);
//     let expected = Ok(vec![
//         Location(Token::Boolean(false), 0),
//         Location(Token::Pipe, 6),
//         Location(Token::Name(String::from("not")), 8),
//         Location(Token::OpenParen, 11),
//         Location(Token::CloseParen, 12),
//     ]);
//     assert_eq!(res, expected);
// }

#[test]
fn tokenize_comment() {
    let input = "-- hello";
    let res = tokenize_and_filter(input);
    let expected = Ok(vec![]);
    assert_eq!(res, expected);
}

#[test]
fn tokenize_complex_comment() {
    let input = "-- <<<>>> () {} }{ )( ><";
    let res = tokenize_and_filter(input);
    let expected = Ok(vec![]);
    assert_eq!(res, expected);
}

#[test]
fn multiline_comment() {
    let input = "-- <<<>>> () {} }{ )( ><\n5 -- five\n-- comment";
    let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.clone()).collect();;
    let expected = vec![Token::Element(Element("5".to_owned()))];
    assert_eq!(res, expected);
}
