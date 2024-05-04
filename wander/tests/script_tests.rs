// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;
use std::{path::PathBuf, fs::File};

use wander::{NoHostType, WanderValue, WanderError};
use wander::preludes::add_print;
use std::io::Read;
use wander::preludes::common;
use wander::run;

#[test]
fn run_script_tests() {
    let mut successes: Vec<String> = vec![];
    let mut failures: HashMap<String, WanderError> = HashMap::new();
    let mut root = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    root.push("tests");
    std::fs::read_dir(root).unwrap().for_each(|entry| {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_string_lossy().into_owned();
        if file_name.ends_with(".test.wander") {
            let mut file = File::open(path).unwrap();
            let mut script = String::new();
            let _ = file.read_to_string(&mut script);
            let mut environment = common::<NoHostType>();
            add_print(&mut environment);
            match run(&script, &mut environment) {
                Ok(WanderValue::List(tests)) => {
                    for test in tests {
                        match test {
                            WanderValue::Record(record) => {
                                let name = match record.get(&"name".to_owned()) {
                                    Some(WanderValue::String(name)) => name.clone(),
                                    _ => todo!(),
                                };
                                let res = match record.get(&"test".to_owned()) {
                                    Some(test) => test.clone(),
                                    _ => todo!(),
                                };
                                let expected = match record.get(&"expect".to_owned()) {
                                    Some(expected) => expected.clone(),
                                    _ => todo!(),
                                };
                                if res == expected {
                                    successes.push(name);
                                } else {
                                    failures.insert(name, WanderError(format!("Not equal:\n\tExpected: {}\n\tRecieved: {}", expected, res)));
                                }
                            },
                            _ => todo!(),
                        }
                    }
                },
                Err(failure) => {failures.insert(format!("Compilation Error {file_name}"), failure);},
                _ => panic!()
            }
        }
    });
    println!("{} tests passed.", successes.len());
    println!("{} tests failed.", failures.len());
    if !failures.is_empty() {
        println!("Test errors:");
        for failure in &failures {
            println!("\t{failure:?}");
        }
    }
    assert!(failures.is_empty());
}
