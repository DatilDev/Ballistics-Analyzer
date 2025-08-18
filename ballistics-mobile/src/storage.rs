// ballistics-mobile/src/storage.rs
use serde_json;
use ballistics_core::{StorageBackend, SavedCalculation, FirearmProfile};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use rusqlite::{Connection, params};

/// Local-only storage implementation for Android
/// All data is stored in the app's private internal storage
/// No cloud sync, no external access, complete privacy
pub struct LocalOnlyStorage {
    base_path: PathBuf,
    db_connection: Connection,
    encryption_enabled: bool,
}

impl LocalOnlyStorage {
    pub fn new(base_path: &str) -> Result<Self> {
        let path = PathBuf::from(base_path);
        
        // Create private directories if they don't exist
        fs::create_dir_all(&path.join("calculations"))?;
        fs::create_dir_all(&path.join("profiles"))?;
        fs::create_dir_all(&path.join("photos"))?;
        
        // Initialize local SQLite database
        let db_path = path.join("ballistics_local.db");
        let conn = Connection::open(&db_path)?;
        
        // Create tables for local storage
        conn.execute(
            "CREATE TABLE IF NOT EXISTS calculations (
                id TEXT PRIMARY KEY,
                timestamp TEXT NOT NULL,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                encrypted INTEGER DEFAULT 0
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS profiles (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                data BLOB NOT NULL,
                encrypted INTEGER DEFAULT 0
            )",
            [],
        )?;
        
        // Privacy settings table
        conn.execute(
            "CREATE TABLE IF NOT EXISTS privacy_settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        // Set default privacy settings
        conn.execute(
            "INSERT OR REPLACE INTO privacy_settings (key, value) VALUES (?, ?)",
            params!["analytics_enabled", "false"],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO privacy_settings (key, value) VALUES (?, ?)",
            params!["data_sharing_enabled", "false"],
        )?;
        
        conn.execute(
            "INSERT OR REPLACE INTO privacy_settings (key, value) VALUES (?, ?)",
            params!["cloud_backup_enabled", "false"],
        )?;
        
        Ok(Self {
            base_path: path,
            db_connection: conn,
            encryption_enabled: true, // Always encrypt sensitive data
        })
    }
    
    fn encrypt_data(&self, data: &[u8]) -> Vec<u8> {
        if self.encryption_enabled {
            // Simple XOR encryption for demonstration
            // In production, use proper encryption like AES-256-GCM
            let key = b"local_only_encryption_key_12345";
            data.iter()
                .zip(key.iter().cycle())
                .map(|(d, k)| d ^ k)
                .collect()
        } else {
            data.to_vec()
        }
    }
    
    fn decrypt_data(&self, data: &[u8]) -> Vec<u8> {
        // XOR is symmetric, so encryption and decryption are the same
        self.encrypt_data(data)
    }
    
    pub fn clear_all_data(&mut self) -> Result<()> {
        // Complete data wipe - user initiated only
        self.db_connection.execute("DELETE FROM calculations", [])?;
        self.db_connection.execute("DELETE FROM profiles", [])?;
        
        // Clear file system data
        fs::remove_dir_all(self.base_path.join("calculations"))?;
        fs::remove_dir_all(self.base_path.join("profiles"))?;
        fs::remove_dir_all(self.base_path.join("photos"))?;
        
        // Recreate directories
        fs::create_dir_all(self.base_path.join("calculations"))?;
        fs::create_dir_all(self.base_path.join("profiles"))?;
        fs::create_dir_all(self.base_path.join("photos"))?;
        
        Ok(())
    }
    
    pub fn export_all_data(&self) -> Result<String> {
        // Export all data for user backup (local only)
        let calculations = self.list_calculations()?;
        let profiles = self.list_profiles()?;
        
        let export = serde_json::json!({
            "version": "1.0.0",
            "export_date": chrono::Utc::now().to_rfc3339(),
            "calculations": calculations,
            "profiles": profiles,
            "privacy_notice": "This data was exported from your local device. No data has been shared with any servers.",
        });
        
        Ok(serde_json::to_string_pretty(&export)?)
    }
    
    pub fn get_storage_size(&self) -> Result<u64> {
        let mut total_size = 0u64;
        
        // Calculate database size
        if let Ok(metadata) = fs::metadata(self.base_path.join("ballistics_local.db")) {
            total_size += metadata.len();
        }
        
        // Calculate file storage size
        for entry in fs::read_dir(&self.base_path)? {
            if let Ok(entry) = entry {
                if let Ok(metadata) = entry.metadata() {
                    total_size += metadata.len();
                }
            }
        }
        
        Ok(total_size)
    }
    
    pub fn is_analytics_enabled(&self) -> bool {
        // Always check current setting - default is false
        self.db_connection
            .query_row(
                "SELECT value FROM privacy_settings WHERE key = ?",
                params!["analytics_enabled"],
                |row| row.get::<_, String>(0),
            )
            .unwrap_or_else(|_| "false".to_string()) == "true"
    }
    
    pub fn set_analytics_enabled(&self, enabled: bool) -> Result<()> {
        // User must explicitly opt-in
        self.db_connection.execute(
            "INSERT OR REPLACE INTO privacy_settings (key, value) VALUES (?, ?)",
            params!["analytics_enabled", enabled.to_string()],
        )?;
        Ok(())
    }
}

impl StorageBackend for LocalOnlyStorage {
    fn save_calculation(&mut self, calc: &SavedCalculation) -> Result<()> {
        let json = serde_json::to_vec(calc)?;
        let encrypted = self.encrypt_data(&json);
        
        self.db_connection.execute(
            "INSERT OR REPLACE INTO calculations (id, timestamp, name, data, encrypted) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                calc.id,
                calc.timestamp.to_rfc3339(),
                calc.name,
                encrypted,
                1
            ],
        )?;
        
