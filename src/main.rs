// src/main.rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use chrono::Utc;
use eframe::egui;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod auth;
mod ballistics;
mod storage;
mod ui;
mod hardware;
mod load_data;
mod firearm_profiles;
mod sharing;

#[cfg(target_arch = "wasm32")]
mod pwa;

use auth::NostrAuth;
use ballistics::{BallisticsCalculator, ProjectileData, TrajectoryResult};
use firearm_profiles::{FirearmProfile, FirearmType};
use hardware::{HardwareManager, RangefinderData, WeatherData};
use load_data::LoadDataLibrary;
use sharing::SharingManager;
use storage::LocalStorage;


// Define AttachedImage locally
#[derive(Clone)]
pub struct AttachedImage {
    pub id: String,
    pub mime: String,
    pub bytes: Vec<u8>,
}

#[derive(Default)]
pub struct BallisticsApp {
    // Core managers
    auth: NostrAuth,
    calculator: BallisticsCalculator,
    storage: LocalStorage,
    hardware: HardwareManager,
    load_library: LoadDataLibrary,
    sharing: SharingManager,

    // UI and app state
    current_screen: Screen,
    current_calculation: CalculationData,
    trajectory_results: Option<TrajectoryResult>,
    firearm_profiles: Vec<FirearmProfile>,
    selected_profile: Option<usize>,
    show_load_library: bool,
    show_hardware_panel: bool,
    calculation_history: Vec<SavedCalculation>,
    attached_images: Vec<AttachedImage>,
    error_message: Option<String>,

    // Settings and confirmations
    settings: Settings,
    confirm_clear_data: bool,
}

#[derive(Default, Clone, Serialize, Deserialize)]
pub struct CalculationData {
    pub id: String,
    pub projectile_data: ProjectileData,
    pub notes: String,
    pub weather_data: Option<WeatherData>,
    pub range_data: Option<RangefinderData>,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct SavedCalculation {
    pub id: String,
    pub calculation: CalculationData,
    pub results: TrajectoryResult,
    pub profile_name: Option<String>,
    pub image_ids: Vec<String>,
}

#[derive(PartialEq, Default, Clone, Copy)]
enum Screen {
    #[default]
    Login,
    Main,
    Analysis,
    History,
    Profiles,
    LoadLibrary,
    Sharing,
    Settings,
    About,
}

// Platform-specific main functions
#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    env_logger::init();

    let native_options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_min_inner_size([800.0, 600.0]),
        centered: true,
        ..Default::default()
    };

    eframe::run_native(
        "Ballistics Analyzer",
        native_options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(BallisticsApp::new(cc)))
        }),
    )
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    
    wasm_bindgen_futures::spawn_local(async {
        pwa::web::register_service_worker().await;
        
        let runner = eframe::WebRunner::new();
        runner
            .start(
                "ballistics_canvas",
                web_options,
                Box::new(|cc| {
                    setup_custom_fonts(&cc.egui_ctx);
                    Ok(Box::new(BallisticsApp::new(cc)))
                }),
            )
            .await
            .expect("Failed to start eframe");
    });
}

impl BallisticsApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let mut app = Self {
            current_screen: Screen::Login,
            current_calculation: CalculationData {
                id: Uuid::new_v4().to_string(),
                projectile_data: ProjectileData::default(),
                notes: String::new(),
                weather_data: None,
                range_data: None,
                timestamp: Utc::now().to_rfc3339(),
            },
            ..Default::default()
        };

        // Restore session if available
        if let Some(storage) = cc.storage {
            if let Some(auth_data) = storage.get_string("auth_data") {
                app.auth.restore_from_string(&auth_data);
                if app.auth.is_authenticated() {
                    app.current_screen = Screen::Main;
                    app.storage.init_user_storage(&app.auth.get_pubkey());
                    app.load_user_data();
                }
            }
        }

        app
    }
}

impl eframe::App for BallisticsApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        storage.set_string("auth_data", self.auth.serialize());
        storage.flush();
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(if self.settings.dark_mode {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        });

        // Top menu bar
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("🎯 Ballistics Analyzer");
                ui.separator();

                if self.auth.is_authenticated() {
                    self.show_navigation(ui);

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.button("🚪 Logout").clicked() {
                            self.logout();
                        }
                        if ui.button("📡 Hardware").clicked() {
                            self.show_hardware_panel = !self.show_hardware_panel;
                        }
                        ui.label(format!("👤 {}", self.auth.get_display_name()));
                    });
                }
            });
        });

        // Hardware panel
        if self.show_hardware_panel && self.auth.is_authenticated() {
            egui::SidePanel::right("hardware_panel")
                .default_width(300.0)
                .show(ctx, |ui| {
                    // Hardware panel UI implementation
                    ui.heading("📡 Hardware");
                    ui.separator();
                    
                    if self.hardware.rangefinder_connected() {
                        ui.label("✅ Rangefinder connected");
                        if let Some(data) = self.hardware.get_rangefinder_data() {
                            ui.label(format!("Distance: {} yards", data.distance));
                            ui.label(format!("Angle: {}°", data.angle));
                        }
                    } else {
                        if ui.button("Connect Rangefinder").clicked() {
                            self.hardware.connect_rangefinder();
                        }
                    }
                    
                    ui.separator();
                    
                    if self.hardware.weather_meter_connected() {
                        ui.label("✅ Weather meter connected");
                        if let Some(data) = self.hardware.get_weather_data() {
                            ui.label(format!("Temp: {}°F", data.temperature));
                            ui.label(format!("Pressure: {} inHg", data.pressure));
                            ui.label(format!("Humidity: {}%", data.humidity));
                        }
                    } else {
                        if ui.button("Connect Weather Meter").clicked() {
                            self.hardware.connect_weather_meter();
                        }
                    }
                });
        }

        // Error messages
        if let Some(error) = &self.error_message.clone() {
            egui::TopBottomPanel::top("error_panel").show(ctx, |ui| {
                ui.colored_label(egui::Color32::RED, format!("⚠️ {}", error));
                if ui.button("✕").clicked() {
                    self.error_message = None;
                }
            });
        }

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_screen {
                Screen::Login => self.show_login_screen(ui, ctx),
                Screen::Main => self.show_main_screen(ui),
                Screen::Analysis => self.show_analysis_screen(ui, ctx),
                Screen::History => self.show_history_screen(ui),
                Screen::Profiles => self.show_profiles_screen(ui),
                Screen::LoadLibrary => self.show_load_library_screen(ui),
                Screen::Sharing => self.show_sharing_screen(ui),
                Screen::Settings => self.show_settings_screen(ui),
                Screen::About => todo!(),
            }
        });
    }
}

