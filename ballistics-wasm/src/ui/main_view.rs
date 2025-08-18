use crate::app::BallisticsWasmApp;
use ballistics_core::BallisticsCalculator;
use egui::{Ui, ScrollArea, CollapsingHeader, Slider, Grid, Button, Color32, RichText};

pub fn render(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.heading("🎯 Ballistics Calculator");
        ui.separator();
        
        // Profile selection
        render_profile_selector(app, ui);
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
                    .size(18.0)
                    .color(Color32::WHITE)
            )
            .fill(Color32::from_rgb(46, 125, 50));
            
            if ui.add_sized([200.0, 40.0], button).clicked() {
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

fn render_profile_selector(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Firearm Profile:");
        
        let current_profile = app.profile_manager
            .get_selected_profile()
            .map(|p| p.name.clone())
            .unwrap_or_else(|| "None Selected".to_string());
        
        egui::ComboBox::from_id_source("profile_selector")
            .selected_text(&current_profile)
            .show_ui(ui, |ui| {
                if ui.selectable_label(
                    app.profile_manager.selected_profile_id.is_none(),
                    "None"
                ).clicked() {
                    app.profile_manager.select_profile(None);
                }
                
                let profiles = app.profile_manager.profiles.clone();
                for profile in profiles {
                    let is_selected = app.profile_manager.selected_profile_id == Some(profile.id.clone());
                    if ui.selectable_label(is_selected, &profile.name).clicked() {
                        app.profile_manager.select_profile(Some(profile.id.clone()));
                        apply_profile_to_calculation(app, &profile.id);
                    }
                }
            });
        
        if ui.button("➕").on_hover_text("Add new profile").clicked() {
            app.current_view = crate::app::View::Profiles;
        }
    });
}

fn render_firearm_inputs(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    CollapsingHeader::new("🔫 Firearm & Ammunition")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("firearm_grid")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Caliber:");
                    ui.text_edit_singleline(&mut app.calculation_data.caliber);
                    ui.end_row();
                    
                    ui.label("Bullet Weight (gr):");
                    ui.add(Slider::new(&mut app.calculation_data.bullet_weight, 50.0..=300.0)
                        .suffix(" gr"));
                    ui.end_row();
                    
                    ui.label("Ballistic Coefficient:");
                    ui.add(Slider::new(&mut app.calculation_data.bc, 0.1..=1.0)
                        .step_by(0.001)
                        .fixed_decimals(3));
                    ui.end_row();
                    
                    ui.label("Muzzle Velocity (fps):");
                    ui.add(Slider::new(&mut app.calculation_data.muzzle_velocity, 1000.0..=4000.0)
                        .suffix(" fps")
                        .step_by(10.0));
                    ui.end_row();
                    
                    ui.label("Sight Height (in):");
                    ui.add(Slider::new(&mut app.calculation_data.sight_height, 0.5..=3.0)
                        .suffix(" in")
                        .step_by(0.1));
                    ui.end_row();
                    
                    ui.label("Zero Range (yd):");
                    ui.add(Slider::new(&mut app.calculation_data.zero_range, 25.0..=300.0)
                        .suffix(" yd")
                        .step_by(5.0));
                    ui.end_row();
                    
                    ui.label("Barrel Twist (in):");
                    ui.add(Slider::new(&mut app.calculation_data.barrel_twist, 7.0..=14.0)
                        .suffix(" in")
                        .step_by(0.5));
                    ui.end_row();
                    
                    ui.label("Bullet Length (in):");
                    ui.add(Slider::new(&mut app.calculation_data.bullet_length, 0.5..=2.0)
                        .suffix(" in")
                        .step_by(0.01));
                    ui.end_row();
                });
        });
}

fn render_environmental_inputs(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    CollapsingHeader::new("🌡 Environmental Conditions")
        .default_open(false)
        .show(ui, |ui| {
            Grid::new("environment_grid")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Temperature (°F):");
                    ui.add(Slider::new(&mut app.calculation_data.temperature, -20.0..=120.0)
                        .suffix(" °F"));
                    ui.end_row();
                    
                    ui.label("Pressure (inHg):");
                    ui.add(Slider::new(&mut app.calculation_data.pressure, 25.0..=32.0)
                        .suffix(" inHg")
                        .step_by(0.01));
                    ui.end_row();
                    
                    ui.label("Humidity (%):");
                    ui.add(Slider::new(&mut app.calculation_data.humidity, 0.0..=100.0)
                        .suffix(" %"));
                    ui.end_row();
                    
                    ui.label("Altitude (ft):");
                    ui.add(Slider::new(&mut app.calculation_data.altitude, -1000.0..=15000.0)
                        .suffix(" ft")
                        .step_by(100.0));
                    ui.end_row();
                    
                    ui.label("Wind Speed (mph):");
                    ui.add(Slider::new(&mut app.calculation_data.wind_speed, 0.0..=50.0)
                        .suffix(" mph"));
                    ui.end_row();
                    
                    ui.label("Wind Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.wind_angle, 0.0..=360.0)
                        .suffix(" °")
                        .step_by(5.0));
                    ui.end_row();
                });
        });
}

