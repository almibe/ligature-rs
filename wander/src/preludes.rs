// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Entry;

use crate::{Command, WanderError, WanderValue};
use std::{
    collections::{HashMap, HashSet},
    rc::Rc,
};

// struct EqFunction {}
// impl HostFunction for EqFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         _bindings: &Environment,
//     ) -> Result<WanderValue, WanderError> {
//         if let [left, right] = arguments {
//             Ok(crate::WanderValue::Bool(left == right))
//         } else {
//             Err(WanderError(
//                 "`eq` function requires two parameters.".to_owned(),
//             ))
//         }
//     }

//     fn binding(&self) -> HostFunctionBinding {
//         HostFunctionBinding {
//             name: "Core.eq".to_owned(),
//             parameters: vec![("left".to_owned(), None), ("right".to_owned(), None)],
//             result: None,
//             doc_string: "Check if two values are equal.".to_owned(),
//         }
//     }
// }

struct LogFunction {}
impl Command for LogFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        state: &HashMap<String, HashSet<Entry>>,
    ) -> Result<WanderValue, WanderError> {
        if let [message] = arguments {
            println!("{message}");
            todo!();
            // Ok(WanderValue::Nothing)
        } else {
            Err(WanderError(
                "`log` function requires a message to print.".to_owned(),
            ))
        }
    }

    // fn binding(&self) -> HostFunctionBinding {
    //     HostFunctionBinding {
    //         name: "log".to_owned(),
    //         parameters: vec![("message".to_owned(), None)],
    //         result: None,
    //         doc_string: "Log a message.".to_owned(),
    //     }
    // }
}

struct AssertEqFunction {}
impl Command for AssertEqFunction {
    fn run(
        &self,
        arguments: &[WanderValue],
        state: &HashMap<String, HashSet<Entry>>,
    ) -> Result<WanderValue, WanderError> {
        if let [left, right] = arguments {
            if left == right {
                todo!();
                // Ok(crate::WanderValue::Nothing)
            } else {
                Err(WanderError("Assertion failed!".to_owned()))
            }
        } else {
            Err(WanderError(
                "`assertEq` function requires two parameters.".to_owned(),
            ))
        }
    }

    // fn binding(&self) -> HostFunctionBinding {
    //     HostFunctionBinding {
    //         name: "Assert.assertEq".to_owned(),
    //         parameters: vec![("value".to_owned(), None), ("expected".to_owned(), None)],
    //         result: None,
    //         doc_string: "Assert that two values are equal.".to_owned(),
    //     }
    // }
}

struct IgnoreFunction {}
impl Command for IgnoreFunction {
    fn run(
        &self,
        _arguments: &[WanderValue],
        state: &HashMap<String, HashSet<Entry>>,
    ) -> Result<WanderValue, WanderError> {
        Ok(WanderValue::Network(HashSet::new()))
    }
}

// struct AndFunction {}
// impl HostFunction for AndFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         _bindings: &Environment,
//     ) -> Result<crate::WanderValue, WanderError> {
//         if let [WanderValue::Bool(left), WanderValue::Bool(right)] = arguments {
//             Ok(crate::WanderValue::Bool(*left && *right))
//         } else {
//             Err(WanderError(
//                 "`and` function requires two boolean parameters.".to_owned(),
//             ))
//         }
//     }

//     fn binding(&self) -> HostFunctionBinding {
//         HostFunctionBinding {
//             name: "Bool.and".to_owned(),
//             parameters: vec![
//                 ("left".to_owned(), None),  // bool
//                 ("right".to_owned(), None), // bool
//             ],
//             result: None, // bool
//             doc_string: "Check if two boolean values are both true.".to_owned(),
//         }
//     }
// }

// struct NotFunction {}
// impl HostFunction for NotFunction {
//     fn run(
//         &self,
//         arguments: &[WanderValue],
//         _bindings: &Environment,
//     ) -> Result<crate::WanderValue, WanderError> {
//         if let [WanderValue::Bool(value)] = arguments {
//             Ok(crate::WanderValue::Bool(!value))
//         } else {
//             Err(WanderError(
//                 "`not` function requires one boolean parameter.".to_owned(),
//             ))
//         }
//     }

//     fn binding(&self) -> HostFunctionBinding {
//         HostFunctionBinding {
//             name: "Bool.not".to_owned(),
//             parameters: vec![("value".to_owned(), None)], // bool
//             result: None,                                 // bool
//             doc_string: "Return the opposite of the boolean value passed.".to_owned(),
//         }
//     }
// }

//TODO https://github.com/almibe/ligature-rs/issues/305
// struct EnvironmentFunction {}
// impl HostFunction for EnvironmentFunction {
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
//         "All Functions in the current Environment.".to_owned()
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

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common() -> HashMap<String, Box<dyn Command>> {
    let mut commands: HashMap<String, Box<dyn Command>> = HashMap::new();
    //    commands.bind_host_function(Rc::new(EqFunction {}));
    commands.insert("assert-eq".to_owned(), Box::new(AssertEqFunction {}));
    commands.insert("ignore".to_owned(), Box::new(IgnoreFunction {}));
    // commands.bind_host_function(Rc::new(AndFunction {}));
    // commands.bind_host_function(Rc::new(NotFunction {}));
    // commands.bind_host_function(Rc::new(EnvironmentFunction {}));
    commands
}

// pub fn add_print(environment: &mut WanderEngine) {
//     environment.bind_host_function(Rc::new(LogFunction {}));
// }
