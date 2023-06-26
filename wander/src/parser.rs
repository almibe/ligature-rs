// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::Gaze;
use ligature::{Identifier, LigatureError};

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Boolean(bool),
    Int(i64),
    String(String),
    Identifier(Identifier),
    Let(String, Box<Element>),
}

fn boolean(gaze: &mut Gaze<Token>) -> Result<Element, LigatureError> {
    match gaze.next() {
        Some(Token::Boolean(value)) => Ok(Element::Boolean(value)),
        _ => Err(LigatureError(String::from("No Match Boolean")))
    }
}

fn int(gaze: &mut Gaze<Token>) -> Result<Element, LigatureError> {
    match gaze.next() {
        Some(Token::Int(value)) => Ok(Element::Int(value)),
        _ => Err(LigatureError(String::from("No Match Integer")))
    }
}

fn string(gaze: &mut Gaze<Token>) -> Result<Element, LigatureError> {
    match gaze.next() {
        Some(Token::String(value)) => Ok(Element::String(value)),
        _ => Err(LigatureError(String::from("No Match String")))
    }
}

fn identifier(gaze: &mut Gaze<Token>) -> Result<Element, LigatureError> {
    match gaze.next() {
        Some(Token::Identifier(value)) => Ok(Element::Identifier(value)),
        _ => Err(LigatureError(String::from("No Match Identifier")))
    }
}

fn let_binding(gaze: &mut Gaze<Token>) -> Result<Element, LigatureError> {
    todo!()
}

fn elements(gaze: &mut Gaze<Token>) -> Result<Vec<Element>, LigatureError> {
    let parsers = vec![boolean, int, string, identifier, let_binding];
    let mut results = vec![];
    'outer: while !gaze.is_complete() {
        for parser in parsers.iter() {
            if let Ok(element) = gaze.attempt(parser) {
                results.push(element);
                continue 'outer;
            }
        }
        return Err(LigatureError(String::from("No Match Elements")))
    }
    Ok(results)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    let mut gaze = Gaze::from_vec(tokens);
    gaze.attempt(&elements)
}
