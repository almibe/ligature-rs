// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses only
//! in-memory persistent data structures for storing data.

use std::{
    cell::RefCell,
    collections::{BTreeMap, BTreeSet},
    rc::Rc,
    sync::RwLock,
};

use ligature::{Dataset, Ligature, LigatureError, Query, Statement};

#[derive(Default)]
pub struct LigatureInMemory {
    datasets: Rc<RwLock<BTreeMap<String, RefCell<BTreeSet<Statement>>>>>,
}

impl Ligature for LigatureInMemory {
    fn datasets(&self) -> Result<Vec<Dataset>, LigatureError> {
        let res = self.datasets.read().unwrap().keys().map(|e| {Dataset::new(e).unwrap()}).collect();
        Ok(res)
    }

    fn add_dataset(&mut self, dataset: &Dataset) -> Result<(), LigatureError> {
        let mut instance = self.datasets.write().unwrap();
        instance.insert(dataset.name().to_owned(), RefCell::new(BTreeSet::new()));
        Ok(())
    }

    fn remove_dataset(&mut self, dataset: &Dataset) -> Result<(), LigatureError> {
        let mut instance = self.datasets.write().unwrap();
        instance.remove(dataset.name());
        Ok(())
    }

    fn statements(&self, dataset: &Dataset) -> Result<Vec<Statement>, LigatureError> {
        let instance = self.datasets.read().unwrap();
        match instance.get(dataset.name()) {
            Some(statements) => {
                let mut results = vec![];
                let statements = statements.borrow();
                for statement in statements.iter() {
                    results.push(statement.clone());
                }
                Ok(results)
            }
            _ => Err(LigatureError("Could not find Dataset.".to_owned())), // do nothing
        }
    }

    fn add_statements(
        &self,
        _dataset: &Dataset,
        _statements: Vec<Statement>,
    ) -> Result<(), LigatureError> {
        todo!()
    }

    fn remove_statements(
        &self,
        _dataset: &Dataset,
        _statements: Vec<Statement>,
    ) -> Result<(), LigatureError> {
        todo!()
    }

    fn query(&self) -> Result<Box<dyn Query>, LigatureError> {
        todo!()
    }
}

impl LigatureInMemory {
    pub fn new() -> Self {
        Self {
            datasets: Rc::new(RwLock::new(BTreeMap::new())),
        }
    }
}
