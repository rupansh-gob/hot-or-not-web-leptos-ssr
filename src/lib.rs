use cfg_if::cfg_if;
pub mod app;
pub mod canister;
pub mod component;
pub mod consts;
pub mod error_template;
pub mod fileserv;
pub mod js;
pub mod page;
pub mod state;
pub mod utils;

cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(App);
    }
}}
