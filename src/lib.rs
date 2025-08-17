#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod auth;
pub mod ballistics;
pub mod storage;
pub mod ui;
pub mod hardware;
pub mod load_data;
pub mod firearm_profiles;
pub mod sharing;
pub mod models;

// Re-export commonly used types
pub use models::{AttachedImage, SavedCalculation, CalculationData};

// WASM entry point
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn main() {
    // Initialize panic hook for better error messages
    console_error_panic_hook::set_once();
    
    // Initialize logging
    tracing_wasm::set_as_global_default();
    
    // Start the app
    let web_options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "canvas", // canvas id
            web_options,
            Box::new(|cc| {
                Box::new(crate::BallisticsApp::new(cc))
            }),
        )
        .await
        .expect("failed to start eframe");
    });
}

// Main app structure (used by both desktop and web)
use crate::storage::LocalStorage;
use crate::auth::NostrAuth;
use crate::hardware::HardwareManager;
use crate::sharing::SharingManager;
use crate::load_data::LoadDataLibrary;
use crate::firearm_profiles::FirearmProfileManager;

pub struct BallisticsApp {
    pub auth: NostrAuth,
    pub hardware: HardwareManager,
    pub sharing: SharingManager,
    pub storage: LocalStorage,
    pub load_data: LoadDataLibrary,
    pub profiles: FirearmProfileManager,
    // ... other fields
}

impl BallisticsApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Setup app initialization
        let mut app = Self {
            auth: NostrAuth::default(),
            hardware: HardwareManager::default(),
            sharing: SharingManager::default(),
            storage: LocalStorage::default(),
            load_data: LoadDataLibrary::new(),
            profiles: FirearmProfileManager::default(),
            // ... initialize other fields
        };
        
        // Load saved state if available
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(storage) = cc.storage {
                if let Some(auth_data) = storage.get_string("auth") {
                    app.auth.restore_from_string(&auth_data);
                }
            }
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            // Use localStorage for WASM
            if let Some(window) = web_sys::window() {
                if let Ok(Some(storage)) = window.local_storage() {
                    if let Ok(Some(auth_data)) = storage.get_item("auth") {
                        app.auth.restore_from_string(&auth_data);
                    }
                }
            }
        }
        
        app
    }
}

impl eframe::App for BallisticsApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Main UI update
        ui::render(self, ctx);
    }
    
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        // Save state
        storage.set_string("auth", self.auth.serialize());
    }
}