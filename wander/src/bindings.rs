// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{NativeFunction, WanderValue};
use std::collections::{HashMap, HashSet};

pub struct Bindings {
    native_functions: HashMap<String, Box<dyn NativeFunction>>,
    scopes: Vec<HashMap<String, WanderValue>>,
}

impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            native_functions: HashMap::new(),
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn read(&self, name: String) -> Option<WanderValue> {
        let mut index = self.scopes.len();
        while index > 0 {
            match self.scopes.get(index - 1) {
                Some(scope) => {
                    if let Some(value) = scope.get(&name) {
                        return Some(value.clone());
                    }
                }
                _ => return None,
            }
            index = index - 1;
        }
        None
    }

    pub fn bind(&mut self, name: String, value: WanderValue) {
        let mut current_scope = self.scopes.pop().unwrap();
        current_scope.insert(name, value);
        self.scopes.push(current_scope);
    }

    pub fn bind_native_function(&mut self, name: String, function: Box<dyn NativeFunction>) {
        self.native_functions.insert(name, function);
    }

    pub fn bound_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for scope in self.scopes.iter() {
            for (name, _) in scope {
                names.insert(name.clone());
            }
        }
        for native_function in self.native_functions.keys() {
            names.insert(native_function.clone());
        }
        names
    }
}
