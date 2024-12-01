// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{run_quote, Command, WanderError, WanderValue};
use ligature::{Element, Ligature};
use std::collections::{BTreeSet, HashMap};

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> HashMap<String, Command> {
    let mut commands: HashMap<String, Command> = HashMap::new();
    // commands.insert("eq".to_owned(), Box::new(EqCommand {}));
    commands.insert(
        "assert-equal".to_owned(),
        Command {
            doc: "Check if two arguments are equal (quotes are evaluated before comparing) and fail if they are not equal.".to_owned(),
            fun: assert_equal_command,
        },
    );
    commands.insert(
        "assert-fail".to_owned(),
        Command {
            doc: "Accept a quote and run it. If the quote results in a failure continue otherwise fail.".to_owned(),
            fun: assert_fail_command,
        },
    );
    commands.insert(
        "ignore".to_owned(),
        Command {
            doc: "Ignore all arguments to this command and return an empty network.".to_owned(),
            fun: ignore_command,
        },
    );
    commands.insert(
        "let".to_owned(),
        Command {
            doc: "Set a named network.".to_owned(),
            fun: let_command,
        },
    );
    commands.insert(
        "read".to_owned(),
        Command {
            doc: "Read a network.".to_owned(),
            fun: read_command,
        },
    );
    commands.insert(
        "id".to_owned(),
        Command {
            doc: "Return a value.".to_owned(),
            fun: id_command,
        },
    );
    commands.insert(
        "docs".to_owned(),
        Command {
            doc: "Get a list of commands and a description.".to_owned(),
            fun: docs_command,
        },
    );
    commands.insert(
        "union".to_owned(),
        Command {
            doc: "Combine two networks.".to_owned(),
            fun: union_command,
        },
    );
    commands
}

fn docs_command(
    args: Vec<WanderValue>,
    _: &mut dyn Ligature,
    commands: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    match &args[..] {
        [] => {
            let mut results = BTreeSet::new();
            commands.iter().for_each(|(name, command)| {
                results.insert(ligature::Entry::Role {
                    first: Element(name.to_owned()),
                    second: Element(command.doc.to_owned()),
                    role: Element("docString".to_owned()),
                });
            });
            Ok(WanderValue::Network(results))
        }
        _ => Err(WanderError("docs takes no arguments.".to_owned())),
    }
}

fn union_command(
    args: Vec<WanderValue>,
    state: &mut dyn Ligature,
    commands: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    match &args[..] {
        [left, right] => {
            let mut left = if let WanderValue::Network(left) = left {
                left.clone()
            } else {
                todo!()
            };
            let mut right = if let WanderValue::Network(right) = right {
                right.clone()
            } else {
                todo!()
            };
            left.append(&mut right);
            Ok(WanderValue::Network(left.clone()))
        }
        _ => Err(WanderError("union takes two arguments.".to_owned())),
    }
}

fn id_command(
    args: Vec<WanderValue>,
    _: &mut dyn Ligature,
    _: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    match &args[..] {
        [value] => Ok(value.clone()),
        _ => Err(WanderError("Id requires a single argument.".to_owned())),
    }
}

fn ignore_command(
    _: Vec<WanderValue>,
    _: &mut dyn Ligature,
    _: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    Ok(WanderValue::Network(BTreeSet::new()))
}

fn assert_equal_command(
    arguments: Vec<WanderValue>,
    state: &mut dyn Ligature,
    commands: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    if let [left, right] = &arguments[..] {
        let left = if let WanderValue::Quote(quote) = left {
            match run_quote(quote, commands, state) {
                Ok(value) => value,
                Err(err) => return Err(err),
            }
        } else {
            left.clone()
        };
        let right = if let WanderValue::Quote(quote) = right {
            match run_quote(quote, commands, state) {
                Ok(value) => value,
                Err(err) => return Err(err),
            }
        } else {
            right.clone()
        };

        if left == right {
            Ok(crate::WanderValue::Network(BTreeSet::new()))
        } else {
            Err(WanderError(format!(
                "Assertion failed, {} != {}",
                left, right
            )))
        }
    } else {
        Err(WanderError(
            "`assertEq` function requires two parameters.".to_owned(),
        ))
    }
}

