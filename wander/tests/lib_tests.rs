// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wander::{run, WanderValue};

#[test]
fn run_wander_true() {
    let input = "true";
    let res = run(input);
    let expected = Ok(WanderValue::Boolean(true));
    assert_eq!(res, expected);
}
