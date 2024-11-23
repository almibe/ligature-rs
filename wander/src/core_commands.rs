// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Element, Ligature};

use crate::{Command, WanderError, WanderValue};
use std::collections::BTreeSet;

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

// pub struct AssertEqCommand {}
// impl<E> Command<E> for AssertEqCommand {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         _: &mut dyn Ligature<E>,
//     ) -> Result<WanderValue, WanderError> {
//         if let [left, right] = arguments {
//             if left == right {
//                 Ok(crate::WanderValue::Network(BTreeSet::new()))
//             } else {
//                 Err(WanderError("Assertion failed!".to_owned()))
//             }
//         } else {
//             Err(WanderError(
//                 "`assertEq` function requires two parameters.".to_owned(),
//             ))
//         }
//     }

//     fn doc(&self) -> String {
//         "".to_owned()
//     }
// }

// pub struct IgnoreCommand {}
// impl<E> Command<E> for IgnoreCommand {
//     fn run(&self, _: &[WanderValue], _: &mut dyn Ligature<E>) -> Result<WanderValue, WanderError> {
//         Ok(WanderValue::Network(BTreeSet::new()))
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
