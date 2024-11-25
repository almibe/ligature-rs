// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

//! This project exposes functionality from the Rust implementation of Wander to WASM and JS runtimes thanks to wasm-bindgen and wasm-pack.

mod utils;
use wander::{WanderError, WanderValue};
use wasm_bindgen::prelude::*;
use serde::Serialize;
use ligature_graph::LigatureGraph;

#[wasm_bindgen]
pub fn run(script: String) -> JsValue {
    let mut bindings = wander::prelude::common();
    let res = wander::run(&script,  bindings, &mut LigatureGraph::new());
    let res = RunResult {
        object: res.clone(),
        string: res.map(|res| format!("{}", res)),
    };
    serde_wasm_bindgen::to_value(&res).unwrap()
}

#[derive(Serialize)]
pub struct RunResult {
    object: Result<WanderValue, WanderError>,
    string: Result<String, WanderError>,
}
