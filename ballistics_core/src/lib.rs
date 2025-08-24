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

// Sharing module (always include since file exists)
pub mod sharing;

// Re-export commonly used types from models
pub use models::{
    FirearmProfile, 
    AmmunitionProfile, 
    CalculationData, 
    TrajectoryPoint,
    EnvironmentalConditions,
    WindConditions,
    DragModel,
};

// Re-export from ballistics
pub use ballistics::{
    BallisticsCalculator,
    atmospheric_correction,
};

// Re-export from firearm_profiles  
pub use firearm_profiles::ProfileManager as FirearmProfileManager;

// Re-export from load_data
pub use load_data::{
    LoadData,
    PowderData,
    FactoryAmmunition,
    get_factory_ammo_database as LoadDataLibrary,
};

// Storage exports
#[cfg(feature = "storage")]
pub use storage::StorageBackend;

// Create a SavedCalculation type that main.rs expects
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCalculation {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub data: CalculationData,
    pub photos: Vec<String>,
    pub notes: String,
    pub weather: Option<EnvironmentalConditions>,
    pub firearm_profile_id: Option<uuid::Uuid>,
}

// Auth exports
#[cfg(feature = "nostr")]
pub use auth::NostrAuth;

// Sharing exports
pub use sharing::{SharingManager, SharedData};