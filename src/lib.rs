pub mod app;
pub mod components;
pub mod database;
pub mod error;
pub mod functions;
pub mod models;
pub mod screens;
pub mod service;
pub mod utils;

pub use error::{Error, Result};

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use app::App;

    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
