// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the trips project.

#![deny(missing_docs)]

use core::hash::Hash;

pub mod mem;

/// The data structure stored in this triple store.
pub struct Trip<T> {
    /// The first element
    pub first: T,
    /// The second element
    pub second: T,
    /// The third element    
    pub third: T
}

/// A trait that defines all the actions a Ligature instance can perform.
/// The API used for storing triples.
pub trait Trips<C: Eq + Hash, T, E> {
    /// Get all Collections.
    fn collections(&self) -> Result<Vec<C>, E>;

    /// Add a new Dataset.
    /// Does nothing if Dataset already exists.
    fn add_collection(&mut self, collection: C) -> Result<(), E>;

    /// Remove a Dataset.
    /// Does nothing if Dataset doesn't exist.
    fn remove_collection(&mut self, collection: C) -> Result<(), E>;

    /// Get all Statements in a given Dataset.
    fn statements(&self, collection: C) -> Result<Vec<Trip<T>>, E>;

    /// Add Statements to a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement already exists in Dataset.
    fn add_triples(
        &mut self,
        collection: C,
        trips: Vec<Trip<T>>,
    ) -> Result<(), E>;
    /// Remove Statements from a given Dataset.
    /// Returns Error if Dataset doesn't exist.
    /// Does nothing if Statement doesn't exist in Dataset.
    fn remove_triples(
        &mut self,
        collection: C,
        trips: Vec<Trip<T>>
    ) -> Result<(), E>;
    /// Run a query against the given Dataset.
    fn query(&self) -> Result<Box<dyn Query<T, E>>, E>; //TODO this is wrong
}

/// Query Ligature instances.
pub trait Query<T, E> {
    /// Find Statements that match the given pattern.
    /// (None, None, None) returns all Statements.
    fn find(
        &self,
        first: Option<T>,
        second: Option<T>,
        third: Option<T>,
    ) -> Result<Vec<Trip<T>>, E>;
}
