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
    Name(String),
    Identifier(Identifier),
    Let(String, Box<Element>),
    FunctionCall(String, Vec<Element>),
    Scope(Vec<Element>),
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
    //    take_all(take_name(), take_open_paren(), take_arguments(), take_close_paren);
    let name = if let Some(Token::Name(function_name)) = gaze.next() {
        function_name
    } else {
        return None;
    };
    if gaze.next() != Some(Token::OpenParen) {
        return None;
    }
    let mut arguments = vec![];
    while gaze.peek() != None && gaze.peek() != Some(Token::CloseParen) {
        match gaze.next() {
            Some(Token::Boolean(value)) => arguments.push(Element::Boolean(value)),
            _ => return None,
        }
    }
    if gaze.next() != Some(Token::CloseParen) {
        return None;
    }
    Some(Element::FunctionCall(name, arguments))
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

fn literal_token_to_element(token: Token) -> Result<Element, LigatureError> {
    match token {
        Token::Boolean(value) => Ok(Element::Boolean(value)),
        Token::Int(value) => Ok(Element::Int(value)),
        Token::Identifier(value) => Ok(Element::Identifier(value)),
        Token::String(value) => Ok(Element::String(value)),
        x => todo!("{:?}", x),
    }
}

fn scope(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenBrace) => (),
        Some(_) => return None,
        _ => return None
    }
    let mut body = vec![];

    loop {
        match gaze.attemptf(&mut element) {
            Some(element) => body.push(element),
            None => break,
        }
    }

    match gaze.next() {
        Some(Token::CloseBrace) => Some(Element::Scope(body)),
        _ => None
    }
}

fn let_binding(gaze: &mut Gaze<Token>) -> Option<Element> {
    let name = match (gaze.next(), gaze.next(), gaze.next()) {
        (Some(Token::Let), Some(Token::Name(name)), Some(Token::EqualSign)) => {
            name
        }
        _ => return None,
    };
    match gaze.attemptf(&mut element) {
        Some(element) => Some(Element::Let(name, Box::new(element))),
        _ => None
    }
}

fn element(gaze: &mut Gaze<Token>) -> Option<Element> {
    let mut parsers = vec![
        function_call,
        name,
        boolean,
        int,
        string,
        identifier,
        let_binding,
        scope,
    ];
    for &mut mut parser in parsers.iter_mut() {
        if let Some(element) = gaze.attemptf(&mut parser) {
            return Some(element);
        }
    }
    None
}

fn elements(gaze: &mut Gaze<Token>) -> Option<Vec<Element>> {
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

//const BOOLEAN: Nibbler<Token, Element> = ConvertNblr { to_match: To };

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut elements) {
        Some(value) => Ok(value),
        None => Err(LigatureError(String::from("Error parsing"))),
    }
}
