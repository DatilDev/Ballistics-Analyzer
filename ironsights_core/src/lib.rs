//! Ballistics Core Library
//! Core calculations and data structures for ballistics analysis

// Existing modules
pub mod ballistics;
pub mod models;
pub mod load_data;
pub mod firearm_profiles;

// Missing modules - create these
pub mod calculation;
pub mod data_card;
pub mod export;
pub mod units;
pub mod utils;

// Optional modules based on features
#[cfg(feature = "storage")]
pub mod storage;

#[cfg(feature = "hardware")]
pub mod hardware;

#[cfg(feature = "gui")]
pub mod gui;

#[cfg(feature = "nostr")]
pub mod nostr_integration;

// Re-exports
pub use ballistics::*;
pub use models::*;
pub use load_data::*;
pub use firearm_profiles::*;
pub use units::*;
pub use utils::*;

#[cfg(feature = "storage")]
pub use storage::{StorageBackend, StorageManager, StorageEntry};

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the library
pub fn init() {
    // Logging is optional
    #[cfg(all(not(target_arch = "wasm32"), feature = "logging"))]
    {
        let _ = env_logger::try_init();
    }
}