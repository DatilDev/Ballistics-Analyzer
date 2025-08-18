use crate::app::BallisticsWasmApp;
use ballistics_core::{FirearmProfile, AmmunitionProfile};
use egui::{Ui, ScrollArea, CollapsingHeader, Grid};
use uuid::Uuid;

pub fn render(app: &mut BallisticsWasmApp, ui: &mut Ui) {
    ScrollArea::vertical().show(ui, |ui| {
        ui.heading("🔫 Firearm Profiles");
        ui.separator();
        
        // Add new profile button
        if ui.button("➕ Add New Profile").clicked() {
            add_new_profile(app);
        }
        
        ui.separator();
        
        // List existing profiles
        let profiles = app.profile_manager.profiles.clone();
        for profile in profiles {
            render_profile_card(app, ui, &profile);
        }
        
        if app.profile_manager.profiles.is_empty() {
            ui.label("No profiles created yet. Click 'Add New Profile' to get started.");
        }
    });
}

fn render_profile_card(app: &mut BallisticsWasmApp, ui: &mut Ui, profile: &FirearmProfile) {
    ui.group(|ui| {
        let is_selected = app.profile_manager.selected_profile_id == Some(profile.id.clone());
        
        CollapsingHeader::new(&profile.name)
            .id_source(&profile.id)
            .default_open(is_selected)
            .show(ui, |ui| {
                let mut edited_profile = profile.clone();
                let mut changed = false;
                
                Grid::new(format!("profile_grid_{}", profile.id))
                    .num_columns(2)
                    .spacing([20.0, 8.0])
                    .show(ui, |ui| {
                        ui.label("Name:");
                        if ui.text_edit_singleline(&mut edited_profile.name).changed() {
                            changed = true;
                        }
                        ui.end_row();
                        
                        ui.label("Caliber:");
                        if ui.text_edit_singleline(&mut edited_profile.caliber).changed() {
                            changed = true;
                        }
                        ui.end_row();
                        
                        ui.label("Barrel Length (in):");
                        if ui.add(egui::DragValue::new(&mut edited_profile.barrel_length)
                            .speed(0.1)
                            .suffix(" in"))
                            .changed() {
                            changed = true;
                        }
                        ui.end_row();
                        
                        ui.label("Barrel Twist (in):");
                        if ui.add(egui::DragValue::new(&mut edited_profile.barrel_twist)
                            .speed(0.1)
                            .suffix(" in"))
                            .changed() {
                            changed = true;
                        }
                        ui.end_row();
                        
                        ui.label("Sight Height (in):");
                        if ui.add(egui::DragValue::new(&mut edited_profile.sight_height)
                            .speed(0.1)
                            .suffix(" in")
                            .clamp_range(0.0..=5.0))
                            .changed() {
                            changed = true;
                        }
                        ui.end_row();
                        
                        ui.label("Zero Range (yd):");
                        if ui.add(egui::DragValue::new(&mut edited_profile.zero_range)
                            .speed(5.0)
                            .suffix(" yd")
                            .clamp_range(0.0..=1000.0))
                            .changed() {
                            changed = true;
                        }
                        ui.end_row();
                    });
                
                ui.separator();
                ui.label("📦 Ammunition Profiles:");
                
                // Ammunition list
                let mut ammo_to_remove = None;
                for (idx, ammo) in edited_profile.ammunition.iter_mut().enumerate() {
                    ui.group(|ui| {
                        render_ammunition_profile(ui, ammo, &mut changed);
                        
                        if ui.small_button("❌ Remove").clicked() {
                            ammo_to_remove = Some(idx);
                            changed = true;
                        }
                    });
                }
                
                if let Some(idx) = ammo_to_remove {
                    edited_profile.ammunition.remove(idx);
                }
                
                if ui.button("➕ Add Ammunition").clicked() {
                    edited_profile.ammunition.push(AmmunitionProfile {
                        id: Uuid::new_v4().to_string(),
                        name: "New Ammunition".to_string(),
                        bullet_weight: 175.0,
                        bc: 0.5,
                        muzzle_velocity: 2600.0,
                        bullet_length: 1.24,
                    });
                    changed = true;
                }
                
                ui.separator();
                
                ui.label("📝 Notes:");
                if ui.text_edit_multiline(&mut edited_profile.notes).changed() {
                    changed = true;
                }
                
                ui.separator();
                
                // Action buttons
                ui.horizontal(|ui| {
                    if ui.button("✓ Use This Profile").clicked() {
                        app.profile_manager.select_profile(Some(profile.id.clone()));
                        app.current_view = crate::app::View::Main;
                    }
                    
                    if changed {
                        if ui.button("💾 Save Changes").clicked() {
                            update_profile(app, edited_profile);
                        }
                    }
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🗑 Delete Profile").clicked() {
                            delete_profile(app, &profile.id);
                        }
                    });
                });
            });
    });
}

