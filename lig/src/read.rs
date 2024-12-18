// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use logos::Logos;
use crate::lexer::Token;

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

pub fn read(script: &str) -> Result<Vec<Role>, LigatureError> {
    todo!()
    //let tokens = tokenize(script)?;
    //read_tokens(tokens)
}
