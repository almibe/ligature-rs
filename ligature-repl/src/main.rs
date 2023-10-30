// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.

use wander_repl::{start_repl, REPLState};
use rustyline::Result;
use wander::preludes::common;
use wander::NoHostType;

fn main() -> Result<()> {
    let bindings = common::<NoHostType>();
    let mut state = REPLState { bindings };
    start_repl(&mut state)
}