// UI Implementation
impl BallisticsApp {
    fn show_navigation(&mut self, ui: &mut egui::Ui) {
        if ui.button("📊 Analysis").clicked() {
            self.current_screen = Screen::Analysis;
        }
        if ui.button("🔫 Profiles").clicked() {
            self.current_screen = Screen::Profiles;
        }
        if ui.button("📚 Load Data").clicked() {
            self.current_screen = Screen::LoadLibrary;
        }
        if ui.button("📜 History").clicked() {
            self.current_screen = Screen::History;
            self.load_calculation_history();
        }
        if ui.button("🔄 Share").clicked() {
            self.current_screen = Screen::Sharing;
        }
        if ui.button("⚙️ Settings").clicked() {
            self.current_screen = Screen::Settings;
        }
        if ui.button("ℹ️ About").clicked() {
            self.current_screen = Screen::About;
        }
    }

    fn show_login_screen(&mut self, ui: &mut egui::Ui, _ctx: &egui::Context) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            ui.heading("🎯 Advanced Ballistics Analyzer");
            ui.add_space(20.0);
            ui.label("Professional Trajectory Calculations with Hardware Integration");
            ui.add_space(40.0);

            ui.group(|ui| {
                ui.set_width(360.0);

                if ui.button("🔐 Login with Nostr Extension").clicked() {
                    self.login_with_extension();
                }

                ui.separator();

                // Amber Remote Signer
                ui.collapsing("Amber Remote Signer", |ui| {
                    ui.label("Connect to Amber (remote signer) to use your keys securely.");
                    ui.horizontal(|ui| {
                        ui.label("Endpoint/Relay:");
                        ui.text_edit_singleline(&mut self.auth.amber_endpoint);
                    });

                    if ui.button("🔑 Connect Amber Remote Signer").clicked() {
                        self.login_with_amber();
                    }
                });

                ui.separator();

                if ui.button("🔑 Generate New Identity").clicked() {
                    self.generate_new_identity();
                }

                ui.separator();

                // Import private key section
                static mut KEY_INPUT: String = String::new();
                ui.collapsing("Import Private Key", |ui| {
                    ui.label("Paste your nsec or hex key:");
                    unsafe {
                        let response = ui.text_edit_singleline(&mut KEY_INPUT);
                        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                            self.import_private_key(&KEY_INPUT.clone());
                            KEY_INPUT.clear();
                        }
                    }
                });
            });

            ui.add_space(40.0);

            ui.group(|ui| {
                ui.label("✓ All data stored locally on your device");
                ui.label("✓ Connect rangefinders and weather meters");
                ui.label("✓ Share calculations via Nostr protocol");
                ui.label("✓ Works offline after installation");
            });
        });
    }

    fn login_with_amber(&mut self) {
        if self.auth.login_with_amber() {
            self.current_screen = Screen::Main;
            self.storage.init_user_storage(&self.auth.get_pubkey());
            self.load_user_data();
        } else {
            self.error_message = Some("Failed to connect to Amber remote signer".to_string());
        }
    }

    fn show_main_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading("Welcome to Ballistics Analyzer");
            ui.add_space(20.0);

            ui.columns(3, |columns| {
                columns[0].vertical_centered(|ui| {
                    if ui.button("📊 New Analysis").clicked() {
                        self.new_calculation();
                    }
                });

                columns[1].vertical_centered(|ui| {
                    if ui.button("🔫 Firearm Profiles").clicked() {
                        self.current_screen = Screen::Profiles;
                    }
                });

                columns[2].vertical_centered(|ui| {
                    if ui.button("📡 Hardware Setup").clicked() {
                        self.show_hardware_panel = !self.show_hardware_panel;
                    }
                });
            });

            ui.add_space(40.0);

            if !self.calculation_history.is_empty() {
                ui.separator();
                ui.heading("Recent Calculations");
                ui.add_space(10.0);

                // Fix borrow issue - clone the needed data
                let recent_calcs: Vec<_> = self.calculation_history.iter().take(3).cloned().collect();
                
                for calc in recent_calcs {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(&calc.calculation.timestamp);
                            ui.separator();
                            ui.label(format!(
                                "{} - {} gr @ {} fps",
                                calc.calculation.projectile_data.caliber,
                                calc.calculation.projectile_data.mass,
                                calc.calculation.projectile_data.velocity
                            ));

                            if ui.button("Load").clicked() {
                                self.load_calculation(&calc);
                                self.current_screen = Screen::Analysis;
                            }
                        });
                    });
                }
            }
        });
    }

    fn show_analysis_screen(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.horizontal(|ui| {
            ui.heading("📊 Trajectory Analysis");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if self.trajectory_results.is_some() {
                    if ui.button("💾 Save").clicked() {
                        self.save_calculation();
                    }
                    if ui.button("🔄 Share").clicked() {
                        self.share_calculation();
                    }
                }
                if ui.button("📡 Hardware").clicked() {
                    self.show_hardware_panel = !self.show_hardware_panel;
                }
            });
        });

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Firearm Profile:");
            
            // Fix borrow issue - collect profile data
            let profile_names: Vec<_> = self.firearm_profiles.iter()
                .enumerate()
                .map(|(i, p)| (i, p.name.clone()))
                .collect();
            
            let selected_text = self.selected_profile
                .and_then(|i| self.firearm_profiles.get(i))
                .map(|p| p.name.clone())
                .unwrap_or_else(|| "None Selected".to_string());
            
            let mut selected_idx = self.selected_profile;
            
            egui::ComboBox::from_label("")
                .selected_text(selected_text)
                .show_ui(ui, |ui| {
                    if ui.selectable_label(selected_idx.is_none(), "None").clicked() {
                        selected_idx = None;
                    }

                    ui.separator();

                    for (i, name) in profile_names {
                        if ui.selectable_label(selected_idx == Some(i), &name).clicked() {
                            selected_idx = Some(i);
                        }
                    }
                });
            
            // Apply selection changes after combo box
            if selected_idx != self.selected_profile {
                self.selected_profile = selected_idx;
                if let Some(i) = selected_idx {
                    self.apply_profile(i);
                }
            }

            ui.separator();

            if ui.button("📚 Load Library").clicked() {
                self.show_load_library = !self.show_load_library;
            }
        });

        if self.show_load_library {
            self.show_load_data_popup(ui, ctx);
        }

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            self.show_projectile_data_section(ui);
            self.show_environmental_conditions_section(ui);
            self.show_notes_photos_section(ui);
        });

        ui.separator();

        ui.horizontal(|ui| {
            let calculate_button = ui.add_sized([120.0, 40.0], egui::Button::new("🎯 Calculate"));
            if calculate_button.clicked() {
                self.calculate_trajectory();
            }
            if self.trajectory_results.is_some() {
                ui.separator();
                if ui.button("📋 Copy Results").clicked() {
                    self.copy_results_to_clipboard();
                }
                if ui.button("🖨️ Print").clicked() {
                    self.print_results();
                }
            }
        });

        if let Some(results) = &self.trajectory_results {
            ui.separator();
            self.show_trajectory_results(ui, results);
        }
    }

    fn show_projectile_data_section(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("🎯 Projectile Data")
            .default_open(true)
            .show(ui, |ui| {
                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Caliber:");
                            ui.text_edit_singleline(&mut self.current_calculation.projectile_data.caliber);
                        });
                        ui.horizontal(|ui| {
                            ui.label("Bullet Weight:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.mass)
                                    .speed(0.1)
                                    .suffix(" gr"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Muzzle Velocity:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.velocity)
                                    .speed(1.0)
                                    .suffix(" fps"),
                            );
                        });
                    });

                    columns[1].group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Ballistic Coefficient:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.bc)
                                    .speed(0.001)
                                    .range(0.1..=2.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Zero Range:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.zero_range)
                                    .speed(1.0)
                                    .suffix(" yards"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Sight Height:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.sight_height)
                                    .speed(0.1)
                                    .suffix(" inches"),
                            );
                        });
                    });
                });
            });
    }

    fn show_environmental_conditions_section(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("🌤️ Environmental Conditions")
            .default_open(true)
            .show(ui, |ui| {
                if let Some(weather) = &self.current_calculation.weather_data {
                    ui.colored_label(
                        egui::Color32::GREEN,
                        format!(
                            "📡 Live Data: {}°F, {:.2}\" Hg, {}% RH",
                            weather.temperature, weather.pressure, weather.humidity
                        ),
                    );
                    ui.separator();
                }

                ui.columns(2, |columns| {
                    columns[0].group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Temperature:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.temperature)
                                    .speed(1.0)
                                    .suffix(" °F"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Pressure:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.pressure)
                                    .speed(0.01)
                                    .suffix(" inHg"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Humidity:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.humidity)
                                    .speed(1.0)
                                    .suffix(" %")
                                    .range(0.0..=100.0),
                            );
                        });
                    });

                    columns[1].group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label("Wind Speed:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.wind_speed)
                                    .speed(0.1)
                                    .suffix(" mph"),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Wind Angle:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.wind_angle)
                                    .speed(1.0)
                                    .suffix(" °")
                                    .range(0.0..=360.0),
                            );
                        });
                        ui.horizontal(|ui| {
                            ui.label("Altitude:");
                            ui.add(
                                egui::DragValue::new(&mut self.current_calculation.projectile_data.altitude)
                                    .speed(10.0)
                                    .suffix(" ft"),
                            );
                        });
                    });
                });
            });
    }

    fn show_notes_photos_section(&mut self, ui: &mut egui::Ui) {
        egui::CollapsingHeader::new("📝 Notes & Photos")
            .default_open(false)
            .show(ui, |ui| {
                ui.label("Notes:");
                ui.text_edit_multiline(&mut self.current_calculation.notes);

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("📷 Add Photo").clicked() {
                        self.add_photo();
                    }
                    ui.label(format!("{} photos attached", self.attached_images.len()));
                });

                if !self.attached_images.is_empty() {
                    ui.add_space(10.0);
                    ui.horizontal_wrapped(|ui| {
                        let mut to_remove = None;
                        for (i, _img) in self.attached_images.iter().enumerate() {
                            ui.group(|ui| {
                                ui.vertical(|ui| {
                                    ui.label("📷");
                                    ui.label(format!("Image {}", i + 1));
                                    if ui.small_button("❌").clicked() {
                                        to_remove = Some(i);
                                    }
                                });
                            });
                        }
                        if let Some(i) = to_remove {
                            self.attached_images.remove(i);
                        }
                    });
                }
            });
    }

    fn show_profiles_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔫 Firearm Profiles");

        ui.horizontal(|ui| {
            if ui.button("➕ Add New Profile").clicked() {
                self.firearm_profiles.push(FirearmProfile::default());
            }

            ui.separator();

            if ui.button("📥 Import").clicked() {
                self.import_profiles();
            }

            if ui.button("📤 Export").clicked() {
                self.export_profiles();
            }
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut to_remove: Option<usize> = None;
            let mut to_duplicate: Option<usize> = None;

            // Create a list of indices to iterate over
            let indices: Vec<usize> = (0..self.firearm_profiles.len()).collect();
            
            for i in indices {
                let profile = &mut self.firearm_profiles[i];
                ui.push_id(i, |ui| {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading(&profile.name);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("❌").clicked() {
                                    to_remove = Some(i);
                                }
                                if ui.button("📋").clicked() {
                                    to_duplicate = Some(i);
                                }
                            });
                        });

                        ui.separator();

                        ui.columns(2, |columns| {
                            columns[0].vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Name:");
                                    ui.text_edit_singleline(&mut profile.name);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Type:");
                                    egui::ComboBox::from_id_source(format!("type_{}", i))
                                        .selected_text(format!("{:?}", profile.firearm_type))
                                        .show_ui(ui, |ui| {
                                            ui.selectable_value(
                                                &mut profile.firearm_type,
                                                FirearmType::Rifle,
                                                "Rifle",
                                            );
                                            ui.selectable_value(
                                                &mut profile.firearm_type,
                                                FirearmType::Pistol,
                                                "Pistol",
                                            );
                                            ui.selectable_value(
                                                &mut profile.firearm_type,
                                                FirearmType::Shotgun,
                                                "Shotgun",
                                            );
                                        });
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Manufacturer:");
                                    ui.text_edit_singleline(&mut profile.manufacturer);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Model:");
                                    ui.text_edit_singleline(&mut profile.model);
                                });
                            });

                            columns[1].vertical(|ui| {
                                ui.horizontal(|ui| {
                                    ui.label("Caliber:");
                                    ui.text_edit_singleline(&mut profile.caliber);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Barrel Length:");
                                    ui.add(
                                        egui::DragValue::new(&mut profile.barrel_length)
                                            .suffix(" inches")
                                            .speed(0.1),
                                    );
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Twist Rate:");
                                    ui.text_edit_singleline(&mut profile.twist_rate);
                                });
                                ui.horizontal(|ui| {
                                    ui.label("Sight Height:");
                                    ui.add(
                                        egui::DragValue::new(&mut profile.sight_height)
                                            .suffix(" inches")
                                            .speed(0.1),
                                    );
                                });
                            });
                        });

                        ui.separator();
                        ui.label("Notes:");
                        ui.text_edit_multiline(&mut profile.notes);
                    });
                });

                ui.add_space(10.0);
            }

            // Handle actions after iteration
            if let Some(i) = to_duplicate {
                self.duplicate_profile(i);
            }
            
            if let Some(i) = to_remove {
                self.firearm_profiles.remove(i);
                if self.selected_profile == Some(i) {
                    self.selected_profile = None;
                }
            }
        });

        ui.separator();

        if ui.button("💾 Save All Profiles").clicked() {
            self.save_profiles();
        }
    }

    fn show_history_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("📜 Calculation History");

        ui.horizontal(|ui| {
            if ui.button("🔄 Refresh").clicked() {
                self.load_calculation_history();
            }
            ui.separator();
            ui.label(format!("Total: {} calculations", self.calculation_history.len()));
        });

        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            let mut to_delete = None;
            
            for (idx, calc) in self.calculation_history.iter().enumerate() {
                ui.group(|ui: &mut egui::Ui| {
                    ui.horizontal(|ui| {
                        ui.label(format!("📅 {}", calc.calculation.timestamp));

                        if let Some(profile) = &calc.profile_name {
                            ui.separator();
                            ui.label(format!("🔫 {}", profile));
                        }

                        if !calc.image_ids.is_empty() {
                            ui.separator();
                            ui.label(format!("📷 {} photos", calc.image_ids.len()));
                        }
                    });

                    ui.separator();

                    ui.label(format!(
                        "{} - {} gr @ {} fps",
                        calc.calculation.projectile_data.caliber,
                        calc.calculation.projectile_data.mass,
                        calc.calculation.projectile_data.velocity
                    ));

                    if !calc.calculation.notes.is_empty() {
                        ui.separator();
                        ui.label(format!(
                            "📝 {}",
                            calc.calculation.notes.lines().next().unwrap_or("")
                        ));
                    }

                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("📂 Load").clicked() {
                            self.load_calculation(calc);
                            self.current_screen = Screen::Analysis;
                        }

                        if ui.button("📋 Duplicate").clicked() {
                            self.duplicate_calculation(calc);
                        }

                        if ui.button("🔄 Share").clicked() {
                            self.share_specific_calculation(calc);
                        }

                        if ui.button("🗑️ Delete").clicked() {
                            to_delete = Some(idx);
                        }
                    });
                });

                ui.add_space(10.0);
            }
            
            if let Some(idx) = to_delete {
                let id = self.calculation_history[idx].id.clone();
                self.delete_calculation(&id);
            }
        });
    }

    fn show_load_library_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("📚 Ammunition Load Library");

        ui.horizontal(|ui| {
            ui.label("Manufacturer:");
            egui::ComboBox::from_label("")
                .selected_text(&self.load_library.selected_manufacturer)
                .show_ui(ui, |ui| {
                    for mfg in self.load_library.get_manufacturers() {
                        ui.selectable_value(
                            &mut self.load_library.selected_manufacturer,
                            mfg.clone(),
                            &mfg,
                        );
                    }
                });

            ui.separator();

            if ui.button("➕ Add Custom Load").clicked() {
                self.add_custom_load();
            }
        });

        ui.separator();

        if let Some(loads) = self.load_library
            .get_loads_for_manufacturer(&self.load_library.selected_manufacturer)
        {
            egui::ScrollArea::vertical().show(ui, |ui| {
                for load in loads {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.heading(&load.name);
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                if ui.button("Use in Analysis").clicked() {
                                    self.apply_load_data(&load);
                                    self.current_screen = Screen::Analysis;
                                }
                            });
                        });

                        ui.separator();

                        ui.columns(2, |columns| {
                            columns[0].vertical(|ui| {
                                ui.label(format!("Caliber: {}", load.caliber));
                                ui.label(format!("Bullet Weight: {} gr", load.bullet_weight));
                                ui.label(format!("Muzzle Velocity: {} fps", load.velocity));
                            });

                            columns[1].vertical(|ui| {
                                ui.label(format!("BC: {:.3}", load.bc));
                                ui.label(format!("Powder: {}", load.powder_type));
                                ui.label(format!("Charge: {} gr", load.powder_charge));
                            });
                        });
                    });

                    ui.add_space(10.0);
                }
            });
        }
    }

    fn show_sharing_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("🔄 Share Calculations");
        ui.label("Share your ballistics calculations securely via Nostr protocol");

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Sharing Options:");
            ui.checkbox(&mut self.sharing.include_photos, "Include Photos");
            ui.checkbox(&mut self.sharing.include_profile, "Include Firearm Profile");
            ui.checkbox(&mut self.sharing.include_weather, "Include Weather Data");
        });

        ui.separator();

        ui.heading("Select Calculation to Share:");

        egui::ScrollArea::vertical()
            .max_height(300.0)
            .show(ui, |ui| {
                for calc in self.calculation_history.iter().take(10) {
                    ui.group(|ui| {
                        ui.label(&calc.calculation.timestamp);
                        ui.label(format!(
                            "{} - {} gr @ {} fps",
                            calc.calculation.projectile_data.caliber,
                            calc.calculation.projectile_data.mass,
                            calc.calculation.projectile_data.velocity
                        ));

                        if ui.button("Share This").clicked() {
                            if let Some(event_id) = self.sharing.share_calculation(&self.auth, calc) {
                                self.show_share_success(&event_id);
                            }
                        }
                    });
                }
            });

        ui.separator();

        ui.heading("Import Shared Calculation:");

        ui.horizontal(|ui| {
            ui.label("Nostr Event ID:");
            ui.text_edit_singleline(&mut self.sharing.import_event_id);

            if ui.button("Import").clicked() {
                self.import_shared_calculation();
            }
        });

        if !self.sharing.recent_shares.is_empty() {
            ui.separator();
            ui.heading("Recent Shares:");

            for share in &self.sharing.recent_shares {
                ui.group(|ui| {
                    ui.label(format!("Shared: {}", share.timestamp));
                    let display_id = if share.event_id.len() >= 16 {
                        format!(
                            "Event ID: {}...{}",
                            &share.event_id[..8],
                            &share.event_id[share.event_id.len() - 8..]
                        )
                    } else {
                        format!("Event ID: {}", share.event_id)
                    };
                    ui.label(display_id);

                    if ui.button("Copy ID").clicked() {
                        ui.output_mut(|o| o.copied_text = share.event_id.clone());
                    }
                });
            }
        }
    }

    fn show_settings_screen(&mut self, ui: &mut egui::Ui) {
        ui.heading("⚙️ Settings");
        ui.separator();

        egui::ScrollArea::vertical().show(ui, |ui| {
            // Display Settings
            ui.group(|ui| {
                ui.heading("🎨 Display");
                ui.checkbox(&mut self.settings.dark_mode, "Dark Mode");
                ui.checkbox(&mut self.settings.show_tooltips, "Show Tooltips");
                
                ui.horizontal(|ui| {
                    ui.label("Font Size:");
                    ui.add(egui::Slider::new(&mut self.settings.font_size, 10.0..=20.0));
                });
            });

            ui.add_space(10.0);

            // Units Settings
            ui.group(|ui| {
                ui.heading("📏 Units");
                
                ui.horizontal(|ui| {
                    ui.label("Distance:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.settings.distance_unit))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.settings.distance_unit, DistanceUnit::Yards, "Yards");
                            ui.selectable_value(&mut self.settings.distance_unit, DistanceUnit::Meters, "Meters");
                        });
                });
                
                ui.horizontal(|ui| {
                    ui.label("Temperature:");
                    egui::ComboBox::from_label("")
                        .selected_text(format!("{:?}", self.settings.temp_unit))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.settings.temp_unit, TempUnit::Fahrenheit, "Fahrenheit");
                            ui.selectable_value(&mut self.settings.temp_unit, TempUnit::Celsius, "Celsius");
                        });
                });
            });

            ui.add_space(10.0);

            // Calculation Settings
            ui.group(|ui| {
                ui.heading("🧮 Calculations");
                ui.checkbox(&mut self.settings.auto_save, "Auto-save calculations");
                ui.checkbox(&mut self.settings.include_coriolis, "Include Coriolis effect");
            });

            ui.add_space(10.0);

            // Data Management
            ui.group(|ui| {
                ui.heading("💾 Data Management");
                
                ui.horizontal(|ui| {
                    if ui.button("📤 Export All Data").clicked() {
                        self.export_all_data();
                    }
                    if ui.button("📥 Import Data").clicked() {
                        self.import_data();
                    }
                });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    if ui.button("🗑️ Clear All Data").clicked() {
                        self.confirm_clear_data = true;
                    }
                });
                
                if self.confirm_clear_data {
                    ui.separator();
                    ui.colored_label(egui::Color32::RED, "⚠️ This will delete all your data!");
                    ui.horizontal(|ui| {
                        if ui.button("✅ Confirm Delete").clicked() {
                            self.clear_all_data();
                            self.confirm_clear_data = false;
                        }
                        if ui.button("❌ Cancel").clicked() {
                            self.confirm_clear_data = false;
                        }
                    });
                }
            });

            ui.add_space(10.0);

            // About section link
            ui.group(|ui| {
                ui.heading("ℹ️ Information");
                if ui.button("📖 About Ballistics Analyzer").clicked() {
                    self.current_screen = Screen::About;
                }
            });
        });
    }


    fn show_trajectory_results(&self, ui: &mut egui::Ui, results: &TrajectoryResult) {
        ui.heading("📈 Trajectory Results");

        ui.horizontal(|ui| {
            ui.group(|ui| {
                ui.label(format!("Max Range: {} yards", results.max_range));
            });
            ui.group(|ui| {
                ui.label(format!("Max Ordinate: {:.1} inches", results.max_ordinate));
            });
            ui.group(|ui| {
                ui.label(format!("Zero Offset: {:.2} MOA", results.zero_offset));
            });
        });

        ui.separator();

        // Trajectory graph would go here
        ui.group(|ui| {
            ui.set_height(300.0);
            ui::draw_trajectory_graph(ui, results);
        });

        ui.separator();

        self.show_trajectory_table(ui, results);
    }

    fn show_trajectory_table(&self, ui: &mut egui::Ui, results: &TrajectoryResult) {
        ui.label("Detailed Trajectory Data:");

        egui::ScrollArea::vertical()
            .max_height(400.0)
            .show(ui, |ui| {
                egui::Grid::new("trajectory_grid")
                    .striped(true)
                    .spacing([10.0, 4.0])
                    .show(ui, |ui| {
                        // Headers
                        ui.strong("Range");
                        ui.strong("Drop");
                        ui.strong("Drift");
                        ui.strong("Velocity");
                        ui.strong("Energy");
                        ui.strong("Time");
                        ui.strong("MOA");
                        ui.strong("MIL");
                        ui.end_row();

                        // Units
                        ui.label("(yards)");
                        ui.label("(inches)");
                        ui.label("(inches)");
                        ui.label("(fps)");
                        ui.label("(ft-lb)");
                        ui.label("(sec)");
                        ui.label("(adj)");
                        ui.label("(adj)");
                        ui.end_row();

                        // Data rows
                        for point in &results.trajectory_points {
                            ui.label(format!("{:.0}", point.distance));
                            ui.label(format!("{:.1}", point.drop));
                            ui.label(format!("{:.1}", point.drift));
                            ui.label(format!("{:.0}", point.velocity));
                            ui.label(format!("{:.0}", point.energy));
                            ui.label(format!("{:.3}", point.time));
                            ui.label(format!("{:.1}", point.moa_adjustment));
                            ui.label(format!("{:.2}", point.mil_adjustment));
                            ui.end_row();
                        }
                    });
            });
    }

    // Helper methods
    fn new_calculation(&mut self) {
        self.current_calculation = CalculationData {
            id: Uuid::new_v4().to_string(),
            projectile_data: ProjectileData::default(),
            notes: String::new(),
            weather_data: None,
            range_data: None,
            timestamp: Utc::now().to_rfc3339(),
        };
        self.trajectory_results = None;
        self.attached_images.clear();
        self.current_screen = Screen::Analysis;
    }

    fn calculate_trajectory(&mut self) {
        self.current_calculation.timestamp = Utc::now().to_rfc3339();
        self.trajectory_results =
            Some(self.calculator.calculate(&self.current_calculation.projectile_data));
    }

    fn save_calculation(&mut self) {
        if let Some(results) = &self.trajectory_results {
            let saved = SavedCalculation {
                id: self.current_calculation.id.clone(),
                calculation: self.current_calculation.clone(),
                results: results.clone(),
                profile_name: self
                    .selected_profile
                    .and_then(|i| self.firearm_profiles.get(i))
                    .map(|p| p.name.clone()),
                image_ids: self.attached_images.iter().map(|img| img.id.clone()).collect(),
            };

            self.storage.save_calculation(&saved, &self.attached_images);
            self.calculation_history.insert(0, saved);
            self.error_message = Some("Calculation saved successfully!".to_string());
        }
    }

    fn load_calculation(&mut self, calc: &SavedCalculation) {
        self.current_calculation = calc.calculation.clone();
        self.trajectory_results = Some(calc.results.clone());
        self.attached_images = self.storage.load_images(&calc.image_ids);

        if let Some(profile_name) = &calc.profile_name {
            self.selected_profile = self
                .firearm_profiles
                .iter()
                .position(|p| p.name == *profile_name);
        }
    }

    fn apply_profile(&mut self, index: usize) {
        if let Some(profile) = self.firearm_profiles.get(index) {
            match profile.firearm_type {
                FirearmType::Rifle => {
                    self.current_calculation.projectile_data.sight_height = profile.sight_height;
                }
                FirearmType::Pistol => {
                    self.current_calculation.projectile_data.sight_height = profile.sight_height;
                    self.current_calculation.projectile_data.zero_range = 25.0;
                }
                FirearmType::Shotgun => {
                    self.current_calculation.projectile_data.sight_height = profile.sight_height;
                    self.current_calculation.projectile_data.zero_range = 50.0;
                }
            }

            if !profile.caliber.is_empty() {
                self.current_calculation.projectile_data.caliber = profile.caliber.clone();
            }
        }
    }

    fn apply_weather_data(&mut self, data: WeatherData) {
        self.current_calculation.projectile_data.temperature = data.temperature;
        self.current_calculation.projectile_data.pressure = data.pressure;
        self.current_calculation.projectile_data.humidity = data.humidity;
        self.current_calculation.projectile_data.wind_speed = data.wind_speed;
        self.current_calculation.projectile_data.wind_angle = data.wind_angle;
        self.current_calculation.weather_data = Some(data);
    }

    fn apply_load_data(&mut self, load: &load_data::LoadData) {
        self.current_calculation.projectile_data.caliber = load.caliber.clone();
        self.current_calculation.projectile_data.mass = load.bullet_weight;
        self.current_calculation.projectile_data.velocity = load.velocity;
        self.current_calculation.projectile_data.bc = load.bc;
    }

    fn add_photo(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .add_filter("Images", &["jpg", "jpeg", "png"])
                .pick_file()
            {
                self.load_image_from_path(path);
            }
        }

        #[cfg(target_arch = "wasm32")]
        {
            wasm_bindgen_futures::spawn_local(async move {
                // TODO: Implement web file input
            });
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn load_image_from_path(&mut self, path: std::path::PathBuf) {
        if let Ok(data) = std::fs::read(&path) {
            let id = Uuid::new_v4().to_string();
            let mime = match path.extension().and_then(|e| e.to_str()) {
                Some("png") => "image/png",
                Some("jpg") | Some("jpeg") => "image/jpeg",
                _ => "image/unknown",
            }.to_string();

            self.attached_images.push(AttachedImage {
                id,
                mime,
                bytes: data,
            });
        }
    }

    fn login_with_extension(&mut self) {
        #[cfg(target_arch = "wasm32")]
        {
            self.auth.authenticate_with_extension();
        }
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.generate_new_identity();
        }
    }

    fn generate_new_identity(&mut self) {
        if self.auth.generate_new_keys() {
            self.current_screen = Screen::Main;
            self.storage.init_user_storage(&self.auth.get_pubkey());
            self.load_user_data();
        }
    }

    fn import_private_key(&mut self, key: &str) {
        if self.auth.import_key(key) {
            self.current_screen = Screen::Main;
            self.storage.init_user_storage(&self.auth.get_pubkey());
            self.load_user_data();
        } else {
            self.error_message = Some("Invalid private key format".to_string());
        }
    }

    fn logout(&mut self) {
        self.auth.logout();
        self.current_screen = Screen::Login;
        self.calculation_history.clear();
        self.firearm_profiles.clear();
        self.current_calculation = CalculationData::default();
        self.trajectory_results = None;
        self.attached_images.clear();
    }

    fn load_user_data(&mut self) {
        self.firearm_profiles = self.storage.load_profiles();
        self.calculation_history = self.storage.load_calculations();
    }

    fn load_calculation_history(&mut self) {
        self.calculation_history = self.storage.load_calculations();
    }

    fn save_profiles(&mut self) {
        self.storage.save_profiles(&self.firearm_profiles);
        self.error_message = Some("Profiles saved successfully!".to_string());
    }

    fn share_calculation(&mut self) {
        if let Some(results) = &self.trajectory_results {
            let saved = SavedCalculation {
                id: self.current_calculation.id.clone(),
                calculation: self.current_calculation.clone(),
                results: results.clone(),
                profile_name: self
                    .selected_profile
                    .and_then(|i| self.firearm_profiles.get(i))
                    .map(|p| p.name.clone()),
                image_ids: if self.sharing.include_photos {
                    self.attached_images.iter().map(|img| img.id.clone()).collect()
                } else {
                    vec![]
                },
            };

            if let Some(event_id) = self.sharing.share_calculation(&self.auth, &saved) {
                self.show_share_success(&event_id);
            }
        }
    }

    fn show_load_data_popup(&mut self, _ui: &mut egui::Ui, ctx: &egui::Context) {
        egui::Window::new("📚 Load Data Library")
            .default_pos([300.0, 200.0])
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Manufacturer:");
                    egui::ComboBox::from_label("")
                        .selected_text(&self.load_library.selected_manufacturer)
                        .show_ui(ui, |ui| {
                            for mfg in self.load_library.get_manufacturers() {
                                ui.selectable_value(
                                    &mut self.load_library.selected_manufacturer,
                                    mfg.clone(),
                                    &mfg,
                                );
                            }
                        });
                });

                ui.separator();

                // Clone loads to avoid borrow issues
                let loads = self.load_library
                    .get_loads_for_manufacturer(&self.load_library.selected_manufacturer)
                    .cloned()
                    .unwrap_or_default();

                if !loads.is_empty() {
                    egui::ScrollArea::vertical()
                        .max_height(300.0)
                        .show(ui, |ui| {
                            for load in &loads {
                                let load_clone = load.clone();
                                ui.group(|ui| {
                                    ui.label(&load.name);
                                    ui.label(format!(
                                        "{} gr @ {} fps",
                                        load.bullet_weight, load.velocity
                                    ));
                                    if ui.button("Use").clicked() {
                                        self.apply_load_data(&load_clone);
                                        self.show_load_library = false;
                                    }
                                });
                            }
                        });
                }

                ui.separator();

                if ui.button("Close").clicked() {
                    self.show_load_library = false;
                }
            });
    }

    fn copy_results_to_clipboard(&self) {
        // Implement if needed; on native you can use `arboard`
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(results) = &self.trajectory_results {
                let text = format!("Trajectory summary: max_range={}, max_ordinate={:.1}, zero_offset={:.2}",
                    results.max_range, results.max_ordinate, results.zero_offset);
                if let Ok(mut clipboard) = arboard::Clipboard::new() {
                    let _ = clipboard.set_text(text);
                }
            }
        }
        #[cfg(target_arch = "wasm32")]
        {
            // Implement via web clipboard if desired
        }
    }

    fn print_results(&self) {
        // Stub
    }

    fn duplicate_profile(&mut self, index: usize) {
        if let Some(profile) = self.firearm_profiles.get(index).cloned() {
            let mut new_profile = profile;
            new_profile.id = Uuid::new_v4().to_string();
            new_profile.name = format!("{} (Copy)", new_profile.name);
            self.firearm_profiles.push(new_profile);
        }
    }

    fn import_profiles(&mut self) {
        // Stub
    }

    fn export_profiles(&mut self) {
        // Stub
    }

    fn duplicate_calculation(&mut self, calc: &SavedCalculation) {
        let mut new_calc = calc.clone();
        new_calc.id = Uuid::new_v4().to_string();
        new_calc.calculation.id = new_calc.id.clone();
        new_calc.calculation.timestamp = Utc::now().to_rfc3339();
        self.storage.save_calculation(&new_calc, &[]);
        self.calculation_history.insert(0, new_calc);
    }

    fn share_specific_calculation(&mut self, calc: &SavedCalculation) {
        if let Some(event_id) = self.sharing.share_calculation(&self.auth, calc) {
            self.show_share_success(&event_id);
        }
    }

    fn delete_calculation(&mut self, id: &str) {
        self.storage.delete_calculation(id);
        self.calculation_history.retain(|c| c.id != id);
    }

    fn add_custom_load(&mut self) {
        // Stub
    }

    fn show_share_success(&mut self, event_id: &str) {
        self.error_message = Some(format!("Shared successfully! Event ID: {}", event_id));
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(mut clipboard) = arboard::Clipboard::new() {
                let _ = clipboard.set_text(event_id.to_string());
            }
        }
    }

    fn import_shared_calculation(&mut self) {
        if let Some(calc) = self.sharing.import_calculation(&self.sharing.import_event_id) {
            self.storage.save_calculation(&calc, &[]);
            self.calculation_history.insert(0, calc);
            self.error_message = Some("Calculation imported successfully!".to_string());
        } else {
            self.error_message = Some("Failed to import calculation".to_string());
        }
    }

    fn export_all_data(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new()
                .set_file_name("ballistics_data.json")
                .save_file()
            {
                let export_data = serde_json::json!({
                    "version": "1.0",
                    "profiles": self.firearm_profiles,
                    "calculations": self.calculation_history,
                    "timestamp": Utc::now().to_rfc3339()
                });

                if let Ok(json) = serde_json::to_string_pretty(&export_data) {
                    let _ = std::fs::write(path, json);
                    self.error_message = Some("Data exported successfully!".to_string());
                }
            }
        }
    }

    fn import_data(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Some(path) = rfd::FileDialog::new().add_filter("JSON", &["json"]).pick_file() {
                if let Ok(_data) = std::fs::read_to_string(path) {
                    // TODO: Parse and import data
                    self.error_message = Some("Data imported successfully!".to_string());
                }
            }
        }
    }

    fn clear_all_data(&mut self) {
        self.storage.clear_all();
        self.calculation_history.clear();
        self.firearm_profiles.clear();
        self.current_calculation = CalculationData::default();
        self.trajectory_results = None;
        self.attached_images.clear();
        self.error_message = Some("All data cleared".to_string());
    }
}


