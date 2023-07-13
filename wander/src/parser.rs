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
    Conditional(Box<Element>, Box<Element>, Box<Element>),
    Lambda(Vec<String>, Vec<Element>),
    List(Vec<Element>),
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

fn scope(gaze: &mut Gaze<Token>) -> Option<Element> {
    match gaze.next() {
        Some(Token::OpenBrace) => (),
        _ => return None,
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
        _ => None,
    }
}

fn conditional(gaze: &mut Gaze<Token>) -> Option<Element> {
    if let Some(Token::If) = gaze.next() {
        ()
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
        ()
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
    loop {
        match gaze.attemptf(&mut element) {
            Some(element) => body.push(element),
            None => break,
        }
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
    loop {
        match gaze.attemptf(&mut element) {
            Some(e) => contents.push(e),
            None => break,          //should be the arrow token failing to match
        }
    }

    match gaze.next() {
        Some(Token::CloseSquare) => Some(Element::List(contents)),
        _ => None,
    }
}

fn let_binding(gaze: &mut Gaze<Token>) -> Option<Element> {
    let name = match (gaze.next(), gaze.next(), gaze.next()) {
        (Some(Token::Let), Some(Token::Name(name)), Some(Token::EqualSign)) => name,
        _ => return None,
    };
    match gaze.attemptf(&mut element) {
        Some(element) => Some(Element::Let(name, Box::new(element))),
        _ => None,
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

//const BOOLEAN: Nibbler<Token, Element> = ConvertNblr { to_match: To };

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut elements) {
        Some(value) => Ok(value),
        None => Err(LigatureError(String::from("Error parsing"))),
    }
}
