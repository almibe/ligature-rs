// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;

use crate::bindings::Bindings;
use crate::parser::Element;
use crate::WanderValue;

pub fn eval(script: Vec<Element>) -> Result<WanderValue, LigatureError> {
    let mut bindings = Bindings::new();
    let mut result = Ok(WanderValue::Nothing);
    for element in script {
        result = match element {
            Element::Boolean(value) => Ok(WanderValue::Boolean(value)),
            Element::Int(value) => Ok(WanderValue::Int(value)),
            Element::String(value) => Ok(WanderValue::String(value.to_string())),
            Element::Identifier(value) => Ok(WanderValue::Identifier(value.clone())),
            Element::Let(name, value) => handle_let(&name, &value, &mut bindings),
            Element::Name(name) => read_name(&name, &mut bindings),
            _ => todo!(),
        };
    }
    result
}

fn handle_let(
    name: &String,
    element: &Box<Element>,
    bindings: &mut Bindings,
) -> Result<WanderValue, LigatureError> {
    match literal_element_to_wander_value(element.as_ref()) {
        Ok(value) => {
            bindings.bind(name.to_string(), value);
            Ok(WanderValue::Nothing)
        }
        _ => todo!(),
    }
}

fn literal_element_to_wander_value(element: &Element) -> Result<WanderValue, LigatureError> {
    match element {
        Element::Boolean(value) => Ok(WanderValue::Boolean(*value)),
        Element::Int(value) => Ok(WanderValue::Int(*value)),
        Element::Identifier(value) => Ok(WanderValue::Identifier(value.clone())),
        Element::String(value) => Ok(WanderValue::String(value.to_string())),
        _ => todo!(),
    }
}

fn read_name(name: &String, bindings: &mut Bindings) -> Result<WanderValue, LigatureError> {
    match bindings.read(name.to_string()) {
        Some(value) => Ok(value),
        _ => Err(LigatureError(format!(
            "Error looking up {}",
            name.to_string()
        ))),
    }
}
