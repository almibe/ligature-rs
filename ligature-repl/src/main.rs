// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This module is the main module for the ligature-repl project.

use ligature_in_memory::LigatureInMemory;
use wander_repl::{start_repl, REPLState};
use rustyline::Result;
use wander::preludes::common;
use wander::NoHostType;
use ligature_wander::bind_instance;
use std::rc::Rc;
use std::sync::RwLock;

fn main() -> Result<()> {
    let mut bindings = common::<NoHostType>();
    let instance = Rc::new(RwLock::new(LigatureInMemory::new()));
    bind_instance(instance, &mut bindings);
    let mut state = REPLState { bindings };
    start_repl(&mut state)
}
