// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;

use crate::bindings::Bindings;
use crate::parser::Element;
use crate::WanderValue;

pub fn eval(script: &Vec<Element>, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    let mut result = Ok(WanderValue::Nothing);
    for element in script {
        result = eval_element(element, bindings);
    }
    result
}

pub fn eval_element(
    element: &Element,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    match element {
        Element::Boolean(value) => Ok(WanderValue::Boolean(*value)),
        Element::Int(value) => Ok(WanderValue::Int(*value)),
        Element::String(value) => Ok(WanderValue::String(value.to_string())),
        Element::Identifier(value) => Ok(WanderValue::Identifier(value.clone())),
        Element::Let(name, value) => handle_let(&name, &value, bindings),
        Element::Name(name) => read_name(&name, bindings),
        Element::FunctionCall(name, arguments) => call_function(name, arguments, bindings),
        Element::Scope(body) => handle_scope(&body, bindings),
        Element::Conditional(c, i, e) => handle_conditional(c, i, e, bindings),
        Element::Lambda(params, body) => handle_lambda(params, body),
        Element::List(values) => handle_list(values, bindings),
    }
}

fn handle_list(elements: &Vec<Element>, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    let mut results = vec![];
    for element in elements {
        match eval_element(element, bindings) {
            Ok(value) => results.push(value),
            Err(err) => return Err(err),
        }
    }
    Ok(WanderValue::List(results))
}

fn handle_lambda(params: &Vec<String>, body: &Vec<Element>) -> Result<WanderValue, LigatureError> {
    Ok(WanderValue::Lambda(params.to_owned(), body.to_owned()))
}

fn handle_conditional(
    cond: &Box<Element>,
    ife: &Box<Element>,
    elsee: &Box<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    match eval_element(cond, bindings)? {
        WanderValue::Boolean(true) => eval_element(ife, bindings),
        WanderValue::Boolean(false) => eval_element(elsee, bindings),
        value => {
            return Err(LigatureError(format!(
                "Conditionals require a bool value found, {value}"
            )))
        }
    }
}

fn handle_scope(
    body: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    bindings.add_scope();
    let res = eval(body, bindings);
    bindings.remove_scope();
    res
}

fn handle_let(
    name: &String,
    element: &Box<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    match eval_element(element.as_ref(), bindings) {
        Ok(value) => {
            bindings.bind(name.to_string(), value);
            Ok(WanderValue::Nothing)
        }
        _ => todo!(),
    }
}

fn read_name(name: &String, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    match bindings.read(&name) {
        Some(value) => Ok(value),
        _ => Err(LigatureError(format!(
            "Error looking up {}",
            name.to_string()
        ))),
    }
}

fn call_function(
    name: &String,
    arguments: &Vec<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    let mut argument_values = vec![];
    for argument in arguments {
        match eval_element(argument, bindings) {
            Ok(value) => argument_values.push(value),
            Err(err) => return Err(err),
        }
    }
    match bindings.read(&name) {
        //corner case of this name shadowing with a native function
        Some(WanderValue::NativeFunction(_)) => {
            todo!()
        }
        //found other value (err), will evntually handle lambdas here
        Some(_) => Err(LigatureError(format!("Function {} is not defined.", &name))),
        None => match bindings.read_native_function(&name) {
            None => Err(LigatureError(format!("Function {} is not defined.", name))),
            Some(nf) => nf.run(&argument_values, bindings),
        },
    }
}