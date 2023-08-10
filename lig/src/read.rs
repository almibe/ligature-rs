// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::{Identifier, LigatureError, Statement, Value};
use logos::{Lexer, Logos, Source};

#[derive(Logos, Debug, PartialEq, Clone)]
#[logos(skip r"[ \t\n\f\r]+")]
pub enum Token {
    #[regex("[-0-9]+", int)]
    Int(i64),

    #[regex(r#""(([^\x00-\x1F"\\]|\\["\\/bfnrt]|\\u[0-9a-fA-F]{4})*)""#, string)]
    String(String),

    #[regex("<[a-zA-Z0-9-._~:/?#\\[\\]@!$&'()*+,;%=]+>", identifier)]
    Identifier(Identifier),
}

fn int(lex: &mut Lexer<Token>) -> Option<i64> {
    let slice = lex.slice();
    match slice.parse::<i64>() {
        Ok(value) => Some(value),
        _ => None,
    }
}

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

pub fn read(script: &str) -> Result<Vec<Statement>, LigatureError> {
    let tokens = tokenize(script)?;
    let mut results = vec![];
    let mut index = 0;
    while index < tokens.len() {
        let entity = &tokens.get(index);
        index += 1;
        let attribute = &tokens.get(index);
        index += 1;
        let value = &tokens.get(index);
        index += 1;
        match (entity, attribute, value) {
            (Some(Token::Identifier(entity)), Some(Token::Identifier(attribute)), Some(value)) => {
                let value: Value = match value {
                    Token::Identifier(value) => Value::Identifier(value.clone()),
                    Token::Int(value) => Value::Integer(*value),
                    Token::String(value) => Value::String(value.clone()),
                };
                let statement = Statement {
                    entity: entity.clone(),
                    attribute: attribute.clone(),
                    value,
                };
                results.push(statement.clone());
            }
            _ => {
                return Err(LigatureError(format!(
                    "Could not match Statement. {:?} {:?} {:?}",
                    entity, attribute, value
                )))
            }
        }
    }
    Ok(results)
}
