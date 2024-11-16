// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use core::hash::Hash;
use hashbag::HashBag;
use std::collections::{BTreeMap, BTreeSet};

pub mod mem;

/// The data structure stored in this triple store.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Trip<T: std::fmt::Debug + Eq + Ord>(pub T, pub T, pub T);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
/// Contains either a Variable or Value, used for Queries.
pub enum Slot<T: std::fmt::Debug + Eq + Ord> {
    /// A Variable.
    Variable(String),
    /// A Value.
    Value(T),
    /// Match any value.
    Any,
}

/// The data structure used to represent queries.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct Query<T: std::fmt::Debug + Eq + Ord>(pub Slot<T>, pub Slot<T>, pub Slot<T>);

/// A trait that defines all the actions a Ligature instance can perform.
/// The API used for storing triples.
pub trait Trips<C: Eq + Hash, T: std::fmt::Debug + Eq + Ord, E> {
    /// Get all Collections.
    fn collections(&self) -> Result<Vec<C>, E>;

    /// Add a new Dataset.
    /// Does nothing if Dataset already exists.
    fn add_collection(&mut self, collection: C) -> Result<(), E>;

    /// Remove a Dataset.
    /// Does nothing if Dataset doesn't exist.
    fn remove_collection(&mut self, collection: C) -> Result<(), E>;

    /// Get all Statements in a given Dataset.
    fn triples(&self, collection: C) -> Result<BTreeSet<Trip<T>>, E>;

    /// Add Statements to a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement already exists in Dataset.
    fn add_triples(&mut self, collection: C, trips: &mut BTreeSet<Trip<T>>) -> Result<(), E>;
    /// Remove Statements from a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement doesn't exist in Dataset.
    fn remove_triples(&mut self, collection: C, trips: &mut BTreeSet<Trip<T>>) -> Result<(), E>;
    /// Run a query against the given Dataset.
    fn query(
        &self,
        collection: C,
        pattern: BTreeSet<Query<T>>,
    ) -> Result<HashBag<BTreeMap<String, T>>, E>;
}
