use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Serialize, Deserialize)]
pub struct LoadData {
    pub manufacturer: String,
    pub name: String,
    pub caliber: String,
    pub bullet_weight: f64,
    pub velocity: f64,
    pub bc: f64,
    pub powder_type: String,
    pub powder_charge: f64,
}

pub struct LoadDataLibrary {
    pub selected_manufacturer: String,
    loads: HashMap<String, Vec<LoadData>>,
}

impl LoadDataLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            selected_manufacturer: "Federal".to_string(),
            loads: HashMap::new(),
        };
        library.initialize_data();
        library
    }
}

impl Default for LoadDataLibrary {
    fn default() -> Self {
        Self::new()
    }
}

impl LoadDataLibrary {
    fn initialize_data(&mut self) {
        self.loads.insert(
            "Federal".to_string(),
            vec![
                LoadData {
                    manufacturer: "Federal".to_string(),
                    name: "Gold Medal Match 308 Win".to_string(),
                    caliber: ".308 Winchester".to_string(),
                    bullet_weight: 175.0,
                    velocity: 2600.0,
                    bc: 0.505,
                    powder_type: "IMR 4064".to_string(),
                    powder_charge: 42.5,
                },
                LoadData {
                    manufacturer: "Federal".to_string(),
                    name: "Premium 6.5 Creedmoor".to_string(),
                    caliber: "6.5 Creedmoor".to_string(),
                    bullet_weight: 140.0,
                    velocity: 2750.0,
                    bc: 0.610,
                    powder_type: "H4350".to_string(),
                    powder_charge: 41.5,
                },
            ],
        );

        self.loads.insert(
            "Hornady".to_string(),
            vec![
                LoadData {
                    manufacturer: "Hornady".to_string(),
                    name: "Match 6.5 Creedmoor".to_string(),
                    caliber: "6.5 Creedmoor".to_string(),
                    bullet_weight: 147.0,
                    velocity: 2695.0,
                    bc: 0.697,
                    powder_type: "H4350".to_string(),
                    powder_charge: 40.8,
                },
                LoadData {
                    manufacturer: "Hornady".to_string(),
                    name: "Precision Hunter 300 Win Mag".to_string(),
                    caliber: ".300 Winchester Magnum".to_string(),
                    bullet_weight: 200.0,
                    velocity: 2850.0,
                    bc: 0.597,
                    powder_type: "H1000".to_string(),
                    powder_charge: 72.0,
                },
            ],
        );
    }

    pub fn get_manufacturers(&self) -> Vec<String> {
        self.loads.keys().cloned().collect()
    }

    pub fn get_loads_for_manufacturer(&self, manufacturer: &str) -> Option<Vec<LoadData>> {
        self.loads.get(manufacturer).cloned()
    }
}