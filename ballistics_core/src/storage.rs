use anyhow::Result;
use rusqlite::{Connection, params};
use std::path::Path;
use crate::{SavedCalculation, FirearmProfile};

// Trait for storage backends
pub trait StorageBackend: Send + Sync {
    fn save_calculation(&mut self, calculation: &SavedCalculation) -> Result<()>;
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>>;
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>>;
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()>;
}

pub struct SqliteStorage {
    connection: Connection,
}

impl SqliteStorage {
    pub fn new<P: AsRef<Path>>(path: P) -> Result<Self> {
        let connection = Connection::open(path)?;
        
        // Create tables
        connection.execute(
            "CREATE TABLE IF NOT EXISTS calculations (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS firearm_profiles (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at TEXT NOT NULL,
                updated_at TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self { connection })
    }
}

impl StorageBackend for SqliteStorage {
    fn save_calculation(&mut self, calculation: &SavedCalculation) -> Result<()> {
        let data = serde_json::to_string(calculation)?;
        self.connection.execute(
            "INSERT OR REPLACE INTO calculations (id, data, created_at) 
             VALUES (?1, ?2, ?3)",
            params![calculation.id, data, calculation.timestamp.to_rfc3339()],
        )?;
        Ok(())
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        let mut stmt = self.connection.prepare(
            "SELECT data FROM calculations ORDER BY created_at DESC"
        )?;
        
        let calculations = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?
        .filter_map(|r| r.ok())
        .filter_map(|data| serde_json::from_str(&data).ok())
        .collect();
        
        Ok(calculations)
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        let mut stmt = self.connection.prepare(
            "SELECT data FROM firearm_profiles ORDER BY updated_at DESC"
        )?;
        
        let profiles = stmt.query_map([], |row| {
            let data: String = row.get(0)?;
            Ok(data)
        })?
        .filter_map(|r| r.ok())
        .filter_map(|data| serde_json::from_str(&data).ok())
        .collect();
        
        Ok(profiles)
    }
    
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()> {
        let data = serde_json::to_string(profile)?;
        self.connection.execute(
            "INSERT OR REPLACE INTO firearm_profiles (id, data, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?4)",
            params![
                profile.id.to_string(), 
                data, 
                profile.created_at.to_rfc3339(),
                profile.updated_at.to_rfc3339()
            ],
        )?;
        Ok(())
    }
}