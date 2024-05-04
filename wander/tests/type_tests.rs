// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//use crate::utilities::introspect_str;
use wander::interpreter::Expression;
use wander::parser::Element;

#[path = "utilities.rs"]
mod utilities;

// #[test]
// fn parse_tagged_name_in_let() {
//     let res = introspect_str("let val x: Int = 5 end");
//     let expected = Element::Let(
//         vec![(
//             "x".to_owned(),
//             Some("Int".to_owned()),
//             Element::Grouping(vec![Element::Int(5)]),
//         )],
//         Box::new(Element::Nothing),
//     );
//     assert_eq!(res.element, expected);
//     let expected = Expression::Let(
//         vec![(
//             "x".to_owned(),
//             Some(Expression::Name("Int".to_owned())),
//             Expression::Int(5),
//         )],
//         Box::new(Expression::Nothing),
//     );
//     assert_eq!(res.expression, expected);
// }

// #[test]
// fn parse_tagged_name_in_lambda() {
//     let res = introspect_str("let val x = \\y: Any -> y end");
//     let expected = Element::Let(
//         vec![(
//             "x".to_owned(),
//             None,
//             Element::Grouping(vec![Element::Lambda(
//                 "y".to_owned(),
//                 Some("Any".to_owned()),
//                 None,
//                 Box::new(Element::Grouping(vec![Element::Name("y".to_owned())])),
//             )]),
//         )],
//         Box::new(Element::Nothing),
//     );
//     assert_eq!(res.element, expected);
//     let expected = Expression::Let(
//         vec![(
//             "x".to_owned(),
//             None,
//             Expression::Lambda(
//                 "y".to_owned(),
//                 Some("Any".to_owned()),
//                 None,
//                 Box::new(Element::Grouping(vec![Element::Name("y".to_owned())])),
//             ),
//         )],
//         Box::new(Expression::Nothing),
//     );
//     assert_eq!(res.expression, expected);
// }
