// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, NoHostType, WanderValue};

// #[test]
// fn basic_let() {
//     let input = "let val x = 5 in x end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Int(5);
//     assert_eq!(res, expected);
// }

// #[test]
// fn basic_let_multiple_vals() {
//     let input = "let val x = true val y = Bool.and x in y x end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn nested_lets() {
//     let input = r#"
//         let
//           val x = true
//           val y =
//               let
//                 val y1 = x
//               in
//                 y1
//               end
//         in
//           y
//         end"#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn lets_with_function_calls_in_decl() {
//     let input = r#"
//         let
//             val x = true
//             val y = Bool.and x false
//         in
//             y
//         end
//     "#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// fn nested_lets_with_function_calls() {
//     let input = r#"
//         let
//           val x = true
//           val y =
//               let
//                 val x = true
//                 val y = Bool.and x false
//               in
//                 y
//               end
//         in
//           y
//         end
//         "#;
//     let res: Vec<WanderValue<NoHostType>> = run(input, &mut common::<NoHostType>())
//       .iter().map(|t| t.unwrap()).collect();
//     let expected = vec![WanderValue::Bool(false)];
//     assert_eq!(res, expected);
// }
