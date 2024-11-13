// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::collections::HashSet;

use crate::{lexer::Token, Call, Quote, WanderError, WanderValue};
use gaze::Gaze;
use ligature::{Element, Entry};

// fn wander_values(gaze: &mut Gaze<Token>) -> Option<WanderValue> {
//     match gaze.next() {
//         Some(Token::Element(value)) => Some(WanderValue::Element(value)),
//         Some(Token::String(value)) => Some(WanderValue::Element(Element(value))),
//         _ => None,
//     }
// }

// fn pipe(gaze: &mut Gaze<Token>) -> Option<ParserElement> {
//     match gaze.next() {
//         Some(Token::Pipe) => Some(ParserElement::Pipe),
//         _ => None,
//     }
// }

// fn call(_gaze: &mut Gaze<Token>) -> Option<Call> {
//     todo!()
//     // let mut expressions: Vec<ParserElement> = vec![];

//     // while let Some(e) = gaze.attemptf(&mut element_inner) {
//     //     expressions.push(e);
//     // }

//     // match &expressions[..] {
//     //     [] => None,
//     //     _ => Some(ParserElement::Call(expressions)),
//     // }
// }

// fn calls(gaze: &mut Gaze<Token>) -> Option<Vec<Call>> {
//     todo!()
//     // let mut results = vec![];
//     // while !gaze.is_complete() {
//     //     if let Some(element) = gaze.attemptf(&mut gaze::nibblers::TakeFirstNblr(vec![&mut call])) {
//     //         results.push(element);
//     //     } else {
//     //         return None;
//     //     }
//     // }
//     // Some(results)
// }

/// Parse a sequence of Tokens into a sequence of ASTs.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Call>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    let mut calls: Vec<Call> = vec![];
    while !gaze.is_complete() {
        let mut current_result = vec![];
        let mut cont = true;
        while !gaze.is_complete() && cont {
            match gaze.next() {
                Some(Token::String(value)) => {
                    current_result.push(WanderValue::Element(Element(value)));
                }
                Some(Token::Element(value)) => {
                    current_result.push(WanderValue::Element(value));
                }
                Some(Token::Comma) => cont = false,
                Some(Token::OpenBrace) => match read_network(&mut gaze) {
                    Ok(res) => {
                        current_result.push(res);
                    }
                    _ => todo!(),
                },
                Some(Token::OpenParen) => match read_quote(&mut gaze) {
                    Ok(res) => {
                        current_result.push(res);
                    }
                    _ => todo!(),
                },
                Some(_) => return Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
                None => return Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
            }
        }
        match &current_result[..] {
            [WanderValue::Element(name)] => calls.push(Call {
                name: name.clone(),
                arguments: vec![],
            }),
            [WanderValue::Element(name), ref args @ ..] => {
                calls.push(Call {
                    name: name.clone(),
                    arguments: args.to_vec(),
                });
            }
            _ => todo!(),
        }
    }
    Ok(calls)
}

fn read_network(gaze: &mut Gaze<Token>) -> Result<WanderValue, WanderError> {
    let mut cont = true;
    let mut result: HashSet<ligature::Entry> = HashSet::new();
    while cont {
        let first = match gaze.next() {
            Some(Token::Element(first)) => first,
            Some(Token::CloseBrace) => return Ok(WanderValue::Network(result)),
            _ => todo!(),
        };
        let second = match gaze.next() {
            Some(Token::Element(second)) => second,
            _ => todo!(),
        };
        let third = match gaze.next() {
            Some(Token::Element(third)) => third,
            _ => todo!(),
        };

        if second == Element(":".to_owned()) {
            result.insert(Entry::Extends {
                element: first,
                concept: third,
            });
        } else if second == Element("¬:".to_owned()) {
            result.insert(Entry::NotExtends {
                element: first,
                concept: third,
            });
        } else {
            result.insert(Entry::Role {
                first,
                second: third,
                role: second,
            });
        }

        match gaze.next() {
            Some(Token::Comma) => (),
            Some(Token::CloseBrace) => cont = false,
            Some(_) => return Err(WanderError("Error parsing Network.".to_owned())),
            None => return Err(WanderError("Error parsing Network.".to_owned())),
        }
    }
    return Ok(WanderValue::Network(result));
}

fn read_quote(gaze: &mut Gaze<Token>) -> Result<WanderValue, WanderError> {
    let mut cont = true;
    let mut values: Vec<WanderValue> = vec![];
    while cont {
        match gaze.next() {
            Some(Token::Element(element)) => {
                values.push(WanderValue::Element(element));
            }
            Some(Token::CloseParen) => cont = false,
            Some(Token::OpenBrace) => {
                todo!()
            }
            _ => todo!(),
        };
    }
    Ok(WanderValue::Quote(Quote(values)))
}
