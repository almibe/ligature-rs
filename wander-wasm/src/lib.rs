// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This project exposes functionality from the Rust implementation of Wander to WASM and JS runtimes thanks to wasm-bindgen and wasm-pack.

mod utils;
use serde::Serialize;
use wander::{HostType, WanderError, WanderValue};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn run(script: String) -> JsValue {
    let mut bindings = wander::preludes::common::<wander::NoHostType>();
    let res = wander::run(&script, &mut bindings);
    let res = RunResult {
        object: res.clone(),
        string: res.map(|res| format!("{}", res)),
    };
    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[derive(Serialize)]
pub struct RunResult<T: HostType> {
    object: Result<WanderValue<T>, WanderError>,
    string: Result<String, WanderError>,
}

#[wasm_bindgen]
pub fn introspect(script: String) -> JsValue {
    let mut bindings = wander::preludes::common::<wander::NoHostType>();
    serde_wasm_bindgen::to_value(&wander::introspect(&script, &mut bindings)).unwrap()
}
