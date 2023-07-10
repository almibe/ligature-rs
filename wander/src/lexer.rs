// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, LigatureError};
use logos::{Lexer, Logos, Source};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum Token {
    #[token("let")]
    Let,

    #[token("=")]
    EqualSign,

    #[token("if")]
    If,

    #[token("else")]
    Else,

    #[token(".")]
    Period,

    #[regex("[-0-9]+", int)]
    Int(i64),

    #[regex("\"[a-zA-Z0-9 ]*\"", string)] //TODO this is wrong
    String(String),

    #[regex("<[a-zA-Z0-9]+>", identifier)] //TODO this is wrong
    Identifier(Identifier),

    #[regex("[_a-zA-Z]+[_a-zA-Z0-9]*", name)]
    Name(String),

    #[regex("(true)|(false)", bool)]
    Boolean(bool),

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("->")]
    Arrow,
}

fn bool(lex: &mut Lexer<Token>) -> Option<bool> {
    let slice = lex.slice();
    match slice {
        "true" => Some(true),
        "false" => Some(false),
        _ => None,
    }
}

fn int(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    match slice.parse::<i64>() {
        Ok(value) => Some(value),
        _ => None,
    }
}

//TODO this is wrong
fn string(lex: &mut Lexer<Token>) -> Option<String> {
    let slice = lex.slice();
    slice.slice(1..(slice.len() - 1)).map(|x| x.into())
}

fn identifier(lex: &mut Lexer<Token>) -> Option<Identifier> {
    let slice = lex.slice();
    match Identifier::new(slice.slice(1..(slice.len() - 1)).unwrap()) {
        Ok(ident) => Some(ident),
        Err(_) => None,
    }
}

fn name(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

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