fn assert_fail_command(
    arguments: Vec<WanderValue>,
    state: &mut dyn Ligature,
    commands: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    if let [WanderValue::Quote(quote)] = &arguments[..] {
        match run_quote(quote, commands, state) {
            Ok(_) => return Err(WanderError("Expected failure.".to_owned())),
            Err(_) => return Ok(WanderValue::Network(BTreeSet::new())),
        }
    } else {
        Err(WanderError(
            "`assert-fail` function expected to be passed a Quote.".to_owned(),
        ))
    }
}

fn let_command(
    arguments: Vec<WanderValue>,
    state: &mut dyn Ligature,
    commands: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    match &arguments[..] {
        [WanderValue::Element(name), WanderValue::Network(network)] => {
            state.add_collection(name.clone());
            state.add_entries(name.clone(), &mut network.clone());
        }
        [WanderValue::Element(name), WanderValue::Quote(quote)] => {
            match run_quote(quote, commands, state) {
                Ok(WanderValue::Network(res)) => {
                    state.add_collection(name.clone());
                    state.add_entries(name.clone(), &mut res.clone());
                }
                _ => todo!(),
            }
        }
        _ => return Err(WanderError("Invalid call to let.".to_owned())),
    }
    Ok(WanderValue::Network(BTreeSet::new()))
}

fn read_command(
    arguments: Vec<WanderValue>,
    state: &mut dyn Ligature,
    _: &HashMap<String, Command>,
) -> Result<WanderValue, WanderError> {
    match &arguments[..] {
        [WanderValue::Element(name)] => match state.entries(name) {
            Ok(entries) => return Ok(WanderValue::Network(entries)),
            Err(err) => Err(WanderError(format!("Error {}", err.0))),
        },
        _ => todo!("Error"),
    }
}

// pub struct EqCommand {}
// impl<E> Command<E> for EqCommand {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         _: &mut dyn Ligature<E>,
//     ) -> Result<WanderValue, WanderError> {
//         if let [left, right] = arguments {
//             if left == right {
//                 Ok(crate::WanderValue::Element(Element("true".to_owned())))
//             } else {
//                 Ok(crate::WanderValue::Element(Element("false".to_owned())))
//             }
//         } else {
//             Err(WanderError(
//                 "`eq` function requires two parameters.".to_owned(),
//             ))
//         }
//     }

//     fn doc(&self) -> String {
//         "".to_owned()
//     }
// }

//TODO https://github.com/almibe/ligature-rs/issues/305
// struct EnvironmentCommand {}
// impl HostCommand for EnvironmentCommand {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         bindings: &Bindings,
//     ) -> Result<WanderValue, WanderError> {
//         if arguments.is_empty() {
//             let b: BTreeSet<Statement> = bindings
//                 .environment()
//                 .iter()
//                 .flat_map(|e| {
//                     let mut statements = vec![];
//                     let name = Identifier::new(e.name.as_str()).unwrap();
//                     statements.push(Statement {
//                         entity: name.clone(),
//                         attribute: Identifier::new("doc").unwrap(),
//                         value: Value::String(e.doc_string.clone()),
//                     });
//                     statements.push(Statement {
//                         entity: name.clone(),
//                         attribute: Identifier::new("parameters").unwrap(),
//                         value: Value::String(format!("{:?}", e.parameters)),
//                     });
//                     statements.push(Statement {
//                         entity: name.clone(),
//                         attribute: Identifier::new("result").unwrap(),
//                         value: Value::String(format!("{:?}", e.result)),
//                     });
//                     statements
//                 })
//                 .collect();
//             Ok(WanderValue::Graph(Graph::new(b)))
//         } else {
//             panic!("should never reach")
//         }
//     }

//     fn doc(&self) -> String {
//         "All Commands in the current Environment.".to_owned()
//     }

//     fn params(&self) -> Vec<WanderType> {
//         vec![]
//     }

//     fn returns(&self) -> WanderType {
//         WanderType::Graph
//     }

//     fn name(&self) -> String {
//         "Halp.environment".to_owned()
//     }
// }
