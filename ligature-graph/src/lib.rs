// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the an in-memory, non-transactional knowledge graph.

use ligature::{Identifier, Statement, Value};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialOrd, Ord, PartialEq, Eq, Deserialize, Serialize, Hash)]
/// An implementation of the Graph trait that stores all Data in a sorted set.
pub struct Graph {
    statements: BTreeSet<Statement>,
}

impl Default for Graph {
    fn default() -> Self {
        Self {
            statements: Default::default(),
        }
    }
}

impl Graph {
    pub fn new(statements: BTreeSet<Statement>) -> Self {
        Self { statements }
    }

    pub fn add_all(&self, graph: Self) -> Self {
        let mut res = self.statements.clone();
        for statement in graph.statements {
            res.insert(statement.clone());
        }
        Graph { statements: res }
    }

    pub fn remove_all(&self, graph: Self) -> Self {
        todo!()
    }

    pub fn all_statements(&self) -> BTreeSet<Statement> {
        self.statements.clone()
    }

    pub fn find(
        &self,
        entity: Option<Identifier>,
        attribute: Option<Identifier>,
        value: Option<Value>,
    ) -> BTreeSet<Statement> {
        if entity.is_none() && attribute.is_none() && value.is_none() {
            return self.all_statements();
        }
        let mut results: Option<BTreeSet<Statement>> = None;
        if let Some(entity) = entity {
            let t: BTreeSet<_> = self
                .statements
                .clone()
                .into_iter()
                .filter(|statement| statement.entity == entity)
                .collect();
            if t.is_empty() {
                return BTreeSet::new();
            }
            results = Some(t);
        }
        if let Some(attribute) = attribute {
            let t: BTreeSet<_> = self
                .statements
                .clone()
                .into_iter()
                .filter(|s| s.attribute == attribute)
                .collect();
            if t.is_empty() {
                return BTreeSet::new();
            }
            match results {
                Some(statements) => {
                    let r: BTreeSet<Statement> = statements
                        .clone()
                        .intersection(&t)
                        .map(|s| s.to_owned())
                        .collect();
                    results = Some(r);
                }
                None => results = Some(t),
            }
        }
        if let Some(value) = value {
            let t: BTreeSet<Statement> = self
                .statements
                .clone()
                .into_iter()
                .filter(|statement| statement.value == value)
                .collect();
            if t.is_empty() {
                return BTreeSet::new();
            }
            match results {
                Some(statements) => {
                    results = Some(
                        statements
                            .clone()
                            .intersection(&t)
                            .map(|s| s.to_owned())
                            .collect(),
                    );
                }
                None => results = Some(t),
            }
        }
        match results {
            Some(statements) => statements,
            None => self.all_statements(),
        }
    }
}
