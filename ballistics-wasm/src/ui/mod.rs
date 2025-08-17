mod main_view;
mod calculation_view;
mod profile_view;
mod settings_view;

use crate::app::{BallisticsWasmApp, View};
use egui::{Context, TopBottomPanel, SidePanel, CentralPanel, ScrollArea, Button, Visuals};

pub fn render_app(app: &mut BallisticsWasmApp, ctx: &Context) {
    // Top bar
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.heading("🎯 Ballistics Analyzer");
            
            ui.separator();
            
            // Navigation buttons
            ui.selectable_value(&mut app.current_view, View::Main, "Calculate");
            ui.selectable_value(&mut app.current_view, View::Saved, "Saved");
            ui.selectable_value(&mut app.current_view, View::Profiles, "Profiles");
            ui.selectable_value(&mut app.current_view, View::LoadData, "Load Data");
            
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // PWA install button
                if !app.install_prompt_shown {
                    if ui.button("📥 Install App").clicked() {
                        install_pwa();
                        app.install_prompt_shown = true;
                    }
                }
                
                // Offline indicator
                if app.is_offline {
                    ui.colored_label(egui::Color32::YELLOW, "🔌 Offline");
                    if !app.pending_sync.is_empty() {
                        ui.label(format!("({} pending)", app.pending_sync.len()));
                    }
                }
                
                // Settings button
                if ui.button("⚙").clicked() {
                    app.current_view = View::Settings;
                }
            });
        });
    });
    
    // Side panel for mobile/tablet
    let is_mobile = ctx.screen_rect().width() < 600.0;
    
    if is_mobile && app.show_side_panel {
        SidePanel::left("side_panel").show(ctx, |ui| {
            render_navigation(app, ui);
        });
    }
    
    // Main content
    CentralPanel::default().show(ctx, |ui| {
        ScrollArea::both().show(ui, |ui| {
            match app.current_view {
                View::Main | View::Calculation => {
                    main_view::render(app, ui);
                }
                View::Saved => {
                    render_saved_calculations(app, ui);
                }
                View::Profiles => {
                    profile_view::render(app, ui);
                }
                View::LoadData => {
                    render_load_data(app, ui);
                }
                View::Settings => {
                    settings_view::render(app, ui);
                }
                View::About => {
                    render_about(app, ui);
                }
            }
        });
    });
    
    // Dialogs
    render_dialogs(app, ctx);
    
    // Notifications
    if let Some(notification) = &app.notification {
        TopBottomPanel::bottom("notification").show(ctx, |ui| {
            ui.horizontal(|ui| {
                let color = match notification.kind {
                    crate::app::NotificationKind::Success => egui::Color32::GREEN,
                    crate::app::NotificationKind::Error => egui::Color32::RED,
                    crate::app::NotificationKind::Info => egui::Color32::BLUE,
                };
                ui.colored_label(color, &notification.message);
                
                if ui.button("✕").clicked() {
                    app.notification = None;
                }
            });
        });
    }
}

fn render_navigation(app: &mut BallisticsWasmApp, ui: &mut egui::Ui) {
    ui.vertical(|ui| {
        if ui.button("Calculate").clicked() {
            app.current_view = View::Main;
            app.show_side_panel = false;
        }
        
        if ui.button("Saved Calculations").clicked() {
            app.current_view = View::Saved;
            app.show_side_panel = false;
        }
        
        if ui.button("Firearm Profiles").clicked() {
            app.current_view = View::Profiles;
            app.show_side_panel = false;
        }
        
        if ui.button("Load Data").clicked() {
            app.current_view = View::LoadData;
            app.show_side_panel = false;
        }
        
        ui.separator();
        
        if ui.button("Settings").clicked() {
            app.current_view = View::Settings;
            app.show_side_panel = false;
        }
        
        if ui.button("About").clicked() {
            app.current_view = View::About;
            app.show_side_panel = false;
        }
    });
}

