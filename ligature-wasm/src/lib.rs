// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This project exposes functionality from the Rust implementation of Ligature to WASM and JS runtimes thanks to wasm-bindgen and wasm-pack.

mod utils;

use std::{sync::RwLock, rc::Rc};

use ligature_wander::bind_instance;
use ligature_in_memory::LigatureInMemory;
use wander::{WanderError, WanderValue, NoHostType};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(script: String) -> JsValue {
    let mut environment = wander::preludes::common::<NoHostType>();
    let instance = LigatureInMemory::new();
    bind_instance(Rc::new(RwLock::new(instance)), &mut environment);
    match wander::run(&script, &mut environment) {
        Ok(value) => serde_wasm_bindgen::to_value(&value).unwrap(),
        Err(err) => serde_wasm_bindgen::to_value(&Err::<WanderValue<NoHostType>, WanderError>(err)).unwrap(),
    }
}
