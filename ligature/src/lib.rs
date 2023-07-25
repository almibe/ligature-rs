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
use uuid::Uuid;

/// A simple type alias for Bytes.
pub type Bytes = Vec<u8>;

/// A string that represents a Dataset by name.
/// Currently can only be ASCII text separated by /
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Serialize, Deserialize)]
pub struct Dataset(String);

impl Dataset {
    /// Creates a new Dataset and returns a Result based on if it is valid or not.
    pub fn new(name: &str) -> Result<Dataset, LigatureError> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^([a-zA-Z_]{1}[a-zA-Z0-9_]*)(/[a-zA-Z_]{1}[a-zA-Z0-9_]*)*$").unwrap();
        }

        if RE.is_match(name) {
            Ok(Dataset(name.to_string()))
        } else {
            Err(LigatureError(format!("Invalid Dataset name {}", name)))
        }
    }

    /// Returns the name of the given Dataset.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// Check if a given identifier is valid.
pub fn validate_identifier(id: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;%=]+$").unwrap();
    }

    RE.is_match(id)
}

/// Check if a given str only contains valid characters.
/// This is a duplicate of valid_identifier for now but the two could deviate potentially.
pub fn validate_identifier_characters(id: &str) -> bool {
    validate_identifier(id)
}

/// An Entity that is identified by a unique String id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Identifier(String);

impl Identifier {
    /// Creates a new Identifier and returns a Result based on if it is valid or not.
    pub fn new(name: &str) -> Result<Self, LigatureError> {
        if validate_identifier(name) {
            Ok(Self(name.to_string()))
        } else {
            Err(LigatureError(format!("Invalid Entity id {}", name)))
        }
    }

    /// Returns the name of the given Identifier.
    pub fn id(&self) -> &str {
        &self.0
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{}>", self.0)
    }
}

/// Creates a new Identifier with an optional prefix.
pub fn new_identifier(prefix: Option<String>) -> Result<Identifier, LigatureError> {
    let uuid = Uuid::new_v4().hyphenated().to_string();
    let p = match prefix {
        Some(s) => s + &uuid,
        None => uuid,
    };
    Identifier::new(p.as_str())
}

/// An enum that represents all the currently supported Value types.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Value {
    /// An Entity.
    Identifier(Identifier),
    /// A String used for a Ligature literal
    StringLiteral(String),
    /// An i64 used for a Ligature literal
    IntegerLiteral(i64),
    // /// An f64 used for a Ligature literal
    // FloatLiteral(f64),
    /// An array of bytes
    BytesLiteral(Bytes),
}

/// A set of enums used to express range queries when it makes sense for that type.
#[derive(Debug)]
pub enum Range {
    /// Represents a String range using basic String comparisons.
    StringLiteralRange {
        /// The starting String (inclusive)
        start: String,
        /// The end String (exclusive)
        end: String,
    },
    /// Represents a String range using basic i64 comparisons.
    IntegerLiteralRange {
        /// The starting i64 (inclusive)
        start: i64,
        /// The end i64 (exclusive)
        end: i64,
    },
    // /// Represents a String range using basic f64 comparisons.
    // FloatLiteralRange {
    //     /// The starting f64 (inclusive)
    //     start: f64,
    //     /// The end f64 (exclusive)
    //     end: f64,
    // },
    /// Represents a String range using basic Bytes comparisons.
    BytesLiteralRange {
        /// The starting Byte array (inclusive)
        start: Bytes,
        /// The end Byte array (exclusive)
        end: Bytes,
    },
}

/// A Statement is a grouping of an Entity, an Attribute, and a Value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Statement {
    /// The Entity of a Statement
    pub entity: Identifier,
    /// The Attribute of a Statement
    pub attribute: Identifier,
    /// The Value of a Statement
    pub value: Value,
}

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug, PartialEq, Eq)]
pub struct LigatureError(pub String);
