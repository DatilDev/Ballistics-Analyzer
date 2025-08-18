use ballistics_core::{
    BallisticsCalculator, CalculationData, FirearmProfileManager,
    LoadDataLibrary, SavedCalculation, StorageBackend,
};
use egui::{Context, FullOutput, RawInput};
use std::sync::{Arc, Mutex};

pub struct MobileApp {
    ctx: Context,
    raw_input: RawInput,
    storage_path: String,
    
    // Core components
    pub calculation_data: CalculationData,
    pub saved_calculations: Vec<SavedCalculation>,
    pub profile_manager: FirearmProfileManager,
    pub load_library: LoadDataLibrary,
    pub storage: Box<dyn StorageBackend>,
    
    // UI state
    pub current_view: ViewType,
    pub selected_calculation_id: Option<String>,
    pub show_settings: bool,
    
    // Platform services
    pub location_service: Arc<Mutex<LocationService>>,
    pub camera_service: Arc<Mutex<CameraService>>,
    pub share_service: Arc<Mutex<ShareService>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ViewType {
    Main,
    Calculation,
    SavedCalculations,
    Profiles,
    LoadData,
    Settings,
}

pub struct LocationService {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f64>,
}

pub struct CameraService {
    pub last_photo: Option<Vec<u8>>,
}

pub struct ShareService {
    pub last_shared_id: Option<String>,
}

impl MobileApp {
    pub fn new(storage_path: &str) -> Self {
        let ctx = Context::default();
        
        // Initialize storage
        let storage: Box<dyn StorageBackend> = Box::new(
            crate::storage::FileBasedStorage::new(storage_path).unwrap()
        );
        
        // Load saved data
        let saved_calculations = storage.list_calculations().unwrap_or_default();
        let mut profile_manager = FirearmProfileManager::new();
        
        // Load profiles from storage
        if let Ok(profiles) = storage.list_profiles() {
            for profile in profiles {
                profile_manager.add_profile(profile);
            }
        }
        
        Self {
            ctx,
            raw_input: RawInput::default(),
            storage_path: storage_path.to_string(),
            calculation_data: CalculationData::default(),
            saved_calculations,
            profile_manager,
            load_library: LoadDataLibrary::new(),
            storage,
            current_view: ViewType::Main,
            selected_calculation_id: None,
            show_settings: false,
            location_service: Arc::new(Mutex::new(LocationService {
                latitude: None,
                longitude: None,
                altitude: None,
            })),
            camera_service: Arc::new(Mutex::new(CameraService {
                last_photo: None,
            })),
            share_service: Arc::new(Mutex::new(ShareService {
                last_shared_id: None,
            })),
        }
    }
    
    pub fn update(&mut self) {
        // Use a different approach that doesn't require borrowing self in closure
        use egui::{CentralPanel, TopBottomPanel};
        
        // Create a new frame
        let output = self.ctx.run(self.raw_input.take(), |ctx| {
            // Top panel
            TopBottomPanel::top("nav_bar").show(ctx, |ui| {
                ui.heading("Ballistics Analyzer");
            });
            
            // Main content - we'll fill this after
            CentralPanel::default().show(ctx, |ui| {
                ui.label("Rendering...");
            });
        });
        
        // Handle any platform output
        if !output.platform_output.copied_text.is_empty() {
            #[cfg(target_os = "android")]
            crate::platform::android::copy_to_clipboard(&output.platform_output.copied_text);
            
            #[cfg(target_os = "ios")]
            crate::platform::ios::copy_to_clipboard(&output.platform_output.copied_text);
        }
        
        // Now do the actual rendering in a separate pass
        self.render_content();
    }
    
    fn render_content(&mut self) {
        // This is called after the initial frame setup
        // and can safely access self
        let ctx = self.ctx.clone();
        
        // Request repaint for next frame
        ctx.request_repaint();
        
        // The actual UI rendering will happen in the platform-specific code
    }
    
