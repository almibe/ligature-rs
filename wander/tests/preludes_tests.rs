// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, NoHostType, WanderValue};

// #[test]
// fn calling_not() {
//     let input = "Bool.not true";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Bool(false));
//     assert_eq!(res, expected);
// }

// #[test]
// fn passing_assert_eq_call() {
//     let input = "Assert.assertEq true true";
//     let res = run(input, &mut common::<NoHostType>());
//     let expected = Ok(WanderValue::Nothing);
//     assert_eq!(res, expected);
// }

// #[test]
// fn failing_assert_eq_call() {
//     let input = "Assert.assertEq true \"true\"";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap();
//     assert!(res.is_err());
// }
