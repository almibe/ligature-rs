// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{lexer::Token, WanderError, Location};
use gaze::Gaze;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};

#[doc(hidden)]
#[derive(Debug, PartialEq, Eq, Clone, Deserialize, Serialize)]
pub enum ParserElement {
    String(String),
    Element(ligature::Element),
    Name(String),
    HostFunction(String),
    Grouping(Vec<Location<ParserElement>>),
    Lambda(String, Option<String>, Option<String>, Box<Location<ParserElement>>),
    Pipe,
}

impl core::hash::Hash for ParserElement {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

fn element(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    match gaze.next() {
        Some(Location(Token::Element(value), position)) => Some(Location(ParserElement::Element(value), position)),
        _ => None,
    }
}

fn string(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    match gaze.next() {
        Some(Location(Token::String(value), position)) => Some(Location(ParserElement::String(value), position)),
        _ => None,
    }
}

fn pipe(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    match gaze.next() {
        Some(Location(Token::Pipe, position)) => Some(Location(ParserElement::Pipe, position)),
        _ => None,
    }
}

fn grouping(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    let mut expressions: Vec<Location<ParserElement>> = vec![];
    let position = match gaze.peek() {
        Some(Location(_, p)) => p,
        None => return None,
    };

    while let Some(e) = gaze.attemptf(&mut element_inner) {
        expressions.push(e);
    }

    match &expressions[..] {
        [] => None,
        _ => Some(Location(ParserElement::Grouping(expressions), position)),
    }
}

fn grouped_application(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    let mut elements = vec![];

    let _position = match gaze.next() {
        Some(Location(Token::OpenParen, position)) => position,
        _ => return None,
    };

    while let Some(e) = gaze.attemptf(&mut element_inner) {
        elements.push(e);
    }

    match gaze.next() {
        Some(Location(Token::CloseParen, position)) => Some(Location(ParserElement::Grouping(elements), position)),
        _ => None,
    }
}

//this function is basically the same as element inner but it matches name instead of application
fn element_inner(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    let mut parsers = vec![
        element,
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

fn parser_element(gaze: &mut Gaze<Location<Token>>) -> Option<Location<ParserElement>> {
    let mut parsers = vec![pipe, grouping, grouped_application];
    for &mut mut parser in parsers.iter_mut() {
        if let Some(element) = gaze.attemptf(&mut parser) {
            return Some(element);
        }
    }
    None
}

fn elements(gaze: &mut Gaze<Location<Token>>) -> Option<Vec<Location<ParserElement>>> {
    let mut results = vec![];
    while !gaze.is_complete() {
        if let Some(element) = gaze.attemptf(&mut parser_element) {
            results.push(element);
        } else {
            return None;
        }
    }
    Some(results)
}

/// Parse a sequence of Tokens into a sequence of ASTs.
pub fn parse(tokens: Vec<Location<Token>>) -> Result<Location<ParserElement>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut elements) {
        Some(values) => {
            if values.len() == 1 {
                Ok(values.first().unwrap().clone())
            } else {
                Ok(Location(ParserElement::Grouping(values), 0))
            }
        }
        None => Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
    }
}
