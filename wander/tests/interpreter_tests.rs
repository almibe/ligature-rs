// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::interpreter::{eval, Expression};
use wander::preludes::common;
use wander::{NoHostType, WanderValue, Location};

#[test]
fn eval_boolean_true() {
    let input = Location(Expression::Boolean(true), 0);
    let res = eval(&input, &mut common::<NoHostType>());
    let expected = Ok(WanderValue::Bool(true));
    assert_eq!(res, expected);
}

#[test]
fn eval_string_with_quotes() {
    let input = Location(Expression::String(r#"\""#.to_owned()), 0);
    let res = eval(&input, &mut common::<NoHostType>());
    let expected = Ok(WanderValue::String(r#"""#.to_owned()));
    assert_eq!(res, expected);
}
