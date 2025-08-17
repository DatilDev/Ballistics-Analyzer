#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod app;
mod storage;
mod ui;

use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global allocator
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// Entry point for WASM
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize panic hook for better error messages
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
    
    // Initialize logger
    tracing_wasm::set_as_global_default();
    
    // Log startup
    web_sys::console::log_1(&"Starting Ballistics Analyzer PWA...".into());
    
    // Get canvas element
    let canvas_id = "ballistics_canvas";
    
    // Start the app
    let web_options = eframe::WebOptions::default();
    
    spawn_local(async {
        eframe::start_web(
            canvas_id,
            web_options,
            Box::new(|cc| Box::new(app::BallisticsWasmApp::new(cc))),
        )
        .await
        .expect("Failed to start eframe");
    });
}

// Export version information
#[wasm_bindgen]
pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

// Export calculation function for external JS usage
#[wasm_bindgen]
pub fn calculate_trajectory(data_json: &str) -> String {
    use ballistics_core::{BallisticsCalculator, CalculationData};
    
    match serde_json::from_str::<CalculationData>(data_json) {
        Ok(mut data) => {
            BallisticsCalculator::calculate_trajectory(&mut data);
            serde_json::to_string(&data).unwrap_or_else(|_| "{}".to_string())
        }
        Err(e) => {
            format!("{{\"error\": \"{}\"}}", e)
        }
    }
}