#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod auth;
pub mod ballistics;
pub mod storage;
pub mod ui;
pub mod hardware;
pub mod load_data;
pub mod firearm_profiles;
pub mod sharing;
pub mod models;  // Add the models module

#[cfg(target_arch = "wasm32")]
pub mod pwa;

// Re-export commonly used types from models
pub use models::{AttachedImage, SavedCalculation, CalculationData};

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}