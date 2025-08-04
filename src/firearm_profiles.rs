use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum FirearmType {
    Rifle,
    Pistol,
    Shotgun,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct FirearmProfile {
    pub id: String,
    pub name: String,
    pub firearm_type: FirearmType,
    pub manufacturer: String,
    pub model: String,
    pub caliber: String,
    pub barrel_length: f64,
    pub twist_rate: String,
    pub sight_height: f64,
    pub notes: String,
}

impl Default for FirearmProfile {
    fn default() -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: "New Profile".to_string(),
            firearm_type: FirearmType::Rifle,
            manufacturer: String::new(),
            model: String::new(),
            caliber: String::new(),
            barrel_length: 20.0,
            twist_rate: "1:10".to_string(),
            sight_height: 1.5,
            notes: String::new(),
        }
    }
}