// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::LigError;
use gaze::steps::{ignore_all, take_string, take_while_str, NoMatch};
use gaze::Gaze;
use hex::decode;
use ligature::{validate_identifier_characters, Identifier, Statement, Value};

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_identifier(input: &str) -> Result<Identifier, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input.trim());
    gaze.attempt(&identifier_step)
        .map_err(|_| LigError("Could not read Entity.".into()))
}

/// Reads a value from a passed str.
/// Ignores white space but will return an Err if there is any input besides an encoded value.
pub fn read_value(input: &str) -> Result<Value, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input.trim());
    gaze.attempt(&value_step)
        .map_err(|_| LigError("Could not read Value.".into()))
}

pub fn read(input: &str) -> Result<Vec<Statement>, LigError> {
    let mut gaze = Gaze::<&str>::from_str(input);
    let mut result: Vec<Statement> = Vec::new();
    while !gaze.is_complete() {
        gaze.ignore(&ws_step);
        let entity = gaze
            .attempt(&identifier_step)
            .map_err(|_| LigError("Error reading Entity.".into()))?;
        gaze.ignore(&ws_step);
        let attribute = gaze
            .attempt(&identifier_step)
            .map_err(|_| LigError("Error reading Attribute.".into()))?;
        gaze.ignore(&ws_step);
        let value = gaze
            .attempt(&value_step)
            .map_err(|_| LigError("Error reading Value.".into()))?;
        gaze.ignore(&ws_step);
        gaze.ignore(&take_string("\n"));
        result.push(Statement {
            entity,
            attribute,
            value,
        });
    }
    Ok(result)
}

fn identifier_step(gaze: &mut Gaze<&str>) -> Result<Entity, NoMatch> {
    gaze.attempt(&take_string("<"))?; //.map_err(|_| LigError("Could not read Entity.".into()))?;
    let res = gaze.attempt(&take_while_str(&|c: &str| {
        validate_identifier_characters(c)
    }))?;
    gaze.attempt(&take_string(">"))?; //.map_err(|_| LigError("Could not read Entity.".into()))?;
    Ok(Entity::new(&res).map_err(|_| NoMatch)?)
}

fn value_step(gaze: &mut Gaze<&str>) -> Result<Value, NoMatch> {
    if let Ok(entity) = gaze.attempt(&identifier_step) {
        return Ok(Value::Entity(entity));
    }
    if let Ok(string) = gaze.attempt(&string_step) {
        return Ok(string);
    }
    if let Ok(hex) = gaze.attempt(&bytes_step) {
        return Ok(hex);
    }
    if let Ok(number) = gaze.attempt(&number_step) {
        return Ok(number);
    }
    Err(NoMatch) //Err(LigError("Could not match Value".into()))
}

fn is_digit(s: &str) -> bool {
    s == "0"
        || s == "1"
        || s == "2"
        || s == "3"
        || s == "4"
        || s == "5"
        || s == "6"
        || s == "7"
        || s == "8"
        || s == "9"
}

fn is_hex(s: &str) -> bool {
    s == "0"
        || s == "1"
        || s == "2"
        || s == "3"
        || s == "4"
        || s == "5"
        || s == "6"
        || s == "7"
        || s == "8"
        || s == "9"
        || s == "a"
        || s == "b"
        || s == "c"
        || s == "d"
        || s == "e"
        || s == "f"
}

fn ws_step(gaze: &mut Gaze<&str>) -> Result<(), NoMatch> {
    let ws: Vec<&str> = vec![" ", "\t"];
    ignore_all(ws)(gaze)
}

/// Attempts to parse an IntegerLiteral or FloatLiteral.
fn number_step(gaze: &mut Gaze<&str>) -> Result<Value, NoMatch> {
    let integer = gaze.attempt(&take_while_str(&is_digit))?;
    let is_float = gaze.attempt(&take_string("."));
    match is_float {
        Ok(_) => {
            let decimal = gaze.attempt(&take_while_str(&is_digit));
            match decimal {
                Ok(decimal) => {
                    let float = format!("{}.{}", integer, decimal);
                    Ok(Value::FloatLiteral(
                        float.parse::<f64>().map_err(|_| NoMatch)?,
                    ))
                }
                Err(_) => Err(NoMatch),
            }
        }
        Err(_) => Ok(Value::IntegerLiteral(
            integer.parse::<i64>().map_err(|_| NoMatch)?,
        )),
    }
}

fn bytes_step(gaze: &mut Gaze<&str>) -> Result<Value, NoMatch> {
    gaze.attempt(&take_string("0x"))?;
    let content = gaze.attempt(&take_while_str(&is_hex));
    match content {
        Ok(content) => {
            let res = decode(content).map_err(|_| NoMatch)?;
            Ok(Value::BytesLiteral(res))
        }
        Err(_) => Ok(Value::BytesLiteral(vec![])),
    }
}

fn string_step(gaze: &mut Gaze<&str>) -> Result<Value, NoMatch> {
    //TODO this doesn't handle escaping
    gaze.attempt(&take_string("\""))?;
    let content = gaze.attempt(&take_while_str(&|c| c != "\""));
    gaze.attempt(&take_string("\""))?;
    match content {
        Ok(content) => Ok(Value::StringLiteral(content)),
        Err(_) => Ok(Value::StringLiteral("".into())),
    }
}

fn statement_step(gaze: &mut Gaze<&str>) -> Result<Statement, NoMatch> {
    todo!()
}
