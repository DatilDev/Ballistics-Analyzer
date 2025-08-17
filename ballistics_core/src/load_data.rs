use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoadData {
    pub caliber: String,
    pub bullet_weight: f64,
    pub powder: String,
    pub charge_weight: f64,
    pub coal: f64, // Cartridge Overall Length
    pub velocity: f64,
    pub pressure: Option<f64>,
    pub notes: String,
}

pub struct LoadDataLibrary {
    data: HashMap<String, Vec<LoadData>>,
}

impl LoadDataLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            data: HashMap::new(),
        };
        library.load_default_data();
        library
    }
    
    fn load_default_data(&mut self) {
        // .308 Winchester loads
        self.data.insert(
            ".308 Winchester".to_string(),
            vec![
                LoadData {
                    caliber: ".308 Winchester".to_string(),
                    bullet_weight: 168.0,
                    powder: "Varget".to_string(),
                    charge_weight: 44.5,
                    coal: 2.810,
                    velocity: 2680.0,
                    pressure: Some(58000.0),
                    notes: "Match load".to_string(),
                },
                LoadData {
                    caliber: ".308 Winchester".to_string(),
                    bullet_weight: 175.0,
                    powder: "IMR 4064".to_string(),
                    charge_weight: 42.0,
                    coal: 2.810,
                    velocity: 2600.0,
                    pressure: Some(57000.0),
                    notes: "Long range load".to_string(),
                },
            ],
        );
        
        // 6.5 Creedmoor loads
        self.data.insert(
            "6.5 Creedmoor".to_string(),
            vec![
                LoadData {
                    caliber: "6.5 Creedmoor".to_string(),
                    bullet_weight: 140.0,
                    powder: "H4350".to_string(),
                    charge_weight: 41.5,
                    coal: 2.810,
                    velocity: 2710.0,
                    pressure: Some(59000.0),
                    notes: "Standard match load".to_string(),
                },
            ],
        );
    }
    
    pub fn get_loads_for_caliber(&self, caliber: &str) -> Vec<LoadData> {
        self.data.get(caliber).cloned().unwrap_or_default()
    }
    
    pub fn add_load(&mut self, load: LoadData) {
        self.data
            .entry(load.caliber.clone())
            .or_insert_with(Vec::new)
            .push(load);
    }
    
    pub fn get_all_calibers(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
}

impl Default for LoadDataLibrary {
    fn default() -> Self {
        Self::new()
    }
}