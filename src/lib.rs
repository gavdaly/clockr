pub mod app;
pub mod components;
pub mod database;
pub mod functions;
pub mod models;
pub mod screens;
pub mod service;
pub mod user;
pub mod utils;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_islands();
}
