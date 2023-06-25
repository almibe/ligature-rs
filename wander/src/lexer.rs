// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use logos::{Logos, Lexer};
use ligature::LigatureError;

fn bool(lex: &mut Lexer<Token>) -> Option<bool> {
    let slice = lex.slice();
    match slice {
        "true" => Some(true),
        "false" => Some(false),
        _ => None
    }
}

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum Token {
    #[token("if")]
    If,

    #[token(".")]
    Period,

    // Or regular expressions.
    #[regex("[a-zA-Z]+")]
    Text,

    #[regex("(true)|(false)", bool)]
    Boolean(bool)
}



pub fn tokenize(script: &str) -> Result<Vec<Token>, LigatureError> {
    let lexer = Token::lexer(script);
    let mut results = vec!();
    for x in lexer {
        match x {
            Ok(token) => results.push(token),
            Err(err) => todo!()
        }
    }
    Ok(results)
}
