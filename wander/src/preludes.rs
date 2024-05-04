// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    environment::Environment, HostFunction, HostFunctionBinding, HostType, WanderError, WanderValue,
};
use std::rc::Rc;

struct EqFunction {}
impl<T: HostType> HostFunction<T> for EqFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _bindings: &Environment<T>,
    ) -> Result<WanderValue<T>, WanderError> {
        if let [left, right] = arguments {
            Ok(crate::WanderValue::Bool(left == right))
        } else {
            Err(WanderError(
                "`eq` function requires two parameters.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "Core.eq".to_owned(),
            parameters: vec![("left".to_owned(), None), ("right".to_owned(), None)],
            result: None,
            doc_string: "Check if two values are equal.".to_owned(),
        }
    }
}

struct LogFunction {}
impl<T: HostType> HostFunction<T> for LogFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _bindings: &Environment<T>,
    ) -> Result<WanderValue<T>, WanderError> {
        if let [message] = arguments {
            println!("{message}");
            Ok(WanderValue::Nothing)
        } else {
            Err(WanderError(
                "`log` function requires a message to print.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "log".to_owned(),
            parameters: vec![("message".to_owned(), None)],
            result: None,
            doc_string: "Log a message.".to_owned(),
        }
    }
}

struct AssertEqFunction {}
impl<T: HostType> HostFunction<T> for AssertEqFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _bindings: &Environment<T>,
    ) -> Result<WanderValue<T>, WanderError> {
        if let [left, right] = arguments {
            if left == right {
                Ok(crate::WanderValue::Nothing)
            } else {
                Err(WanderError("Assertion failed!".to_owned()))
            }
        } else {
            Err(WanderError(
                "`assertEq` function requires two parameters.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "Assert.assertEq".to_owned(),
            parameters: vec![("value".to_owned(), None), ("expected".to_owned(), None)],
            result: None,
            doc_string: "Assert that two values are equal.".to_owned(),
        }
    }
}

struct AndFunction {}
impl<T: HostType> HostFunction<T> for AndFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _bindings: &Environment<T>,
    ) -> Result<crate::WanderValue<T>, WanderError> {
        if let [WanderValue::Bool(left), WanderValue::Bool(right)] = arguments {
            Ok(crate::WanderValue::Bool(*left && *right))
        } else {
            Err(WanderError(
                "`and` function requires two boolean parameters.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "Bool.and".to_owned(),
            parameters: vec![
                ("left".to_owned(), None),  // bool
                ("right".to_owned(), None), // bool
            ],
            result: None, // bool
            doc_string: "Check if two boolean values are both true.".to_owned(),
        }
    }
}

struct NotFunction {}
impl<T: HostType> HostFunction<T> for NotFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _bindings: &Environment<T>,
    ) -> Result<crate::WanderValue<T>, WanderError> {
        if let [WanderValue::Bool(value)] = arguments {
            Ok(crate::WanderValue::Bool(!value))
        } else {
            Err(WanderError(
                "`not` function requires one boolean parameter.".to_owned(),
            ))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "Bool.not".to_owned(),
            parameters: vec![("value".to_owned(), None)], // bool
            result: None,                                 // bool
            doc_string: "Return the opposite of the boolean value passed.".to_owned(),
        }
    }
}

struct AtFunction {}
impl<T: HostType> HostFunction<T> for AtFunction {
    fn run(
        &self,
        arguments: &[WanderValue<T>],
        _: &Environment<T>,
    ) -> Result<WanderValue<T>, WanderError> {
        if let [WanderValue::Int(index), WanderValue::List(value)] = arguments {
            let index: usize = index.to_owned().try_into().unwrap();
            if index < value.len() {
                let t: Option<&WanderValue<T>> = value.get(index);
                match t {
                    Some(t) => Ok(t.to_owned()),
                    None => Err(WanderError("`at` function err.".to_owned())),
                }
            } else {
                Err(WanderError("`at` function err.".to_owned()))
            }
        } else {
            Err(WanderError("`at` function err.".to_owned()))
        }
    }

    fn binding(&self) -> HostFunctionBinding {
        HostFunctionBinding {
            name: "List.at".to_owned(),
            parameters: vec![
                ("offset".to_owned(), None), //Int
                ("list".to_owned(), None),   //List
            ],
            result: None,
            doc_string: "Get the value at a given location.".to_owned(),
        }
    }
}

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
pub fn common<T: HostType>() -> Environment<T> {
    let mut bindings = Environment::new();
    bindings.bind_host_function(Rc::new(EqFunction {}));
    bindings.bind_host_function(Rc::new(AssertEqFunction {}));
    bindings.bind_host_function(Rc::new(AndFunction {}));
    bindings.bind_host_function(Rc::new(NotFunction {}));
    bindings.bind_host_function(Rc::new(AtFunction {}));
    // bindings.bind_host_function(Rc::new(EnvironmentFunction {}));
    bindings
}

pub fn add_print<T: HostType>(environment: &mut Environment<T>) {
    environment.bind_host_function(Rc::new(LogFunction {}));
}
