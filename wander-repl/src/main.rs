// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.

pub use rustyline::Result;
use wander::preludes::common;
use wander::NoHostType;
use wander_repl::{start_repl, REPLState};
use wander::preludes::add_print;

fn main() -> Result<()> {
    let mut environment = common::<NoHostType>();
    add_print(&mut environment);
    let mut state = REPLState { environment };
    start_repl(&mut state)
}
