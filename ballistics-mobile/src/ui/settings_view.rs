use crate::app::MobileApp;
use egui::{Ui, Grid, Separator, Hyperlink};

pub fn render(app: &mut MobileApp, ui: &mut Ui) {
    ui.heading("⚙ Settings");
    ui.separator();
    
    // Display settings
    ui.collapsing("Display", |ui| {
        Grid::new("display_settings")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label("Theme:");
                egui::ComboBox::from_label("")
                    .selected_text("System")
                    .show_ui(ui, |ui| {
                        ui.selectable_label(false, "System");
                        ui.selectable_label(false, "Light");
                        ui.selectable_label(false, "Dark");
                    });
                ui.end_row();
                
                ui.label("Units:");
                egui::ComboBox::from_label("")
                    .selected_text("Imperial")
                    .show_ui(ui, |ui| {
                        ui.selectable_label(true, "Imperial");
                        ui.selectable_label(false, "Metric");
                    });
                ui.end_row();
                
                ui.label("Keep Screen On:");
                let mut keep_on = false;
                if ui.checkbox(&mut keep_on, "").changed() {
                    #[cfg(target_os = "android")]
                    crate::platform::android::keep_screen_on(keep_on);
                    
                    #[cfg(target_os = "ios")]
                    crate::platform::ios::keep_screen_on(keep_on);
                }
                ui.end_row();
            });
    });
    
    ui.add(Separator::default().spacing(10.0));
    
    // Calculation settings
    ui.collapsing("Calculation", |ui| {
        Grid::new("calc_settings")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label("Drag Model:");
                egui::ComboBox::from_label("")
                    .selected_text("G1")
                    .show_ui(ui, |ui| {
                        ui.selectable_label(true, "G1");
                        ui.selectable_label(false, "G7");
                        ui.selectable_label(false, "Custom");
                    });
                ui.end_row();
                
                ui.label("Atmosphere Model:");
                egui::ComboBox::from_label("")
                    .selected_text("ICAO")
                    .show_ui(ui, |ui| {
                        ui.selectable_label(true, "ICAO");
                        ui.selectable_label(false, "Army Standard");
                        ui.selectable_label(false, "Custom");
                    });
                ui.end_row();
                
                ui.label("Include Coriolis:");
                let mut coriolis = true;
                ui.checkbox(&mut coriolis, "");
                ui.end_row();
                
                ui.label("Include Spin Drift:");
                let mut spin = true;
                ui.checkbox(&mut spin, "");
                ui.end_row();
            });
    });
    
    ui.add(Separator::default().spacing(10.0));
    
    // Data management
    ui.collapsing("Data Management", |ui| {
        ui.horizontal(|ui| {
            if ui.button("📤 Export All Data").clicked() {
                export_all_data(app);
            }
            
            if ui.button("📥 Import Data").clicked() {
                import_data(app);
            }
        });
        
        ui.separator();
        
        ui.horizontal(|ui| {
            if ui.button("🗑 Clear All Calculations").clicked() {
                clear_all_calculations(app);
            }
            
            if ui.button("🔄 Reset to Defaults").clicked() {
                reset_to_defaults(app);
            }
        });
        
        ui.separator();
        
        // Storage info
        let storage_info = get_storage_info(app);
        ui.label(format!("Storage Used: {}", storage_info));
    });
    
    ui.add(Separator::default().spacing(10.0));
    
    // About section
    ui.collapsing("About", |ui| {
        ui.label("Ballistics Analyzer");
        ui.label("Version: 1.0.0");
        ui.separator();
        
        ui.label("Open Source Ballistics Calculator");
        ui.add(Hyperlink::from_label_and_url(
            "GitHub Repository",
            "https://github.com/DatilDev/Ballistics-Analyzer"
        ));
        
        ui.separator();
        
        ui.label("Privacy Policy:");
        ui.label("• All data stored locally");
        ui.label("• No data collection");
        ui.label("• No analytics or tracking");
        
        ui.separator();
        
        ui.label("Licenses:");
        ui.small("MIT License");
        ui.small("Uses egui, serde, and other open source libraries");
    });
}

fn export_all_data(app: &mut MobileApp) {
    let mut export_data = serde_json::json!({
        "version": "1.0.0",
        "calculations": app.saved_calculations,
        "profiles": app.profile_manager.profiles,
        "timestamp": chrono::Utc::now().to_rfc3339(),
    });
    
    let json = serde_json::to_string_pretty(&export_data).unwrap();
    
    #[cfg(target_os = "android")]
    crate::platform::android::share_text(&json, "Ballistics Analyzer Export");
    
    #[cfg(target_os = "ios")]
    crate::platform::ios::share_text(&json, "Ballistics Analyzer Export");
}

fn import_data(_app: &mut MobileApp) {
    // TODO: Implement file picker and import
}

fn clear_all_calculations(app: &mut MobileApp) {
    for calc in &app.saved_calculations {
        let _ = app.storage.delete_calculation(&calc.id);
    }
    app.saved_calculations.clear();
}

fn reset_to_defaults(app: &mut MobileApp) {
    app.calculation_data = ballistics_core::CalculationData::default();
    app.profile_manager = ballistics_core::FirearmProfileManager::new();
}

fn get_storage_info(app: &MobileApp) -> String {
    let calc_count = app.saved_calculations.len();
    let profile_count = app.profile_manager.profiles.len();
    
    // Estimate storage size
    let estimated_size = calc_count * 2048 + profile_count * 1024; // Rough estimate
    
    format!(
        "{} calculations, {} profiles (~{})",
        calc_count,
        profile_count,
        crate::platform::format_file_size(estimated_size)
    )
}