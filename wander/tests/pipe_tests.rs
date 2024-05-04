// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::parser::Element;
use wander::{run, NoHostType, WanderValue};

//use crate::utilities::{introspect_str, parse_str};
use wander::interpreter::Expression;
use wander::preludes::common;
use wander::translation::translate;

#[path = "utilities.rs"]
mod utilities;

// #[test]
// fn parse_pipe_value_to_name() {
//     let res = introspect_str("false | not");
//     let expected = Element::Grouping(vec![
//         Element::Grouping(vec![Element::Boolean(false)]),
//         Element::Pipe,
//         Element::Grouping(vec![Element::Name("not".to_owned())]),
//     ]);
//     assert_eq!(res.element, expected);
//     let expected = Expression::Application(vec![
//         Expression::Name("not".to_owned()),
//         Expression::Boolean(false),
//     ]);
//     assert_eq!(res.expression, expected);
// }

// #[test]
// fn parse_pipe_value_to_application() {
//     let res = parse_str("false | Bool.and true");
//     let expected = Element::Grouping(vec![
//         Element::Grouping(vec![Element::Boolean(false)]),
//         Element::Pipe,
//         Element::Grouping(vec![
//             Element::Name("Bool.and".to_owned()),
//             Element::Boolean(true),
//         ]),
//     ]);
//     assert_eq!(res, expected);
//     let res = translate(res).unwrap();
//     let expected = Expression::Application(vec![
//         Expression::Name("Bool.and".to_owned()),
//         Expression::Boolean(true),
//         Expression::Boolean(false),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn parse_pipe_application_to_application() {
//     let res = parse_str("Bool.not false | Bool.and true");
//     let expected = Element::Grouping(vec![
//         Element::Grouping(vec![
//             Element::Name("Bool.not".to_owned()),
//             Element::Boolean(false),
//         ]),
//         Element::Pipe,
//         Element::Grouping(vec![
//             Element::Name("Bool.and".to_owned()),
//             Element::Boolean(true),
//         ]),
//     ]);
//     assert_eq!(res, expected);
//     let res = translate(res).unwrap();
//     let expected = Expression::Application(vec![
//         Expression::Name("Bool.and".to_owned()),
//         Expression::Boolean(true),
//         Expression::Application(vec![
//             Expression::Name("Bool.not".to_owned()),
//             Expression::Boolean(false),
//         ]),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_pipe_value_to_name() {
//     let res = run("false | Bool.not", &mut common::<NoHostType>());
//     let res = res.first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_pipe_value_to_application() {
//     let res = run("false | Bool.and true", &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_pipe_application_to_application() {
//     let res = run(
//         "Bool.not false | Bool.and true",
//         &mut common::<NoHostType>(),
//     ).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_multiple_pipes() {
//     let res = run(
//         "Bool.not false | Bool.and true | Bool.not",
//         &mut common::<NoHostType>(),
//     ).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// #[ignore = "https://github.com/almibe/wander/issues/35"]
// fn run_pipe_in_let_expression() {
//     let res = run("let in true | Bool.not end", &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// #[ignore = "https://github.com/almibe/wander/issues/35"]
// fn run_pipe_in_let_expression_decl() {
//     let res = run(
//         "let val x = true | Bool.not in x end",
//         &mut common::<NoHostType>(),
//     ).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }
