use crate::app::MobileApp;
use egui::{Ui, ScrollArea, CollapsingHeader, Slider, Grid, Button, Color32, RichText};

pub fn render(app: &mut MobileApp, ui: &mut Ui) {
    ui.heading("Ballistics Calculator");
    ui.separator();
    
    ScrollArea::vertical().show(ui, |ui| {
        // Quick actions bar
        ui.horizontal(|ui| {
            if ui.button("📍 Use GPS").clicked() {
                use_current_location(app);
            }
            
            if ui.button("🌤 Get Weather").clicked() {
                fetch_current_weather(app);
            }
            
            if ui.button("📷 Attach Photo").clicked() {
                app.take_photo();
            }
        });
        
        ui.separator();
        
        // Profile selection
        ui.horizontal(|ui| {
            ui.label("Firearm Profile:");
            
            let current_profile = app.profile_manager
                .get_selected_profile()
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "None Selected".to_string());
            
            egui::ComboBox::from_label("")
                .selected_text(&current_profile)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(false, "None").clicked() {
                        app.profile_manager.select_profile(None);
                    }
                    
                    let profiles = app.profile_manager.profiles.clone();
                    for profile in &profiles {
                        let profile_id = profile.id.clone();
                        let profile_name = profile.name.clone();
                        if ui.selectable_label(
                            app.profile_manager.selected_profile_id == Some(profile_id.clone()),
                            &profile_name
                        ).clicked() {
                            app.profile_manager.select_profile(Some(profile_id.clone()));
                            apply_profile_to_calculation(app, &profile_id);
                        }
                    }
                });
        });
        
        ui.separator();
        
        // Input sections
        render_firearm_inputs(app, ui);
        render_environmental_inputs(app, ui);
        render_target_inputs(app, ui);
        
        ui.separator();
        
        // Calculate button
        ui.vertical_centered(|ui| {
            let button = Button::new(
                RichText::new("Calculate Trajectory")
                    .size(20.0)
                    .color(Color32::WHITE)
            )
            .fill(Color32::from_rgb(46, 125, 50));
            
            if ui.add_sized([200.0, 50.0], button).clicked() {
                app.calculate_trajectory();
            }
        });
        
        // Results section
        if !app.calculation_data.trajectory.is_empty() {
            ui.separator();
            render_results(app, ui);
        }
    });
}

fn render_firearm_inputs(app: &mut MobileApp, ui: &mut Ui) {
    CollapsingHeader::new("🔫 Firearm & Ammunition")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("firearm_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Caliber:");
                    ui.text_edit_singleline(&mut app.calculation_data.caliber);
                    ui.end_row();
                    
                    ui.label("Bullet Weight (gr):");
                    ui.add(Slider::new(&mut app.calculation_data.bullet_weight, 50.0..=300.0));
                    ui.end_row();
                    
                    ui.label("Ballistic Coefficient:");
                    ui.add(Slider::new(&mut app.calculation_data.bc, 0.1..=1.0)
                        .step_by(0.001));
                    ui.end_row();
                    
                    ui.label("Muzzle Velocity (fps):");
                    ui.add(Slider::new(&mut app.calculation_data.muzzle_velocity, 1000.0..=4000.0));
                    ui.end_row();
                    
                    ui.label("Sight Height (in):");
                    ui.add(Slider::new(&mut app.calculation_data.sight_height, 0.5..=3.0)
                        .step_by(0.1));
                    ui.end_row();
                    
                    ui.label("Zero Range (yd):");
                    ui.add(Slider::new(&mut app.calculation_data.zero_range, 25.0..=300.0)
                        .step_by(5.0));
                    ui.end_row();
                    
                    ui.label("Barrel Twist (in):");
                    ui.add(Slider::new(&mut app.calculation_data.barrel_twist, 7.0..=14.0)
                        .step_by(0.5));
                    ui.end_row();
                    
                    ui.label("Bullet Length (in):");
                    ui.add(Slider::new(&mut app.calculation_data.bullet_length, 0.5..=2.0)
                        .step_by(0.01));
                    ui.end_row();
                });
        });
}

fn render_environmental_inputs(app: &mut MobileApp, ui: &mut Ui) {
    CollapsingHeader::new("🌡 Environmental Conditions")
        .default_open(false)
        .show(ui, |ui| {
            Grid::new("environment_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Temperature (°F):");
                    ui.add(Slider::new(&mut app.calculation_data.temperature, -20.0..=120.0));
                    ui.end_row();
                    
                    ui.label("Pressure (inHg):");
                    ui.add(Slider::new(&mut app.calculation_data.pressure, 25.0..=32.0)
                        .step_by(0.01));
                    ui.end_row();
                    
                    ui.label("Humidity (%):");
                    ui.add(Slider::new(&mut app.calculation_data.humidity, 0.0..=100.0));
                    ui.end_row();
                    
                    ui.label("Altitude (ft):");
                    ui.add(Slider::new(&mut app.calculation_data.altitude, -1000.0..=15000.0));
                    ui.end_row();
                    
                    ui.label("Wind Speed (mph):");
                    ui.add(Slider::new(&mut app.calculation_data.wind_speed, 0.0..=50.0));
                    ui.end_row();
                    
                    ui.label("Wind Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.wind_angle, 0.0..=360.0));
                    ui.end_row();
                    
                    ui.label("Shooting Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.shooting_angle, -60.0..=60.0));
                    ui.end_row();
                    
                    ui.label("Latitude (°):");
                    ui.add(Slider::new(&mut app.calculation_data.latitude, -90.0..=90.0));
                    ui.end_row();
                    
                    ui.label("Azimuth (°):");
                    ui.add(Slider::new(&mut app.calculation_data.azimuth, 0.0..=360.0));
                    ui.end_row();
                });
        });
}