        Ok(())
    }
    
    fn load_calculation(&self, id: &str) -> Result<SavedCalculation> {
        let encrypted_data: Vec<u8> = self.db_connection.query_row(
            "SELECT data FROM calculations WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        
        let decrypted = self.decrypt_data(&encrypted_data);
        Ok(serde_json::from_slice(&decrypted)?)
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        let mut stmt = self.db_connection.prepare(
            "SELECT data FROM calculations ORDER BY timestamp DESC"
        )?;
        
        let calculations = stmt
            .query_map([], |row| {
                let encrypted_data: Vec<u8> = row.get(0)?;
                Ok(encrypted_data)
            })?
            .filter_map(|data| {
                data.ok().and_then(|encrypted| {
                    let decrypted = self.decrypt_data(&encrypted);
                    serde_json::from_slice(&decrypted).ok()
                })
            })
            .collect();
        
        Ok(calculations)
    }
    
    fn delete_calculation(&mut self, id: &str) -> Result<()> {
        self.db_connection.execute(
            "DELETE FROM calculations WHERE id = ?1",
            params![id],
        )?;
        
        // Also delete any associated photos from local storage
        let photo_path = self.base_path.join("photos").join(id);
        if photo_path.exists() {
            fs::remove_dir_all(photo_path)?;
        }
        
        Ok(())
    }
    
    fn save_profile(&mut self, profile: &FirearmProfile) -> Result<()> {
        let json = serde_json::to_vec(profile)?;
        let encrypted = self.encrypt_data(&json);
        
        self.db_connection.execute(
            "INSERT OR REPLACE INTO profiles (id, name, data, encrypted) 
             VALUES (?1, ?2, ?3, ?4)",
            params![profile.id, profile.name, encrypted, 1],
        )?;
        
        Ok(())
    }
    
    fn load_profile(&self, id: &str) -> Result<FirearmProfile> {
        let encrypted_data: Vec<u8> = self.db_connection.query_row(
            "SELECT data FROM profiles WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        
        let decrypted = self.decrypt_data(&encrypted_data);
        Ok(serde_json::from_slice(&decrypted)?)
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        let mut stmt = self.db_connection.prepare(
            "SELECT data FROM profiles ORDER BY name"
        )?;
        
        let profiles = stmt
            .query_map([], |row| {
                let encrypted_data: Vec<u8> = row.get(0)?;
                Ok(encrypted_data)
            })?
            .filter_map(|data| {
                data.ok().and_then(|encrypted| {
                    let decrypted = self.decrypt_data(&encrypted);
                    serde_json::from_slice(&decrypted).ok()
                })
            })
            .collect();
        
        Ok(profiles)
    }
    
    fn delete_profile(&mut self, id: &str) -> Result<()> {
        self.db_connection.execute(
            "DELETE FROM profiles WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}