// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature project.
//! It represents to common types and traits used by Ligature.

#![deny(missing_docs)]

#[macro_use]
extern crate lazy_static;

use std::fmt::Display;
use regex::Regex;
use serde::{Deserialize, Serialize};

/// A simple type alias for Bytes.
pub type Bytes = Vec<u8>;


/// A Slot is a place holder in a Network used for pattern matching.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize, Hash)]
pub struct Slot(String);


impl Display for Slot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${}", self.0)
    }
}

/// Check if a given name is valid.
pub fn validate_name(id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;%=\x{00A0}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFEF}\x{10000}-\x{1FFFD}\x{20000}-\x{2FFFD}\x{30000}-\x{3FFFD}\x{40000}-\x{4FFFD}\x{50000}-\x{5FFFD}\x{60000}-\x{6FFFD}\x{70000}-\x{7FFFD}\x{80000}-\x{8FFFD}\x{90000}-\x{9FFFD}\x{A0000}-\x{AFFFD}\x{B0000}-\x{BFFFD}\x{C0000}-\x{CFFFD}\x{D0000}-\x{DFFFD}\x{E1000}-\x{EFFFD}]+$").unwrap();
    }

    RE.is_match(id)
}

/// Check if a given str only contains valid characters.
/// This is a duplicate of valid_name for now but the two could deviate potentially.
pub fn validate_name_characters(id: &str) -> bool {
    validate_name(id)
}

/// An Entity that is identified by a unique String id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Name(pub String);

impl Display for Name {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// An enum that represents all the currently supported Value types.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Value {
    /// A Name.
    Name(Name),
    /// A Slot.
    Slot(Slot),
    /// A String used for a Ligature literal
    String(String),
    /// An i64 used for a Ligature literal
    Integer(i64),
    /// An array of bytes
    Bytes(Bytes),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Name(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
            Value::Integer(value) => write!(f, "{value}"),
            Value::Bytes(value) => write!(f, "{value:?}"),
            Value::Slot(value) => write!(f, "{value}"),
        }
    }
}

/// An enum that represents a Name or Slot.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Pattern {
    /// A Name.
    Name(Name),
    /// A Slot.
    Slot(Slot),
}

/// A Statement is a grouping of an Entity, an Attribute, and a Value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Statement {
    /// The Entity of a Statement
    pub entity: Name,
    /// The Attribute of a Statement
    pub attribute: Name,
    /// The Value of a Statement
    pub value: Value,
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.entity, self.attribute, self.value)
    }
}

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct LigatureError(pub String);