fn render_target_inputs(app: &mut MobileApp, ui: &mut Ui) {
    CollapsingHeader::new("🎯 Target")
        .default_open(false)
        .show(ui, |ui| {
            Grid::new("target_grid")
                .num_columns(2)
                .spacing([20.0, 10.0])
                .show(ui, |ui| {
                    ui.label("Distance (yd):");
                    ui.add(Slider::new(&mut app.calculation_data.target_distance, 0.0..=2000.0)
                        .step_by(25.0));
                    ui.end_row();
                    
                    ui.label("Target Speed (mph):");
                    ui.add(Slider::new(&mut app.calculation_data.target_speed, 0.0..=30.0));
                    ui.end_row();
                    
                    ui.label("Target Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.target_angle, 0.0..=360.0));
                    ui.end_row();
                });
        });
}

fn render_results(app: &mut MobileApp, ui: &mut Ui) {
    ui.heading("📊 Trajectory Results");
    
    // Summary stats
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Max Range: {:.0} yd", app.calculation_data.max_range));
            ui.separator();
            ui.label(format!("Max Ordinate: {:.2} in", app.calculation_data.max_ordinate));
        });
    });
    
    // Trajectory table
    ScrollArea::horizontal().show(ui, |ui| {
        Grid::new("trajectory_table")
            .striped(true)
            .num_columns(7)
            .show(ui, |ui| {
                // Headers
                ui.strong("Distance");
                ui.strong("Drop (in)");
                ui.strong("Drop (MOA)");
                ui.strong("Wind (in)");
                ui.strong("Wind (MOA)");
                ui.strong("Velocity");
                ui.strong("Energy");
                ui.end_row();
                
                // Data rows
                for point in &app.calculation_data.trajectory {
                    ui.label(format!("{:.0} yd", point.distance));
                    ui.label(format!("{:.1}", point.drop));
                    ui.label(format!("{:.1}", point.drop_moa));
                    ui.label(format!("{:.1}", point.windage));
                    ui.label(format!("{:.1}", point.windage_moa));
                    ui.label(format!("{:.0} fps", point.velocity));
                    ui.label(format!("{:.0} ft-lb", point.energy));
                    ui.end_row();
                }
            });
    });
    
    // Save button
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("💾 Save Calculation").clicked() {
            save_calculation_dialog(app, ui);
        }
        
        if ui.button("📤 Share").clicked() {
            let json = serde_json::to_string_pretty(&app.calculation_data).unwrap();
            
            #[cfg(target_os = "android")]
            crate::platform::android::share_text(&json, "Ballistics Calculation");
            
            #[cfg(target_os = "ios")]
            crate::platform::ios::share_text(&json, "Ballistics Calculation");
        }
    });
}

fn use_current_location(app: &mut MobileApp) {
    if let Ok(location) = app.location_service.lock() {
        if let Some(lat) = location.latitude {
            app.calculation_data.latitude = lat;
        }
        if let Some(alt) = location.altitude {
            app.calculation_data.altitude = alt;
        }
    }
}

fn fetch_current_weather(_app: &mut MobileApp) {
    // TODO: Implement weather API integration
}

fn apply_profile_to_calculation(app: &mut MobileApp, profile_id: &str) {
    if let Some(profile) = app.profile_manager.get_profile(profile_id) {
        app.calculation_data.caliber = profile.caliber.clone();
        app.calculation_data.barrel_twist = profile.barrel_twist;
        app.calculation_data.sight_height = profile.sight_height;
        app.calculation_data.zero_range = profile.zero_range;
        
        // Apply first ammunition profile if available
        if let Some(ammo) = profile.ammunition.first() {
            app.calculation_data.bullet_weight = ammo.bullet_weight;
            app.calculation_data.bc = ammo.bc;
            app.calculation_data.muzzle_velocity = ammo.muzzle_velocity;
            app.calculation_data.bullet_length = ammo.bullet_length;
        }
    }
}

fn save_calculation_dialog(app: &mut MobileApp, ui: &mut Ui) {
    egui::Window::new("Save Calculation")
        .collapsible(false)
        .resizable(false)
        .show(ui.ctx(), |ui| {
            let mut name = String::new();
            let mut notes = String::new();
            
            ui.label("Name:");
            ui.text_edit_singleline(&mut name);
            
            ui.label("Notes:");
            ui.text_edit_multiline(&mut notes);
            
            ui.horizontal(|ui| {
                if ui.button("Save").clicked() {
                    app.save_calculation(name, notes);
                }
                
                if ui.button("Cancel").clicked() {
                    // Close dialog
                }
            });
        });
}