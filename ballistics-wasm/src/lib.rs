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
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();
    
    let canvas_id = "ballistics_canvas";
    
    // Use wasm_bindgen_futures for async
    wasm_bindgen_futures::spawn_local(async {
        let start_result = eframe::WebRunner::new()
            .start(
                canvas_id,
                eframe::WebOptions::default(),
                Box::new(|cc| Box::new(app::BallisticsWasmApp::new(cc))),
            )
            .await;
            
        if let Err(e) = start_result {
            web_sys::console::error_1(&format!("Failed to start: {:?}", e).into());
        }
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