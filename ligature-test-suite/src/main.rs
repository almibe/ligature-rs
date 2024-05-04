// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

use std::{fmt::Display, rc::Rc, sync::RwLock};
use wander::{environment::Environment, NoHostType};

use ligature_in_memory::LigatureInMemory;
//use ligature_sqlite::LigatureSQLite;
use wander::{
    preludes::common,
    run, WanderError, WanderValue, HostType
};
use ligature_wander::bind_instance;

struct LigatureTestCase<'a, T: HostType> {
    name: &'a str,
    input: &'a str,
    result: Result<WanderValue<T>, WanderError>,
    skippable: bool,
}

fn ident<T: HostType>(id: &str) -> WanderValue<T> {
    WanderValue::Identifier(wander::identifier::Identifier::new(id).unwrap())
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

// #[allow(dead_code)]
// fn create_sqlite_bindings() -> Bindings {
//     let mut environment = common();
//     let instance = LigatureSQLite::new_memory_store().unwrap();
//     instance.add_bindings(&mut bindings);
//     bindings
// }

// #[allow(dead_code)]
// fn create_redb_bindings() -> Bindings {
//     let mut bindings = common();
//     let instance = LigatureRedb::temp().unwrap();
//     instance.add_bindings(&mut bindings);
//     bindings
// }

#[allow(dead_code)]
fn create_memory_bindings() -> Environment<NoHostType> {
    let mut bindings = common::<NoHostType>();
    let instance = Rc::new(RwLock::new(LigatureInMemory::new()));
//    instance.add_bindings(&mut bindings);
    bind_instance(instance, &mut bindings);
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
        // LigatureTestCase {
        //     name: "Empty test",
        //     input: "",
        //     result: Ok(WanderValue::Nothing),
        //     skippable: true,
        // },
        // LigatureTestCase {
        //     name: "Parse Boolean",
        //     input: "true",
        //     result: Ok(WanderValue::Bool(true)),
        //     skippable: true,
        // },
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