fn render_saved_calculations(app: &mut BallisticsWasmApp, ui: &mut egui::Ui) {
    ui.heading("📁 Saved Calculations");
    ui.separator();
    
    if app.saved_calculations.is_empty() {
        ui.label("No saved calculations yet.");
        return;
    }
    
    for calc in app.saved_calculations.clone() {
        ui.group(|ui| {
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading(&calc.name);
                    ui.label(calc.timestamp.format("%Y-%m-%d %H:%M").to_string());
                    if !calc.notes.is_empty() {
                        ui.small(&calc.notes);
                    }
                });
                
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    if ui.button("🗑").on_hover_text("Delete").clicked() {
                        app.delete_calculation(&calc.id);
                    }
                    
                    if ui.button("📤").on_hover_text("Share").clicked() {
                        app.share_calculation(&calc);
                    }
                    
                    if ui.button("📋").on_hover_text("Copy").clicked() {
                        let json = serde_json::to_string_pretty(&calc).unwrap();
                        app.copy_to_clipboard(&json);
                    }
                    
                    if ui.button("Load").clicked() {
                        app.load_calculation(&calc.id);
                    }
                });
            });
        });
    }
}

fn render_load_data(app: &mut BallisticsWasmApp, ui: &mut egui::Ui) {
    ui.heading("📚 Load Data Library");
    ui.separator();
    
    for caliber in app.load_library.get_all_calibers() {
        ui.collapsing(&caliber, |ui| {
            let loads = app.load_library.get_loads_for_caliber(&caliber);
            
            for load in loads {
                ui.group(|ui| {
                    ui.label(format!(
                        "{} gr {} - {} gr @ {} fps",
                        load.bullet_weight,
                        load.powder,
                        load.charge_weight,
                        load.velocity
                    ));
                    
                    if !load.notes.is_empty() {
                        ui.small(&load.notes);
                    }
                    
                    if ui.button("Apply").clicked() {
                        app.calculation_data.caliber = load.caliber;
                        app.calculation_data.bullet_weight = load.bullet_weight;
                        app.calculation_data.muzzle_velocity = load.velocity;
                        app.current_view = View::Main;
                    }
                });
            }
        });
    }
}

fn render_about(_app: &BallisticsWasmApp, ui: &mut egui::Ui) {
    ui.heading("About Ballistics Analyzer");
    ui.separator();
    
    ui.label(format!("Version: {}", env!("CARGO_PKG_VERSION")));
    ui.label("Open Source Ballistics Calculator");
    
    ui.separator();
    
    ui.hyperlink_to(
        "GitHub Repository",
        "https://github.com/DatilDev/Ballistics-Analyzer"
    );
    
    ui.separator();
    
    ui.heading("Features:");
    ui.label("• Professional ballistics calculations");
    ui.label("• Works offline (PWA)");
    ui.label("• Multiple firearm profiles");
    ui.label("• Load data library");
    ui.label("• Import/Export data");
    ui.label("• No data collection or tracking");
    
    ui.separator();
    
    ui.heading("Privacy:");
    ui.label("All data is stored locally in your browser.");
    ui.label("No server communication or analytics.");
}

fn render_dialogs(app: &mut BallisticsWasmApp, ctx: &Context) {
    // Save dialog
    if app.save_dialog_open {
        egui::Window::new("Save Calculation")
            .collapsible(false)
            .resizable(false)
            .show(ctx, |ui| {
                ui.label("Name:");
                ui.text_edit_singleline(&mut app.save_name);
                
                ui.label("Notes:");
                ui.text_edit_multiline(&mut app.save_notes);
                
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        app.save_calculation();
                    }
                    
                    if ui.button("Cancel").clicked() {
                        app.save_dialog_open = false;
                        app.save_name.clear();
                        app.save_notes.clear();
                    }
                });
            });
    }
    
    // Import/Export dialog
    if app.export_dialog_open {
        egui::Window::new("Import/Export Data")
            .collapsible(false)
            .resizable(true)
            .show(ctx, |ui| {
                ui.heading("Export");
                if ui.button("Export All Data").clicked() {
                    let data = app.export_data();
                    app.copy_to_clipboard(&data);
                }
                
                ui.separator();
                
                ui.heading("Import");
                ui.label("Paste JSON data:");
                ui.text_edit_multiline(&mut app.import_text);
                
                if ui.button("Import").clicked() {
                    app.import_data(&app.import_text);
                    app.import_text.clear();
                }
                
                ui.separator();
                
                if ui.button("Close").clicked() {
                    app.export_dialog_open = false;
                    app.import_text.clear();
                }
            });
    }
}

fn install_pwa() {
    // Trigger PWA install
    if let Some(window) = web_sys::window() {
        let _ = window.eval("if(deferredPrompt){deferredPrompt.prompt();}");
    }
}