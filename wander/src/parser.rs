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
    Boolean(bool),
    Int(i64),
    String(String),
    Identifier(Identifier),
    Name(String),
    TaggedName(String, Box<Location<Element>>),
    HostFunction(String),
    Let(Vec<(String, Option<String>, Location<Element>)>, Box<Location<Element>>),
    Grouping(Vec<Location<Element>>),
    Conditional(Box<Location<Element>>, Box<Location<Element>>, Box<Location<Element>>),
    Lambda(String, Option<String>, Option<String>, Box<Location<Element>>),
    Tuple(Vec<Location<Element>>),
    List(Vec<Location<Element>>),
    Set(HashSet<Location<Element>>),
    Record(HashMap<String, Location<Element>>),
    Nothing,
    Pipe,
}

impl core::hash::Hash for Element {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        core::mem::discriminant(self).hash(state);
    }
}

fn boolean(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Boolean(value), position)) => Some(Location(Element::Boolean(value), position)),
        _ => None,
    }
}

fn int(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Int(value), position)) => Some(Location(Element::Int(value), position)),
        _ => None,
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

fn nothing(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    match gaze.next() {
        Some(Location(Token::Nothing, position)) | Some(Location(Token::QuestionMark, position)) => Some(Location(Element::Nothing, position)),
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

fn let_scope(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::Let, position)) => position,
        _ => return None,
    };

    let mut decls = vec![];
    while let Some(element) = gaze.attemptf(&mut val_binding) {
        decls.push(element);
    }

    match gaze.next() {
        Some(Location(Token::In, position)) => {
            let body = if let Some(element) = gaze.attemptf(&mut element) {
                element
            } else {
                Location(Element::Nothing, position)
            };

            match gaze.next() {
                Some(Location(Token::End, position)) => Some(Location(Element::Let(decls, Box::new(body)), position)),
                _ => None,
            }
        }
        _ => Some(Location(Element::Let(decls, Box::new(Location(Element::Nothing, 0))), position)),
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

    let position = match gaze.next() {
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

fn conditional(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::If, position)) => position,
        _ => return None,
    };
    let cond = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };

    match gaze.next() {
        Some(Location(Token::Then, _)) => (),
        _ => return None,
    }

    let ife = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };
    if let Some(Location(Token::Else, position)) = gaze.next() {
        //do nothing
    } else {
        return None;
    }
    let elsee = match gaze.attemptf(&mut element) {
        Some(d) => d,
        None => return None,
    };
    if let Some(Location(Token::End, _)) = gaze.next() {
        //do nothing
    } else {
        return None;
    }
    Some(Location(Element::Conditional(
        Box::new(cond),
        Box::new(ife),
        Box::new(elsee),
    ), position))
}

fn lambda(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::Lambda, position)) => position,
        _ => return None,
    };

    let mut params: Vec<(String, Option<String>)> = vec![];

    while let Some(Location(Element::Name(name), _)) = gaze.attemptf(&mut name) {
        let tag = if let Some(Location(Token::Colon, _)) = gaze.peek() {
            gaze.next();
            match gaze.next() {
                Some(Location(Token::Name(name), _)) => Some(name),
                _ => return None, //no match
            }
        } else {
            None
        };
        params.push((name, tag));
    }

    match gaze.next() {
        Some(Location(Token::Arrow, _)) => (),
        _ => return None,
    }

    gaze.attemptf(&mut element).map(|body| {
        let mut final_lambda = None;
        params.reverse();
        for (name, tag) in params {
            match final_lambda {
                Some(prev_lambda) => {
                    final_lambda = Some(Location(Element::Lambda(
                        name.clone(),
                        tag,
                        None,
                        Box::new(prev_lambda),
                    ), 0))
                }
                None => {
                    final_lambda = Some(Location(Element::Lambda(
                        name.clone(),
                        tag,
                        None,
                        Box::new(body.clone()),), 0
                    ))
                }
            }
        }
        final_lambda.unwrap()
    })
}

fn list(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::OpenSquare, position)) => position,
        _ => return None,
    };

    let mut contents = vec![];
    while let Some(e) = gaze.attemptf(&mut element_inner) {
        contents.push(e)
    }

    match gaze.next() {
        Some(Location(Token::CloseSquare, _)) => Some(Location(Element::List(contents), position)),
        _ => None,
    }
}

fn record(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::OpenBrace, position)) => position,
        _ => return None,
    };

    let mut contents = HashMap::new();
    while let Some(Location(Element::Name(name), _)) = gaze.attemptf(&mut name) {
        match gaze.next() {
            Some(Location(Token::EqualSign, _)) => (),
            _ => return None,
        };
        match gaze.attemptf(&mut element_inner) {
            Some(element) => contents.insert(name, element),
            None => None,
        };
    }

    match gaze.next() {
        Some(Location(Token::CloseBrace, _)) => Some(Location(Element::Record(contents), position)),
        _ => None,
    }
}

fn set(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::Hash, position)) => position,
        _ => return None,
    };

    match gaze.next() {
        Some(Location(Token::OpenParen, _)) => (),
        _ => return None,
    }

    let mut contents = HashSet::new();
    while let Some(e) = gaze.attemptf(&mut element_inner) {
        contents.insert(e);
    }

    match gaze.next() {
        Some(Location(Token::CloseParen, _)) => Some(Location(Element::Set(contents), position)),
        _ => None,
    }
}

fn tuple(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let position = match gaze.next() {
        Some(Location(Token::SingleQuote, position)) => position,
        _ => return None,
    };

    match gaze.next() {
        Some(Location(Token::OpenParen, _)) => (),
        _ => return None,
    }

    let mut contents = vec![];
    while let Some(e) = gaze.attemptf(&mut element_inner) {
        contents.push(e)
    }

    match gaze.next() {
        Some(Location(Token::CloseParen, _)) => Some(Location(Element::Tuple(contents), position)),
        _ => None,
    }
}

fn val_binding(gaze: &mut Gaze<Location<Token>>) -> Option<(String, Option<String>, Location<Element>)> {
    let name = match gaze.next() {
        Some(Location(Token::Name(name), _)) => name,
        _ => return None,
    };
    let tag = match gaze.peek() {
        Some(Location(Token::Colon, _)) => {
            gaze.next();
            if let Some(Location(Token::Name(name), _)) = gaze.next() {
                Some(name)
            } else {
                return None;
            }
        }
        _ => None,
    };

    match gaze.next() {
        Some(Location(Token::EqualSign, _)) => (),
        _ => return None,
    };

    gaze.attemptf(&mut element).map(|body| (name, tag, body))
}

//this function is basically the same as element inner but it matches name instead of application
fn element_inner(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut parsers = vec![
        tuple,
        set,
        record,
        name,
        boolean,
        nothing,
        int,
        identifier,
        string,
        let_scope,
        grouped_application,
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

fn element(gaze: &mut Gaze<Location<Token>>) -> Option<Location<Element>> {
    let mut parsers = vec![pipe, let_scope, grouping, grouped_application, conditional];
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
