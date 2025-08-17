use crate::models::{SavedCalculation, FirearmProfile};
use anyhow::Result;
use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use rusqlite::{Connection, params};

pub trait StorageBackend: Send + Sync {
    fn save_calculation(&mut self, calc: &SavedCalculation) -> Result<()>;
    fn load_calculation(&self, id: &str) -> Result<SavedCalculation>;
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>>;
    fn delete_calculation(&mut self, id: &str) -> Result<()>;
    
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()>;
    fn load_profile(&self, id: &str) -> Result<FirearmProfile>;
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>>;
    fn delete_profile(&mut self, id: &str) -> Result<()>;
}

// Default implementation for desktop platforms
#[cfg(not(target_arch = "wasm32"))]
pub struct SqliteStorage {
    conn: Arc<Mutex<Connection>>,
}

#[cfg(not(target_arch = "wasm32"))]
impl SqliteStorage {
    pub fn new(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        
        // Create tables
        conn.execute(
            "CREATE TABLE IF NOT EXISTS calculations (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                notes TEXT,
                firearm_profile_id TEXT
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS firearm_profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { 
            conn: Arc::new(Mutex::new(conn))
        })
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl StorageBackend for SqliteStorage {
    fn save_calculation(&mut self, calc: &SavedCalculation) -> Result<()> {
        let data = serde_json::to_string(calc)?;
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO calculations (id, timestamp, name, data, notes, firearm_profile_id) 
             VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![
                calc.id,
                calc.timestamp.to_rfc3339(),
                calc.name,
                data,
                calc.notes,
                calc.firearm_profile_id
            ],
        )?;
        
        Ok(())
    }
    
    fn load_calculation(&self, id: &str) -> Result<SavedCalculation> {
        let conn = self.conn.lock().unwrap();
        let data: String = conn.query_row(
            "SELECT data FROM calculations WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        
        Ok(serde_json::from_str(&data)?)
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT data FROM calculations ORDER BY timestamp DESC"
        )?;
        
        let calculations = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(serde_json::from_str(&data).unwrap())
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(calculations)
    }
    
    fn delete_calculation(&mut self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM calculations WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
    
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()> {
        let data = serde_json::to_string(profile)?;
        let conn = self.conn.lock().unwrap();
        
        conn.execute(
            "INSERT OR REPLACE INTO firearm_profiles (id, name, data) VALUES (?1, ?2, ?3)",
            params![profile.id, profile.name, data],
        )?;
        
        Ok(())
    }
    
    fn load_profile(&self, id: &str) -> Result<FirearmProfile> {
        let conn = self.conn.lock().unwrap();
        let data: String = conn.query_row(
            "SELECT data FROM firearm_profiles WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        
        Ok(serde_json::from_str(&data)?)
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        let conn = self.conn.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT data FROM firearm_profiles ORDER BY name"
        )?;
        
        let profiles = stmt
            .query_map([], |row| {
                let data: String = row.get(0)?;
                Ok(serde_json::from_str(&data).unwrap())
            })?
            .collect::<Result<Vec<_>, _>>()?;
        
        Ok(profiles)
    }
    
    fn delete_profile(&mut self, id: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM firearm_profiles WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}

// WASM implementation stub
#[cfg(target_arch = "wasm32")]
pub struct IndexedDbStorage;

#[cfg(target_arch = "wasm32")]
impl StorageBackend for IndexedDbStorage {
    fn save_calculation(&mut self, _calc: &SavedCalculation) -> Result<()> {
        Ok(())
    }
    
    fn load_calculation(&self, _id: &str) -> Result<SavedCalculation> {
        unimplemented!("IndexedDB storage for WASM")
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        Ok(Vec::new())
    }
    
    fn delete_calculation(&mut self, _id: &str) -> Result<()> {
        Ok(())
    }
    
    fn save_profile(&mut self, _profile: &FirearmProfile) -> Result<()> {
        Ok(())
    }
    
    fn load_profile(&self, _id: &str) -> Result<FirearmProfile> {
        unimplemented!("IndexedDB storage for WASM")
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        Ok(Vec::new())
    }
    
    fn delete_profile(&mut self, _id: &str) -> Result<()> {
        Ok(())
    }
}
