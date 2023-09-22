// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{HostFunction, TokenTransformer, WanderType, WanderValue};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

#[derive(Default)]
pub struct Bindings {
    token_transformers: RefCell<HashMap<String, Rc<TokenTransformer>>>,
    host_functions: RefCell<HashMap<String, Rc<dyn HostFunction>>>,
    scopes: Vec<HashMap<String, WanderValue>>,
}

pub struct EnvironmentBinding {
    pub name: String,
    pub parameters: Vec<WanderType>,
    pub result: WanderType,
    pub doc_string: String,
}

pub trait BindingsProvider {
    fn add_bindings(&self, bindings: &mut Bindings);
}

impl Bindings {
    pub fn new() -> Bindings {
        Bindings {
            token_transformers: RefCell::new(HashMap::new()),
            host_functions: RefCell::new(HashMap::new()),
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
            index -= 1;
        }
        None
    }

    pub fn bind(&mut self, name: String, value: WanderValue) {
        let mut current_scope = self.scopes.pop().unwrap();
        current_scope.insert(name, value);
        self.scopes.push(current_scope);
    }

    pub fn bind_host_function(&mut self, function: Rc<dyn HostFunction>) {
        let full_name = function.name().to_string();
        self.host_functions.borrow_mut().insert(full_name, function);
    }

    pub fn read_host_function(&self, name: &String) -> Option<Rc<dyn HostFunction>> {
        match self.host_functions.borrow().get(name) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }

    pub fn bind_token_transformer(
        &mut self,
        module: String,
        name: String,
        transformer: Rc<TokenTransformer>,
    ) {
        let full_name = format!("{module}.{name}");
        self.token_transformers
            .borrow_mut()
            .insert(full_name, transformer);
    }

    pub fn read_token_transformer(&self, name: &String) -> Option<Rc<TokenTransformer>> {
        self.token_transformers.borrow().get(name).cloned()
    }

    pub fn bound_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();
        for native_function in self.host_functions.borrow().keys() {
            names.insert(native_function.clone());
        }
        for scope in self.scopes.iter() {
            for name in scope.keys() {
                names.insert(name.clone());
            }
        }
        names
    }

    pub fn environment(&self) -> Vec<EnvironmentBinding> {
        let mut env_bindings = Vec::new();
        for native_function in self.host_functions.borrow().iter() {
            let binding = EnvironmentBinding {
                name: native_function.0.to_owned(),
                doc_string: native_function.1.doc(),
                parameters: native_function.1.params(),
                result: native_function.1.returns(),
            };
            env_bindings.push(binding);
        }
        env_bindings
    }
}
