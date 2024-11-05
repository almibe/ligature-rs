// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::environment::Environment;

use crate::parser::ParserElement;
use crate::translation::express;
use crate::{WanderError, WanderValue, Location};

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Expression {
    String(String),
    Element(ligature::Element),
    Name(String),
    HostFunction(String),
    Application(Vec<Location<Expression>>),
    Lambda(String, Option<String>, Option<String>, Box<Location<ParserElement>>),
    Network(HashSet<ligature::Entry>),
}

impl core::hash::Hash for Expression {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

pub fn eval(
    expression: &Location<Expression>,
    environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    match expression {
        Location(Expression::String(value), _) => Ok(WanderValue::String(unescape_string(value.to_string()))),
        Location(Expression::Element(value), _) => Ok(WanderValue::Element(value.clone())),
        Location(Expression::Name(_name), _) => todo!(),//read_name(name, environment),
        Location(Expression::Application(expressions), _) => handle_function_call(expressions, environment),
        Location(Expression::Network(_values), _) => todo!(),//handle_record(values, environment),
        Location(Expression::Lambda(_name, _input, _output, _body), _) => {
            todo!();
            // handle_lambda(name.clone(), input.clone(), output.clone(), body)
        }
        Location(Expression::HostFunction(name), _) => handle_host_function(name, environment),
        // Expression::Grouping(expressions) => handle_grouping(expressions.clone(), environment),
    }
}

fn unescape_string(value: String) -> String {
    let mut result = String::new();
    let mut last_char = ' ';
    let mut idx = 0;
    value.chars().for_each(|c| {
        idx += 1;
        if last_char == '\\' {
            match c {
                'n' => {
                    result.push('\n');
                    last_char = c
                }
                '\\' => {
                    result.push('\\');
                    last_char = ' '
                }
                't' => {
                    result.push('\t');
                    last_char = c
                }
                '"' => {
                    result.push(c);
                    last_char = c
                }
                _ => todo!(),
            }
        } else if c == '\\' {
            last_char = c
        } else {
            result.push(c);
            last_char = c
        }
    });
    if last_char == '\\' {
        panic!()
    }
    result
}

fn handle_host_function(
    name: &str,
    environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    todo!()
    // let host_function = environment.read_host_function(&name.to_owned()).unwrap();
    // let params = host_function.binding().parameters;
    // let mut arguments = vec![];
    // for (name, wander_type) in params {
    //     match environment.read(&name) {
    //         Some(value) => arguments.push(value),
    //         None => return Err(WanderError(format!("Could not read {}", name))),
    //     }
    // }
    // host_function.run(&arguments, environment)
}

fn handle_lambda(
    name: String,
    input: Option<String>,
    output: Option<String>,
    body: &Location<ParserElement>,
) -> Result<WanderValue, WanderError> {
    Ok(WanderValue::InnerCall(
        name,
        input.clone(),
        output.clone(),
        Box::new(body.clone()),
    ))
}

fn run_lambda(
    name: String,
    input: Option<String>,
    output: Option<String>,
    lambda_body: Location<ParserElement>,
    expressions: &mut Vec<Location<Expression>>,
    environment: &mut Environment,
) -> Option<Result<WanderValue, WanderError>> {
    if expressions.is_empty() {
        Some(Ok(WanderValue::InnerCall(
            name,
            input,
            output,
            Box::new(lambda_body),
        )))
    } else {
        let argument_expression = expressions.pop().unwrap();
        let _argument_value = match eval(&argument_expression, environment) {
            Err(e) => return Some(Err(e)),
            Ok(e) => e,
        };
//        environment.bind(name, argument_value);
        let expression = match express(&lambda_body) {
            Ok(e) => e,
            Err(e) => return Some(Err(e)),
        };
        let function = match eval(&expression, environment) {
            Ok(e) => e,
            Err(err) => return Some(Err(err)),
        };
        match function {
            WanderValue::InnerCall(_, _, _, b) => {
                let Ok(expression) = express(&b) else {
                    return None;
                };
                match eval(&expression, environment) {
                    Ok(value) => {
                        expressions.push(value_to_expression(value));
                        None
                    }
                    Err(err) => Some(Err(err)),
                }
            }
            _ => {
                if expressions.is_empty() {
                    Some(Ok(function))
                } else {
                    Some(Err(WanderError(format!(
                        "Invalid function call, expected expressions {expressions:?}."
                    ))))
                }
            }
        }
    }
}

fn handle_function_call(
    expressions: &Vec<Location<Expression>>,
    environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    if expressions.len() == 1 {
        let expression = expressions.first().unwrap();
        return eval(expression, environment);
    }
    let mut expressions = expressions.clone();
    expressions.reverse();
    while let Some(expression) = expressions.pop() {
        match expression {
            Location(Expression::Application(contents), _position) => {
                match handle_function_call(&contents, environment)? {
                    WanderValue::InnerCall(name, input, output, element) => {
                        if let Some(res) =
                            run_lambda(name, input, output, *element, &mut expressions, environment)
                        {
                            return res;
                        }
                    }
                    e => return Ok(e),
                }
            },
            Location(Expression::Lambda(name, input, output, lambda_body), _position) => {
                if let Some(res) = run_lambda(
                    name,
                    input,
                    output,
                    *lambda_body,
                    &mut expressions,
                    environment,
                ) {
                    return res;
                }
            }
            Location(Expression::Name(name), position) => match eval(&Location(Expression::Name(name), position), environment) {
                Ok(value) => match value {
                    WanderValue::InnerCall(p, _i, _o, b) => {
                        let argument_expression = expressions.pop().unwrap();
                        let _argument_value = eval(&argument_expression, environment)?;
                        //environment.bind(p, argument_value);
                        match eval(&express(&b)?, environment) {
                            Ok(value) => expressions.push(value_to_expression(value)),
                            Err(err) => return Err(err),
                        }
                    }
                    _ => {
                        return Err(WanderError(format!(
                            "Invalid function call, was expecting a lambda and found {value}."
                        )))
                    }
                },
                Err(err) => return Err(err),
            },
            value => {
                if expressions.is_empty() {
                    return eval(&value, environment);
                } else {
                    return Err(WanderError(format!("Invalid function call {value:?}.")));
                }
            }
        };
    }
    panic!()
}

fn value_to_expression(value: WanderValue) -> Location<Expression> {
    match value {
        WanderValue::String(value) => Location(Expression::String(value), 0),
        WanderValue::Element(value) => Location(Expression::Element(value), 0),
        WanderValue::InnerCall(p, i, o, b) => Location(Expression::Lambda(p, i, o, b), 0),
        WanderValue::Network(_value_record) => {
            todo!()
            // let mut record = HashMap::new();
            // for (name, value) in value_record {
            //     record.insert(name, value_to_expression(value));
            // }
            // Location(Expression::Network(record), 0)
        }
    }
}

fn call_function(
    _name: &String,
    arguments: &Vec<Location<Expression>>,
    environment: &mut Environment,
) -> Result<WanderValue, WanderError> {
    let mut argument_values = vec![];
    for argument in arguments {
        match eval(argument, environment) {
            Ok(value) => argument_values.push(value),
            Err(err) => return Err(err),
        }
    }
    todo!()
    // match environment.read(name) {
    //     //found other value (err), will evntually handle lambdas here
    //     Some(_) => Err(WanderError(format!("Function {} is not defined.", &name))),
    //     None => match environment.read_host_function(name) {
    //         None => Err(WanderError(format!("Function {} is not defined.", name))),
    //         Some(function) => {
    //             // if argument_values.len() == function.binding().parameters.len() {
    //             //     function.run(&argument_values, environment)
    //             // } else {
    //             //     // Ok(WanderValue::PartialApplication(Box::new(
    //             //     //     PartialApplication {
    //             //     //         arguments: argument_values,
    //             //     //         callee: WanderValue::HostedFunction(name.clone()),
    //             //     //     },
    //             //     // )))
    //             todo!()
    //             // }
    //         }
    //     },
    // }
}
