#![recursion_limit="1024"]

extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub mod components;
pub mod model;

mod app;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<app::App>::new().mount_to_body();
}

