// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::{HashMap, HashSet};

use wander::{preludes::common, run, WanderValue};

#[test]
fn calling_ignore() {
    let input = "ignore";
    let res = run(input, common(), HashMap::new());
    let expected = Ok(WanderValue::Network(HashSet::new()));
    assert_eq!(res, expected);
}

#[test]
fn calling_ignore_with_args() {
    let input = "ignore test (test {test test test}) {test test test}";
    let res = run(input, common(), HashMap::new());
    let expected = Ok(WanderValue::Network(HashSet::new()));
    assert_eq!(res, expected);
}

// #[test]
// fn basic_let() {
//     let input = "let test {a b c}";
//     let res = run(input, &mut common::<NoHostType>()).first().unwrap().unwrap();
//     let expected = WanderValue::Network...;
//     assert_eq!(res, expected);
// }

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
