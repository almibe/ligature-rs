// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};

#[cfg(feature = "heed")]
pub mod heed;
pub mod mem;

/// The data structure stored in this triple store.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Trip(pub String, pub String, pub String);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Contains either a Variable or Value, used for Queries.
pub enum Slot {
    /// A Variable.
    Variable(String),
    /// A Value.
    Value(String),
    /// Match any value.
    Any,
}

/// The data structure used to represent queries.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Query(pub Slot, pub Slot, pub Slot);

/// A trait that defines all the actions a Ligature instance can perform.
/// The API used for storing triples.
pub trait Trips<E> {
    /// Get all Collections.
    fn collections(&self) -> Result<Vec<String>, E>;

    /// Add a new Dataset.
    /// Does nothing if Dataset already exists.
    fn add_collection(&mut self, collection: String) -> Result<(), E>;

    /// Remove a Dataset.
    /// Does nothing if Dataset doesn't exist.
    fn remove_collection(&mut self, collection: String) -> Result<(), E>;

    /// Get all Statements in a given Dataset.
    fn triples(&self, collection: String) -> Result<BTreeSet<Trip>, E>;

    /// Add Statements to a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement already exists in Dataset.
    fn add_triples(&mut self, collection: String, trips: &mut BTreeSet<Trip>) -> Result<(), E>;
    /// Remove Statements from a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement doesn't exist in Dataset.
    fn remove_triples(&mut self, collection: String, trips: &mut BTreeSet<Trip>) -> Result<(), E>;
    /// Run a query against the given Dataset.
    fn query(
        &self,
        collection: String,
        pattern: BTreeSet<Query>,
    ) -> Result<HashBag<BTreeMap<String, String>>, E>;
}
