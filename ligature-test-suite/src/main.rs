// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

use ligature::LigatureError;
use ligature_redb::LigatureRedb;
use wander::{preludes::common, run, WanderValue};

struct LigatureTestCase<'a> {
    name: &'a str,
    input: &'a str,
    result: Result<WanderValue, LigatureError>,
}

#[derive(Debug)]
struct TestResults<'a> {
    failed_tests: Vec<&'a str>,
    passed_tests: Vec<&'a str>,
}

pub fn main() {
    let mut results = TestResults {
        failed_tests: vec![],
        passed_tests: vec![],
    };
    let tests = vec![
        LigatureTestCase {
            name: "Empty test",
            input: "",
            result: Ok(WanderValue::Nothing),
        },
        LigatureTestCase {
            name: "Parse Boolean",
            input: "true",
            result: Ok(WanderValue::Boolean(true)),
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: "datasets()",
            result: Ok(WanderValue::List(vec![])),
        },
        LigatureTestCase {
            name: "add single Datasets",
            input: r#"addDataset("hello") datasets()"#,
            result: Ok(WanderValue::List(vec![WanderValue::String("hello".to_owned())])),
        }
    ];

    for test in tests {
        let instance = match LigatureRedb::temp() {
            Ok(i) => i,
            Err(err) => panic!("{err}"),
        };
        let mut bindings = common();
        instance.add_bindings(&mut bindings);
        let result = run(test.input, &mut bindings);
        if result == test.result {
            results.passed_tests.push(test.name);
        } else {
            results.failed_tests.push(test.name);
        }
    }
    println!("Results:\n{:?}", results);
}
