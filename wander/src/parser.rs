// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{lexer::Token, WanderError, WanderValue};
use gaze::Gaze;
use ligature::Element;

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
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Vec<WanderValue>>, WanderError> {
    let mut gaze = Gaze::from_vec(tokens);
    let mut results = vec![];
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
                None => return Err(WanderError(format!("Error parsing {:?}", gaze.peek()))),
                _ => todo!()
            }    
        }
        results.push(current_result);
    }
    Ok(results)
}
