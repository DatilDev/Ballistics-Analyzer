// Location: ballistics-desktop/src/ui.rs
use crate::BallisticsDesktopApp;
use egui::{Ui, ScrollArea, Grid, Button};

pub fn render_main_view(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    // Main calculator view
    ScrollArea::vertical().show(ui, |ui| {
        // Input sections
        ui.heading("Ballistics Calculator");
        
        // Add input fields for calculation data
        Grid::new("calc_grid")
            .num_columns(2)
            .spacing([20.0, 10.0])
            .show(ui, |ui| {
                ui.label("Target Distance (yd):");
                ui.add(egui::DragValue::new(&mut app.calculation_data.target_distance));
                ui.end_row();
                // Add more fields...
            });
        
        if ui.button("Calculate").clicked() {
            app.calculate_trajectory();
        }
    });
}

pub fn render_saved_view(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    ui.heading("Saved Calculations");
    // List saved calculations
}

pub fn render_profiles_view(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    ui.heading("Firearm Profiles");
    // Profile management UI
}

pub fn render_load_data_view(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    ui.heading("Load Data Library");
    // Load data UI
}

pub fn render_settings_view(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    ui.heading("Settings");
    // Settings UI
}

#[cfg(feature = "hardware")]
pub fn render_hardware_panel(app: &mut BallisticsDesktopApp, ui: &mut Ui) {
    ui.heading("Hardware Devices");
    
    if ui.button("Scan Devices").clicked() {
        app.hardware_manager.scan_devices().ok();
    }
    
    for device in app.hardware_manager.get_connected_devices() {
        ui.group(|ui| {
            ui.label(&device.name);
            ui.label(format!("Battery: {:?}", device.battery_level));

            if device.connected {
            if ui.button("Read").clicked() {
                if let Ok(Some(reading)) = app.hardware_manager.read_device(&device.id) {
                    // Handle reading
                    ui.label(format!("Last reading: {:?}", reading));
                }
            }
            if ui.button("Disconnect").clicked() {
                app.hardware_manager.disconnect_device(&device.id).ok();
            }
        } else {
            if ui.button("Connect").clicked() {
                app.hardware_manager.connect_device(&device.id).ok();
        };
    }
});
}
}
