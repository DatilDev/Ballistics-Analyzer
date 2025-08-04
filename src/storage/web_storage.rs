#[cfg(target_arch = "wasm32")]
pub mod web_storage {
    use web_sys::{window, Storage};
    use serde::{Serialize, Deserialize};
    use crate::{SavedCalculation, FirearmProfile};
    
    pub struct WebStorage {
        storage: Storage,
        user_prefix: String,
    }
    
    impl WebStorage {
        pub fn new(user_pubkey: &str) -> Option<Self> {
            let window = window()?;
            let storage = window.local_storage().ok()??;
            
            Some(Self {
                storage,
                user_prefix: format!("ballistics_{}_", &user_pubkey[..8]),
            })
        }
        
        pub fn save_calculation(&self, calc: &SavedCalculation) -> Result<(), JsValue> {
            let key = format!("{}calc_{}", self.user_prefix, calc.id);
            let value = serde_json::to_string(calc).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.storage.set_item(&key, &value)
        }
        
        pub fn load_calculations(&self) -> Vec<SavedCalculation> {
            let mut calculations = Vec::new();
            let prefix = format!("{}calc_", self.user_prefix);
            
            for i in 0..self.storage.length().unwrap_or(0) {
                if let Ok(Some(key)) = self.storage.key(i) {
                    if key.starts_with(&prefix) {
                        if let Ok(Some(value)) = self.storage.get_item(&key) {
                            if let Ok(calc) = serde_json::from_str::<SavedCalculation>(&value) {
                                calculations.push(calc);
                            }
                        }
                    }
                }
            }
            
            calculations.sort_by(|a, b| b.calculation.timestamp.cmp(&a.calculation.timestamp));
            calculations
        }
        
        pub fn save_profiles(&self, profiles: &[FirearmProfile]) -> Result<(), JsValue> {
            let key = format!("{}profiles", self.user_prefix);
            let value = serde_json::to_string(profiles).map_err(|e| JsValue::from_str(&e.to_string()))?;
            self.storage.set_item(&key, &value)
        }
        
        pub fn load_profiles(&self) -> Vec<FirearmProfile> {
            let key = format!("{}profiles", self.user_prefix);
            
            if let Ok(Some(value)) = self.storage.get_item(&key) {
                if let Ok(profiles) = serde_json::from_str::<Vec<FirearmProfile>>(&value) {
                    return profiles;
                }
            }
            
            Vec::new()
        }
        
        pub fn save_image(&self, id: &str, data: &[u8]) -> Result<(), JsValue> {
            let key = format!("{}img_{}", self.user_prefix, id);
            let value = base64::encode(data);
            self.storage.set_item(&key, &value)
        }
        
        pub fn load_image(&self, id: &str) -> Option<Vec<u8>> {
            let key = format!("{}img_{}", self.user_prefix, id);
            
            if let Ok(Some(value)) = self.storage.get_item(&key) {
                if let Ok(data) = base64::decode(&value) {
                    return Some(data);
                }
            }
            
            None
        }
    }
}