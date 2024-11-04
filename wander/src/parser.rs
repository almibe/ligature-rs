// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{identifier::Identifier, lexer::Token, WanderError, Location};
use gaze::Gaze;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum Element {
    String(String),
    Identifier(Identifier),
    Name(String),
    HostFunction(String),
    Grouping(Vec<Location<Element>>),
    Lambda(String, Option<String>, Option<String>, Box<Location<Element>>),
    Pipe,
}

impl core::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

fn identifier(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Identifier(value), position)) => Some(Location(Element::Identifier(value), position)),
        _ => None,
    }
}

fn string(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::String(value), position)) => Some(Location(Element::String(value), position)),
        _ => None,
    }
}

fn pipe(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Pipe, position)) => Some(Location(Element::Pipe, position)),
        _ => None,
    }
}

fn name(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Name(value), position)) => Some(Location(Element::Name(value), position)),
        _ => None,
    }
}

fn grouping(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut expressions: Vec<Location<Element>> = vec![];
    let position = match gaze.peek() {
        Some(Location(_, p)) => p,
        None => return None,
    };

    while let Some(e) = gaze.attemptf(&mut element_inner) {
        expressions.push(e);
    }

    match &expressions[..] {
        [] => None,
        _ => Some(Location(Element::Grouping(expressions), position)),
    }
}

fn grouped_application(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut elements = vec![];

    let _position = match gaze.next() {
        Some(Location(Token::OpenParen, position)) => position,
        _ => return None,
    };

    while let Some(e) = gaze.attemptf(&mut element_inner) {
        elements.push(e);
    }

    match gaze.next() {
        Some(Location(Token::CloseParen, position)) => Some(Location(Element::Grouping(elements), position)),
        _ => None,
    }
}

//this function is basically the same as element inner but it matches name instead of application
fn element_inner(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut parsers = vec![
        name,
        identifier,
        string,
        grouped_application,
    ];
    for &mut mut parser in parsers.iter_mut() {
        if let Some(element) = gaze.attemptf(&mut parser) {
            return Some(element);
        }
    }
    None
}

fn element(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut parsers = vec![pipe, grouping, grouped_application];
    for &mut mut parser in parsers.iter_mut() {
        if let Some(element) = gaze.attemptf(&mut parser) {
            return Some(element);
        }
    }
    None
}

fn elements(gaze: &mut Gaze<Location<Token>>) -> Option<Vec<Location<Element>>> {
    let mut results = vec![];
    while !gaze.is_complete() {
        if let Some(element) = gaze.attemptf(&mut element) {
            results.push(element);
        } else {
            return None;
        }
    }
    Some(results)
}

/// Parse a sequence of Tokens into a sequence of ASTs.
pub fn parse(tokens: Vec<Location<Token>>) -> Result<Location<Element>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut elements) {
        Some(values) => {
            if values.len() == 1 {
                Ok(values.first().unwrap().clone())
            } else {
                Ok(Location(Element::Grouping(values), 0))
            }
        }
        None => Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
    }
}
