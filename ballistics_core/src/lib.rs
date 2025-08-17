#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Core modules that are platform-agnostic
pub mod ballistics;
pub mod models;
pub mod firearm_profiles;
pub mod load_data;

// Optional feature-gated modules
#[cfg(feature = "storage")]
pub mod storage;

#[cfg(feature = "nostr")]
pub mod auth;

#[cfg(feature = "nostr")]
pub mod sharing;

// Re-export commonly used types
pub use models::*;
pub use ballistics::*;
pub use firearm_profiles::*;
pub use load_data::*;

#[cfg(feature = "storage")]
pub use storage::StorageBackend;

#[cfg(feature = "nostr")]
pub use auth::NostrAuth;

#[cfg(feature = "nostr")]
pub use sharing::SharingManager;