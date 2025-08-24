use crate::models::FirearmProfile;
use uuid::Uuid;
use chrono::Utc;
use anyhow::Result;

pub struct ProfileManager {
    pub profiles: Vec<FirearmProfile>,
    pub selected_profile_id: Option<Uuid>,
}

impl ProfileManager {
    pub fn new() -> Self {
        Self {
            profiles: Vec::new(),
            selected_profile_id: None,
        }
    }
    
    pub fn add_profile(&mut self, profile: FirearmProfile) {
        if !self.profiles.iter().any(|p| p.id == profile.id) {
            self.profiles.push(profile);
        }
    }
    
    pub fn create_profile(&mut self, name: String, caliber: String) -> Result<FirearmProfile> {
        let profile = FirearmProfile {
            id: Uuid::new_v4(),
            name,
            caliber,
            barrel_length: 24.0,
            twist_rate: 10.0,
            sight_height: 1.5,
            zero_distance: 100.0,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        
        self.profiles.push(profile.clone());
        Ok(profile)
    }
    
    pub fn get_profile(&self, id: &Uuid) -> Option<&FirearmProfile> {
        self.profiles.iter().find(|p| p.id == *id)
    }
    
    pub fn list_profiles(&self) -> &[FirearmProfile] {
        &self.profiles
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        Self::new()
    }
}