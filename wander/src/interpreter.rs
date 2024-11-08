// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::environment::Environment;

use crate::{Call, WanderError, WanderValue};

pub fn eval(
    _calls: &[Call],
    _environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    todo!()
    // match expression {
    //     WanderValue::Element(value) => Ok(WanderValue::Element(value.clone())),
    //     WanderValue::Call(_expressions) => todo!(),//handle_function_call(expressions, environment),
    //     WanderValue::Network(_values) => todo!(),//handle_record(values, environment),
    // }
}

pub fn eval_call(
    _call: &Call,
    _environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    todo!()
    // match expression {
    //     WanderValue::Element(value) => Ok(WanderValue::Element(value.clone())),
    //     WanderValue::Call(_expressions) => todo!(),//handle_function_call(expressions, environment),
    //     WanderValue::Network(_values) => todo!(),//handle_record(values, environment),
    // }
}

// fn unescape_string(value: String) -> String {
//     let mut result = String::new();
//     let mut last_char = ' ';
//     let mut idx = 0;
//     value.chars().for_each(|c| {
//         idx += 1;
//         if last_char == '\\' {
//             match c {
//                 'n' => {
//                     result.push('\n');
//                     last_char = c
//                 }
//                 '\\' => {
//                     result.push('\\');
//                     last_char = ' '
//                 }
//                 't' => {
//                     result.push('\t');
//                     last_char = c
//                 }
//                 '"' => {
//                     result.push(c);
//                     last_char = c
//                 }
//                 _ => todo!(),
//             }
//         } else if c == '\\' {
//             last_char = c
//         } else {
//             result.push(c);
//             last_char = c
//         }
//     });
//     if last_char == '\\' {
//         panic!()
//     }
//     result
// }

// fn handle_host_function(
//     _name: &str,
//     _environment: &mut Environment,
// ) -> Result<WanderValue, WanderError> {
//     todo!()
//     // let host_function = environment.read_host_function(&name.to_owned()).unwrap();
//     // let params = host_function.binding().parameters;
//     // let mut arguments = vec![];
//     // for (name, wander_type) in params {
//     //     match environment.read(&name) {
//     //         Some(value) => arguments.push(value),
//     //         None => return Err(WanderError(format!("Could not read {}", name))),
//     //     }
//     // }
//     // host_function.run(&arguments, environment)
// }

// fn handle_function_call(
//     _call: Call,
//     _environment: &mut Environment,
// ) -> Result<WanderValue, WanderError> {
//     // if expressions.len() == 1 {
//     //     let expression = expressions.first().unwrap();
//     //     return eval(expression, environment);
//     // }
//     // let mut expressions = expressions.clone();
//     // expressions.reverse();
//     // while let Some(expression) = expressions.pop() {
//     //     match expression {
//     //         WanderValue::Call(contents) => {
//     //             match handle_function_call(&contents, environment)? {
//     //                 WanderValue::Call(parts) => {
//     //                     todo!()
//     //                     // if let Some(res) =
//     //                     //     run_lambda(name, input, output, *element, &mut expressions, environment)
//     //                     // {
//     //                     //     return res;
//     //                     // }
//     //                 }
//     //                 e => return Ok(e),
//     //             }
//     //         },
//     //         value => {
//     //             if expressions.is_empty() {
//     //                 return eval(&value, environment);
//     //             } else {
//     //                 return Err(WanderError(format!("Invalid function call {value:?}.")));
//     //             }
//     //         }
//     //     };
//     // }
//     todo!()
// }

// fn call_function(
//     _name: &String,
//     arguments: &Vec<Expression>,
//     environment: &mut Environment,
// ) -> Result<WanderValue, WanderError> {
//     let mut argument_values = vec![];
//     for argument in arguments {
//         match eval(argument, environment) {
//             Ok(value) => argument_values.push(value),
//             Err(err) => return Err(err),
//         }
//     }
//     todo!()
//     // match environment.read(name) {
//     //     //found other value (err), will evntually handle lambdas here
//     //     Some(_) => Err(WanderError(format!("Function {} is not defined.", &name))),
//     //     None => match environment.read_host_function(name) {
//     //         None => Err(WanderError(format!("Function {} is not defined.", name))),
//     //         Some(function) => {
//     //             // if argument_values.len() == function.binding().parameters.len() {
//     //             //     function.run(&argument_values, environment)
//     //             // } else {
//     //             //     // Ok(WanderValue::PartialApplication(Box::new(
//     //             //     //     PartialApplication {
//     //             //     //         arguments: argument_values,
//     //             //     //         callee: WanderValue::HostedFunction(name.clone()),
//     //             //     //     },
//     //             //     // )))
//     //             todo!()
//     //             // }
//     //         }
//     //     },
//     // }
// }
