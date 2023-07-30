//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;
use ligature_wasm::run;
use wander::preludes::common;
use wasm_bindgen::prelude::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn handle_basic_literals() {
    // let string = run("\"hello\"".to_owned());
    // assert_eq!(string, JsValue);
    // let integer = result_to_jsvalue(run("12345", &mut common()));
    // assert_eq!(integer, JsValue::bigint_from_str("12345"));
    // let identifier = result_to_jsvalue(run("<hello>", &mut common()));
    // assert_eq!(identifier, JsValue::from_str("hello"));

    // //boolean
    // let string = result_to_jsvalue(run("\"hello\"", &mut common()));
    // assert_eq!(string, JsValue::from_str("hello"));

    // //nothing
    // let string = result_to_jsvalue(run("\"hello\"", &mut common()));
    // assert_eq!(string, JsValue::from_str("hello"));
}

//lists
