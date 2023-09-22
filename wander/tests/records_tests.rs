// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use wander::{preludes::common, run, WanderValue};

#[test]
fn basic_record() {
    let input = "(a: 24)";
    let res = run(input, &mut common()).unwrap();
    let res = format!("{res}");
    let res = run(&res, &mut common()).unwrap();
    let mut record = HashMap::new();
    record.insert("a".to_owned(), WanderValue::Int(24));
    let expected = WanderValue::Record(record);
    assert_eq!(res, expected);
}
