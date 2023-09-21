// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

use std::fmt::Display;

use ligature::{Identifier, LigatureError};
use ligature_in_memory::LigatureInMemory;
use ligature_redb::LigatureRedb;
use ligature_sqlite::LigatureSQLite;
use wander::{
    bindings::{Bindings, BindingsProvider},
    preludes::common,
    run, ScriptValue, WanderError,
};

struct LigatureTestCase<'a> {
    name: &'a str,
    input: &'a str,
    result: Result<ScriptValue, WanderError>,
    skippable: bool,
}

fn ident(id: &str) -> ScriptValue {
    ScriptValue::Identifier(Identifier::new(id).unwrap())
}

#[derive(Debug)]
struct TestResults<'a> {
    failed_tests: Vec<&'a str>,
    passed_tests: Vec<&'a str>,
    skipped_tests: Vec<&'a str>,
}

impl Display for TestResults<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Passed Tests")?;
        for passed_test in &self.passed_tests {
            writeln!(f, " - {passed_test}")?;
        }
        writeln!(f, "Skipped Tests")?;
        for skipped_test in &self.skipped_tests {
            writeln!(f, " - {skipped_test}")?;
        }
        writeln!(f, "Failed Tests")?;
        for failed_test in &self.failed_tests {
            writeln!(f, " - {failed_test}")?;
        }
        Ok(())
    }
}

#[allow(dead_code)]
fn create_sqlite_bindings() -> Bindings {
    let mut bindings = common();
    let instance = LigatureSQLite::new_memory_store().unwrap();
    instance.add_bindings(&mut bindings);
    bindings
}

#[allow(dead_code)]
fn create_redb_bindings() -> Bindings {
    let mut bindings = common();
    let instance = LigatureRedb::temp().unwrap();
    instance.add_bindings(&mut bindings);
    bindings
}

#[allow(dead_code)]
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
            result: Ok(ScriptValue::Nothing),
            skippable: true,
        },
        LigatureTestCase {
            name: "Parse Boolean",
            input: "true",
            result: Ok(ScriptValue::Boolean(true)),
            skippable: true,
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: "Ligature.datasets()",
            result: Ok(ScriptValue::List(vec![])),
            skippable: true,
        },
        LigatureTestCase {
            name: "add single Datasets",
            input: r#"Ligature.addDataset("hello") Ligature.datasets()"#,
            result: Ok(ScriptValue::List(vec![ScriptValue::String(
                "hello".to_owned(),
            )])),
            skippable: true,
        },
        LigatureTestCase {
            name: "add and remove Datasets",
            input: r#"Ligature.addDataset("hello") Ligature.addDataset("world") Ligature.removeDataset("hello") Ligature.datasets()"#,
            result: Ok(ScriptValue::List(vec![ScriptValue::String(
                "world".to_owned(),
            )])),
            skippable: true,
        },
        LigatureTestCase {
            name: "Datasets should start empty",
            input: r#"Ligature.addDataset("hello") Ligature.statements("hello")"#,
            result: Ok(ScriptValue::List(vec![])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Add Statements to Dataset",
            input: r#"Ligature.addDataset("hello") Ligature.addStatements("hello" [[<a> <b> <c>]]) Ligature.statements("hello")"#,
            result: Ok(ScriptValue::List(vec![ScriptValue::List(vec![
                ident("a"),
                ident("b"),
                ident("c"),
            ])])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Remove Statements from Dataset",
            input: r#"
                Ligature.addDataset("hello") 
                Ligature.addStatements("hello" [[<a> <b> <c>] [<d> <e> <f>]])
                Ligature.removeStatements("hello" [[<a> <b> <c>]])
                Ligature.statements("hello")"#,
            result: Ok(ScriptValue::List(vec![ScriptValue::List(vec![
                ident("d"),
                ident("e"),
                ident("f"),
            ])])),
            skippable: false,
        },
        LigatureTestCase {
            name: "Query Statements",
            input: r#"
                Ligature.addDataset("hello")
                Ligature.addStatements("hello" [[<a> <b> <c>][<a> <b> <d>][<a> <c> <d>]])
                Ligature.query("hello" <a> <b> ?)"#,
            result: Ok(ScriptValue::List(vec![
                ScriptValue::List(vec![ident("a"), ident("b"), ident("c")]),
                ScriptValue::List(vec![ident("a"), ident("b"), ident("d")]),
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
                test.name, test.result, result
            );
        }
    }
    println!("Results:\n{results}");
}
