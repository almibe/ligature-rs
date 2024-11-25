// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

fn main() {
    println!("Running Ligature Test Suite!");
    let location = std::env::var("LIGATURE_TEST_SUITE").unwrap();
    println!("Running tests in {}.", location);
    for path in std::fs::read_dir(location).unwrap() {
        let path = path.unwrap();
        if path.file_name().into_string().unwrap().ends_with(".wander") {
            println!(" - Starting {:?}...", path);
            let script = std::fs::read_to_string(path.path()).unwrap();
            match wander::run(&script, wander::prelude::common(), &mut ligature_graph::LigatureGraph::new()) {
                Ok(_) => println!(" - Success!"),
                Err(err) => println!(" - FAILED! {:?}", err)
            }
        }
    }
}
