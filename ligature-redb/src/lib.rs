// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is an implementation of Ligature that uses
//! [redb](https://redb.org) for storing data.

use redb::{Database, Error, TableDefinition};

const IDS_TABLE: TableDefinition<&str, u64> = TableDefinition::new("IDS");
const DATASETS_TABLE: TableDefinition<&str, u64> = TableDefinition::new("DATASETS");

pub struct LigatureRedb {
    db: Database
}

pub struct Config {
    pub location: String,
}

pub fn create(config: Config) -> Result<LigatureRedb, Error> {
    let db = Database::create(config.location)?;
    let instance = LigatureRedb { db };
    Ok(instance)
}
