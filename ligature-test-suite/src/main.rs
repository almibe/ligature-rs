// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the a test suite for Ligature implementations.

use wander::WanderError;
use colored::Colorize;

fn main() {
    let mut total = 0;
    let mut failed = 0;
    println!("👟 Running Ligature Test Suite!");
    let location = std::env::var("LIGATURE_TEST_SUITE").unwrap();
    println!("📁 Running tests in {}.", location);
    for path in std::fs::read_dir(location).unwrap() {
        let path = path.unwrap();
        if path.file_name().into_string().unwrap().ends_with(".wander") {
            total = total + 1;
            println!("🧪{}", path.file_name().into_string().unwrap().cyan());
            let script = std::fs::read_to_string(path.path()).unwrap();
            match wander::run(&script, &wander::prelude::common(), &mut ligature_graph::LigatureGraph::new()) {
                Ok(_) => println!(" 😀 {}", "Success!".bright_magenta()),
                Err(WanderError(err)) => {
                    failed = failed + 1;
                    println!(" 😅 {} {}", "Failed".red(), err)
                }
            }
        }
    }
    println!("Completed {} out of {} tests.", (total - failed), total);
}
