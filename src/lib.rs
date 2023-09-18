#![cfg_attr(not(test), no_std)]

pub mod model;
pub mod state;
pub mod web;

extern crate alloc;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet() {
    alert("Hello, burn-with-wasm!!!");
}
