// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Identifier;
use logos::{Lexer, Logos, Source};

use crate::{bindings::Bindings, WanderError};

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

    #[regex("-?[0-9]+", int)]
    Int(i64),

    #[regex(r#""(([^\x00-\x1F"\\]|\\["\\/bfnrt]|\\u[0-9a-fA-F]{4})*)""#, string)]
    String(String),

    #[regex("<[a-zA-Z0-9-._~:/?#\\[\\]@!$&'()*+,;%=\\x{00A0}-\\x{D7FF}\\x{F900}-\\x{FDCF}\\x{FDF0}-\\x{FFEF}\\x{10000}-\\x{1FFFD}\\x{20000}-\\x{2FFFD}\\x{30000}-\\x{3FFFD}\\x{40000}-\\x{4FFFD}\\x{50000}-\\x{5FFFD}\\x{60000}-\\x{6FFFD}\\x{70000}-\\x{7FFFD}\\x{80000}-\\x{8FFFD}\\x{90000}-\\x{9FFFD}\\x{A0000}-\\x{AFFFD}\\x{B0000}-\\x{BFFFD}\\x{C0000}-\\x{CFFFD}\\x{D0000}-\\x{DFFFD}\\x{E1000}-\\x{EFFFD}]+>", identifier)]
    Identifier(Identifier),

    #[regex("[_a-zA-Z]+[_a-zA-Z0-9.?]*", name)]
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

    #[token("[")]
    OpenSquare,

    #[token("]")]
    CloseSquare,

    #[token(">>")]
    Forward,

    #[token("->")]
    Arrow,

    #[token("nothing")]
    Nothing,

    #[token("?")]
    QuestionMark,

    #[token("`")]
    Backtick,

    #[regex("--.*\n?")]
    Comment,
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

fn string(lex: &mut Lexer<Token>) -> Option<String> {
    Some(lex.slice().to_string())
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

pub fn tokenize(script: &str) -> Result<Vec<Token>, WanderError> {
    let lexer = Token::lexer(script);
    let mut results = vec![];
    for token in lexer {
        match token {
            Ok(token) => results.push(token),
            Err(_) => return Err(WanderError(String::from("Error tokenizing input."))),
        }
    }
    results.retain(|token| !matches!(token, Token::Comment));
    Ok(results)
}

pub fn transform(input: &[Token], bindings: &Bindings) -> Result<Vec<Token>, WanderError> {
    let mut index = 0;
    let mut results = vec![];
    while let Some(token) = input.get(index) {
        if token == &Token::Backtick {
            let mut internal_results = vec![];
            let transformer = match input.get(index - 1) {
                Some(Token::Name(name)) => match bindings.read_token_transformer(name) {
                    Some(transformer) => transformer,
                    None => {
                        return Err(WanderError(format!(
                            "{name} Token Transformer doesn't exist."
                        )))
                    }
                },
                _ => return Err(WanderError("Token Transforms require a name.".to_owned())),
            };
            results.pop(); //remove transformer's name token
            index += 1; //skip first `
            while let Some(token) = input.get(index) {
                if token == &Token::Backtick {
                    let transformed_content = transformer(&internal_results).unwrap();
                    results.append(&mut transformed_content.to_vec());
                    break;
                } else {
                    internal_results.push(token.to_owned());
                }
                index += 1;
            }
        } else {
            results.push(token.to_owned());
        }
        index += 1;
    }
    Ok(results)
}
