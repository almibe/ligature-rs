// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of the an in-memory, non-transactional knowledge graph.

use ligature::{LigatureError, Statement};

pub trait Graph {
    fn add_statements(statements: Vec<Statement>) -> Result<(), LigatureError>;
    fn remove_statements(statements: Vec<Statement>) -> Result<(), LigatureError>;
    fn all_statements() -> Result<Vec<Statement>, LigatureError>;
}

/// An implementation of the Graph trait that stores all Data in a sorted set.
pub struct SetGraph {}
