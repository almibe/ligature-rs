// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{parser::Element, preludes::common, run, NoHostType, WanderValue};

#[test]
fn basic_currying() {
    let input = r#"
    let
      isTrue = Bool.and true
    in 
      [(isTrue true) (isTrue false)]
    end
    "#;
    let res = run(input, &mut common::<NoHostType>()).clone().unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common::<NoHostType>()).clone().unwrap();
    let expected = WanderValue::List(vec![WanderValue::Bool(true), WanderValue::Bool(false)]);
    assert_eq!(res, expected);
}

// #[test]
// fn currying_with_lambda() {
//     let input = r#"
//         let
//           val and = \x y -> Bool.and(x y)
//           val isTrue = and true
//         in
//           [true false]
//           --[isTrue true isTrue false]
//         end
//         "#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let res = format!("{res}");
//     let res = run(&res, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::List(vec![WanderValue::Bool(true), WanderValue::Bool(false)]);
//     assert_eq!(res, expected);
// }

// #[test]
// fn currying_twice_with_lambda() {
//     let input = r#"
//         let
//             val and3 = \x y z -> Bool.and x (Bool.and y z)
//             val and = and3 true 
//             val isTrue = and true
//         in
//             and (isTrue true) (isTrue false)
//         end
//     "#;
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     // let res = format!("{res}");
//     // let res = run(&res, &mut common::<NoHostType>()).unwrap();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }

// #[test]
// fn parse_lambda() {
//     let input = "\\x -> x";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Lambda(
//         "x".to_owned(),
//         None,
//         None,
//         Box::new(Element::Grouping(vec![Element::Name("x".to_owned())])),
//     );
//     assert_eq!(res, expected);
// }

// #[test]
// fn parse_multi_line_lambda() {
//     let input = "\\x -> let val x = true in x end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Lambda(
//         "x".to_owned(),
//         None,
//         None,
//         Box::new(Element::Let(
//             vec![(
//                 "x".to_owned(),
//                 None,
//                 Element::Grouping(vec![Element::Boolean(true)]),
//             )],
//             Box::new(Element::Grouping(vec![Element::Name("x".to_owned())])),
//         )),
//     );
//     assert_eq!(res, expected);
// }

// #[test]
// #[ignore = "Revisit when working on https://github.com/almibe/wander/issues/37"]
// fn multi_param_lambda() {
//     let input = "Core.eq (\\x y -> x) (\\x -> \\y -> x)";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn define_and_call_lambda() {
//     let input = "let val x = \\x -> true in x 45 end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn define_and_partially_call_lambda() {
//     let input = "let val x = \\x y -> 31 in x 5 end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Lambda(
//         "y".to_owned(),
//         None,
//         None,
//         Box::new(Element::Grouping(vec![Element::Int(31)])),
//     );
//     assert_eq!(res, expected);
// }

// #[test]
// fn group_a_value() {
//     let input = "(true)";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn group_a_function_call() {
//     let input = "Bool.and false (Bool.not true)";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().as_ref();
//     let expected = WanderValue::Bool(false);
//     assert_eq!(res, expected);
// }
