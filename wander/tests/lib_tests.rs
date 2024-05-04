// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, NoHostType, WanderValue};

// #[test]
// fn run_wander_true() {
//     let input = "true";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Bool(true));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_wander_integer() {
//     let input = "-100";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Int(-100));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_wander_string() {
//     let input = "\"Hello world\"";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::String(String::from("Hello world")));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_wander_let_binding() {
//     let input = "let val x = true end";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Nothing);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_wander_let_binding_and_reference() {
//     let input = "let val x = true in x end";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Bool(true));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_native_function() {
//     let input = "Bool.not true";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Bool(false));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_nested_function_calls() {
//     let input = "Bool.not (Bool.not false)";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Bool(false));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_scope() {
//     let input = "let val a = true val b = 5 val c = 6 in c end";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Int(6));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_conditional() {
//     let input = "if true then if Bool.not true then 5 else 6 end else 7 end";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Int(6));
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_list() {
//     let input = "[1 [2] []]";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::List(vec![
//         WanderValue::Int(1),
//         WanderValue::List(vec![WanderValue::Int(2)]),
//         WanderValue::List(vec![]),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_tuple() {
//     let input = "'('() '(1 '(2) '()))";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Tuple(vec![
//         WanderValue::Tuple(vec![]),
//         WanderValue::Tuple(vec![
//             WanderValue::Int(1),
//             WanderValue::Tuple(vec![WanderValue::Int(2)]),
//             WanderValue::Tuple(vec![]),
//         ]),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_lambda() {
//     let input = "let val id = \\x -> x in id 5 end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Int(5);
//     assert_eq!(res, expected);
// }

// #[test]
// fn host_function_calls() {
//     let input = r#"Bool.not true"#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// fn run_lambda_with_host_function_calls() {
//     let input = r#"
//     let
//         val id = \x -> x
//     in
//         id [(Bool.not true) (Bool.not false)]
//     end"#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::List(vec![
//         WanderValue::Bool(false),
//         WanderValue::Bool(true),
//     ]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn pipe_operator() {
//     let input = "true | Bool.not";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// // #[test]
// // fn write_float_literals() {
// //     assert_eq!(write_value(&Value::FloatLiteral(5.5)), "5.5");
// //     assert_eq!(write_value(&Value::FloatLiteral(5f64)), "5.0");
// // }
