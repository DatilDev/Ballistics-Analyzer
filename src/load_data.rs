use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{params, Connection};

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
    custom_loads: Vec<LoadData>,
    #[cfg(not(target_arch = "wasm32"))]
    db_connection: Option<Connection>,
}

impl LoadDataLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            selected_manufacturer: "Federal".to_string(),
            loads: HashMap::new(),
            custom_loads: Vec::new(),
            #[cfg(not(target_arch = "wasm32"))]
            db_connection: None,
        };
        
        // Initialize database on desktop
        #[cfg(not(target_arch = "wasm32"))]
        {
            library.init_database();
            library.load_from_database();
        }
        
        // For WASM, load embedded data
        #[cfg(target_arch = "wasm32")]
        {
            library.load_embedded_data();
        }
        
        library.load_custom_loads();
        library
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    fn init_database(&mut self) {
        // Try to open or create the database
        let db_path = self.get_db_path();
        
        match Connection::open(&db_path) {
            Ok(conn) => {
                // Check if tables exist, if not, initialize
                if !self.tables_exist(&conn) {
                    self.create_and_populate_database(&conn);
                }
                self.db_connection = Some(conn);
            }
            Err(e) => {
                eprintln!("Failed to open database: {}", e);
                // Fall back to embedded data
                self.load_embedded_data();
            }
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    fn get_db_path(&self) -> PathBuf {
        // Get the data directory for the application
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("ballistics_analyzer");
        let _ = std::fs::create_dir_all(&path);
        path.push("ammo_data.db");
        path
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    fn tables_exist(&self, conn: &Connection) -> bool {
        let query = "SELECT name FROM sqlite_master WHERE type='table' AND name='load_data'";
        match conn.query_row(query, [], |_| Ok(())) {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    #[cfg(not(target_arch = "wasm32"))]
    fn create_and_populate_database(&self, conn: &Connection) {
        // Read and execute the SQL initialization script
        let sql = include_str!("../migrations/init_ammo_db.sql");
        if let Err(e) = conn.execute_batch(sql) {
            eprintln!("Failed to initialize database: {}", e);
        }
    }
    
   #[cfg(not(target_arch = "wasm32"))]
fn load_from_database(&mut self) {
    // Take ownership temporarily to avoid borrow checker issues
    let conn = match self.db_connection.take() {
        Some(c) => c,
        None => return,
    };
    
    let query = "
        SELECT m.name, l.name, l.caliber, l.bullet_weight, 
               l.velocity, l.bc, l.powder_type, l.powder_charge
        FROM load_data l
        JOIN manufacturers m ON l.manufacturer_id = m.id
        ORDER BY m.name, l.caliber, l.bullet_weight
    ";
    
    let mut loaded_successfully = false;
    
    match conn.prepare(query) {
        Ok(mut stmt) => {
            let load_iter = stmt.query_map([], |row| {
                Ok(LoadData {
                    manufacturer: row.get(0)?,
                    name: row.get(1)?,
                    caliber: row.get(2)?,
                    bullet_weight: row.get(3)?,
                    velocity: row.get(4)?,
                    bc: row.get(5)?,
                    powder_type: row.get(6)?,
                    powder_charge: row.get(7)?,
                })
            });
            
            if let Ok(loads) = load_iter {
                for load in loads.flatten() {
                    self.loads
                        .entry(load.manufacturer.clone())
                        .or_insert_with(Vec::new)
                        .push(load);
                }
                loaded_successfully = true;
            }
        }
        Err(e) => {
            eprintln!("Failed to load ammunition data: {}", e);
        }
    }
    
    // Restore the connection
    self.db_connection = Some(conn);
    
    // Load embedded data if database loading failed
    if !loaded_successfully {
        self.load_embedded_data();
    }
}
    
    // Fallback embedded data for WASM or database failure
    fn load_embedded_data(&mut self) {
        // Basic embedded data as fallback
        self.loads.insert(
            "Federal".to_string(),
            vec![
                LoadData {
                    manufacturer: "Federal".to_string(),
                    name: "Gold Medal Match 308 Win 175gr".to_string(),
                    caliber: ".308 Winchester".to_string(),
                    bullet_weight: 175.0,
                    velocity: 2600.0,
                    bc: 0.505,
                    powder_type: "IMR 4064".to_string(),
                    powder_charge: 42.5,
                },
                LoadData {
                    manufacturer: "Federal".to_string(),
                    name: "Premium 6.5 Creedmoor 140gr".to_string(),
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
                    name: "Match 6.5 Creedmoor 147gr ELD-M".to_string(),
                    caliber: "6.5 Creedmoor".to_string(),
                    bullet_weight: 147.0,
                    velocity: 2695.0,
                    bc: 0.697,
                    powder_type: "H4350".to_string(),
                    powder_charge: 40.8,
                },
            ],
        );
    }
    
    // Load custom user-defined loads
    fn load_custom_loads(&mut self) {
        // Custom loads stored locally in the code
        self.custom_loads = vec![
            LoadData {
                manufacturer: "Custom".to_string(),
                name: "My 308 Match Load".to_string(),
                caliber: ".308 Winchester".to_string(),
                bullet_weight: 168.0,
                velocity: 2650.0,
                bc: 0.462,
                powder_type: "Varget".to_string(),
                powder_charge: 44.5,
            },
            LoadData {
                manufacturer: "Custom".to_string(),
                name: "My 6.5 CM Hunting Load".to_string(),
                caliber: "6.5 Creedmoor".to_string(),
                bullet_weight: 143.0,
                velocity: 2700.0,
                bc: 0.625,
                powder_type: "H4350".to_string(),
                powder_charge: 40.5,
            },
        ];
        
        // Add custom loads to the main collection
        self.loads.insert("Custom".to_string(), self.custom_loads.clone());
    }
    
    pub fn get_manufacturers(&self) -> Vec<String> {
        let mut manufacturers: Vec<String> = self.loads.keys().cloned().collect();
        manufacturers.sort();
        manufacturers
    }
    
    pub fn get_loads_for_manufacturer(&self, manufacturer: &str) -> Option<Vec<LoadData>> {
        self.loads.get(manufacturer).cloned()
    }
    
    pub fn get_loads_by_caliber(&self, caliber: &str) -> Vec<LoadData> {
        self.loads
            .values()
            .flatten()
            .filter(|load| load.caliber == caliber)
            .cloned()
            .collect()
    }
    
    pub fn add_custom_load(&mut self, load: LoadData) {
        self.custom_loads.push(load.clone());
        self.loads
            .entry("Custom".to_string())
            .or_insert_with(Vec::new)
            .push(load);
    }
    
    pub fn remove_custom_load(&mut self, index: usize) {
        if index < self.custom_loads.len() {
            self.custom_loads.remove(index);
            self.loads.insert("Custom".to_string(), self.custom_loads.clone());
        }
    }
    
  #[cfg(not(target_arch = "wasm32"))]
pub fn search_loads(&self, query: &str) -> Vec<LoadData> {
    let mut results = Vec::new();
    
    if let Some(ref conn) = self.db_connection {
        let sql = "
            SELECT m.name, l.name, l.caliber, l.bullet_weight, 
                   l.velocity, l.bc, l.powder_type, l.powder_charge
            FROM load_data l
            JOIN manufacturers m ON l.manufacturer_id = m.id
            WHERE l.name LIKE ?1 OR l.caliber LIKE ?1 OR m.name LIKE ?1
            ORDER BY m.name, l.caliber, l.bullet_weight
        ";
        
        let query_pattern = format!("%{}%", query);
        
        if let Ok(mut stmt) = conn.prepare(sql) {
            if let Ok(load_iter) = stmt.query_map([query_pattern], |row| {
                Ok(LoadData {
                    manufacturer: row.get(0)?,
                    name: row.get(1)?,
                    caliber: row.get(2)?,
                    bullet_weight: row.get(3)?,
                    velocity: row.get(4)?,
                    bc: row.get(5)?,
                    powder_type: row.get(6)?,
                    powder_charge: row.get(7)?,
                })
            }) {
                for load in load_iter.flatten() {
                    results.push(load);
                }
            }
        }
    }
    
    results
}
}

impl Default for LoadDataLibrary {
    fn default() -> Self {
        Self::new()
    }
}