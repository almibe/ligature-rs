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
    skippable: bool,
}

#[derive(Debug)]
struct TestResults<'a> {
    failed_tests: Vec<&'a str>,
    passed_tests: Vec<&'a str>,
    skipped_tests: Vec<&'a str>,
}

pub fn main() {
    let mut results = TestResults {
        failed_tests: vec![],
        passed_tests: vec![],
        skipped_tests: vec![],
    };
    let tests = vec![
        LigatureTestCase {
            name: "Empty test",
            input: "",
            result: Ok(WanderValue::Nothing),
            skippable: true,
        },
        LigatureTestCase {
            name: "Parse Boolean",
            input: "true",
            result: Ok(WanderValue::Boolean(true)),
            skippable: true,
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: "datasets()",
            result: Ok(WanderValue::List(vec![])),
            skippable: true,
        },
        LigatureTestCase {
            name: "add single Datasets",
            input: r#"addDataset("hello") datasets()"#,
            result: Ok(WanderValue::List(vec![WanderValue::String("hello".to_owned())])),
            skippable: true,
        },
        LigatureTestCase {
            name: "add and remove Datasets",
            input: r#"addDataset("hello") addDataset("world") removeDataset("hello") datasets()"#,
            result: Ok(WanderValue::List(vec![WanderValue::String("world".to_owned())])),
            skippable: true,
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: r#"addDataset("hello") statements("hello")"#,
            result: Ok(WanderValue::List(vec![])),
            skippable: false,
        },
    ];

    for test in tests {
        if test.skippable {
            results.skipped_tests.push(test.name);
            continue;
        }
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
            println!("{:?} failed\n  Expected: {:?}\n  Recieved: {:?}", test.name, result, test.result);
        }
    }
    println!("Results:\n{:?}", results);
}
