// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use hex::encode;
use ligature::{Attribute, Bytes, Entity, Statement, Value};

/// Writes out an Entity to a String.
pub fn write_entity(entity: &Entity) -> String {
    format!("<{}>", entity.id())
}

pub fn write_attribute(attribute: &Attribute) -> String {
    format!("@<{}>", attribute.name())
}

pub fn write_value(value: &Value) -> String {
    match value {
        Value::Entity(entity) => write_entity(entity),
        Value::IntegerLiteral(integer) => write_integer(integer),
        Value::FloatLiteral(float) => write_float(float),
        Value::StringLiteral(string) => write_string(string),
        Value::BytesLiteral(bytes) => write_bytes(bytes),
    }
}

pub fn write_integer(integer: &i64) -> String {
    format!("{}", integer)
}

pub fn write_float(float: &f64) -> String {
    let res = format!("{}", float);
    if res.contains(".") {
        res
    } else {
        res + ".0"
    }
}

pub fn write_string(string: &str) -> String {
    //TODO this could be done better
    let escaped_string = string
        .replace("\"", "\\\"")
        //.replace("\f", "\\b") <-- TODO not sure how to handle this or if I really need to
        //.replace("\b", "\\b") <-- TODO not sure how to handle this or if I really need to
        .replace("\n", "\\n")
        .replace("\r", "\\r")
        .replace("\t", "\\t")
        .replace("\\", "\\\\");
    format!("\"{}\"", escaped_string)
}

pub fn write_bytes(bytes: &Bytes) -> String {
    format!("0x{}", encode(bytes))
}

pub fn write_statement(statement: &Statement) -> String {
    format!(
        "{} {} {} {}\n",
        write_entity(&statement.entity),
        write_attribute(&statement.attribute),
        write_value(&statement.value),
        write_entity(&statement.context)
    )
}

pub fn write(statements: std::slice::Iter<Statement>) -> String {
    let mut result = String::new();
    for statement in statements {
        result += &*write_statement(statement);
    }
    result
}
