use egui;
use crate::BallisticsDesktopApp;

pub fn render_main_view(app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Ballistics Calculator");
    
    ui.separator();
    
    if ui.button("Calculate Trajectory").clicked() {
        app.calculate_trajectory();
    }
    
    if ui.button("Save Calculation").clicked() {
        app.save_dialog_open = true;
    }
}

pub fn render_saved_view(app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Saved Calculations");
    
    for calc in &app.saved_calculations {
        ui.horizontal(|ui| {
            ui.label(&calc.name);
            ui.label(calc.timestamp.format("%Y-%m-%d %H:%M").to_string());
        });
    }
}

pub fn render_profiles_view(app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Firearm Profiles");
    
    for profile in app.profile_manager.list_profiles() {
        ui.horizontal(|ui| {
            ui.label(&profile.name);
            ui.label(&profile.caliber);
        });
    }
}

pub fn render_load_data_view(_app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Load Data Library");
}

pub fn render_settings_view(_app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Settings");
}

#[cfg(feature = "hardware")]
pub fn render_hardware_panel(_app: &mut BallisticsDesktopApp, ui: &mut egui::Ui) {
    ui.heading("Hardware Devices");
}