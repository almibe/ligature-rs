// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use gaze::Gaze;
use serde::{Deserialize, Serialize};

use crate::{lexer::Token, WanderError};

#[derive(Debug, PartialEq, Clone, Deserialize, Serialize)]
pub enum Element {
    Boolean(bool),
    Int(i64),
    String(String),
    Name(String),
    Let(String, Box<Element>),
    FunctionCall(String, Vec<Element>),
    Scope(Vec<Element>),
    Conditional(Box<Element>, Box<Element>, Box<Element>),
    Lambda(Vec<String>, Vec<Element>),
    Tuple(Vec<Element>),
    List(Vec<Element>),
    Record(HashMap<String, Element>),
    Nothing,
    Forward,
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
    while gaze.peek().is_some() && gaze.peek() != Some(Token::CloseParen) {
        match gaze.attemptf(&mut element) {
            Some(element) => arguments.push(element),
            None => return None,
        }
    }
    if gaze.next() != Some(Token::CloseParen) {
        return None;
    }
    Some(Element::FunctionCall(name, arguments))
}

fn nothing(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Nothing) | Some(Token::QuestionMark) => Some(Element::Nothing),
        _ => None,
    }
}

fn forward(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Forward) => Some(Element::Forward),
        _ => None,
    }
}

fn name(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::Name(value)) => Some(Element::Name(value)),
        _ => None,
    }
}

fn scope(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenBrace) => (),
        _ => return None,
    }

    let mut body = vec![];
    while let Some(element) = gaze.attemptf(&mut element) {
        body.push(element)
    }

    match gaze.next() {
        Some(Token::CloseBrace) => Some(Element::Scope(body)),
        _ => None,
    }
}

fn conditional(gaze: &mut Gaze<Token>) -> Option<Element> {
    if let Some(Token::If) = gaze.next() {
        //do nothing
    } else {
        return None;
    }
    let cond = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };
    let ife = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };
    if let Some(Token::Else) = gaze.next() {
        //do nothing
    } else {
        return None;
    }
    let elsee = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };
    Some(Element::Conditional(
        Box::new(cond),
        Box::new(ife),
        Box::new(elsee),
    ))
}

fn lambda(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenBrace) => (),
        _ => return None,
    }

    let mut names = vec![];
    loop {
        match gaze.attemptf(&mut element) {
            Some(Element::Name(name)) => names.push(name),
            Some(_) => return None, //only allow names
            None => break,          //should be the arrow token failing to match
        }
    }

    match gaze.next() {
        Some(Token::Arrow) => (),
        _ => return None,
    }

    let mut body = vec![];
    while let Some(element) = gaze.attemptf(&mut element) {
        body.push(element)
    }

    match gaze.next() {
        Some(Token::CloseBrace) => Some(Element::Lambda(names, body)),
        _ => None,
    }
}

fn list(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenSquare) => (),
        _ => return None,
    }

    let mut contents = vec![];
    while let Some(e) = gaze.attemptf(&mut element) {
        contents.push(e)
    }

    match gaze.next() {
        Some(Token::CloseSquare) => Some(Element::List(contents)),
        _ => None,
    }
}

fn record(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenParen) => (),
        _ => return None,
    }

    let mut contents = HashMap::new();
    while let Some(Element::Name(name)) = gaze.attemptf(&mut name) {
        match gaze.next() {
            Some(Token::Colon) => (),
            _ => return None,
        };
        match gaze.attemptf(&mut element) {
            Some(element) => contents.insert(name, element),
            None => None,
        };
    }

    match gaze.next() {
        Some(Token::CloseParen) => Some(Element::Record(contents)),
        _ => None,
    }
}

fn tuple(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenParen) => (),
        _ => return None,
    }

    let mut contents = vec![];
    while let Some(e) = gaze.attemptf(&mut element) {
        contents.push(e)
    }

    match gaze.next() {
        Some(Token::CloseParen) => Some(Element::Tuple(contents)),
        _ => None,
    }
}

fn let_binding(gaze: &mut Gaze<Token>) -> Option<Element> {
    let name = match (gaze.next(), gaze.next(), gaze.next()) {
        (Some(Token::Let), Some(Token::Name(name)), Some(Token::EqualSign)) => name,
        _ => return None,
    };
    gaze.attemptf(&mut element)
        .map(|element| Element::Let(name, Box::new(element)))
}

fn element(gaze: &mut Gaze<Token>) -> Option<Element> {
    let mut parsers = vec![
        tuple,
        record,
        function_call,
        name,
        boolean,
        nothing,
        forward,
        int,
        string,
        let_binding,
        scope,
        conditional,
        lambda,
        list,
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

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut elements) {
        Some(value) => Ok(value),
        None => Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
    }
}
