// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn insert_lig(lig: &str) -> String {
    String::from("Called Insert Lig")
}

#[wasm_bindgen]
pub fn remove_lig(lig: &str) -> String {
    String::from("Called Remove Lig")
}

#[wasm_bindgen]
pub fn wander_query(script: &str) -> String {
    String::from("Called Wander Query")
}
