// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use logos::{Lexer, Logos};
use serde::Serialize;

use crate::{environment::Environment, WanderError, Location};

#[derive(Logos, Debug, PartialEq, Eq, Clone, Serialize)]
#[logos()]
pub enum Token {
    #[regex("[ \t\n\r]+", ws)]
    WS(String),

    #[regex(r#""(([^\x00-\x1F"\\]|\\["\\/bfnrt]|\\u[0-9a-fA-F]{4})*)""#, string)]
    String(String),

    #[token("(")]
    OpenParen,

    #[token(")")]
    CloseParen,

    #[token("{")]
    OpenBrace,

    #[token("}")]
    CloseBrace,

    #[token("[")]
    OpenSquare,

    #[token("]")]
    CloseSquare,

    #[regex("[a-zA-Z0-9-._~:/?#\\[\\]@!$&'()*+,;%=\\x{00A0}-\\x{D7FF}\\x{F900}-\\x{FDCF}\\x{FDF0}-\\x{FFEF}\\x{10000}-\\x{1FFFD}\\x{20000}-\\x{2FFFD}\\x{30000}-\\x{3FFFD}\\x{40000}-\\x{4FFFD}\\x{50000}-\\x{5FFFD}\\x{60000}-\\x{6FFFD}\\x{70000}-\\x{7FFFD}\\x{80000}-\\x{8FFFD}\\x{90000}-\\x{9FFFD}\\x{A0000}-\\x{AFFFD}\\x{B0000}-\\x{BFFFD}\\x{C0000}-\\x{CFFFD}\\x{D0000}-\\x{DFFFD}\\x{E1000}-\\x{EFFFD}]+", identifier)]
    Element(ligature::Element),

    #[token("|")]
    Pipe,

    #[regex("--.*\n?", comment)]
    Comment(String),
}

fn trim_string(value: &str) -> &str {
    let mut chars = value.chars();
    assert_eq!(chars.next().unwrap(), '"');
    assert_eq!(chars.next_back().unwrap(), '"');
    chars.as_str()
}

fn string(lex: &mut Lexer<Token>) -> Option<String> {
    Some(trim_string(lex.slice()).to_string())
}

fn name(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

fn identifier(lex: &mut Lexer<Token>) -> Option<ligature::Element> {
    Some(ligature::Element(lex.slice().to_string()))
}

fn comment(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

fn ws(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
}

pub fn tokenize(script: &str) -> Result<Vec<Location<Token>>, WanderError> {
    let lexer = Token::lexer(script).spanned();
    let mut results = vec![];
    for (token, range) in lexer {
        match token {
            Ok(token) => results.push(Location(token, range.start)),
            Err(_) => return Err(WanderError(String::from("Error tokenizing input."))),
        }
    }
    Ok(results)
}

pub fn tokenize_and_filter(script: &str) -> Result<Vec<Location<Token>>, WanderError> {
    let tokens = tokenize(script);
    tokens.map(|mut tokens| {
        tokens
            .retain(|Location(token, _)| !matches!(token, Token::Comment(_)) && !matches!(token, Token::WS(_)));
        tokens
    })
}
