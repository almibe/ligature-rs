// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the lig serialization format for Ligature.

use ligature::{Entity, Attribute, LigatureError};
use bytes::Bytes;

struct LigError(String);

fn write_entity(entity: Entity) -> String {
    todo!()
}

fn read_entity(entity: &str) -> Result<Entity, LigError> {
    todo!()
}

fn write_attribute(attribute: Attribute) -> String {
    todo!()
}

fn read_attribute(attribute: &str) -> Result<Attribute, LigError> {
    todo!()
}

fn write_integer(integer: i64) -> String {
    todo!()
}

fn read_integer(integer: &str) -> Result<i64, LigError> {
    todo!()
}

fn write_float(float: f64) -> String {
    todo!()
}

fn read_float(float: &str) -> Result<f64, LigError> {
    todo!()
}

fn write_string(string: String) -> String {
    todo!()
}

fn read_string(string: &str) -> Result<String, LigError> {
    todo!()
}

fn write_bytes(bytes: Bytes) -> String {
    todo!()
}

fn read_bytes(bytes: &str) -> Result<Bytes, LigError> {
    todo!()
}