fn render_target_inputs(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    CollapsingHeader::new("🎯 Target")
        .default_open(true)
        .show(ui, |ui| {
            Grid::new("target_grid")
                .num_columns(2)
                .spacing([40.0, 10.0])
                .striped(true)
                .show(ui, |ui| {
                    ui.label("Distance (yd):");
                    ui.add(Slider::new(&mut app.calculation_data.target_distance, 0.0..=2000.0)
                        .suffix(" yd")
                        .step_by(25.0));
                    ui.end_row();
                    
                    ui.label("Shooting Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.shooting_angle, -60.0..=60.0)
                        .suffix(" °"));
                    ui.end_row();
                    
                    ui.label("Target Speed (mph):");
                    ui.add(Slider::new(&mut app.calculation_data.target_speed, 0.0..=30.0)
                        .suffix(" mph"));
                    ui.end_row();
                    
                    ui.label("Target Angle (°):");
                    ui.add(Slider::new(&mut app.calculation_data.target_angle, 0.0..=360.0)
                        .suffix(" °"));
                    ui.end_row();
                });
        });
}

fn render_results(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    ui.heading("📊 Trajectory Results");
    
    // Summary stats
    ui.group(|ui| {
        ui.horizontal(|ui| {
            ui.label(format!("Max Range: {:.0} yd", app.calculation_data.max_range));
            ui.separator();
            ui.label(format!("Max Ordinate: {:.2} in", app.calculation_data.max_ordinate));
        });
    });
    
    ui.separator();
    
    // Trajectory table
    ScrollArea::horizontal().show(ui, |ui| {
        Grid::new("trajectory_table")
            .striped(true)
            .num_columns(9)
            .min_col_width(80.0)
            .show(ui, |ui| {
                // Headers
                ui.strong("Distance");
                ui.strong("Drop (in)");
                ui.strong("Drop (MOA)");
                ui.strong("Drop (MIL)");
                ui.strong("Wind (in)");
                ui.strong("Wind (MOA)");
                ui.strong("Wind (MIL)");
                ui.strong("Velocity");
                ui.strong("Energy");
                ui.end_row();
                
                // Data rows
                for point in &app.calculation_data.trajectory {
                    ui.label(format!("{:.0} yd", point.distance));
                    ui.label(format!("{:.1}", point.drop));
                    ui.label(format!("{:.1}", point.drop_moa));
                    ui.label(format!("{:.2}", point.drop_mil));
                    ui.label(format!("{:.1}", point.windage));
                    ui.label(format!("{:.1}", point.windage_moa));
                    ui.label(format!("{:.2}", point.windage_mil));
                    ui.label(format!("{:.0} fps", point.velocity));
                    ui.label(format!("{:.0} ft-lb", point.energy));
                    ui.end_row();
                }
            });
    });
    
    ui.separator();
    
    // Action buttons
    ui.horizontal(|ui| {
        if ui.button("💾 Save").clicked() {
            app.save_dialog_open = true;
        }
        
        if ui.button("📋 Copy to Clipboard").clicked() {
            let text = format_results_as_text(&app.calculation_data);
            app.copy_to_clipboard(&text);
        }
        
        if ui.button("📤 Export JSON").clicked() {
            app.export_dialog_open = true;
        }
    });
}

fn apply_profile_to_calculation(app: &mut BallisticsWasmApp, profile_id: &str) {
    if let Some(profile) = app.profile_manager.get_profile(profile_id) {
        app.calculation_data.caliber = profile.caliber.clone();
        app.calculation_data.barrel_twist = profile.barrel_twist;
        app.calculation_data.sight_height = profile.sight_height;
        app.calculation_data.zero_range = profile.zero_range;
        
        if let Some(ammo) = profile.ammunition.first() {
            app.calculation_data.bullet_weight = ammo.bullet_weight;
            app.calculation_data.bc = ammo.bc;
            app.calculation_data.muzzle_velocity = ammo.muzzle_velocity;
            app.calculation_data.bullet_length = ammo.bullet_length;
        }
    }
}

fn format_results_as_text(data: &ballistics_core::CalculationData) -> String {
    let mut text = String::from("Ballistics Calculation Results\n");
    text.push_str("================================\n\n");
    text.push_str(&format!("Caliber: {}\n", data.caliber));
    text.push_str(&format!("Bullet Weight: {} gr\n", data.bullet_weight));
    text.push_str(&format!("Muzzle Velocity: {} fps\n", data.muzzle_velocity));
    text.push_str(&format!("BC: {:.3}\n", data.bc));
    text.push_str(&format!("Zero Range: {} yd\n\n", data.zero_range));
    
    text.push_str("Distance | Drop | Windage | Velocity | Energy\n");
    text.push_str("---------|------|---------|----------|-------\n");
    
    for point in &data.trajectory {
        text.push_str(&format!(
            "{:8.0} | {:5.1} | {:7.1} | {:8.0} | {:6.0}\n",
            point.distance, point.drop, point.windage, point.velocity, point.energy
        ));
    }
    
    text
}