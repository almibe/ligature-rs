// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module contains Ligature's data model in Rust.

#![deny(missing_docs)]

use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};
use serde::{Deserialize, Serialize};

/// An Element that is identified by a unique String value.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub struct Element(pub String);

/// A single entry in a Network.
#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
pub enum Entry {
    /// A relationship between two Elements.
    Role {
        /// The first Element.
        first: Element,
        /// The second Element.
        second: Element,
        /// The Role between the two Elements.
        role: Element,
    },
    /// Represens an Element extending a Concept.
    Extends {
        /// The Element.
        element: Element,
        /// The Concept the Element extends.
        concept: Element,
    },
    /// Represens an Element not extending a Concept.
    NotExtends {
        /// The Element.
        element: Element,
        /// The Concept the Element does not extend.
        concept: Element,
    },
}

/// A simple error message.
pub struct LigatureError(pub String);

/// A trait that defines all the actions a Ligature instance can perform.
/// The API used for storing triples.
pub trait Ligature {
    /// Get all Collections.
    fn collections(&self) -> Result<Vec<Element>, LigatureError>;

    /// Add a new Dataset.
    /// Does nothing if Dataset already exists.
    fn add_collection(&mut self, collection: Element) -> Result<(), LigatureError>;

    /// Remove a Dataset.
    /// Does nothing if Dataset doesn't exist.
    fn remove_collection(&mut self, collection: Element) -> Result<(), LigatureError>;

    /// Get all Statements in a given Dataset.
    fn entries(&self, collection: Element) -> Result<BTreeSet<Entry>, LigatureError>;

    /// Add Statements to a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement already exists in Dataset.
    fn add_entries(&mut self, collection: Element, entries: &mut BTreeSet<Entry>) -> Result<(), LigatureError>;
    /// Remove Statements from a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement doesn't exist in Dataset.
    fn remove_entries(
        &mut self,
        collection: Element,
        entries: &mut BTreeSet<Entry>,
    ) -> Result<(), LigatureError>;
    /// Run a query against the given Dataset.
    fn query(
        &self,
        collection: Element,
        pattern: BTreeSet<Entry>,
    ) -> Result<HashBag<BTreeMap<String, String>>, LigatureError>;
}
