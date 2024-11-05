// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::{interpreter::Expression, parser::ParserElement, WanderError, Location};

// Handle any tranlations needed before creating an expression.
pub fn translate(element: Location<ParserElement>) -> Result<Location<Expression>, WanderError> {
    let element = process_pipes(&element)?;
    express(&element)
}

fn process_pipes(element: &Location<ParserElement>) -> Result<Location<ParserElement>, WanderError> {
    let (elements, position) = match element {
        Location(ParserElement::Grouping(elements), position) => (elements, position),
        e => return Ok(e.clone()),
    };
    let mut index = 0;
    let mut results = vec![];
    while let Some(element) = elements.get(index) {
        if element.0 == ParserElement::Pipe {
            index += 1;
            match elements.get(index) {
                Some(Location(ParserElement::Grouping(next_elements), _position)) => {
                    let mut next_elements = next_elements.clone();
                    let mut new_results = vec![];
                    next_elements.append(&mut results);
                    new_results.push(Location(ParserElement::Grouping(next_elements.clone()), element.1));
                    results = new_results;
                }
                _ => return Err(WanderError("Invalid pipe.".to_owned())),
            }
        } else {
            results.push(element.clone());
        }
        index += 1;
    }
    Ok(Location(ParserElement::Grouping(results), *position))
}

fn express_optional_name(name: &Option<String>) -> Result<Option<Location<Expression>>, WanderError> {
    match name {
        Some(element) => Ok(Some(express(&Location(ParserElement::Name(element.to_string()), 0))?)),
        None => Ok(None),
    }
}

pub fn express(element: &Location<ParserElement>) -> Result<Location<Expression>, WanderError> {
    let expression = match element {
        Location(ParserElement::String(val), position) => Location(Expression::String(val.clone()), *position),
        Location(ParserElement::Element(value), position) => Location(Expression::Element(value.clone()), *position),
        Location(ParserElement::Name(name), position) => Location(Expression::Name(name.clone()), *position),
        Location(ParserElement::Grouping(elements), _position) => return handle_grouping(elements),
        Location(ParserElement::Lambda(p, i, o, b), position) => {
            Location(Expression::Lambda(p.clone(), i.clone(), o.clone(), b.clone()), *position)
        }
        Location(ParserElement::Pipe, _position) => {
            return Err(WanderError(
                "Cannot process pipe, Should never reach.".to_owned(),
            ))
        }
        Location(ParserElement::HostFunction(name), position) => Location(Expression::HostFunction(name.clone()), *position),
    };
    Ok(expression)
}

fn handle_grouping(elements: &[Location<ParserElement>]) -> Result<Location<Expression>, WanderError> {
    let expressions: Vec<Location<Expression>> = elements.iter().map(|e| express(e).unwrap()).collect();
    let expressions: Vec<Location<Expression>> = expressions
        .iter()
        .map(|e| match e {
            Location(Expression::Application(application), _position) => {
                if application.len() == 1 {
                    application.first().unwrap().clone()
                } else {
                    e.clone()
                }
            }
            e => e.clone(),
        })
        .collect();
    if expressions.len() == 1 {
        Ok(expressions.first().unwrap().clone())
    } else {
        let position = expressions.first().unwrap().1;
        let res = Location(Expression::Application(expressions), position);
        Ok(res)
    }
}
