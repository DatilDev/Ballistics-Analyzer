#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod auth;
pub mod ballistics;
pub mod storage;
pub mod ui;
pub mod hardware;
pub mod load_data;
pub mod firearm_profiles;
pub mod sharing;

#[cfg(target_arch = "wasm32")]
pub mod pwa;

// Re-export main types from main.rs
// Note: These types are defined in main.rs, not in separate modules
// If you want to use them from lib, you should move them to a separate module

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}