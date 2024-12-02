// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the library module for the ligature-repl project.

use ligature::Entry;
use ligature_graph::LigatureGraph;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};
use tabled::{
    settings::{object::Rows, Modify, Width},
    Table, Tabled,
};
use wander::run;

pub struct REPLState {
    pub state: LigatureGraph,
}

pub fn start_repl(state: &mut REPLState) -> Result<()> {
    println!("Welcome to Wander's REPL!");
    println!("Press Ctrl+C or Ctrl+D or enter `:q` to quit.");
    println!("Enter :help or :h for help.");
    println!("---");
    let mut rl = DefaultEditor::new()?;
    if rl.load_history("history.txt").is_err() {
        println!("No previous history.");
    }
    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;
                if line.trim().starts_with(':') {
                    if !handle_command(&line, state) {
                        break;
                    }
                } else {
                    match run(line.as_str(), &wander::prelude::common(), &mut state.state) {
                        Ok(result) => {
                            match result {
                                wander::WanderValue::Element(ligature::Element(element)) => {
                                    println!("{}", element)
                                }
                                wander::WanderValue::Quote(quote) => todo!(), //println!("{}", quote),
                                wander::WanderValue::Network(btree_set) => {
                                    if btree_set.is_empty() {
                                        println!("{{}}");
                                    } else {
                                        println!("{{");
                                        for entry in btree_set.iter() {
                                            match entry {
                                                Entry::Extends { element, concept } => {
                                                    println!("  {} : {}", element, concept);
                                                },
                                                Entry::Role { first, second, role } => {
                                                    println!("  {} {} {}", first, role, second);
                                                },
                                                Entry::NotExtends { element, concept } => {
                                                    println!("  {} ¬: {}", element, concept);
                                                },
                                            }
                                        }
                                        println!("}}");    
                                    }
                                }
                            }
                        }
                        Err(err) => println!("{err:?}"),
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

#[derive(Tabled)]
struct Triple {
    first: String,
    second: String,
    third: String,
}

fn handle_command(input: &str, instance: &mut REPLState) -> bool {
    let mut parts = input.split_whitespace();
    match parts.next().unwrap() {
        //":remote" => todo!(),
        //":local" => todo!(),
        ":status" | ":s" => status(),
        ":quit" | ":q" => quit(),
        //":bindings" | ":b" => bindings(&instance.environment),
        //":environment" | ":e" => environment(&mut instance.environment),
        ":help" | ":h" => help(),
        ":broadcast" => broadcast(input),
        s => {
            println!("Unknown command - {s}");
            true
        }
    }
}

fn broadcast(_input: &str) -> bool {
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
