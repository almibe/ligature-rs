// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.
//! It is an application that .

#![deny(missing_docs)]

use lig::read::read;
use ligature::{Dataset, Ligature};
use ligature_sqlite::LigatureSQLite;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use std::fs::read_to_string;
use tabled::{Table, Tabled, settings::{Modify, Width, object::Rows}};
use wander::bindings::{BindingsProvider, Bindings, EnvironmentBinding};
use wander::preludes::common;
use wander::run;

struct REPLState {
    instance: LigatureSQLite,
    bindings: Bindings,
}

fn main() -> Result<()> {
    //    let mut instance = LigatureRedb::default();
    let mut instance = LigatureSQLite::default();
    let mut bindings = common();
    instance.add_bindings(&mut bindings);
    let mut state = REPLState { instance, bindings };
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
                    if !handle_command(&line, &mut state) {
                        break;
                    }
                } else {
                    let result = run(line.as_str(), &mut state.bindings);
                    match result {
                        Ok(result) => println!("{result}"),
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

fn handle_command(input: &str, instance: &mut REPLState) -> bool {
    let mut parts = input.split_whitespace();
    match parts.next().unwrap() {
        //":remote" => todo!(),
        //":local" => todo!(),
        ":status" | ":s" => status(),
        ":quit" | ":q" => quit(),
        ":bindings" | ":b" => bindings(&instance.bindings),
        ":environment" | ":e" => environment(&mut instance.bindings),
        ":help" | ":h" => help(),
        ":load" => load(input, &mut instance.instance),
        ":broadcast" => broadcast(input, &mut instance.instance),
        _ => todo!(),
    }
}

fn broadcast(input: &str, instance: &mut dyn Ligature) -> bool {
    true
}

fn load(input: &str, instance: &mut dyn Ligature) -> bool {
    let input: Vec<&str> = input.split(" ").collect();
    let dataset_name = input[1];
    let file = input[2];
    let dataset = if let Ok(dataset) = Dataset::new(dataset_name.trim_end()) {
        dataset
    } else {
        println!("Invalid Dataset name.");
        return true;
    };
    match read_to_string(file.trim_end()) {
        Ok(lig) => match read(&lig) {
            Ok(statements) => match instance.add_statements(&dataset, statements) {
                Ok(_) => println!("Loaded."),
                Err(e) => println!("Error adding Statements: {e:?}"),
            },
            Err(e) => {
                println!("Error reading lig from file: {e:?}");
            }
        },
        Err(_) => println!("Error reading file."),
    }
    true
}

fn bindings(bindings: &Bindings) -> bool {
    bindings.bound_names().iter().for_each(|binding| { 
        println!("{binding}")
    });
    true
}

fn environment(bindings: &mut Bindings) -> bool {
    let mut display: Vec<EnvironmentDisplay> = bindings.environment().into_iter().map(EnvironmentDisplay::from).collect();
    display.sort();
    let mut table = Table::new(display);
    table
        .with(Modify::new(Rows::new(1..)).with(Width::wrap(30).keep_words()).with(Width::increase(20)))
        .with(Width::increase(150));
    println!("{table}");
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

///
#[derive(PartialEq, Eq, PartialOrd, Ord, Tabled)]
pub struct EnvironmentDisplay {
    ///
    pub name: String,
    ///
    pub parameters: String,
    ///
    pub result: String,
    ///
    pub doc_string: String
}

impl From<EnvironmentBinding> for EnvironmentDisplay {
    fn from(value: EnvironmentBinding) -> Self {
        EnvironmentDisplay { 
            name: value.name, 
            parameters: format!("{:?}", value.parameters), 
            result: format!("{:?}", value.result),
            doc_string: value.doc_string }
    }
}