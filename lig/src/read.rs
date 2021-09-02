// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::parser::Parser;
use crate::LigError;
use ligature::{Attribute, Bytes, Entity, Statement, Value};

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    //TODO rewrite to use using str.trim() instead of ignore_ws
    let mut parser = Parser::new(input);
    parser.ignore_ws();
    let res = parse_entity(&mut parser);
    match res {
        Ok(_) => {
            parser.ignore_ws();
            if parser.is_complete() {
                res
            } else {
                Err(LigError(format!("Invalid Entity: Extra input provided.")))
            }
        }
        Err(_) => res,
    }
}

/// Parses an Entity from the passed parser instance.
/// Will only read an Entity and ignore the rest.
pub fn parse_entity(parser: &mut Parser) -> Result<Entity, LigError> {
    parser.save();
    let opening = parser.take("<");
    if !opening {
        parser.rollback();
        return Err(LigError(format!(
            "Invalid Entity: Entities must start with <"
        )));
    }
    let entity = parser.take_until(Box::new(|c: char| c == '>'));
    match entity {
        None => {
            parser.rollback();
            Err(LigError(format!("Invalid Entity")))
        }
        Some(e) => Ok(Entity::new(e.as_str())?),
    }
}

pub fn read_attribute(attribute: &str) -> Result<Attribute, LigError> {
    let trimmed = attribute.trim();
    let mut parser = Parser::new(trimmed);
    let res = parse_attribute(&mut parser);
    match res {
        Ok(_) => {
            if parser.is_complete() {
                res
            } else {
                Err(LigError(format!("Invalid Entity: Extra input provided.")))
            }
        }
        Err(_) => res,
    }
}

pub fn parse_attribute(parser: &mut Parser) -> Result<Attribute, LigError> {
    parser.save();
    let opening = parser.take("@<");
    if !opening {
        parser.rollback();
        return Err(LigError(format!(
            "Invalid Attribute: Attributes must start with @<"
        )));
    }
    let attribute = parser.take_until(Box::new(|c: char| c == '>'));
    match attribute {
        None => {
            parser.rollback();
            Err(LigError(format!("Invalid Attribute")))
        }
        Some(e) => Ok(Attribute::new(e.as_str())?),
    }
}

/// Reads a value from a passed str.
/// Ignores white space but will return an Err if there is any input besides an encoded value.
pub fn read_value(value: &str) -> Result<Value, LigError> {
    let trimmed = value.trim();
    let mut parser = Parser::new(trimmed);
    let res = parse_value(&mut parser);
    match res {
        Ok(_) => {
            if parser.is_complete() {
                res
            } else {
                Err(LigError(format!("Invalid Value: Extra input provided.")))
            }
        }
        Err(_) => res,
    }
}

/// Attempts to parse a value out of the given Parser.
/// If it fails the location of the Parser will be the same but an Err will be returned.
pub fn parse_value(parser: &mut Parser) -> Result<Value, LigError> {
    let peek = parser.peek();
    match peek {
        None => Err(LigError("Could not read value.".into())),
        Some(c) => match c {
            '<' => Ok(Value::Entity(parse_entity(parser)?)),
            '0'..='9' => parse_number(parser),
            '"' => parse_string(parser),
            _ => Err(LigError("Could not read value.".into())),
        },
    }
}

/// Attempts to parse an IntegerLiteral, a FloatLiteral, or a BytesLiteral  from the given Parser.
pub fn parse_number(parser: &mut Parser) -> Result<Value, LigError> {
    parser.save();
    let mut value = String::new();
    loop {
        let c = parser.next();
        match c {
            None => {
                if value.len() > 0 {
                    todo!("return integer literal")
                } else {
                    return Err(LigError("Could not parse number.".into()));
                }
            }
            Some(c) => {
                match c {
                    '0'..='9' => value.push(c),
                    '.' => {
                        return if value.len() > 0 {
                            parse_float(parser, &mut value)
                        } else {
                            Err(LigError("Could not parse number.".into()))
                        }
                    },
                    'x' => {
                        return if value == "0" {
                            value.push(c);
                            parse_bytes(parser, &mut value)
                        } else {
                            Err(LigError("Could not parse number.".into()))
                        }
                    },
                    //TODO add support for
                    _ => return Err(LigError("Could not parse number.".into())),
                }
            }
        }
    }

    fn parse_float(parser: &mut Parser, value: &mut String) -> Result<Value, LigError> {
        todo!()
    }

    fn parse_bytes(parser: &mut Parser, value: &mut String) -> Result<Value, LigError> {
        todo!()
    }
}

/// Attempts to parse a StringLiteral from the given Parser.
pub fn parse_string(parser: &mut Parser) -> Result<Value, LigError> {
    todo!()
}

pub fn read(statements: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}
