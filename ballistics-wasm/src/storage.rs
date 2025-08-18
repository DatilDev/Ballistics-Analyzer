use ballistics_core::{SavedCalculation, FirearmProfile};
use web_sys::{window, Storage};
use serde::{Serialize, Deserialize};

const STORAGE_KEY_CALCULATIONS: &str = "ballistics_calculations";
const STORAGE_KEY_PROFILES: &str = "ballistics_profiles";
const STORAGE_KEY_SETTINGS: &str = "ballistics_settings";

pub struct WebStorage;

impl WebStorage {
    fn get_local_storage() -> Option<Storage> {
        window()?.local_storage().ok()?
    }
    
    pub fn save_calculation(calc: &SavedCalculation) {
        if let Some(storage) = Self::get_local_storage() {
            let mut calculations = Self::load_all_calculations();
            
            // Update or add calculation
            if let Some(existing) = calculations.iter_mut().find(|c| c.id == calc.id) {
                *existing = calc.clone();
            } else {
                calculations.push(calc.clone());
            }
            
            // Save to localStorage
            match serde_json::to_string(&data) {
    Ok(json) => {
        let _ = storage.set_item(key, &json);
    }
    Err(e) => {
        web_sys::console::error_1(&format!("Serialization error: {}", e).into());
    }
}
        }
    }
    
    pub fn load_all_calculations() -> Vec<SavedCalculation> {
        if let Some(storage) = Self::get_local_storage() {
            if let Ok(Some(json)) = storage.get_item(STORAGE_KEY_CALCULATIONS) {
                if let Ok(calculations) = serde_json::from_str(&json) {
                    return calculations;
                }
            }
        }
        Vec::new()
    }
    
    pub fn delete_calculation(id: &str) {
        if let Some(storage) = Self::get_local_storage() {
            let mut calculations = Self::load_all_calculations();
            calculations.retain(|c| c.id != id);
            
            match serde_json::to_string(&data) {
    Ok(json) => {
        let _ = storage.set_item(key, &json);
    }
    Err(e) => {
        web_sys::console::error_1(&format!("Serialization error: {}", e).into());
    }
}
        }
    }
    
    pub fn save_profile(profile: &FirearmProfile) {
        if let Some(storage) = Self::get_local_storage() {
            let mut profiles = Self::load_profiles().unwrap_or_default();
            
            if let Some(existing) = profiles.iter_mut().find(|p| p.id == profile.id) {
                *existing = profile.clone();
            } else {
                profiles.push(profile.clone());
            }
            
            match serde_json::to_string(&data) {
    Ok(json) => {
        let _ = storage.set_item(key, &json);
    }
    Err(e) => {
        web_sys::console::error_1(&format!("Serialization error: {}", e).into());
    }
}
        }
    }
    
    pub fn load_profiles() -> Option<Vec<FirearmProfile>> {
        if let Some(storage) = Self::get_local_storage() {
            if let Ok(Some(json)) = storage.get_item(STORAGE_KEY_PROFILES) {
                if let Ok(profiles) = serde_json::from_str(&json) {
                    return Some(profiles);
                }
            }
        }
        None
    }
    
    pub fn delete_profile(id: &str) {
        if let Some(storage) = Self::get_local_storage() {
            let mut profiles = Self::load_profiles().unwrap_or_default();
            profiles.retain(|p| p.id != id);
            
           match serde_json::to_string(&data) {
    Ok(json) => {
        let _ = storage.set_item(key, &json);
    }
    Err(e) => {
        web_sys::console::error_1(&format!("Serialization error: {}", e).into());
    }
}
        }
    }
    
    pub fn clear_all() {
        if let Some(storage) = Self::get_local_storage() {
            let _ = storage.remove_item(STORAGE_KEY_CALCULATIONS);
            let _ = storage.remove_item(STORAGE_KEY_PROFILES);
        }
    }
    
    pub fn get_storage_size() -> usize {
        let mut size = 0;
        
        if let Some(storage) = Self::get_local_storage() {
            if let Ok(Some(calc_json)) = storage.get_item(STORAGE_KEY_CALCULATIONS) {
                size += calc_json.len();
            }
            if let Ok(Some(profile_json)) = storage.get_item(STORAGE_KEY_PROFILES) {
                size += profile_json.len();
            }
            if let Ok(Some(settings_json)) = storage.get_item(STORAGE_KEY_SETTINGS) {
                size += settings_json.len();
            }
        }
        
        size
    }
}

// Currenlty disabled to allow build,IndexedDB support for larger data and better performance
/*pub mod indexed_db {
    use wasm_bindgen::prelude::*;
    use wasm_bindgen::JsCast;
    use web_sys::{IdbDatabase, IdbObjectStore, IdbRequest};
    
    const DB_NAME: &str = "BallisticsAnalyzer";
    const DB_VERSION: u32 = 1;
    
    pub async fn open_database() -> Result<IdbDatabase, JsValue> {
        let window = web_sys::window().unwrap();
        let idb = window.indexed_db()?.unwrap();
        
        let request = idb.open_with_u32(DB_NAME, DB_VERSION)?;
        
        // Setup database schema on upgrade
        let on_upgrade = Closure::once(move |event: web_sys::Event| {
            let request = event.target().unwrap().dyn_into::<IdbRequest>().unwrap();
            let db = request.result().unwrap().dyn_into::<IdbDatabase>().unwrap();
            
            // Create object stores
            if !db.object_store_names().contains("calculations") {
                let store = db.create_object_store_with_optional_parameters(
                    "calculations",
                    &IdbObjectStoreParameters::new().key_path(Some(&JsValue::from_str("id")))
                )?;
                store.create_index("timestamp", &JsValue::from_str("timestamp"))?;
            }
            
            if !db.object_store_names().contains("profiles") {
                db.create_object_store_with_optional_parameters(
                    "profiles",
                    &IdbObjectStoreParameters::new().key_path(Some(&JsValue::from_str("id")))
                )?;
            }
            
            Ok(())
        });
        
        request.set_onupgradeneeded(Some(on_upgrade.as_ref().unchecked_ref()));
        on_upgrade.forget();
        
        // Wait for database to open
        JsFuture::from(request).await?;
        
        Ok(request.result()?.dyn_into::<IdbDatabase>()?)
    }
}
    */