// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use ligature::LigatureError;

use crate::{parser::Element, NativeFunction, WanderValue};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

pub struct Bindings {
    native_functions: RefCell<HashMap<String, Rc<dyn NativeFunction>>>,
    scopes: Vec<HashMap<String, WanderValue>>,
}

impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            native_functions: RefCell::new(HashMap::new()),
            scopes: vec![HashMap::new()],
        }
    }

    pub fn add_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn remove_scope(&mut self) {
        self.scopes.pop();
    }

    pub fn read(&self, name: &String) -> Option<WanderValue> {
        let mut index = self.scopes.len();
        while index > 0 {
            match self.scopes.get(index - 1) {
                Some(scope) => {
                    if let Some(value) = scope.get(name) {
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

    pub fn bind_native_function(&mut self, name: String, function: Rc<dyn NativeFunction>) {
        self.native_functions.borrow_mut().insert(name, function);
    }

    pub fn read_native_function(&self, name: &String) -> Option<Rc<dyn NativeFunction>> {
        match self.native_functions.borrow().get(name) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }

    pub fn run_native_function(
        &mut self,
        name: &String,
        arguments: Vec<Element>,
    ) -> Result<WanderValue, LigatureError> {
        match self.read_native_function(&name) {
            None => Err(LigatureError(format!("Function {} is not defined.", name))),
            Some(nf) => nf.run(arguments, self),
        }
    }

    pub fn bound_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for scope in self.scopes.iter() {
            for (name, _) in scope {
                names.insert(name.clone());
            }
        }
        for native_function in self.native_functions.borrow().keys() {
            names.insert(native_function.clone());
        }
        names
    }
}
