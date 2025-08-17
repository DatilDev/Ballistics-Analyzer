#[cfg(target_os = "android")]
pub mod android;

#[cfg(target_os = "ios")]
pub mod ios;

use std::sync::Mutex;

// Platform service trait for abstraction
pub trait PlatformService {
    fn get_storage_path(&self) -> String;
    fn request_location_permission(&self) -> bool;
    fn request_camera_permission(&self) -> bool;
    fn get_device_info(&self) -> DeviceInfo;
    fn vibrate(&self, duration_ms: u32);
    fn keep_screen_on(&self, enable: bool);
}

#[derive(Debug, Clone)]
pub struct DeviceInfo {
    pub model: String,
    pub os_version: String,
    pub screen_width: u32,
    pub screen_height: u32,
    pub scale_factor: f32,
}

// Platform-specific service instance
static PLATFORM_SERVICE: Mutex<Option<Box<dyn PlatformService + Send + Sync>>> = Mutex::new(None);

pub fn set_platform_service(service: Box<dyn PlatformService + Send + Sync>) {
    *PLATFORM_SERVICE.lock().unwrap() = Some(service);
}

pub fn get_platform_service() -> Option<Box<dyn PlatformService + Send + Sync>> {
    PLATFORM_SERVICE.lock().unwrap().clone()
}

// Common platform utilities
pub fn format_file_size(bytes: usize) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    format!("{:.2} {}", size, UNITS[unit_index])
}

pub fn get_timestamp() -> String {
    chrono::Local::now().format("%Y%m%d_%H%M%S").to_string()
}