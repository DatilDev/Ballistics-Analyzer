use ballistics_core::{StorageBackend, SavedCalculation, FirearmProfile};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;

pub struct MobileStorage {
    base_path: PathBuf,
}

impl MobileStorage {
    pub fn new(base_path: &str) -> Result<Self> {
        let path = PathBuf::from(base_path);
        
        // Create directories if they don't exist
        fs::create_dir_all(&path.join("calculations"))?;
        fs::create_dir_all(&path.join("profiles"))?;
        
        Ok(Self { base_path: path })
    }
    
    fn calculations_path(&self) -> PathBuf {
        self.base_path.join("calculations")
    }
    
    fn profiles_path(&self) -> PathBuf {
        self.base_path.join("profiles")
    }
}

impl StorageBackend for MobileStorage {
    fn save_calculation(&mut self, calc: &SavedCalculation) -> Result<()> {
        let path = self.calculations_path().join(format!("{}.json", calc.id));
        let json = serde_json::to_string_pretty(calc)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    fn load_calculation(&self, id: &str) -> Result<SavedCalculation> {
        let path = self.calculations_path().join(format!("{}.json", id));
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        let mut calculations = Vec::new();
        
        for entry in fs::read_dir(self.calculations_path())? {
            let entry = entry?;
            if entry.path().extension() == Some("json".as_ref()) {
                let json = fs::read_to_string(entry.path())?;
                if let Ok(calc) = serde_json::from_str(&json) {
                    calculations.push(calc);
                }
            }
        }
        
        calculations.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(calculations)
    }
    
    fn delete_calculation(&mut self, id: &str) -> Result<()> {
        let path = self.calculations_path().join(format!("{}.json", id));
        fs::remove_file(path)?;
        Ok(())
    }
    
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()> {
        let path = self.profiles_path().join(format!("{}.json", profile.id));
        let json = serde_json::to_string_pretty(profile)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    fn load_profile(&self, id: &str) -> Result<FirearmProfile> {
        let path = self.profiles_path().join(format!("{}.json", id));
        let json = fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        let mut profiles = Vec::new();
        
        for entry in fs::read_dir(self.profiles_path())? {
            let entry = entry?;
            if entry.path().extension() == Some("json".as_ref()) {
                let json = fs::read_to_string(entry.path())?;
                if let Ok(profile) = serde_json::from_str(&json) {
                    profiles.push(profile);
                }
            }
        }
        
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(profiles)
    }
    
    fn delete_profile(&mut self, id: &str) -> Result<()> {
        let path = self.profiles_path().join(format!("{}.json", id));
        fs::remove_file(path)?;
        Ok(())
    }
}