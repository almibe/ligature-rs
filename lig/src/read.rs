// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use gaze::Gaze;
use ligature::{Identifier, LigatureError, Statement, Value};
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

pub fn read(script: &str) -> Result<Vec<Statement>, LigatureError> {
    let tokens = tokenize(script)?;
    read_tokens(tokens)
}

fn identifier_nib(gaze: &mut Gaze<Token>) -> Option<Identifier> {
    match gaze.next() {
        Some(Token::Identifier(value)) => Some(value),
        _ => None,
    }
}

pub fn read_tokens(tokens: Vec<Token>) -> Result<Vec<Statement>, LigatureError> {
    let mut gaze = Gaze::from_vec(tokens);
    let mut results = vec![];

    while !gaze.is_complete() {
        handle_expression(&mut gaze, &mut results)?
    }
    Ok(results)
}

fn handle_expression(
    gaze: &mut Gaze<Token>,
    results: &mut Vec<Statement>,
) -> Result<(), LigatureError> {
    if let Some(entity) = gaze.attemptf(&mut identifier_nib) {
        match gaze.next() {
            Some(Token::Identifier(attribute)) => {
                handle_value_expression(&entity, &attribute, gaze, results)
            }
            Some(Token::OpenBrace) => handle_entity_expansion(entity, gaze, results),
            _ => Err(LigatureError(
                "Invalid input expecting Identifier or Entity Expansion.".to_owned(),
            )),
        }
    } else {
        Err(LigatureError(format!(
            "Lig expressions must start with an Identifier found {:?}.",
            gaze.peek()
        )))
    }
}

fn add_statement(
    entity: &Identifier,
    attribuate: &Identifier,
    value: Value,
    results: &mut Vec<Statement>,
) -> Result<(), LigatureError> {
    results.push(Statement {
        entity: entity.to_owned(),
        attribute: attribuate.to_owned(),
        value,
    });
    Ok(())
}

fn handle_value_expression(
    entity: &Identifier,
    attribute: &Identifier,
    gaze: &mut Gaze<Token>,
    results: &mut Vec<Statement>,
) -> Result<(), LigatureError> {
    match gaze.next() {
        Some(Token::Int(value)) => add_statement(entity, attribute, Value::Integer(value), results),
        Some(Token::String(value)) => {
            add_statement(entity, attribute, Value::String(value), results)
        }
        Some(Token::Identifier(value)) => {
            add_statement(entity, attribute, Value::Identifier(value), results)
        }
        Some(Token::OpenSquare) => {
            let mut values = vec![];
            loop {
                match gaze.next() {
                    Some(Token::Int(value)) => values.push(Value::Integer(value)),
                    Some(Token::String(value)) => values.push(Value::String(value)),
                    Some(Token::Identifier(value)) => {
                        if gaze.peek() == Some(Token::OpenBrace) {
                            gaze.next(); //read {
                            add_statement(
                                entity,
                                attribute,
                                Value::Identifier(value.to_owned()),
                                results,
                            )?;
                            handle_entity_expansion(value.to_owned(), gaze, results)?;
                        } else {
                            values.push(Value::Identifier(value))
                        }
                    }
                    Some(Token::CloseSquare) => break,
                    _ => return Err(LigatureError("Expecting Value.".to_owned())),
                }
            }
            for value in values {
                results.push(Statement {
                    entity: entity.to_owned(),
                    attribute: attribute.to_owned(),
                    value,
                });
            }
            Ok(())
        }
        _ => Err(LigatureError("Invalue Value.".to_owned())),
    }
}

fn handle_entity_expansion(
    entity: Identifier,
    gaze: &mut Gaze<Token>,
    results: &mut Vec<Statement>,
) -> Result<(), LigatureError> {
    loop {
        match gaze.next() {
            Some(Token::Identifier(attribute)) => {
                handle_value_expression(&entity, &attribute, gaze, results)?
            }
            Some(Token::CloseBrace) => return Ok(()),
            token => {
                return Err(LigatureError(format!(
                    "Error handling Entity Expansion - {:?}.",
                    token
                )))
            }
        }
    }
}
