// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

use ligature::LigatureError;
use ligature_in_memory::LigatureInMemory;
use ligature_redb::LigatureRedb;
use ligature_sqlite::LigatureSQLite;
use wander::{
    bindings::{Bindings, BindingsProvider},
    preludes::common,
    run, WanderValue,
};

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

fn create_sqlite_bindings() -> Bindings {
    let mut bindings = common();
    let instance = LigatureSQLite::new_memory_store().unwrap();
    instance.add_bindings(&mut bindings);
    bindings
}

fn create_redb_bindings() -> Bindings {
    let mut bindings = common();
    let instance = LigatureRedb::temp().unwrap();
    instance.add_bindings(&mut bindings);
    bindings
}

fn create_memory_bindings() -> Bindings {
    let mut bindings = common();
    let instance = LigatureInMemory::new();
    instance.add_bindings(&mut bindings);
    bindings
}

pub fn main() {
    let skip = false;
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
            result: Ok(WanderValue::List(vec![WanderValue::String(
                "hello".to_owned(),
            )])),
            skippable: true,
        },
        LigatureTestCase {
            name: "add and remove Datasets",
            input: r#"addDataset("hello") addDataset("world") removeDataset("hello") datasets()"#,
            result: Ok(WanderValue::List(vec![WanderValue::String(
                "world".to_owned(),
            )])),
            skippable: true,
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: r#"addDataset("hello") statements("hello")"#,
            result: Ok(WanderValue::List(vec![])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Add Statements to Dataset",
            input: r#"addDataset("hello") addStatements("hello" [[<a> <b> <c>]]) statements("hello")"#,
            result: Ok(WanderValue::List(vec![WanderValue::List(vec![
                WanderValue::String("a".to_owned()),
                WanderValue::String("b".to_owned()),
                WanderValue::String("c".to_owned()),
            ])])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Remove Statements from Dataset",
            input: r#"
                addDataset("hello") 
                addStatements("hello" [[<a> <b> <c>] [<d> <e> <f>]])
                removeStatements("hello" [[<a> <b> <c>]])
                statements("hello")"#,
            result: Ok(WanderValue::List(vec![WanderValue::List(vec![
                WanderValue::String("d".to_owned()),
                WanderValue::String("e".to_owned()),
                WanderValue::String("f".to_owned()),
            ])])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Query Statements",
            input: r#"
                addDataset("hello")
                addStatements("hello" [[<a> <b> <c>][<a> <b> <d>][<a> <c> <d>]])
                query("hello" <a> <b> ?)
            "#,
            result: Ok(WanderValue::List(vec![
                WanderValue::List(vec![
                    WanderValue::String("a".to_owned()),
                    WanderValue::String("b".to_owned()),
                    WanderValue::String("c".to_owned()),
                ]),
                WanderValue::List(vec![
                    WanderValue::String("a".to_owned()),
                    WanderValue::String("b".to_owned()),
                    WanderValue::String("d".to_owned()),
                ]),
            ])),
            skippable: false,
        },
    ];

    for test in tests {
        if test.skippable && skip {
            results.skipped_tests.push(test.name);
            continue;
        }
        // let mut bindings = create_redb_bindings();
        // let mut bindings = create_sqlite_bindings();
        let mut bindings = create_memory_bindings();
        let result = run(test.input, &mut bindings);
        if result == test.result {
            results.passed_tests.push(test.name);
        } else {
            results.failed_tests.push(test.name);
            println!(
                "{:?} failed\n  Expected: {:?}\n  Recieved: {:?}",
                test.name, result, test.result
            );
        }
    }
    println!("Results:\n{:?}", results);
}
