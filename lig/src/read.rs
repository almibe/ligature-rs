// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::parser::Parser;
use crate::LigError;
use ligature::{Attribute, Bytes, Entity, Statement, Value};

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    let mut parser = Parser::new(input);
    parser.ignore_ws();
    let res = parse_entity(&mut parser);
    parser.ignore_ws();
    //TODO double check that parser is complete
    res
}

/// Parses an Entity from the passed parser instance.
/// Will only read an Entity and ignore the rest.
pub fn parse_entity(parser: &mut Parser) -> Result<Entity, LigError> {
    parser.save();
    let opening = parser.take("<");
    if !opening {
        parser.rollback();
        todo!("throw error")
    }

    todo!()
}

pub fn read_attribute(attribute: &str) -> Result<Attribute, LigError> {
    todo!()
}

pub fn read_value(value: &str) -> Result<Value, LigError> {
    todo!()
}

pub fn read_integer(integer: &str) -> Result<i64, LigError> {
    todo!()
}

pub fn read_float(float: &str) -> Result<f64, LigError> {
    todo!()
}

pub fn read_string(string: &str) -> Result<String, LigError> {
    todo!()
}

pub fn read_bytes(bytes: &str) -> Result<Bytes, LigError> {
    todo!()
}

pub fn read(statements: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}
