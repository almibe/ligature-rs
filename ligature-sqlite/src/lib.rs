// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the Ligature-SQLite project.
//! It implements the traits supplied by Ligature and persists data via SQLite3.

#![deny(missing_docs)]

use ligature::{Ligature, WriteFn, QueryFn, LigatureError, Dataset};

/// The main struct used for working with the SQLite stored version of Ligature.
pub struct LigatureSQLite {
    //connection:
}

impl LigatureSQLite {
    fn create_or_open_file(path: String) -> LigatureSQLite {
        todo!()
    }

    fn new_memory_store() -> LigatureSQLite {
        todo!()
    }
}

impl Ligature for LigatureSQLite {
    fn all_datasets(&self) -> Box<dyn Iterator<Item=Result<Dataset, LigatureError>>> {
        todo!()
    }

    fn dataset_exists(&self, dataset: &Dataset) -> Result<bool, LigatureError> {
        todo!()
    }

    fn match_datasets_prefix(&self, prefix: &str) -> Box<dyn Iterator<Item=Result<Dataset, LigatureError>>> {
        todo!()
    }

    fn match_datasets_range(&self, start: &str, end: &str) -> Box<dyn Iterator<Item=Result<Dataset, LigatureError>>> {
        todo!()
    }

    fn create_dataset(&self, dataset: &Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn delete_dataset(&self, dataset: &Dataset) -> Result<(), LigatureError> {
        todo!()
    }

    fn query<T>(&self, dataset: &Dataset, f: QueryFn<T>) -> Result<T, LigatureError> {
        todo!()
    }

    fn write<T>(&self, dataset: &Dataset, f: WriteFn<T>) -> Result<T, LigatureError> {
        todo!()
    }
}
