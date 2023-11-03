// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{LigatureError, Statement};
use logos::Logos;
use wander::lexer::Token;

pub fn tokenize(script: &str) -> Result<Vec<Token>, LigatureError> {
    let lexer = Token::lexer(script);
    let mut results = vec![];
    for token in lexer {
        match token {
            Ok(token) => results.push(token),
            Err(_) => return Err(LigatureError(String::from("Error tokenizing input."))),
        }
    }
    Ok(results)
}

// pub fn read(script: &str) -> Result<Vec<Statement>, LigatureError> {
//     let tokens = tokenize(script)?;
//     wander::lig::read_tokens(tokens)
// }
