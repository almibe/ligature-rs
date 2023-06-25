// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, LigatureError};

use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Element {
    Boolean(bool),
    Int(i64),
    String(String),
    Identifier(Identifier),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Element>, LigatureError> {
    let mut results = vec![];
    for token in tokens {
        match token {
            Token::Boolean(value) => results.push(Element::Boolean(value)),
            Token::Int(value) => results.push(Element::Int(value)),
            Token::String(value) => results.push(Element::String(value)),
            Token::Identifier(value) => results.push(Element::Identifier(value)),
            _ => todo!(),
        }
    }
    Ok(results)
}
