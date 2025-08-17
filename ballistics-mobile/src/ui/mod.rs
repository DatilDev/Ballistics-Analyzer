mod main_view;
mod calculation_view;
mod profile_view;
mod settings_view;

use crate::app::{MobileApp, ViewType};
use egui::{Context, ScrollArea, TopBottomPanel, CentralPanel, Button, Layout, Align};

pub fn render_mobile_ui(app: &mut MobileApp, ctx: &Context) {
    // Top navigation bar
    TopBottomPanel::top("nav_bar").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("Ballistics Analyzer");
            
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.button("⚙").clicked() {
                    app.show_settings = !app.show_settings;
                }
                
                if ui.button("📤").clicked() {
                    if let Some(id) = &app.selected_calculation_id {
                        app.share_calculation(id);
                    }
                }
            });
        });
    });
    
    // Bottom navigation
    TopBottomPanel::bottom("bottom_nav").show(ctx, |ui| {
        ui.horizontal_centered(|ui| {
            ui.style_mut().spacing.button_padding = egui::vec2(20.0, 10.0);
            
            if ui.selectable_label(
                app.current_view == ViewType::Main,
                "Calculate"
            ).clicked() {
                app.current_view = ViewType::Main;
            }
            
            if ui.selectable_label(
                app.current_view == ViewType::SavedCalculations,
                "Saved"
            ).clicked() {
                app.current_view = ViewType::SavedCalculations;
            }
            
            if ui.selectable_label(
                app.current_view == ViewType::Profiles,
                "Profiles"
            ).clicked() {
                app.current_view = ViewType::Profiles;
            }
            
            if ui.selectable_label(
                app.current_view == ViewType::LoadData,
                "Load Data"
            ).clicked() {
                app.current_view = ViewType::LoadData;
            }
        });
    });
    
    // Main content area
    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::both().show(ui, |ui| {
            match app.current_view {
                ViewType::Main | ViewType::Calculation => {
                    calculation_view::render(app, ui);
                }
                ViewType::SavedCalculations => {
                    render_saved_calculations(app, ui);
                }
                ViewType::Profiles => {
                    profile_view::render(app, ui);
                }
                ViewType::LoadData => {
                    render_load_data(app, ui);
                }
                ViewType::Settings => {
                    settings_view::render(app, ui);
                }
            }
        });
    });
    
    // Settings overlay
    if app.show_settings {
        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ctx, |ui| {
                settings_view::render(app, ui);
                
                ui.separator();
                
                if ui.button("Close").clicked() {
                    app.show_settings = false;
                }
            });
    }
}

fn render_saved_calculations(app: &mut MobileApp, ui: &mut egui::Ui) {
    ui.heading("Saved Calculations");
    
    ui.separator();
    
    for calc in app.saved_calculations.clone() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.label(&calc.name);
                    ui.small(&calc.timestamp.format("%Y-%m-%d %H:%M").to_string());
                });
                
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if ui.button("Load").clicked() {
                        app.load_calculation(&calc.id);
                    }
                    
                    if ui.button("Share").clicked() {
                        app.share_calculation(&calc.id);
                    }
                    
                    if ui.button("Delete").clicked() {
                        app.delete_calculation(&calc.id);
                    }
                });
            });
        });
    }
}

fn render_load_data(app: &mut MobileApp, ui: &mut egui::Ui) {
    ui.heading("Load Data Library");
    
    ui.separator();
    
    let calibers = app.load_library.get_all_calibers();
    
    for caliber in calibers {
        ui.collapsing(&caliber, |ui| {
            let loads = app.load_library.get_loads_for_caliber(&caliber);
            
            for load in loads {
                ui.group(|ui| {
                    ui.label(format!(
                        "{} gr - {} ({} gr)",
                        load.bullet_weight,
                        load.powder,
                        load.charge_weight
                    ));
                    ui.label(format!("Velocity: {} fps", load.velocity));
                    ui.small(&load.notes);
                    
                    if ui.button("Apply").clicked() {
                        app.calculation_data.caliber = load.caliber.clone();
                        app.calculation_data.bullet_weight = load.bullet_weight;
                        app.calculation_data.muzzle_velocity = load.velocity;
                        app.current_view = ViewType::Calculation;
                    }
                });
            }
        });
    }
}