use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Don't declare modules here - remove all "pub mod" statements

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadData {
    pub id: String,
    pub name: String,
    pub powder: PowderData,
    pub charge_weight: f64,
    pub oal: f64, // Overall length
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PowderData {
    pub manufacturer: String,
    pub name: String,
    pub burn_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FactoryAmmunition {
    pub manufacturer: String,
    pub product_line: String,
    pub caliber: String,
    pub bullet_weight: f64,
    pub muzzle_velocity: f64,
    pub ballistic_coefficient: f64,
}

// Factory ammo database
pub fn get_factory_ammo_database() -> HashMap<String, Vec<FactoryAmmunition>> {
    let mut db = HashMap::new();
    
    // Add common factory loads
    db.insert("308 Winchester".to_string(), vec![
        FactoryAmmunition {
            manufacturer: "Federal".to_string(),
            product_line: "Gold Medal Match".to_string(),
            caliber: "308 Winchester".to_string(),
            bullet_weight: 168.0,
            muzzle_velocity: 2650.0,
            ballistic_coefficient: 0.462,
        },
    ]);
    
    db
}