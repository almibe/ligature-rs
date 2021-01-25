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

/// A string that represents a Dataset by name.
/// Currently can only be ASCII text separated by /
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
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

/// A node that is only identified by a unique u64 id.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct BlankNode(u64);

/// An IRI is represented via https://www.ietf.org/rfc/rfc3987.txt
/// TODO add validator and tests
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct IRI(String);

/// A unit struct used to represent the concept of a Default Graph in a quad store.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct DefaultGraph;

/// A wrapper type that represents a language tag.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct LangTag(String);

impl LangTag {
    /// Create a new LangTag from a &str and returns a Result based on if it is valid or not.
    pub fn new(tag: &str) -> Result<LangTag, LigatureError> {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^[a-zA-Z]+(\-[a-zA-Z0-9]+)*$").unwrap();
        }

        if RE.is_match(tag) {
            Ok(LangTag(tag.to_string()))
        } else {
            Err(LigatureError(format!("Invalid LangTag value {}", tag)))
        }
    }

    /// Returns the value of the LangTag.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// A struct containing text and a language tag that denotes what language the text is expressed in.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct LangLiteral {
    /// The String literal that is represented by this LangLiteral
    pub value: String,
    /// The LangTag that represents the language used to express the value.
    pub lang_tag: LangTag,
}

/// A struct containing a value represented as a String and the type of the value represented by an IRI.
/// TODO probably need a function that double checks a given UnknownLiteral is actually unknown
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub struct UnknownLiteral {
    /// The value of this literal represented as a String.
    pub value: String,
    /// The IRI that represents this type.
    pub r#type: IRI,
}

/// An enum that represents all the currently supported literal types.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Literal {
    /// A tagged LangLiteral used for an RDF literal
    LangLiteral(LangLiteral),
    /// A tagged String used for an RDF literal
    StringLiteral(String),
    /// A tagged bool used for an RDF literal
    BooleanLiteral(bool),
    /// A tagged i64 used for an RDF literal
    LongLiteral(i64),
    /// A tagged f64 used for an RDF literal
    DoubleLiteral(f64),
    /// A tagged UnknownLiteral used for an RDF literal
    UnknownLiteral(UnknownLiteral),
}

/// A set of enums used to express range queries when it makes sense for that type (ie no support for BooleanLiteralRange or UnknownLiteralRange since they don't make sense).
#[derive(Debug)]
pub enum Range {
    /// Represents a range of LangLiterals
    /// Note: LangTag needs to match for any comparison to take place.
    LangLiteralRange {
        /// The starting LangLiteral (inclusive)
        start: LangLiteral,
        /// The end LangLiteral (exclusive)
        end: LangLiteral,
    },
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

/// The set of valid types that can be used as a Subject.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Subject {
    /// A tagged IRI used as a Subject.
    IRI(IRI),
    /// A tagged BlankNode used as a Subject.
    BlankNode(BlankNode),
    /// A tagged DefaultGraph used as a Subject.
    DefaultGraph(DefaultGraph),
}

/// The set of valid types that can be used as a Predicate.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Predicate {
    /// A tagged IRI used as a Predicate.
    IRI(IRI),
}

/// The set of valid types that can be used as an Object.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub enum Object {
    /// A tagged Subject used as an Object.
    Subject(Subject),
    /// A tagged Literal used as an Object.
    Literal(Literal),
}

/// The set of valid types that can be used as a Graph name.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq)]
pub enum Graph {
    /// A tagged IRI used as a Graph.
    IRI(IRI),
    /// A tagged BlankNode used as a Graph.
    BlankNode(BlankNode),
    /// A tagged DefaultGraph used as a Graph.
    DefaultGraph(DefaultGraph),
}

/// A Statement is a grouping of Subject, Predicate, and Object.
#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Statement {
    /// The Subject of a Statement
    pub subject: Subject,
    /// The Predicate of a Statement
    pub predicate: Predicate,
    /// The Object of a Statement
    pub object: Object,
    /// The Graph this Statement is in
    pub graph: Graph,
}

//val a: IRI = IRI("http://www.w3.org/1999/02/22-rdf-syntax-ns#type").getOrElse(???)

/// A general struct for representing errors involving Ligature.
/// TODO should probably be an enum with a bunch of specific cases
#[derive(Debug)]
pub struct LigatureError(String);

/// A struct that is returned from SPAQRL and wander queries.
pub struct QueryResult {
    /// A Vec of headers for the results.
    pub headers: Vec<String>,
    /// A stream of results, the inner Vec has the same lenth as the headers Vec.
    pub results: Box<dyn Iterator<Item = Vec<Object>>>,
}

/// A trait that all Ligature implementations implement.
pub trait Ligature {
    /// Returns all Datasets in a Ligature instance.
    fn all_datasets(&self) -> Box<dyn Iterator<Item = Dataset>>;

    /// Returns all Datasets in a Ligature instance that start with the given prefix.
    fn match_datasets(&self, prefix: &str) -> Box<dyn Iterator<Item = Dataset>>;

    /// Returns all Datasets in a Ligature instance that are in a given range (inclusive, exclusive].
    fn match_datasets_range(&self, start: &str, end: &str) -> Box<dyn Iterator<Item = Dataset>>;

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
    /// Returns all Statements in this Dataset as Statements.
    /// TODO should probably return a Result
    fn all_statements(&self) -> Box<dyn Iterator<Item = Statement>>;

    /// Run a SPARQL query.
    fn sparql_query(&self, query: String) -> Result<QueryResult, LigatureError>;

    /// Run a wander query.
    fn wander_query(&self, query: String) -> Result<QueryResult, LigatureError>;
}

/// Represents a WriteTx within the context of a Ligature instance and a single Dataset
pub trait WriteTx {
    /// Creates a new, unique BlankNode within this Dataset.
    /// Note: BlankNodes are shared across named graphs in a given Dataset.
    fn new_blank_node(&self) -> Result<BlankNode, LigatureError>;

    /// Adds a given Statement to this Dataset.
    /// If the Statement already exists nothing happens.
    /// Note: Potentally could trigger a ValidationError
    fn add_statement(&self, statement: Statement) -> Result<Statement, LigatureError>;

    /// Removes a given Statement from this Dataset.
    /// If the Statement doesn't exist nothing happens.
    /// Note: Potentally could trigger a ValidationError.
    fn remove_statement(&self, statement: Statement) -> Result<Statement, LigatureError>;

    /// Cancels this transaction so that none of the changes made so far will be stored.
    /// This also closes this transaction so no other methods can be called.
    fn cancel(&self) -> Result<(), LigatureError>;

    /// Commits this transaction.
    /// This also closes this transaction so no other methods can be called.
    fn commit(&self) -> Result<(), LigatureError>;
}
