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
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_]+(/[a-zA-Z0-9_]+)*$").unwrap();
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

/// A node that is identified by a unique u64 id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Node(u64);

/// A named connection between two nodes.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct Arrow(String);

impl Arrow {
    /// Creates a new Arrow and returns a Result based on if it is valid or not.
    pub fn new(name: &str) -> Result<Self, LigatureError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z_]+(/[a-zA-Z0-9_]+)*$").unwrap();
        }

        if RE.is_match(name) {
            Ok(Self(name.to_string()))
        } else {
            Err(LigatureError(format!("Invalid Arrow name {}", name)))
        }
    }

    /// Returns the name of the given Arrow.
    pub fn name(&self) -> &str {
        &self.0
    }
}

/// An enum that represents all the currently supported literal types.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Vertex {
    /// A node.
    Node(Node),
    /// A tagged String used for an RDF literal
    StringLiteral(String),
    /// A tagged bool used for an RDF literal
    BooleanLiteral(bool),
    /// A tagged i64 used for an RDF literal
    LongLiteral(i64),
    /// A tagged f64 used for an RDF literal
    DoubleLiteral(f64),
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
    LongLiteralRange {
        /// The starting i64 (inclusive)
        start: i64,
        /// The end i64 (exclusive)
        end: i64,
    },
    /// Represents a String range using basic f64 comparisons.
    DoubleLiteralRange {
        /// The starting f64 (inclusive)
        start: f64,
        /// The end f64 (exclusive)
        end: f64,
    },
}

/// A Link is a grouping of a source, an arrow, and a target.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Link {
    /// The Source of a Link
    pub source: Vertex,
    /// The Arrow of a Link
    pub arrow: Arrow,
    /// The Target of a Link
    pub target: Vertex,
}

/// A Link that has been persisted so it has an assoicated context.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct PersistedLink {
    /// The Target of a Link
    pub link: Link,
    /// The Context of this Link
    pub context: Node,
}

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug, PartialEq, Eq)]
pub struct LigatureError(pub String);

/// A struct that is returned from SPAQRL and wander queries.
pub struct QueryResult {
    /// A Vec of headers for the results.
    pub headers: Vec<String>,
    /// A stream of results, the inner Vec has the same lenth as the headers Vec.
    pub results: Box<dyn Iterator<Item = Result<Vec<Vertex>, LigatureError>>>,
}

/// A trait that all Ligature implementations implement.
pub trait Ligature {
    /// Returns all Datasets in a Ligature instance.
    fn all_datasets(&self) -> Box<dyn Iterator<Item = Result<Dataset, LigatureError>>>;

    /// Returns all Datasets in a Ligature instance that start with the given prefix.
    fn match_datasets(
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
    fn create_dataset(&self, dataset: Dataset) -> Result<(), LigatureError>;

    /// Deletes a dataset with the given name.
    /// TODO should probably return its own error type { InvalidDataset, CouldNotDeleteDataset }
    fn delete_dataset(&self, dataset: Dataset) -> Result<(), LigatureError>;

    /// Initiazes a QueryTx
    /// TODO should probably return its own error type CouldNotInitializeQueryTx
    fn query(&self, dataset: Dataset) -> Result<Box<dyn QueryTx>, LigatureError>;

    /// Initiazes a WriteTx
    /// TODO should probably return its own error type CouldNotInitializeWriteTx
    fn write(&self, dataset: Dataset) -> Result<Box<dyn WriteTx>, LigatureError>;
}

/// Represents a QueryTx within the context of a Ligature instance and a single Dataset
pub trait QueryTx {
    /// Returns all PersistedLinks in this Dataset.
    fn all_links(&self) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>>;

    /// Returns all PersistedLinks that match the given criteria.
    /// If a parameter is None then it matches all, so passing all Nones is the same as calling all_statements.
    fn match_links(
        &self,
        source: Option<Vertex>,
        arrow: Option<Arrow>,
        target: Option<Vertex>,
    ) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>>;

    /// Retuns all PersistedLinks that match the given criteria.
    /// If a parameter is None then it matches all.
    fn match_links_range(
        &self,
        source: Option<Vertex>,
        arrow: Option<Arrow>,
        target: Range,
    ) -> Box<dyn Iterator<Item = Result<PersistedLink, LigatureError>>>;

    /// Returns the PersistedLink for the given context.
    fn link_for_context(&self, context: Node) -> Result<PersistedLink, LigatureError>;

    /// Run a wander query.
    fn wander_query(&self, query: String) -> Result<QueryResult, LigatureError>;
}

/// Represents a WriteTx within the context of a Ligature instance and a single Dataset
pub trait WriteTx {
    /// Creates a new, unique Node within this Dataset.
    /// Note: Nodes are shared across named graphs in a given Dataset.
    fn new_node(&self) -> Result<Node, LigatureError>;

    /// Adds a given Link to this Dataset.
    /// If the Link already exists nothing happens (TODO maybe add it with a new context?).
    /// Note: Potentally could trigger a ValidationError
    fn add_link(&self, link: Link) -> Result<PersistedLink, LigatureError>;

    /// Removes a given PersistedLink from this Dataset.
    /// If the PersistedLink doesn't exist nothing happens.
    /// Note: Potentally could trigger a ValidationError.
    fn remove_link(&self, persisted_link: PersistedLink) -> Result<PersistedLink, LigatureError>;

    /// Cancels this transaction so that none of the changes made so far will be stored.
    /// This also closes this transaction so no other methods can be called.
    fn cancel(&self) -> Result<(), LigatureError>;

    /// Commits this transaction.
    /// This also closes this transaction so no other methods can be called.
    fn commit(&self) -> Result<(), LigatureError>;
}
