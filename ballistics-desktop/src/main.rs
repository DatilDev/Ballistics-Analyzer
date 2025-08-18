#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod hardware;
mod ui;

use ballistics_core::{
    BallisticsCalculator, CalculationData, FirearmProfileManager,
    LoadDataLibrary, SavedCalculation, StorageBackend,
};
use eframe::egui;
use std::sync::Arc;

#[derive(Default)]
pub struct BallisticsDesktopApp {
    // Core data
    calculation_data: CalculationData,
    saved_calculations: Vec<SavedCalculation>,
    profile_manager: FirearmProfileManager,
    load_library: LoadDataLibrary,
    
    // Storage
    storage: Option<Box<dyn StorageBackend>>,
    
    // Hardware
    #[cfg(feature = "hardware")]
    hardware_manager: hardware::HardwareManager,
    
    // UI State
    current_view: ViewType,
    selected_calculation_id: Option<String>,
    show_hardware_panel: bool,
    
    // Nostr
    #[cfg(feature = "nostr")]
    nostr_auth: ballistics_core::NostrAuth,
    
    // Temporary UI state
    save_dialog_open: bool,
    save_name: String,
    save_notes: String,
}

#[derive(Debug, Clone, PartialEq)]
enum ViewType {
    Main,
    Saved,
    Profiles,
    LoadData,
    Settings,
}

impl Default for ViewType {
    fn default() -> Self {
        ViewType::Main
    }
}

impl BallisticsDesktopApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Configure fonts and visuals
        configure_fonts(&cc.egui_ctx);
        cc.egui_ctx.set_visuals(egui::Visuals::dark());
        
        // Initialize storage
        let storage_path = dirs::data_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("ballistics-analyzer")
            .join("data.db");
        
        std::fs::create_dir_all(storage_path.parent().unwrap()).ok();
        
        let storage: Box<dyn StorageBackend> = Box::new(
            ballistics_core::storage::SqliteStorage::new(
                storage_path.to_str().unwrap()
            ).expect("Failed to initialize storage")
        );
        
        let mut app = Self {
            storage: Some(storage),
            ..Default::default()
        };
        
        // Load saved data
        app.load_saved_data();
        
        app
    }
    
    fn load_saved_data(&mut self) {
        if let Some(storage) = &self.storage {
            if let Ok(calculations) = storage.list_calculations() {
                self.saved_calculations = calculations;
            }
            
            if let Ok(profiles) = storage.list_profiles() {
                for profile in profiles {
                    self.profile_manager.add_profile(profile);
                }
            }
        }
    }
    
    fn calculate_trajectory(&mut self) {
        BallisticsCalculator::calculate_trajectory(&mut self.calculation_data);
    }
    
    fn save_calculation(&mut self) {
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
        
        if let Some(storage) = &mut self.storage {
            if storage.save_calculation(&calculation).is_ok() {
                self.saved_calculations.push(calculation);
                self.save_dialog_open = false;
                self.save_name.clear();
                self.save_notes.clear();
            }
        }
    }
}

