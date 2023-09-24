// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

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

    #[token(":")]
    Colon,

    #[token("::")]
    DoubleColon,

    #[regex("-?[0-9]+", int)]
    Int(i64),

    #[regex(r#""(([^\x00-\x1F"\\]|\\["\\/bfnrt]|\\u[0-9a-fA-F]{4})*)""#, string)]
    String(String),


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
