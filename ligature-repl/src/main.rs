// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.
//! It is an application that .

#![deny(missing_docs)]

use wander::run;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

fn main() -> Result<()> {
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
                let result = run(line.as_str());
                println!("Result: {:?}", result);
            }
            Err(ReadlineError::Interrupted) => {
                println!("Quitting...");
                break;
            }
            Err(ReadlineError::Eof) => {
                println!("Quitting...");
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
