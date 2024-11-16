// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.

pub use rustyline::Result;
use wander_repl::{start_repl, REPLState};
use ligature_graph::LigatureGraph;

fn main() -> Result<()> {
    let mut state = REPLState { state: LigatureGraph::new() };
    start_repl(&mut state)
}
