use cfg_if::cfg_if;

pub mod app;
pub mod components;
pub mod database;
pub mod error_template;
pub mod fileserv;
pub mod models;
pub mod screens;
pub mod service;
pub mod user;
pub mod utils;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::prelude::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use leptos::mount::mount_to_body;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        mount_to_body(App);
    }
}}
