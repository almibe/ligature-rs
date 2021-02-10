// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature project.
//! It represents to common types and traits used by Ligature.

#![deny(missing_docs)]
//#![deny(missing_doc_example)] <-- for later, when I'm swole

#[macro_use]
extern crate lazy_static;

use regex::Regex;
use serde::{Deserialize, Serialize};

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

/// An Entity that is identified by a unique u64 id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Entity(pub u64);

/// A named connection between an Entity and a Value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Attribute(String);

impl Attribute {
    /// Creates a new Arrow and returns a Result based on if it is valid or not.
    pub fn new(name: &str) -> Result<Self, LigatureError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_]{1}[a-zA-Z0-9_]*$").unwrap();
        }

        if RE.is_match(name) {
            Ok(Self(name.to_string()))
        } else {
            Err(LigatureError(format!("Invalid Attribute name {}", name)))
        }
    }

    /// Returns the name of the given Arrow.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// An enum that represents all the currently supported Value types.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Value {
    /// An Entity.
    Entity(Entity),
    /// A String used for a Ligature literal
    StringLiteral(String),
    /// An i64 used for a Ligature literal
    IntegerLiteral(i64),
    /// An f64 used for a Ligature literal
    FloatLiteral(f64),
}

/// A set of enums used to express range queries when it makes sense for that type (ie no support for BooleanLiteralRange or UnknownLiteralRange since they don't make sense).
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
    /// Represents a String range using basic f64 comparisons.
    FloatLiteralRange {
        /// The starting f64 (inclusive)
        start: f64,
        /// The end f64 (exclusive)
        end: f64,
    },
}

/// A Statement is a grouping of an Entity, an Attribute, and a Value.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Statement {
    /// The Entity of a Statement
    pub entity: Entity,
    /// The Attribute of a Statement
    pub attribute: Attribute,
    /// The Value of a Statement
    pub value: Value,
}

/// A Statement that has been persisted so it has an assoicated Context Entity.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PersistedStatement {
    /// The target Statment
    pub statement: Statement,
    /// The Context of this Statement
    pub context: Entity,
}

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug, PartialEq, Eq)]
pub struct LigatureError(pub String);

/// A trait that all Ligature implementations implement.
pub trait Ligature {
    /// Returns all Datasets in a Ligature instance.
    fn all_datasets(&self) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>>;

    /// Check if a given Dataset exists.
    fn dataset_exists(&self, dataset: &Dataset) -> Result<bool, LigatureError>;

    /// Returns all Datasets in a Ligature instance that start with the given prefix.
    fn match_datasets_prefix(
        &self,
        prefix: &str,
    ) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>>;

    /// Returns all Datasets in a Ligature instance that are in a given range (inclusive, exclusive].
    fn match_datasets_range(
        &self,
        start: &str,
        end: &str,
    ) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>>;

    /// Creates a dataset with the given name.
    /// TODO should probably return its own error type { InvalidDataset, DatasetExists, CouldNotCreateDataset }
    fn create_dataset(&self, dataset: &Dataset) -> Result<(), LigatureError>;

    /// Deletes a dataset with the given name.
    /// TODO should probably return its own error type { InvalidDataset, CouldNotDeleteDataset }
    fn delete_dataset(&self, dataset: &Dataset) -> Result<(), LigatureError>;

    /// Initiazes a QueryTx
    /// TODO should probably return its own error type CouldNotInitializeQueryTx
    fn query<T>(&self, dataset: &Dataset, f: QueryFn<T>) -> Result<T, LigatureError>;

    /// Initiazes a WriteTx
    /// TODO should probably return its own error type CouldNotInitializeWriteTx
    fn write<T>(&self, dataset: &Dataset, f: WriteFn<T>) -> Result<T, LigatureError>;
}

/// An Fn that is used when making a Query tranaction.
pub type QueryFn<T> = Box<dyn Fn(Box<dyn QueryTx>) -> Result<T, LigatureError>>;

/// An Fn that is used when making a Write transaction.
pub type WriteFn<T> = Box<dyn Fn(Box<dyn WriteTx>) -> Result<T, LigatureError>>;

/// Represents a QueryTx within the context of a Ligature instance and a single Dataset
pub trait QueryTx {
    /// Returns all PersistedStatements in this Dataset.
    fn all_statements(&self)
        -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>>;

    /// Returns all PersistedStatements that match the given criteria.
    /// If a parameter is None then it matches all, so passing all Nones is the same as calling all_statements.
    fn match_statements(
        &self,
        source: Option<Entity>,
        arrow: Option<Attribute>,
        target: Option<Value>,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>>;

    /// Retuns all PersistedStatements that match the given criteria.
    /// If a parameter is None then it matches all.
    fn match_statements_range(
        &self,
        source: Option<Entity>,
        arrow: Option<Attribute>,
        target: Range,
    ) -> Box<dyn Iterator<Item = Result<PersistedStatement, LigatureError>>>;

    /// Returns the PersistedStatement for the given context.
    fn statement_for_context(
        &self,
        context: &Entity,
    ) -> Result<Option<PersistedStatement>, LigatureError>;
}

/// Represents a WriteTx within the context of a Ligature instance and a single Dataset
pub trait WriteTx {
    /// Creates a new, unique Entity within this Dataset.
    /// Note: Entitys are shared across named graphs in a given Dataset.
    fn new_entity(&self) -> Result<Entity, LigatureError>;

    /// Adds a given Statement to this Dataset.
    /// If the Statement already exists nothing happens (TODO maybe add it with a new context?).
    /// Note: Potentally could trigger a ValidationError
    fn add_statement(&self, statement: &Statement) -> Result<PersistedStatement, LigatureError>;

    /// Removes a given PersistedStatement from this Dataset.
    /// If the PersistedStatement doesn't exist nothing happens.
    /// Note: Potentally could trigger a ValidationError.
    fn remove_statement(
        &self,
        persisted_statement: &PersistedStatement,
    ) -> Result<(), LigatureError>;

    /// Cancels this transaction so that none of the changes made so far will be stored.
    /// This also closes this transaction so no other methods can be called without returning a LigatureError.
    fn cancel(&self) -> Result<(), LigatureError>;
}
