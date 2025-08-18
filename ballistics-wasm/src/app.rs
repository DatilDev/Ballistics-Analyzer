use ballistics_core::{
    BallisticsCalculator, CalculationData, FirearmProfileManager,
    LoadDataLibrary, SavedCalculation, FirearmProfile,
};
use eframe::{App, CreationContext, Frame, Storage};
use egui::Context;
use crate::storage::WebStorage;
use crate::ui;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum View {
    Main,
    Calculation,
    Saved,
    Profiles,
    LoadData,
    Settings,
    About,
}

#[derive(Serialize, Deserialize)]
#[serde(default)]
pub struct BallisticsWasmApp {
    // Core data
    pub calculation_data: CalculationData,
    pub saved_calculations: Vec<SavedCalculation>,
    pub profile_manager: FirearmProfileManager,
    #[serde(skip, default = "LoadDataLibrary::new")]
    pub load_library: LoadDataLibrary,
    
    // UI state
    pub current_view: View,
    pub selected_calculation_id: Option<String>,
    pub show_side_panel: bool,
    pub theme: Theme,
    pub units: Units,
    
    // PWA features
    pub is_offline: bool,
    pub pending_sync: VecDeque<SyncAction>,
    pub install_prompt_shown: bool,
    
    // Temporary UI state (not serialized)
    #[serde(skip)]
    pub save_dialog_open: bool,
    #[serde(skip)]
    pub save_name: String,
    #[serde(skip)]
    pub save_notes: String,
    #[serde(skip)]
    pub export_dialog_open: bool,
    #[serde(skip)]
    pub import_text: String,
    #[serde(skip)]
    pub notification: Option<Notification>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Units {
    Imperial,
    Metric,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncAction {
    SaveCalculation(SavedCalculation),
    DeleteCalculation(String),
    SaveProfile(FirearmProfile),
    DeleteProfile(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Notification {
    pub message: String,
    pub kind: NotificationKind,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotificationKind {
    Success,
    Error,
    Info,
}

impl Default for BallisticsWasmApp {
    fn default() -> Self {
        Self {
            calculation_data: CalculationData::default(),
            saved_calculations: Vec::new(),
            profile_manager: FirearmProfileManager::new(),
            load_library: LoadDataLibrary::new(),
            current_view: View::Main,
            selected_calculation_id: None,
            show_side_panel: false,
            theme: Theme::System,
            units: Units::Imperial,
            is_offline: false,
            pending_sync: VecDeque::new(),
            install_prompt_shown: false,
            save_dialog_open: false,
            save_name: String::new(),
            save_notes: String::new(),
            export_dialog_open: false,
            import_text: String::new(),
            notification: None,
        }
    }
}

impl BallisticsWasmApp {
        pub fn new(cc: &CreationContext<'_>) -> Self {
        configure_fonts(&cc.egui_ctx);
        
        // Load from localStorage instead of eframe storage
        let mut app = Self::default();
        app.load_from_storage();
        app
    }
    
    pub fn calculate_trajectory(&mut self) {
        BallisticsCalculator::calculate_trajectory(&mut self.calculation_data);
        self.show_notification("Trajectory calculated successfully", NotificationKind::Success);
    }
    
    pub fn save_calculation(&mut self) {
        let calculation = SavedCalculation {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            name: self.save_name.clone(),
            data: self.calculation_data.clone(),
            photos: Vec::new(),
            notes: self.save_notes.clone(),
            weather: None,
            firearm_profile_id: self.profile_manager.selected_profile_id.clone(),
        };
        
        // Save to IndexedDB
        WebStorage::save_calculation(&calculation);
        
        self.saved_calculations.push(calculation.clone());
        
        // Queue for sync if offline
        if self.is_offline {
            self.pending_sync.push_back(SyncAction::SaveCalculation(calculation));
        }
        
        self.save_dialog_open = false;
        self.save_name.clear();
        self.save_notes.clear();
        
        self.show_notification("Calculation saved", NotificationKind::Success);
    }
    
    pub fn load_calculation(&mut self, id: &str) {
        if let Some(calc) = self.saved_calculations.iter().find(|c| c.id == id) {
            self.calculation_data = calc.data.clone();
            self.selected_calculation_id = Some(id.to_string());
            self.current_view = View::Calculation;
            self.show_notification("Calculation loaded", NotificationKind::Info);
        }
    }
    
    pub fn delete_calculation(&mut self, id: &str) {
        WebStorage::delete_calculation(id);
        self.saved_calculations.retain(|c| c.id != id);
        
        if self.is_offline {
            self.pending_sync.push_back(SyncAction::DeleteCalculation(id.to_string()));
        }
        
        self.show_notification("Calculation deleted", NotificationKind::Info);
    }
    
    pub fn export_data(&self) -> String {
        let export = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "calculations": self.saved_calculations,
            "profiles": self.profile_manager.profiles,
            "current_calculation": self.calculation_data,
        });
        
        serde_json::to_string_pretty(&export).unwrap_or_default()
    }
    
    pub fn import_data(&mut self, json: &str) {
        match serde_json::from_str::<serde_json::Value>(json) {
            Ok(data) => {
                if let Some(calcs) = data["calculations"].as_array() {
                    for calc_value in calcs {
                        if let Ok(calc) = serde_json::from_value::<SavedCalculation>(calc_value.clone()) {
                            WebStorage::save_calculation(&calc);
                            self.saved_calculations.push(calc);
                        }
                    }
                }
                
                if let Some(profiles) = data["profiles"].as_array() {
                    for profile_value in profiles {
                        if let Ok(profile) = serde_json::from_value::<FirearmProfile>(profile_value.clone()) {
                            self.profile_manager.add_profile(profile);
                        }
                    }
                }
                
                self.show_notification("Data imported successfully", NotificationKind::Success);
            }
            Err(e) => {
                self.show_notification(&format!("Import failed: {}", e), NotificationKind::Error);
            }
        }
    }
    
    pub fn share_calculation(&self, calc: &SavedCalculation) {
    if let Some(window) = web_sys::window() {
        // Navigator.share() is not directly available, use clipboard fallback
        self.copy_to_clipboard(&serde_json::to_string_pretty(calc).unwrap_or_default());
    }
}
    
    
    pub fn copy_to_clipboard(&self, text: &str) {
    if let Some(window) = web_sys::window() {
        if let Some(navigator) = window.navigator().clipboard() {
            let promise = navigator.write_text(text);
            let _ = wasm_bindgen_futures::spawn_local(async move {
                let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
            });
        }
    }
}
    
    fn load_from_storage(&mut self) {
        self.saved_calculations = WebStorage::load_all_calculations();
        
        if let Some(profiles) = WebStorage::load_profiles() {
            self.profile_manager.profiles = profiles;
        }
    }
    
    fn check_online_status(&mut self) {
    if let Some(window) = web_sys::window() {
        // navigator.on_line() returns bool, not Option
        self.is_offline = !window.navigator().on_line();
        }
    }
    
    fn register_service_worker(&self) {
        spawn_local(async {
            if let Some(window) = web_sys::window() {
                if let Ok(navigator) = window.navigator().service_worker() {
                    match navigator.register("./sw.js").await {
                        Ok(_) => web_sys::console::log_1(&"Service Worker registered".into()),
                        Err(e) => web_sys::console::error_1(&format!("SW registration failed: {:?}", e).into()),
                    }
                }
            }
        });
    }
    
    fn setup_install_prompt(&mut self) {
        // Listen for beforeinstallprompt event
        if let Some(window) = web_sys::window() {
            let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
                // Show install button in UI
                web_sys::console::log_1(&"Install prompt available".into());
            }) as Box<dyn Fn(_)>);
            
            window.add_event_listener_with_callback(
                "beforeinstallprompt",
                closure.as_ref().unchecked_ref()
            ).unwrap();
            
            closure.forget();
        }
    }
    
