use crate::models::{FirearmProfile, AmmunitionProfile};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Default, Serialize, Deserialize)]
pub struct FirearmProfileManager {
    pub profiles: Vec<FirearmProfile>,
    pub selected_profile_id: Option<String>,
}

impl FirearmProfileManager {
    pub fn new() -> Self {
        let mut manager = Self::default();
        manager.load_default_profiles();
        manager
    }
    
    pub fn load_default_profiles(&mut self) {
        self.profiles = vec![
            FirearmProfile {
                id: Uuid::new_v4().to_string(),
                name: "Remington 700 .308".to_string(),
                caliber: ".308 Winchester".to_string(),
                barrel_length: 24.0,
                barrel_twist: 10.0,
                sight_height: 1.5,
                zero_range: 100.0,
                ammunition: vec![
                    AmmunitionProfile {
                        id: Uuid::new_v4().to_string(),
                        name: "Federal Gold Medal Match 175gr".to_string(),
                        bullet_weight: 175.0,
                        bc: 0.505,
                        muzzle_velocity: 2600.0,
                        bullet_length: 1.24,
                    },
                ],
                notes: "Factory configuration".to_string(),
            },
            FirearmProfile {
                id: Uuid::new_v4().to_string(),
                name: "AR-15 5.56mm".to_string(),
                caliber: "5.56x45mm NATO".to_string(),
                barrel_length: 16.0,
                barrel_twist: 7.0,
                sight_height: 2.6,
                zero_range: 50.0,
                ammunition: vec![
                    AmmunitionProfile {
                        id: Uuid::new_v4().to_string(),
                        name: "M193 55gr FMJ".to_string(),
                        bullet_weight: 55.0,
                        bc: 0.243,
                        muzzle_velocity: 3240.0,
                        bullet_length: 0.755,
                    },
                ],
                notes: "Standard carbine".to_string(),
            },
        ];
    }
    
    pub fn add_profile(&mut self, profile: FirearmProfile) {
        self.profiles.push(profile);
    }
    
    pub fn remove_profile(&mut self, id: &str) {
        self.profiles.retain(|p| p.id != id);
        if Some(id.to_string()) == self.selected_profile_id {
            self.selected_profile_id = None;
        }
    }
    
    pub fn get_profile(&self, id: &str) -> Option<&FirearmProfile> {
        self.profiles.iter().find(|p| p.id == id)
    }
    
    pub fn get_selected_profile(&self) -> Option<&FirearmProfile> {
        self.selected_profile_id
            .as_ref()
            .and_then(|id| self.get_profile(id))
    }
    
    pub fn select_profile(&mut self, id: Option<String>) {
        self.selected_profile_id = id;
    }
}