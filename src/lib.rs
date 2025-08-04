#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod app;
pub mod auth;
pub mod ballistics;
pub mod storage;
pub mod ui;
pub mod hardware;
pub mod load_data;
pub mod firearm_profiles;
pub mod sharing;
pub mod pwa;

pub use app::BallisticsApp;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}