use wasm_bindgen::prelude::*;
use wasm_logger;
use yew::prelude::*;

mod api;
mod app;
mod components;
mod pages;
mod route;
mod types;

#[macro_use]
extern crate log;

#[wasm_bindgen(start)]
pub fn run_app() {
    wasm_logger::init(wasm_logger::Config::default());

    // panics produce better errors
    console_error_panic_hook::set_once();
    App::<app::App>::new().mount_to_body();
}
