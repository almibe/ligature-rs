// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{
    parser::ParserElement,
    WanderValue, Location, Command
};
use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    rc::Rc,
};

/// A structure used to setup the environment a Wander program is executed in.
pub struct Environment {
    host_functions: RefCell<HashMap<String, Rc<dyn Command>>>,
}

///
// pub trait BindingsProvider<T: Clone> {
//     fn add_bindings(&self, bindings: &mut Bindings);
// }

impl Environment {
    /// Create a new empty Bindings.
    pub fn new() -> Environment {
        Environment {
            host_functions: RefCell::new(HashMap::new()),
        }
    }

    /// Add a new HostFunction.
    pub fn bind_host_function(&mut self, function: Rc<dyn Command>) {
        todo!()
        // let full_name = function.binding().name.to_string();
        // self.host_functions
        //     .borrow_mut()
        //     .insert(full_name.clone(), function.clone());
        // let mut parameters = function.binding().parameters.clone();
        // let mut result = None;
        // parameters.reverse();
        // parameters.iter().for_each(|(name, tag)| match &result {
        //     Some(value) => match value {
        //         WanderValue::InnerCall(innerp, i, o, b) => {
        //             let p = parameters.clone();
        //             result = Some(WanderValue::InnerCall(
        //                 name.clone(),
        //                 tag.clone(),
        //                 None,
        //                 Box::new(Location(Element::Lambda(
        //                     innerp.clone(),
        //                     i.clone(),
        //                     o.clone(),
        //                     b.clone(),
        //                 ), 0),)
        //             ));
        //         }
        //         _ => panic!("Should never reach."),
        //     },
        //     None => {
        //         let p = parameters.clone();
        //         result = Some(WanderValue::InnerCall(
        //             name.clone(),
        //             tag.clone(),
        //             None,
        //             Box::new(Location(Element::HostFunction(full_name.clone()),0),)
        //         ));
        //     }
        // });
        // self.bind(full_name, result.unwrap());
    }

    /// Read a HostFunction.
    pub fn read_host_function(&self, name: &String) -> Option<Rc<dyn Command>> {
        match self.host_functions.borrow().get(name) {
            None => None,
            Some(value) => Some(value.clone()),
        }
    }
}
