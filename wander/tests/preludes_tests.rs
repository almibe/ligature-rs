// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{preludes::common, run, ScriptValue};

#[test]
fn passing_assert_eq_call() {
    let input = "assertEq(true true)";
    let res = run(input, &mut common());
    let expected = Ok(ScriptValue::Nothing);
    assert_eq!(res, expected);
}

#[test]
fn failing_assert_eq_call() {
    let input = "assertEq(true \"true\")";
    let res = run(input, &mut common());
    assert!(res.is_err());
}
