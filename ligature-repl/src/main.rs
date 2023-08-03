// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.
//! It is an application that .

#![deny(missing_docs)]

use ligature_sqlite::LigatureSQLite;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use wander::bindings::BindingsProvider;
use wander::preludes::common;
use wander::run;

fn main() -> Result<()> {
    //    let mut instance = LigatureRedb::default();
    let instance = LigatureSQLite::default();
    let mut bindings = common();
    instance.add_bindings(&mut bindings);
    println!("Welcome to Ligature's REPL!");
    println!("Press Ctrl+C or Ctrl+D or enter `:q` to quit.");
    println!("Enter :help or :h for help.");
    println!("---");
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
                    match result {
                        Ok(result) => println!("Result: {result}"),
                        Err(err) => println!("Error: {err:?}"),
                    }
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
        ":status" | ":s" => status(),
        ":quit" | ":q" => quit(),
        ":bindings" | ":b" => bindings(),
        ":help" | ":h" => help(),
        _ => todo!(),
    }
}

fn bindings() -> bool {
    true
}

fn help() -> bool {
    true
}

fn status() -> bool {
    println!("...");
    true
}

fn quit() -> bool {
    false
}
