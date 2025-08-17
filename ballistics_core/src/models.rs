use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SavedCalculation {
    pub id: String,
    pub timestamp: DateTime<Utc>,
    pub name: String,
    pub data: CalculationData,
    pub photos: Vec<AttachedImage>,
    pub notes: String,
    pub weather: Option<WeatherData>,
    pub firearm_profile_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationData {
    // Firearm data
    pub caliber: String,
    pub bullet_weight: f64,
    pub bc: f64,
    pub muzzle_velocity: f64,
    pub sight_height: f64,
    pub zero_range: f64,
    pub barrel_twist: f64,
    pub bullet_length: f64,
    
    // Environmental conditions
    pub temperature: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub altitude: f64,
    pub wind_speed: f64,
    pub wind_angle: f64,
    
    // Shooting conditions
    pub shooting_angle: f64,
    pub latitude: f64,
    pub azimuth: f64,
    
    // Target
    pub target_distance: f64,
    pub target_speed: f64,
    pub target_angle: f64,
    
    // Results
    pub trajectory: Vec<TrajectoryPoint>,
    pub max_range: f64,
    pub max_ordinate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub distance: f64,
    pub drop: f64,
    pub drop_moa: f64,
    pub drop_mil: f64,
    pub windage: f64,
    pub windage_moa: f64,
    pub windage_mil: f64,
    pub velocity: f64,
    pub energy: f64,
    pub time: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttachedImage {
    pub id: String,
    pub data: String, // Base64 encoded
    pub caption: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeatherData {
    pub temperature: f64,
    pub pressure: f64,
    pub humidity: f64,
    pub wind_speed: f64,
    pub wind_angle: f64,
    pub density_altitude: f64,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FirearmProfile {
    pub id: String,
    pub name: String,
    pub caliber: String,
    pub barrel_length: f64,
    pub barrel_twist: f64,
    pub sight_height: f64,
    pub zero_range: f64,
    pub ammunition: Vec<AmmunitionProfile>,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AmmunitionProfile {
    pub id: String,
    pub name: String,
    pub bullet_weight: f64,
    pub bc: f64,
    pub muzzle_velocity: f64,
    pub bullet_length: f64,
}

impl Default for SavedCalculation {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            timestamp: Utc::now(),
            name: String::new(),
            data: CalculationData::default(),
            photos: Vec::new(),
            notes: String::new(),
            weather: None,
            firearm_profile_id: None,
        }
    }
}

impl Default for CalculationData {
    fn default() -> Self {
        Self {
            caliber: ".308 Winchester".to_string(),
            bullet_weight: 175.0,
            bc: 0.505,
            muzzle_velocity: 2600.0,
            sight_height: 1.5,
            zero_range: 100.0,
            barrel_twist: 10.0,
            bullet_length: 1.24,
            temperature: 59.0,
            pressure: 29.92,
            humidity: 50.0,
            altitude: 0.0,
            wind_speed: 10.0,
            wind_angle: 90.0,
            shooting_angle: 0.0,
            latitude: 45.0,
            azimuth: 0.0,
            target_distance: 1000.0,
            target_speed: 0.0,
            target_angle: 0.0,
            trajectory: Vec::new(),
            max_range: 0.0,
            max_ordinate: 0.0,
        }
    }
}