// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use wander::{preludes::common, run, NoHostType, WanderValue};

// #[test]
// fn basic_record() {
//     let input = "{a = 24}";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let res = format!("{res}");
//     let res = run(&res, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let mut record = HashMap::new();
//     record.insert("a".to_owned(), WanderValue::Int(24));
//     let expected = WanderValue::Record(record);
//     assert_eq!(res, expected);
// }

// //#[test]
// fn nested_record() {
//     let input = "{a = 24 b = 123 c = {d = '(321)}}";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let res = format!("{res}");
//     let res = run(&res, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let mut record = HashMap::new();
//     record.insert("a".to_owned(), WanderValue::Int(24));
//     record.insert("b".to_owned(), WanderValue::Int(123));

//     let mut inner_record = HashMap::new();
//     inner_record.insert(
//         "d".to_owned(),
//         WanderValue::Tuple(vec![WanderValue::Int(321)]),
//     );

//     record.insert("c".to_owned(), WanderValue::Record(inner_record));

//     let expected = WanderValue::Record(record);
//     assert_eq!(res, expected);
// }

// #[test]
// fn record_field_access() {
//     let input = "let val x = {a = 24 b = true} in x.b end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Bool(true);
//     assert_eq!(res, expected);
// }

// #[test]
// fn nested_record_field_access() {
//     let input = "let val x = {a = 45 b = {a = 45}} in x.b.a end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Int(45);
//     assert_eq!(res, expected);
// }

// #[test]
// fn nested_record_field_access2() {
//     let input = "let val x = {a = 24 b = {a = [] b = {c = 45}}} in x.b.b.c end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Int(45);
//     assert_eq!(res, expected);
// }

// #[test]
// fn missing_record_field_access() {
//     let input = "let val x = (a = 24 b = true) in x.c end";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().is_err();
//     assert!(res);
// }

// #[test]
// fn nested_missing_record_field_access() {
//     let input = "val x = (a = 24 b = (a = [], b = (c = 45))) x.b.b.d";
//     let binding = run(input, &mut common::<NoHostType>());
//     let res = binding.first().unwrap();
//     assert!(res.is_err());
// }

// #[test]
// fn nested_missing_record_field_access2() {
//     let input = "val x = (a = 24 b = (a = [], b = (c = 45))) x.c.b.d";
//     let res = run(input, &mut common::<NoHostType>());
//     let res = res.first().unwrap();
//     assert!(res.is_err());
// }