fn render_ammunition_profile(ui: &mut Ui, ammo: &mut AmmunitionProfile, changed: &mut bool) {
    Grid::new(format!("ammo_grid_{}", ammo.id))
        .num_columns(2)
        .spacing([20.0, 5.0])
        .show(ui, |ui| {
            ui.label("Name:");
            if ui.text_edit_singleline(&mut ammo.name).changed() {
                *changed = true;
            }
            ui.end_row();
            
            ui.label("Bullet Weight (gr):");
            if ui.add(egui::DragValue::new(&mut ammo.bullet_weight)
                .speed(1.0)
                .suffix(" gr")
                .clamp_range(20.0..=800.0))
                .changed() {
                *changed = true;
            }
            ui.end_row();
            
            ui.label("BC:");
            if ui.add(egui::DragValue::new(&mut ammo.bc)
                .speed(0.001)
                .fixed_decimals(3)
                .clamp_range(0.1..=1.5))
                .changed() {
                *changed = true;
            }
            ui.end_row();
            
            ui.label("Muzzle Velocity (fps):");
            if ui.add(egui::DragValue::new(&mut ammo.muzzle_velocity)
                .speed(10.0)
                .suffix(" fps")
                .clamp_range(500.0..=5000.0))
                .changed() {
                *changed = true;
            }
            ui.end_row();
            
            ui.label("Bullet Length (in):");
            if ui.add(egui::DragValue::new(&mut ammo.bullet_length)
                .speed(0.01)
                .suffix(" in")
                .fixed_decimals(2)
                .clamp_range(0.2..=3.0))
                .changed() {
                *changed = true;
            }
            ui.end_row();
        });
}

fn add_new_profile(app: &mut BallisticsWasmApp) {
    let new_profile = FirearmProfile {
        id: Uuid::new_v4().to_string(),
        name: format!("Profile {}", app.profile_manager.profiles.len() + 1),
        caliber: ".308 Winchester".to_string(),
        barrel_length: 24.0,
        barrel_twist: 10.0,
        sight_height: 1.5,
        zero_range: 100.0,
        ammunition: vec![
            AmmunitionProfile {
                id: Uuid::new_v4().to_string(),
                name: "Default Load".to_string(),
                bullet_weight: 175.0,
                bc: 0.505,
                muzzle_velocity: 2600.0,
                bullet_length: 1.24,
            }
        ],
        notes: String::new(),
    };
    
    app.profile_manager.add_profile(new_profile.clone());
    crate::storage::WebStorage::save_profile(&new_profile);
}

fn update_profile(app: &mut BallisticsWasmApp, profile: FirearmProfile) {
    // Update in manager
    if let Some(existing) = app.profile_manager.profiles
        .iter_mut()
        .find(|p| p.id == profile.id) {
        *existing = profile.clone();
    }
    
    // Save to storage
    crate::storage::WebStorage::save_profile(&profile);
}

fn delete_profile(app: &mut BallisticsWasmApp, id: &str) {
    app.profile_manager.remove_profile(id);
    crate::storage::WebStorage::delete_profile(id);
}