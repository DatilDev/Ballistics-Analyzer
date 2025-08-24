use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// Don't declare modules here - they're declared in lib.rs
// Remove all the "pub mod" statements

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirearmProfile {
    pub id: Uuid,
    pub name: String,
    pub caliber: String,
    pub barrel_length: f64,
    pub twist_rate: f64,
    pub sight_height: f64,
    pub zero_distance: f64,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionProfile {
    pub id: Uuid,
    pub name: String,
    pub bullet_weight: f64,
    pub muzzle_velocity: f64,
    pub ballistic_coefficient: f64,
    pub drag_model: DragModel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DragModel {
    G1,
    G7,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationData {
    pub firearm: FirearmProfile,
    pub ammunition: AmmunitionProfile,
    pub environment: EnvironmentalConditions,
    pub wind: WindConditions,
    pub target_distance: f64,
    pub(crate) firearm_name: (),
    pub(crate) ammo_name: (),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentalConditions {
    pub temperature: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub altitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindConditions {
    pub speed: f64,
    pub direction: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub distance: f64,
    pub drop: f64,
    pub drift: f64,
    pub velocity: f64,
    pub energy: f64,
    pub time: f64,
}