impl eframe::App for BallisticsDesktopApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("New Calculation").clicked() {
                        self.calculation_data = CalculationData::default();
                        ui.close_menu();
                    }
                    
                    if ui.button("Open...").clicked() {
                        // File dialog
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Export...").clicked() {
                        export_data(self);
                        ui.close_menu();
                    }
                    
                    if ui.button("Import...").clicked() {
                        import_data(self);
                        ui.close_menu();
                    }
                    
                    ui.separator();
                    
                    if ui.button("Exit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                
                ui.menu_button("View", |ui| {
                    if ui.button("Calculator").clicked() {
                        self.current_view = ViewType::Main;
                        ui.close_menu();
                    }
                    
                    if ui.button("Saved Calculations").clicked() {
                        self.current_view = ViewType::Saved;
                        ui.close_menu();
                    }
                    
                    if ui.button("Firearm Profiles").clicked() {
                        self.current_view = ViewType::Profiles;
                        ui.close_menu();
                    }
                    
                    if ui.button("Load Data").clicked() {
                        self.current_view = ViewType::LoadData;
                        ui.close_menu();
                    }
                });
                
                #[cfg(feature = "hardware")]
                ui.menu_button("Hardware", |ui| {
                    if ui.button("Device Manager").clicked() {
                        self.show_hardware_panel = !self.show_hardware_panel;
                        ui.close_menu();
                    }
                });
                
                ui.menu_button("Help", |ui| {
                    if ui.button("About").clicked() {
                        ui.close_menu();
                    }
                });
            });
        });
        
        // Hardware panel
        #[cfg(feature = "hardware")]
        if self.show_hardware_panel {
            egui::SidePanel::right("hardware_panel")
                .default_width(300.0)
                .show(ctx, |ui| {
                    ui::render_hardware_panel(self, ui);
                });
        }
        
        // Main content
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                ViewType::Main => ui::render_main_view(self, ui),
                ViewType::Saved => ui::render_saved_view(self, ui),
                ViewType::Profiles => ui::render_profiles_view(self, ui),
                ViewType::LoadData => ui::render_load_data_view(self, ui),
                ViewType::Settings => ui::render_settings_view(self, ui),
            }
        });
        
        // Dialogs
        if self.save_dialog_open {
            egui::Window::new("Save Calculation")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.label("Name:");
                    ui.text_edit_singleline(&mut self.save_name);
                    
                    ui.label("Notes:");
                    ui.text_edit_multiline(&mut self.save_notes);
                    
                    ui.horizontal(|ui| {
                        if ui.button("Save").clicked() {
                            self.save_calculation();
                        }
                        
                        if ui.button("Cancel").clicked() {
                            self.save_dialog_open = false;
                        }
                    });
                });
        }
    }
}

fn main() -> eframe::Result {
    env_logger::init();
    
    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Ballistics Analyzer")
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0])
            .with_icon(load_icon()),
        ..Default::default()
    };
    
    eframe::run_native(
        "Ballistics Analyzer",
        native_options,
        Box::new(|cc| Ok(Box::new(BallisticsDesktopApp::new(cc)))),
    )
}

fn configure_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    
    fonts.font_data.insert(
        "custom".to_owned(),
        egui::FontData::from_static(include_bytes!(
            "../assets/Roboto-Regular.ttf"
        ))
    );
    
    fonts.families.insert(
        egui::FontFamily::Proportional,
        vec!["custom".to_owned()],
    );
    
    ctx.set_fonts(fonts);
}

fn load_icon() -> Arc<egui::IconData> {
    let icon_bytes = include_bytes!("../assets/icon.png");
    let icon = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = icon.dimensions();
    
    Arc::new(egui::IconData {
        rgba: icon.into_raw(),
        width,
        height,
    })
}

fn export_data(app: &BallisticsDesktopApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .save_file()
    {
        let export_data = serde_json::json!({
            "version": env!("CARGO_PKG_VERSION"),
            "calculation": app.calculation_data,
            "profiles": app.profile_manager.profiles,
        });
        
        if let Ok(json) = serde_json::to_string_pretty(&export_data) {
            std::fs::write(path, json).ok();
        }
    }
}

fn import_data(app: &mut BallisticsDesktopApp) {
    if let Some(path) = rfd::FileDialog::new()
        .add_filter("JSON", &["json"])
        .pick_file()
    {
        if let Ok(json) = std::fs::read_to_string(path) {
            if let Ok(data) = serde_json::from_str::<serde_json::Value>(&json) {
                // Import calculation data
                if let Some(calc) = data.get("calculation") {
                    if let Ok(calculation_data) = serde_json::from_value(calc.clone()) {
                        app.calculation_data = calculation_data;
                    }
                }
                
                // Import profiles
                if let Some(profiles) = data.get("profiles") {
                    if let Ok(profile_list) = serde_json::from_value::<Vec<ballistics_core::FirearmProfile>>(profiles.clone()) {
                        for profile in profile_list {
                            app.profile_manager.add_profile(profile);
                        }
                    }
                }
            }
        }
    }
}