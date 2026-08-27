#![recursion_limit = "256"]

pub mod app;
pub mod download;
pub mod features;
pub mod guides;
pub mod home;
pub mod icons;
pub mod pages;
pub mod privacy;
pub mod releases;
pub mod shell;
pub mod support;

#[cfg(feature = "ssr")]
pub mod sponsors;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