    // Add this helper method
    fn take(&mut self) -> egui::RawInput {
        std::mem::take(&mut self.raw_input)
    }  
    pub fn handle_touch(&mut self, x: f32, y: f32, event_type: i32) {
        use egui::{Event, Pos2, TouchDeviceId, TouchId, TouchPhase};
        
        let pos = Pos2::new(x, y);
        let touch_id = TouchId::from(0);
        let device_id = TouchDeviceId(0);
        
        let phase = match event_type {
            0 => TouchPhase::Start,
            1 => TouchPhase::Move,
            2 => TouchPhase::End,
            3 => TouchPhase::Cancel,
            _ => return,
        };
        
        self.raw_input.events.push(Event::Touch {
            device_id,
            id: touch_id,
            phase,
            pos,
            force: None,
        });
    }
    
    pub fn on_resume(&mut self) {
        // Refresh data when app resumes
        self.saved_calculations = self.storage.list_calculations().unwrap_or_default();
        
        // Start location updates
        #[cfg(target_os = "android")]
        crate::platform::android::start_location_updates();
        
        #[cfg(target_os = "ios")]
        crate::platform::ios::start_location_updates();
    }
    
    pub fn on_pause(&mut self) {
        // Save current state
        if self.calculation_data.trajectory.len() > 0 {
            let calculation = SavedCalculation {
                id: uuid::Uuid::new_v4().to_string(),
                timestamp: chrono::Utc::now(),
                name: "Auto-saved".to_string(),
                data: self.calculation_data.clone(),
                photos: Vec::new(),
                notes: String::new(),
                weather: None,
                firearm_profile_id: self.profile_manager.selected_profile_id.clone(),
            };
            
            let _ = self.storage.save_calculation(&calculation);
        }
        
        // Stop location updates
        #[cfg(target_os = "android")]
        crate::platform::android::stop_location_updates();
        
        #[cfg(target_os = "ios")]
        crate::platform::ios::stop_location_updates();
    }
    
    pub fn calculate_trajectory(&mut self) {
        BallisticsCalculator::calculate_trajectory(&mut self.calculation_data);
    }
    
    pub fn save_calculation(&mut self, name: String, notes: String) {
        let calculation = SavedCalculation {
            id: uuid::Uuid::new_v4().to_string(),
            timestamp: chrono::Utc::now(),
            name,
            data: self.calculation_data.clone(),
            photos: Vec::new(),
            notes,
            weather: None,
            firearm_profile_id: self.profile_manager.selected_profile_id.clone(),
        };
        
        if let Ok(()) = self.storage.save_calculation(&calculation) {
            self.saved_calculations.push(calculation);
        }
    }
    
    pub fn get_calculation_result(&self) -> String {
        serde_json::to_string(&self.calculation_data).unwrap_or_default()
    }
    
    pub fn load_calculation(&mut self, id: &str) {
        if let Ok(calc) = self.storage.load_calculation(id) {
            self.calculation_data = calc.data;
            self.selected_calculation_id = Some(id.to_string());
            self.current_view = ViewType::Calculation;
        }
    }
    
    pub fn delete_calculation(&mut self, id: &str) {
        if let Ok(()) = self.storage.delete_calculation(id) {
            self.saved_calculations.retain(|c| c.id != id);
        }
    }
    
    pub fn take_photo(&mut self) {
        #[cfg(target_os = "android")]
        crate::platform::android::take_photo();
        
        #[cfg(target_os = "ios")]
        crate::platform::ios::take_photo();
    }
    
    pub fn share_calculation(&mut self, id: &str) {
        if let Ok(calc) = self.storage.load_calculation(id) {
            let json = serde_json::to_string_pretty(&calc).unwrap_or_default();
            
            #[cfg(target_os = "android")]
            crate::platform::android::share_text(&json, "Ballistics Calculation");
            
            #[cfg(target_os = "ios")]
            crate::platform::ios::share_text(&json, "Ballistics Calculation");
            
            self.share_service.lock().unwrap().last_shared_id = Some(id.to_string());
        }
    }
    
    fn handle_platform_events(&mut self, output: &FullOutput) {
        // Handle clipboard, IME, etc.
        if !output.platform_output.copied_text.is_empty() {
            #[cfg(target_os = "android")]
            crate::platform::android::copy_to_clipboard(&output.platform_output.copied_text);
            
            #[cfg(target_os = "ios")]
            crate::platform::ios::copy_to_clipboard(&output.platform_output.copied_text);
        }
    }
}