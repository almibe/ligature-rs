// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, WanderValue};

#[test]
fn basic_currying() {
    let input = "let isTrue = Bool.and(true) [isTrue(true) isTrue(false)]";
    let res = run(input, &mut common()).unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common()).unwrap();
    let expected = WanderValue::List(vec![WanderValue::Boolean(true), WanderValue::Boolean(false)]);
    assert_eq!(res, expected);
}

#[test]
fn currying_with_lambda() {
    let input = "let and = { x y -> Bool.and(x y) } let isTrue = and(true) [isTrue(true) isTrue(false)]";
    let res = run(input, &mut common()).unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common()).unwrap();
    let expected = WanderValue::List(vec![WanderValue::Boolean(true), WanderValue::Boolean(false)]);
    assert_eq!(res, expected);
}

#[test]
fn partial_application_twice_with_lambda() {
    let input = "let and3 = { x y z -> Bool.and(z Bool.and(x y)) } let and = and3(true) let isTrue = and(true) and(isTrue(true) isTrue(false))";
    let res = run(input, &mut common()).unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common()).unwrap();
    let expected = WanderValue::Boolean(false);
    assert_eq!(res, expected);
}
