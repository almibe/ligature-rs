// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::LigError;
use ligature::{Attribute, Bytes, Entity, Statement, Value};

/// Reads an Entity from the given &str.
/// Will return an error if there is anything other than an Entity + whitespace in the input.
pub fn read_entity(input: &str) -> Result<Entity, LigError> {
    todo!()
}

pub fn read_attribute(attribute: &str) -> Result<Attribute, LigError> {
    todo!()
}

/// Reads a value from a passed str.
/// Ignores white space but will return an Err if there is any input besides an encoded value.
pub fn read_value(value: &str) -> Result<Value, LigError> {
    todo!()
}

pub fn read(statements: &str) -> Result<Vec<Statement>, LigError> {
    todo!()
}
