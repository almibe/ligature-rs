// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use crate::{interpreter::Expression, parser::Element, WanderError, Location};

// Handle any tranlations needed before creating an expression.
pub fn translate(element: Location<Element>) -> Result<Location<Expression>, WanderError> {
    let element = process_pipes(&element)?;
    express(&element)
}

fn process_pipes(element: &Location<Element>) -> Result<Location<Element>, WanderError> {
    let (elements, position) = match element {
        Location(Element::Grouping(elements), position) => (elements, position),
        e => return Ok(e.clone()),
    };
    let mut index = 0;
    let mut results = vec![];
    while let Some(element) = elements.get(index) {
        if element.0 == Element::Pipe {
            index += 1;
            match elements.get(index) {
                Some(Location(Element::Grouping(next_elements), position)) => {
                    let mut next_elements = next_elements.clone();
                    let mut new_results = vec![];
                    next_elements.append(&mut results);
                    new_results.push(Location(Element::Grouping(next_elements.clone()), element.1));
                    results = new_results;
                }
                _ => return Err(WanderError("Invalid pipe.".to_owned())),
            }
        } else {
            results.push(element.clone());
        }
        index += 1;
    }
    Ok(Location(Element::Grouping(results), *position))
}

fn express_optional_name(name: &Option<String>) -> Result<Option<Location<Expression>>, WanderError> {
    match name {
        Some(element) => Ok(Some(express(&Location(Element::Name(element.to_string()), 0))?)),
        None => Ok(None),
    }
}

pub fn express(element: &Location<Element>) -> Result<Location<Expression>, WanderError> {
    let expression = match element {
        Location(Element::Boolean(val), position) => Location(Expression::Boolean(*val), *position),
        Location(Element::Int(val), position) => Location(Expression::Int(*val), *position),
        Location(Element::String(val), position) => Location(Expression::String(val.clone()), *position),
        Location(Element::Identifier(value), position) => Location(Expression::Identifier(value.clone()), *position),
        Location(Element::Name(name), position) => Location(Expression::Name(name.clone()), *position),
        Location(Element::Let(decls, body), position) => Location(Expression::Let(
            decls
                .clone()
                .iter()
                .map(|e| {
                    (
                        e.0.clone(),
                        express_optional_name(&e.1).unwrap(),
                        express(&e.2).unwrap(),
                    )
                })
                .collect(),
            Box::new(express(body).unwrap()),
        ), *position),
        Location(Element::Grouping(elements), position) => return handle_grouping(elements),
        Location(Element::Conditional(i, ie, ee), position) => Location(Expression::Conditional(
            Box::new(express(i).unwrap()),
            Box::new(express(ie).unwrap()),
            Box::new(express(ee).unwrap()),
        ), *position),
        Location(Element::Lambda(p, i, o, b), position) => {
            Location(Expression::Lambda(p.clone(), i.clone(), o.clone(), b.clone()), *position)
        }
        Location(Element::Tuple(values), position) => {
            Location(Expression::Tuple(values.clone().iter().map(|e| express(e).unwrap()).collect()), *position)
        }
        Location(Element::List(values), position) => {
            Location(Expression::List(values.clone().iter().map(|e| express(e).unwrap()).collect()), *position)
        }
        Location(Element::Set(values), position) => {
            Location(Expression::Set(values.clone().iter().map(|e| express(e).unwrap()).collect()), *position)
        }
        Location(Element::Record(values), position) => {
            let mut result: HashMap<String, Location<Expression>> = HashMap::new();
            values
                .iter()
                .map(|e| (e.0, express(e.1).unwrap()))
                .for_each(|e| {
                    result.insert(e.0.clone(), e.1);
                });
            Location(Expression::Record(result), *position)
        }
        Location(Element::Nothing, position) => Location(Expression::Nothing, *position),
        Location(Element::Pipe, position) => {
            return Err(WanderError(
                "Cannot process pipe, Should never reach.".to_owned(),
            ))
        }
        Location(Element::HostFunction(name), position) => Location(Expression::HostFunction(name.clone()), *position),
        Location(Element::TaggedName(name, tag), position) => {
            Location(Expression::TaggedName(name.clone(), Box::new(express(tag).unwrap())), *position)
        }
    };
    Ok(expression)
}

fn handle_grouping(elements: &[Location<Element>]) -> Result<Location<Expression>, WanderError> {
    let expressions: Vec<Location<Expression>> = elements.iter().map(|e| express(e).unwrap()).collect();
    let expressions: Vec<Location<Expression>> = expressions
        .iter()
        .map(|e| match e {
            Location(Expression::Application(application), position) => {
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
