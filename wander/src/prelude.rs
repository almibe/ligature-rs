// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{Command, WanderError, WanderValue};
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
            doc: "Check if two arguments are equal and fail if they are not equal.".to_owned(),
            fun: assert_equal_command,
        },
    );
    commands.insert(
        "ignore".to_owned(),
        Command {
            doc: "Ignore all arguments to this command and return an empty network.".to_owned(),
            fun: ignore_command,
        },
    );
    // commands.insert("let".to_owned(), Box::new(LetCommand {}));
    // commands.insert("read".to_owned(), Box::new(ReadCommand {}));
    // // commands.bind_host_function(Rc::new(AndFunction {}));
    // // commands.bind_host_function(Rc::new(NotFunction {}));
    // // commands.bind_host_function(Rc::new(EnvironmentFunction {}));
    commands
}

fn ignore_command(_: Vec<WanderValue>, _: &mut dyn Ligature) -> Result<WanderValue, WanderError> {
    Ok(WanderValue::Network(BTreeSet::new()))
}

fn assert_equal_command(
    arguments: Vec<WanderValue>,
    _: &mut dyn Ligature,
) -> Result<WanderValue, WanderError> {
    if let [left, right] = &arguments[..] {
        if left == right {
            Ok(crate::WanderValue::Network(BTreeSet::new()))
        } else {
            Err(WanderError("Assertion failed!".to_owned()))
        }
    } else {
        Err(WanderError(
            "`assertEq` function requires two parameters.".to_owned(),
        ))
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

// pub struct LetCommand {}
// impl<E> Command<E> for LetCommand {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         state: &mut dyn Ligature<E>,
//     ) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::Element(name), WanderValue::Network(network)] => {
//                 state.add_collection(name.clone());
//                 state.add_entries(name.clone(), &mut network.clone());
//             }
//             _ => todo!("Error"),
//         }
//         Ok(WanderValue::Network(BTreeSet::new()))
//     }

//     fn doc(&self) -> String {
//         "".to_owned()
//     }
// }

// pub struct ReadCommand {}
// impl<E> Command<E> for LetCommand {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         state: &mut dyn Ligature<E>,
//     ) -> Result<WanderValue, WanderError> {
//         match arguments {
//             [WanderValue::Element(name)] => {
//                 match state.entries(name.clone()) {
//                     _ => return Ok(WanderValue::Network(BTreeSet::new()))
//                 }
//             }
//             _ => todo!("Error"),
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
