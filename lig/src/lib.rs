// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the lig serialization format for Ligature.

use ligature::{LigatureError, Ligature, Dataset};

use crate::read::read;

pub mod read;
pub mod write;

/// A error related to parsing Lig.
#[derive(Debug, PartialEq, Eq)]
pub struct LigError(pub String);

impl From<LigatureError> for LigError {
    fn from(err: LigatureError) -> Self {
        LigError(err.0)
    }
}

pub fn load_lig_from_str(dataset: Dataset, input: &str, ligature: &dyn Ligature) -> Result<(), LigatureError> {
    println!("parsing lig {input}");
    let statements = read(input)?;
    println!("after parse");
    ligature.add_statements(&dataset, statements)
}