    pub fn show_notification(&self, message: &str, kind: NotificationKind) {
        // For now, just log to console
        // In a real app, you'd update the notification field
        match kind {
            NotificationKind::Success => web_sys::console::log_1(&format!("✓ {}", message).into()),
            NotificationKind::Error => web_sys::console::error_1(&message.into()),
            NotificationKind::Info => web_sys::console::info_1(&message.into()),
        }
    }
    
    pub fn sync_pending_actions(&mut self) {
        if !self.is_offline && !self.pending_sync.is_empty() {
            while let Some(action) = self.pending_sync.pop_front() {
                match action {
                    SyncAction::SaveCalculation(calc) => {
                        WebStorage::save_calculation(&calc);
                    }
                    SyncAction::DeleteCalculation(id) => {
                        WebStorage::delete_calculation(&id);
                    }
                    SyncAction::SaveProfile(profile) => {
                        WebStorage::save_profile(&profile);
                    }
                    SyncAction::DeleteProfile(id) => {
                        WebStorage::delete_profile(&id);
                    }
                }
            }
            self.show_notification("Pending changes synced", NotificationKind::Success);
        }
    }

    fn save_to_storage(&self) {
        // Save current state to localStorage
        if let Ok(json) = serde_json::to_string(&self) {
            if let Some(storage) = web_sys::window()
                .and_then(|w| w.local_storage().ok())
                .flatten() {
                let _ = storage.set_item("ballistics_app_state", &json);
            }
        }
    }
}



impl App for BallisticsWasmApp {
    fn save(&mut self, storage: &mut dyn Storage) {
        // WASM handles storage differenlty
        // Data saved via WebStorage in custom implementation
        self.save_to_storage();
    }
    
    fn update(&mut self, ctx: &Context, _frame: &mut Frame){
        // Apply theme
        match self.theme {
            Theme::Light => ctx.set_visuals(egui::Visuals::light()),
            Theme::Dark => ctx.set_visuals(egui::Visuals::dark()),
            Theme::System => {
                // Detect system preference
                if let Some(window) = web_sys::window() {
                    if let Some(media) = window.match_media("(prefers-color-scheme: dark)").ok().flatten() {
                        if media.matches() {
                            ctx.set_visuals(egui::Visuals::dark());
                        } else {
                            ctx.set_visuals(egui::Visuals::light());
                        }
                    }
                }
            }
        }
        
        // Check for online status changes
        self.check_online_status();
        
        // Sync pending actions if back online
        if !self.is_offline {
            self.sync_pending_actions();
        }
        
        // Render UI
        ui::render_app(self, ctx);

        // Save to local Storage periodically
        self.save_to_storage();
    }


}

fn configure_fonts(ctx: &Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    // Add custom fonts if needed
    fonts.font_data.insert(
        "Roboto".to_owned(),
        egui::FontData::from_static(include_bytes!("../assets/Roboto-Regular.ttf")),
    );
    
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["Roboto".to_owned(), "Segoe UI".to_owned()],
    );
    
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["sans-serif".to_owned()],
    );
    ctx.set_fonts(fonts);
}

use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::prelude::*;