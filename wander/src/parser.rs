// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{lexer::Token, Call, WanderError, WanderValue};
use gaze::Gaze;
use ligature::Element;

fn element(gaze: &mut Gaze<Token>) -> Option<WanderValue> {
    match gaze.next() {
        Some(Token::Element(value)) => Some(WanderValue::Element(value)),
        Some(Token::String(value)) => Some(WanderValue::Element(Element(value))),
        _ => None,
    }
}

// fn pipe(gaze: &mut Gaze<Token>) -> Option<ParserElement> {
//     match gaze.next() {
//         Some(Token::Pipe) => Some(ParserElement::Pipe),
//         _ => None,
//     }
// }

fn call(_gaze: &mut Gaze<Token>) -> Option<Call> {
    todo!()
    // let mut expressions: Vec<ParserElement> = vec![];

    // while let Some(e) = gaze.attemptf(&mut element_inner) {
    //     expressions.push(e);
    // }

    // match &expressions[..] {
    //     [] => None,
    //     _ => Some(ParserElement::Call(expressions)),
    // }
}

fn calls(gaze: &mut Gaze<Token>) -> Option<Vec<Call>> {
    let mut results = vec![];
    while !gaze.is_complete() {
        if let Some(element) = gaze.attemptf(&mut call) {
            results.push(element);
        } else {
            return None;
        }
    }
    Some(results)
}

/// Parse a sequence of Tokens into a sequence of ASTs.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Call>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    match gaze.attemptf(&mut calls) {
        Some(values) => {
            Ok(values)
        }
        None => Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
    }
}
