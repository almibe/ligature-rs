// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::Ligature;

use crate::core_commands::{AssertEqCommand, EqCommand, IgnoreCommand, LetCommand};
use crate::{Command, WanderError, WanderValue};
use std::collections::{BTreeSet, HashMap};

/// Creates a set of Bindings for Wander that consists of all of the common
/// functionality, but doesn't interact with an instance of Ligature.
pub fn common<E>() -> HashMap<String, Box<dyn Command<E>>> {
    let mut commands: HashMap<String, Box<dyn Command<E>>> = HashMap::new();
    commands.insert("eq".to_owned(), Box::new(EqCommand {}));
    commands.insert("assert-eq".to_owned(), Box::new(AssertEqCommand {}));
    commands.insert("ignore".to_owned(), Box::new(IgnoreCommand {}));
    commands.insert("let".to_owned(), Box::new(LetCommand {}));

    // commands.bind_host_function(Rc::new(AndFunction {}));
    // commands.bind_host_function(Rc::new(NotFunction {}));
    // commands.bind_host_function(Rc::new(EnvironmentFunction {}));
    commands
}
