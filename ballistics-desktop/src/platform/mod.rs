// Platform-specific implementations

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "macos")]
pub mod macos;

#[cfg(target_os = "android")]
pub mod android;

// Default platform config
pub struct PlatformConfig {
    pub name: &'static str,
    pub supports_hardware: bool,
}

impl PlatformConfig {
    pub fn current() -> Self {
        #[cfg(target_os = "linux")]
        {
            Self {
                name: "Linux",
                supports_hardware: cfg!(feature = "hardware"),
            }
        }
        
        #[cfg(target_os = "windows")]
        {
            Self {
                name: "Windows",
                supports_hardware: false,
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            Self {
                name: "macOS",
                supports_hardware: false,
            }
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "windows", target_os = "macos")))]
        {
            Self {
                name: "Unknown",
                supports_hardware: false,
            }
        }
    }
}