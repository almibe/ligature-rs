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
        static ref RE: Regex = Regex::new(r"^[a-zA-Z0-9-._~:/?#\[\]@!$&'()*+,;%=\x{00A0}-\x{D7FF}\x{F900}-\x{FDCF}\x{FDF0}-\x{FFEF}\x{10000}-\x{1FFFD}\x{20000}-\x{2FFFD}\x{30000}-\x{3FFFD}\x{40000}-\x{4FFFD}\x{50000}-\x{5FFFD}\x{60000}-\x{6FFFD}\x{70000}-\x{7FFFD}\x{80000}-\x{8FFFD}\x{90000}-\x{9FFFD}\x{A0000}-\x{AFFFD}\x{B0000}-\x{BFFFD}\x{C0000}-\x{CFFFD}\x{D0000}-\x{DFFFD}\x{E1000}-\x{EFFFD}]+$").unwrap();
    }

    RE.is_match(id)
}

/// Check if a given str only contains valid characters.
/// This is a duplicate of valid_identifier for now but the two could deviate potentially.
pub fn validate_identifier_characters(id: &str) -> bool {
    validate_identifier(id)
}

/// An Entity that is identified by a unique String id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
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
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Value {
    /// An Entity.
    Identifier(Identifier),
    /// A String used for a Ligature literal
    String(String),
    /// An i64 used for a Ligature literal
    Integer(i64),
    // /// An f64 used for a Ligature literal
    // FloatLiteral(f64),
    /// An array of bytes
    Bytes(Bytes),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Identifier(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
            Value::Integer(value) => write!(f, "{value}"),
            Value::Bytes(value) => write!(f, "{value:?}"),
        }
    }
}

/// A set of enums used to express range queries when it makes sense for that type.
#[derive(Debug, Deserialize, Serialize)]
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
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Statement {
    /// The Entity of a Statement
    pub entity: Identifier,
    /// The Attribute of a Statement
    pub attribute: Identifier,
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

/// A trait that defines all the actions a Ligature instance can perform.
pub trait Ligature {
    /// Get all Datasets.
    fn datasets(&self) -> Result<Vec<Dataset>, LigatureError>;
    /// Add a new Dataset.
    /// Does nothing if Dataset already exists.
    fn add_dataset(&mut self, dataset: &Dataset) -> Result<(), LigatureError>;
    /// Remove a Dataset.
    /// Does nothing if Dataset doesn't exist.
    fn remove_dataset(&mut self, dataset: &Dataset) -> Result<(), LigatureError>;
    /// Get all Statements in a given Dataset.
    fn statements(&self, dataset: &Dataset) -> Result<Vec<Statement>, LigatureError>;
    /// Add Statements to a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement already exists in Dataset.
    fn add_statements(
        &self,
        dataset: &Dataset,
        statements: Vec<Statement>,
    ) -> Result<(), LigatureError>;
    /// Remove Statements from a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement doesn't exist in Dataset.
    fn remove_statements(
        &self,
        dataset: &Dataset,
        statements: Vec<Statement>,
    ) -> Result<(), LigatureError>;
    /// Run a query against the given Dataset.
    fn query(&self) -> Result<Box<dyn Query>, LigatureError>; //TODO this is wrong
}

/// Query Ligature instances.
pub trait Query {
    /// Find Statements that match the given pattern.
    /// (None, None, None) returns all Statements.
    fn find(
        &self,
        entity: Option<Identifier>,
        attribute: Option<Identifier>,
        value: Option<Value>,
    ) -> Result<Vec<Statement>, LigatureError>;
}
