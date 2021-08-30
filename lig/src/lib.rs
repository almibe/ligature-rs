// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the lig serialization format for Ligature.

mod parser;

use ligature::{Entity, Attribute, Value, LigatureError, Statement};
use bytes::Bytes;
use parser::Parser;

/// A error related to parsing Lig.
pub struct LigError(String);

/// Writes out an Entity to a String.
pub fn write_entity(entity: &Entity) -> String {
    format!("<{}>", entity.id())
}

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    let mut parser = Parser::new(input);
    //TODO trim white space
    let res = parse_entity(&mut parser);
    //TODO assert only white space is left
    res
}

/// Parses an Entity from the passed parser instance.
/// Will only read an Entity and ignore the rest.
pub fn parse_entity(parser: &mut Parser) -> Result<Entity, LigError> {
    //parser.take()
    todo!()
}

pub fn write_attribute(attribute: &Attribute) -> String {
    format!("@<{}>", attribute.name())
}

pub fn read_attribute(attribute: &str) -> Result<Attribute, LigError> {
    todo!()
}

pub fn write_value(value: &Value) -> String {
    todo!()
}

pub fn write_integer(integer: i64) -> String {
    todo!()
}

pub fn read_integer(integer: &str) -> Result<i64, LigError> {
    todo!()
}

pub fn write_float(float: f64) -> String {
    todo!()
}

pub fn read_float(float: &str) -> Result<f64, LigError> {
    todo!()
}

pub fn write_string(string: &str) -> String {
    todo!()
}

pub fn read_string(string: &str) -> Result<String, LigError> {
    todo!()
}

pub fn write_bytes(bytes: &Bytes) -> String {
    todo!()
}

pub fn read_bytes(bytes: &str) -> Result<Bytes, LigError> {
    todo!()
}

pub fn write_statement(statement: &Statement) -> &str {
    todo!()
}

pub fn read(statements: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}

pub fn write(statements: std::slice::Iter<Statement>) -> String {
    let mut result = String::new();
    for statement in statements {
        result += write_statement(statement);
        result += "\n";
    }
    result
}
