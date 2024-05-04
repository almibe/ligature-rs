// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, WanderValue};

// #[test]
// fn read_write_test_strings() {
//     let input = vec![
//         "\"\"".to_owned(),
//         "\"hello, world\"".to_owned(),
//         "\"hello,\\nworld\"".to_owned(),
//     ];
//     let res: Vec<WanderValue<String>> = input
//         .iter()
//         .map(|s| run(s, &mut common()).first().unwrap().unwrap())
//         .collect();
//     let res: Vec<String> = res.iter().map(|s| format!("{s}")).collect();
//     assert_eq!(input, res);
// }

// #[test]
// fn read_write_test_identifiers() {
//     let input = vec![
//         "<a>".to_owned(),
//         "<123>".to_owned(),
//         "<https://github.com/almibe/ligature-rs>".to_owned(),
//     ];
//     let res: Vec<WanderValue<String>> = input
//         .iter()
//         .map(|s| run(s, &mut common()).first().unwrap())
//         .collect();
//     let res: Vec<String> = res.iter().map(|s| format!("{s}")).collect();
//     assert_eq!(input, res);
// }
