use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ClientSettings {
    pub use_local_relay: bool,
    pub local_relay_addr: String,   // e.g., "127.0.0.1:7777"
    pub broadcast_enabled: bool,
    pub extra_relays: Vec<String>,  // e.g., ["wss://relay.damus.io"]
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct AttachedImage {
    pub id: String,           // logical id or filename
    pub mime: String,         // "image/png", "image/jpeg"
    pub bytes: Vec<u8>,       // raw image bytes
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct TrajectoryResult {
    pub range_m: f32,
    pub drop_mrad: f32,
    pub wind_mrad: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ProjectileData {
    pub caliber: String,      // e.g., "6.5 Creedmoor"
    pub mass_gr: f32,         // grain
    pub bc_g1: f32,           // ballistic coefficient
    pub muzzle_vel_fps: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Calculation {
    pub projectile_data: ProjectileData,
    pub result: Option<TrajectoryResult>,
    pub notes: Option<String>,
    pub images: Vec<AttachedImage>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct SavedCalculation {
    pub id: String,                 // UUID or hash
    pub created_epoch: i64,         // seconds since epoch
    pub title: String,
    pub calculation: Calculation,
}

pub struct BallisticsApp {
    pub settings: ClientSettings,
    pub items: Vec<SavedCalculation>,
}

impl Default for BallisticsApp {
    fn default() -> Self {
        Self {
            settings: ClientSettings {
                use_local_relay: true,
                local_relay_addr: "127.0.0.1:7777".to_string(),
                broadcast_enabled: false,
                extra_relays: vec![],
            },
            items: vec![],
        }
    }
}