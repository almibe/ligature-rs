// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::{Gaze, Step};
use ligature::{Identifier, LigatureError, Value};

use crate::lexer::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Element {
    Boolean(bool),
    Int(i64),
    String(String),
    Name(String),
    Identifier(Identifier),
    Let(String, Box<Element>),
    FunctionCall(String, Vec<Element>),
}

fn boolean(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Boolean(value)) => Some(Element::Boolean(value)),
        _ => None,
    }
}

fn int(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Int(value)) => Some(Element::Int(value)),
        _ => None,
    }
}

fn string(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::String(value)) => Some(Element::String(value)),
        _ => None,
    }
}

fn function_call(gaze: &mut Gaze<Token>) -> Option<Element> {
    todo!()
}

fn name(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Name(value)) => Some(Element::Name(value)),
        _ => None,
    }
}

fn identifier(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Identifier(value)) => Some(Element::Identifier(value)),
        _ => None,
    }
}

fn take_token(token: Token) -> Box<Step<Token, Element>> {
    todo!()
}

fn literal_token_to_element(token: Token) -> Result<Element, LigatureError> {
    match token {
        Token::Boolean(value) => Ok(Element::Boolean(value)),
        Token::Int(value) => Ok(Element::Int(value)),
        Token::Identifier(value) => Ok(Element::Identifier(value)),
        Token::String(value) => Ok(Element::String(value)),
        _ => todo!(),
    }
}

fn let_binding(gaze: &mut Gaze<Token>) -> Option<Element> {
    match (gaze.next(), gaze.next(), gaze.next(), gaze.next()) {
        (Some(Token::Let), Some(Token::Name(name)), Some(Token::EqualSign), Some(value)) => {
            match literal_token_to_element(value) {
                Ok(element) => Some(Element::Let(name, Box::new(element))),
                _ => None,
            }
        }
        _ => None,
    }
}

fn elements(gaze: &mut Gaze<Token>) -> Option<Vec<Element>> {
    let parsers = vec![name, boolean, int, string, identifier, let_binding];
    let mut results = vec![];
    'outer: while !gaze.is_complete() {
        for parser in parsers.iter() {
            if let Some(element) = gaze.attempt(parser) {
                results.push(element);
                continue 'outer;
            }
        }
        return None;
    }
    Some(results)
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attempt(&elements) {
        Some(value) => Ok(value),
        None => Err(LigatureError(String::from("Error parsing")))
    }
}
