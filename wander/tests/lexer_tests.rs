// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{lexer::{tokenize_and_filter, Token}, Location};

// #[test]
// fn tokenize_boolean_true() {
//     let input = "true";
//     let res = tokenize_and_filter(input).unwrap().iter().map(|t| t.0).collect();
//     let expected = Ok(vec![Token::Boolean(true)]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_boolean_false() {
//     let input = "false";
//     let res = tokenize_and_filter(input).iter().map(|t| t.first().unwrap().0).collect();
//     let expected = Ok(vec![Token::Boolean(false)]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_booleans() {
//     let input = "true false false";
//     let res = tokenize_and_filter(input).iter().map(|t| t.0).collect();
//     let expected = Ok(vec![
//         Token::Boolean(true),
//         Token::Boolean(false),
//         Token::Boolean(false),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_integer() {
//     let input = "123450";
//     let res = tokenize_and_filter(input).iter().map(|t| t.first().unwrap().0).collect();
//     let expected = Ok(vec![Token::Int(123450)]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_integers() {
//     let input = "0 -100 4200";
//     let res: Vec<_> = tokenize_and_filter(input).unwrap().iter().map(|t| t.0).collect();
//     let expected = vec![Token::Int(0), Token::Int(-100), Token::Int(4200)];
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_strings() {
//     let input = "\"Hello, world\"";
//     let res = tokenize_and_filter(input).unwrap().first().unwrap().0;
//     let expected = Token::String(String::from("Hello, world"));
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_strings_with_quotes() {
//     let input = "\"\\\"Hello, world\\\"\"";
//     let res = tokenize_and_filter(input).unwrap().first().unwrap().0;
//     let expected = Token::String(String::from("\\\"Hello, world\\\""));
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_name() {
//     let input = "hello123";
//     let res = tokenize_and_filter(input).unwrap().first().unwrap().0;
//     let expected = Token::Name(String::from("hello123"));
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_names_keywords_and_symbols() {
//     let input = "val x = 5";
//     let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.0).collect();
//     let expected = vec![
//         Token::Val,
//         Token::Name(String::from("x")),
//         Token::EqualSign,
//         Token::Int(5),
//     ];
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_function_call() {
//     let input = "not(false)";
//     let res: Vec<Token> = tokenize_and_filter(input).unwrap().iter().map(|t| t.0).collect();
//     let expected = vec![
//         Token::Name(String::from("not")),
//         Token::OpenParen,
//         Token::Boolean(false),
//         Token::CloseParen,
//     ];
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_function_call_with_pipe() {
//     let input = "false | not()";
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

// #[test]
// fn tokenize_comment() {
//     let input = "--hello";
//     let res = tokenize_and_filter(input);
//     let expected = Ok(vec![]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn tokenize_complex_comment() {
//     let input = "-- <<<>>> () {} }{ )( ><";
//     let res = tokenize_and_filter(input);
//     let expected = Ok(vec![]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn multiline_comment() {
//     let input = "-- <<<>>> () {} }{ )( ><\n5--five\n--comment";
//     let res = tokenize_and_filter(input);
//     let expected = Ok(vec![Location(Token::Int(5), 0)]);
//     assert_eq!(res, expected);
// }