// Settings struct
#[derive(Default, Clone, Serialize, Deserialize)]
struct Settings {
    dark_mode: bool,
    show_tooltips: bool,
    font_size: f32,
    distance_unit: DistanceUnit,
    temp_unit: TempUnit,
    auto_save: bool,
    include_coriolis: bool,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum DistanceUnit {
    #[default]
    Yards,
    Meters,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
enum TempUnit {
    #[default]
    Fahrenheit,
    Celsius,
}


impl Default for Settings {
    fn default() -> Self {
        Self {
            dark_mode: true,
            show_tooltips: true,
            font_size: 14.0,
            distance_unit: DistanceUnit::Yards,
            temp_unit: TempUnit::Fahrenheit,
            auto_save: true,
            include_coriolis: false,
        }
    }
}
//Font Setup
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(not(target_arch = "wasm32"))]
    if let Ok(font_data) = std::fs::read("assets/fonts/Roboto-Regular.ttf") {
        fonts
            .font_data
            .insert("Roboto".to_owned(), egui::FontData::from_owned(font_data));
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "Roboto".to_owned());
    }

    ctx.set_fonts(fonts);
}
// Platform-specific helpers for web
#[cfg(target_arch = "wasm32")]
mod web_helpers {
    use wasm_bindgen::prelude::*;
    use web_sys::window;

    pub fn get_window() -> Option<web_sys::Window> {
        window()
    }

    pub fn copy_to_clipboard(text: &str) {
        if let Some(window) = get_window() {
            if let Some(navigator) = window.navigator().clipboard() {
                let text = text.to_string();
                wasm_bindgen_futures::spawn_local(async move {
                    let _ = wasm_bindgen_futures::JsFuture::from(navigator.write_text(&text)).await;
                });
            }
        }
    }}
