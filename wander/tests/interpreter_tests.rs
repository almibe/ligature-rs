// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::interpreter::eval;
use wander::parser::Element;
use wander::preludes::common;
use wander::WanderValue;

#[test]
fn eval_boolean_true() {
    let input = vec![Element::Boolean(true)];
    let res = eval(&input, &mut common());
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}

#[test]
fn eval_string_with_quotes() {
    let input = vec![Element::String("\"\\\"\"".to_owned())];
    let res = eval(&input, &mut common());
    let expected = Ok(WanderValue::String("\"".to_owned()));
    assert_eq!(res, expected);
}
