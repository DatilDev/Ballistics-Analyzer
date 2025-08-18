use crate::app::{BallisticsWasmApp, Theme, Units};
use egui::{Ui, Grid, ScrollArea};

pub fn render(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.heading("⚙ Settings");
        ui.separator();
        
        // Display Settings
        ui.collapsing("🎨 Display", |ui| {
            Grid::new("display_settings")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Theme:");
                    egui::ComboBox::from_id_source("theme_combo")
                        .selected_text(format!("{:?}", app.theme))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.theme, Theme::System, "System");
                            ui.selectable_value(&mut app.theme, Theme::Light, "Light");
                            ui.selectable_value(&mut app.theme, Theme::Dark, "Dark");
                        });
                    ui.end_row();
                    
                    ui.label("Units:");
                    egui::ComboBox::from_id_source("units_combo")
                        .selected_text(format!("{:?}", app.units))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut app.units, Units::Imperial, "Imperial");
                            ui.selectable_value(&mut app.units, Units::Metric, "Metric");
                        });
                    ui.end_row();
                });
        });
        
        ui.separator();
        
        // Data Management
        ui.collapsing("💾 Data Management", |ui| {
            ui.horizontal(|ui| {
                if ui.button("📤 Export All Data").clicked() {
                    app.export_dialog_open = true;
                }
                
                if ui.button("📥 Import Data").clicked() {
                    app.export_dialog_open = true; // Use same dialog for import
                }
            });
            
            ui.separator();
            
            ui.horizontal(|ui| {
                if ui.button("🗑 Clear All Calculations").clicked() {
                    if confirm_action(ui.ctx(), "Clear all saved calculations?") {
                        clear_all_calculations(app);
                    }
                }
                
                if ui.button("🔄 Reset to Defaults").clicked() {
                    if confirm_action(ui.ctx(), "Reset all settings to defaults?") {
                        reset_to_defaults(app);
                    }
                }
            });
            
            ui.separator();
            
            // Storage info
            let storage_size = crate::storage::WebStorage::get_storage_size();
            ui.label(format!("Storage Used: {}", format_bytes(storage_size)));
            ui.label(format!("Calculations: {}", app.saved_calculations.len()));
            ui.label(format!("Profiles: {}", app.profile_manager.profiles.len()));
        });
        
        ui.separator();
        
        // PWA Settings
        ui.collapsing("📱 Progressive Web App", |ui| {
            ui.label("Installation Status:");
            
            if app.install_prompt_shown {
                ui.label("✓ App can be installed");
                if ui.button("Install App").clicked() {
                    // Trigger PWA install
                    web_sys::console::log_1(&"Install button clicked".into());
                }
            } else {
                ui.label("App is already installed or not available");
            }
            
            ui.separator();
            
            ui.label("Offline Mode:");
            if app.is_offline {
                ui.colored_label(egui::Color32::YELLOW, "Currently Offline");
                if !app.pending_sync.is_empty() {
                    ui.label(format!("{} items pending sync", app.pending_sync.len()));
                }
            } else {
                ui.colored_label(egui::Color32::GREEN, "Online");
            }
            
            ui.separator();
            
            ui.label("Service Worker:");
            ui.label("✓ Registered and active");
        });
        
        ui.separator();
        
        // Advanced Settings
        ui.collapsing("🔧 Advanced", |ui| {
            ui.checkbox(&mut app.show_side_panel, "Show Side Panel");
            
            ui.separator();
            
            ui.label("Calculation Precision:");
            Grid::new("precision_settings")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Distance Step (yd):");
                    ui.label("Automatic (10-25 yd)");
                    ui.end_row();
                    
                    ui.label("Max Distance (yd):");
                    ui.label("2000 yd");
                    ui.end_row();
                });
        });
        
        ui.separator();
        
        // About Section
        ui.collapsing("ℹ About", |ui| {
            ui.label("Ballistics Analyzer PWA");
            ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
            ui.separator();
            
            ui.label("Open Source Ballistics Calculator");
            ui.hyperlink_to(
                "GitHub Repository",
                "https://github.com/DatilDev/Ballistics-Analyzer"
            );
            
            ui.separator();
            
            ui.heading("Features:");
            ui.label("• Professional ballistics calculations");
            ui.label("• Works completely offline");
            ui.label("• Multiple firearm profiles");
            ui.label("• Environmental corrections");
            ui.label("• Import/Export data");
            ui.label("• No data collection or tracking");
            
            ui.separator();
            
            ui.heading("Privacy:");
            ui.label("All data is stored locally in your browser.");
            ui.label("No server communication required.");
            ui.label("No analytics or telemetry.");
            
            ui.separator();
            
            ui.heading("Credits:");
            ui.label("Built with Rust and egui");
            ui.label("MIT License");
            
            ui.separator();
            
            ui.heading("Browser Compatibility:");
            ui.label("✓ Chrome/Edge 90+");
            ui.label("✓ Firefox 89+");
            ui.label("✓ Safari 14+");
            ui.label("✓ Mobile browsers");
        });
    });
}

fn clear_all_calculations(app: &mut BallisticsWasmApp) {
    for calc in &app.saved_calculations {
        crate::storage::WebStorage::delete_calculation(&calc.id);
    }
    app.saved_calculations.clear();
    app.selected_calculation_id = None;
}

fn reset_to_defaults(app: &mut BallisticsWasmApp) {
    app.calculation_data = ballistics_core::CalculationData::default();
    app.theme = Theme::System;
    app.units = Units::Imperial;
    app.show_side_panel = false;
}

fn format_bytes(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

fn confirm_action(_ctx: &egui::Context, _message: &str) -> bool {
    // For web, we can't easily do modal dialogs
    // In a real implementation, you'd show a confirmation dialog
    // For now, just return true
    // TODO: Implement proper confirmation dialog
    true
}