// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::bindings::Bindings;
use crate::parser::Element;
use crate::{WanderValue, WanderError};

pub fn eval(script: &Vec<Element>, bindings: &mut Bindings) -> Result<WanderValue, WanderError> {
    let mut result = Ok(WanderValue::Nothing);
    for element in script {
        result = Ok(eval_element(element, bindings)?);
    }
    result
}

pub fn eval_element(
    element: &Element,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    match element {
        Element::Boolean(value) => Ok(WanderValue::Boolean(*value)),
        Element::Int(value) => Ok(WanderValue::Int(*value)),
        Element::String(value) => Ok(WanderValue::String(unescape_string(value.to_string()))),
        Element::Identifier(value) => Ok(WanderValue::Identifier(value.clone())),
        Element::Let(name, value) => handle_let(name, value, bindings),
        Element::Name(name) => read_name(name, bindings),
        Element::FunctionCall(name, arguments) => call_function(name, arguments, bindings),
        Element::Scope(body) => handle_scope(body, bindings),
        Element::Conditional(c, i, e) => handle_conditional(c, i, e, bindings),
        Element::Lambda(params, body) => handle_lambda(params, body),
        Element::List(values) => handle_list(values, bindings),
        Element::Nothing => Ok(WanderValue::Nothing),
        Element::Forward => panic!("Should never reach."),
        Element::Tuple(values) => handle_tuple(values, bindings),
    }
}

fn unescape_string(value: String) -> String {
    let mut result = String::new();
    let mut last_char = ' ';
    let mut idx = 0;
    value.chars().for_each(|c| {
        if idx == 0 || idx == value.chars().count() - 1 {
            idx += 1;
        } else {
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
        }
    });
    if last_char == '\\' {
        panic!()
    }
    result
}

fn handle_tuple(
    elements: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    let mut results = vec![];
    for element in elements {
        match eval_element(element, bindings) {
            Ok(value) => results.push(value),
            Err(err) => return Err(err),
        }
    }
    Ok(WanderValue::Tuple(results))
}

fn handle_list(
    elements: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    let mut results = vec![];
    for element in elements {
        match eval_element(element, bindings) {
            Ok(value) => results.push(value),
            Err(err) => return Err(err),
        }
    }
    Ok(WanderValue::List(results))
}

fn handle_lambda(params: &Vec<String>, body: &Vec<Element>) -> Result<WanderValue, WanderError> {
    Ok(WanderValue::Lambda(params.to_owned(), body.to_owned()))
}

fn handle_conditional(
    cond: &Element,
    ife: &Element,
    elsee: &Element,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    match eval_element(cond, bindings)? {
        WanderValue::Boolean(true) => eval_element(ife, bindings),
        WanderValue::Boolean(false) => eval_element(elsee, bindings),
        value => Err(WanderError(format!(
            "Conditionals require a bool value found, {value}"
        ))),
    }
}

fn handle_scope(
    body: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    bindings.add_scope();
    let res = eval(body, bindings);
    bindings.remove_scope();
    res
}

fn handle_let(
    name: &String,
    element: &Element,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    match eval_element(element, bindings) {
        Ok(value) => {
            bindings.bind(name.to_string(), value);
            Ok(WanderValue::Nothing)
        }
        _ => todo!(),
    }
}

fn read_name(name: &String, bindings: &mut Bindings) -> Result<WanderValue, WanderError> {
    if let Some(value) = bindings.read(name) {
        Ok(value)
    } else {
        match bindings.read_native_function(name) {
            Some(_) => Ok(WanderValue::NativeFunction(name.to_owned())),
            None => Err(WanderError(format!("Error looking up {name}"))),
        }
    }
}

fn call_function(
    name: &String,
    arguments: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, WanderError> {
    let mut argument_values = vec![];
    for argument in arguments {
        match eval_element(argument, bindings) {
            Ok(value) => argument_values.push(value),
            Err(err) => return Err(err),
        }
    }
    match bindings.read(name) {
        //corner case of this name shadowing with a native function
        Some(WanderValue::NativeFunction(nf_name)) => {
            match bindings.read_native_function(&nf_name) {
                Some(nf) => nf.run(&argument_values, &bindings),
                None => Err(WanderError(
                    "Could not read function {name} that references NativeFunction {nf_name}"
                        .to_owned(),
                )),
            }
        }
        Some(WanderValue::Lambda(parameters, body)) => {
            if parameters.len() == arguments.len() {
                bindings.add_scope();
                for (i, parameter) in parameters.iter().enumerate() {
                    bindings.bind(
                        parameter.to_owned(),
                        argument_values.get(i).unwrap().clone(),
                    );
                }
                let res = eval(&body, bindings);
                bindings.remove_scope();
                res
            } else {
                Err(WanderError(format!(
                    "Incorrect number of arguments, {}, passed to {}, expecting {}.",
                    arguments.len(),
                    name,
                    parameters.len()
                )))
            }
        }
        //found other value (err), will evntually handle lambdas here
        Some(_) => Err(WanderError(format!("Function {} is not defined.", &name))),
        None => match bindings.read_native_function(name) {
            None => Err(WanderError(format!("Function {} is not defined.", name))),
            Some(nf) => nf.run(&argument_values, &bindings),
        },
    }
}
