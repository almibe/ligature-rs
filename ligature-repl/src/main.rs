// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.
//! It is an application that .

#![deny(missing_docs)]

use ligature_redb::LigatureRedb;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use wander::preludes::common;
use wander::run;

enum Connection {}

struct ReplState {}

fn main() -> Result<()> {
    let mut instance = LigatureRedb::default();
    let mut bindings = common();
    instance.add_bindings(&mut bindings);
    println!("Welcome to Ligature REPL!!!");
    println!("Press Ctrl+C or Ctrl+D or type `exit()` to quit.");
    let mut rl = Editor::<()>::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if line.trim().starts_with(":") {
                    if !handle_command(&line) {
                        break;
                    }
                } else {
                    let result = run(line.as_str(), &mut bindings);
                    println!("Result: {:?}", result);
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Good Bye!");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Good Bye!");
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}

fn handle_command(input: &str) -> bool {
    let mut parts = input.split_whitespace();
    match parts.next().unwrap() {
        //":remote" => todo!(),
        //":local" => todo!(),
        ":status" => status(),
        ":quit" | ":q" => quit(),
        _ => todo!(),
    }
}

fn status() -> bool {
    println!("...");
    true
}

fn quit() -> bool {
    false
}
