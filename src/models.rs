// src/models.rs - Shared type definitions
use serde::{Deserialize, Serialize};
use crate::ballistics::{TrajectoryResult, ProjectileData};
use crate::hardware::{RangefinderData, WeatherData};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AttachedImage {
    pub id: String,           // logical id or filename
    pub mime: String,         // "image/png", "image/jpeg"
    pub bytes: Vec<u8>,       // raw image bytes
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