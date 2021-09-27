// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses only
//! in-memory persistent data structures for storing data.

use rpds::{RedBlackTreeSet};

struct LigatureInMemory {
    datasets: RedBlackTreeSet<String>
}

impl LigatureInMemory {
    pub fn new() -> LigatureInMemory {
        LigatureInMemory {
            datasets: RedBlackTreeSet::new()
        }
    }
}
