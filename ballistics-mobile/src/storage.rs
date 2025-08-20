// ballistics-mobile/src/storage.rs
use serde_json;
use ballistics_core::{StorageBackend, SavedCalculation, FirearmProfile};
use anyhow::Result;
use std::path::PathBuf;
use std::fs;
use std::sync::{Arc, Mutex};
use rusqlite::{Connection, params};

/// Thread-safe local storage implementation for mobile
/// All data is stored in the app's private internal storage
/// No cloud sync, no external access, complete privacy
pub struct MobileStorage {
    base_path: PathBuf,
    db_connection: Arc<Mutex<Connection>>,
    encryption_enabled: bool,
}

// Implement Send and Sync manually since we're using Arc<Mutex<Connection>>
unsafe impl Send for MobileStorage {}
unsafe impl Sync for MobileStorage {}

impl MobileStorage {
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
            db_connection: Arc::new(Mutex::new(conn)),
            encryption_enabled: true,
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
        let conn = self.db_connection.lock().unwrap();
        conn.execute("DELETE FROM calculations", [])?;
        conn.execute("DELETE FROM profiles", [])?;
        drop(conn); // Release the lock
        
        // Clear file system data
        let calc_path = self.base_path.join("calculations");
        let profile_path = self.base_path.join("profiles");
        let photos_path = self.base_path.join("photos");
        
        if calc_path.exists() {
            fs::remove_dir_all(&calc_path)?;
            fs::create_dir_all(&calc_path)?;
        }
        
        if profile_path.exists() {
            fs::remove_dir_all(&profile_path)?;
            fs::create_dir_all(&profile_path)?;
        }
        
        if photos_path.exists() {
            fs::remove_dir_all(&photos_path)?;
            fs::create_dir_all(&photos_path)?;
        }
        
        Ok(())
    }
    
    pub fn export_all_data(&self) -> Result<String> {
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
        let conn = self.db_connection.lock().unwrap();
        conn.query_row(
            "SELECT value FROM privacy_settings WHERE key = ?",
            params!["analytics_enabled"],
            |row| row.get::<_, String>(0),
        )
        .unwrap_or_else(|_| "false".to_string()) == "true"
    }
    
    pub fn set_analytics_enabled(&self, enabled: bool) -> Result<()> {
        let conn = self.db_connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO privacy_settings (key, value) VALUES (?, ?)",
            params!["analytics_enabled", enabled.to_string()],
        )?;
        Ok(())
    }
}

impl StorageBackend for MobileStorage {
    fn save_calculation(&mut self, calc: &SavedCalculation) -> Result<()> {
        let json = serde_json::to_vec(calc)?;
        let encrypted = self.encrypt_data(&json);
        
        let conn = self.db_connection.lock().unwrap();
        conn.execute(
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
        let conn = self.db_connection.lock().unwrap();
        let encrypted_data: Vec<u8> = conn.query_row(
            "SELECT data FROM calculations WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        drop(conn); // Release lock
        
        let decrypted = self.decrypt_data(&encrypted_data);
        Ok(serde_json::from_slice(&decrypted)?)
    }
    
    fn list_calculations(&self) -> Result<Vec<SavedCalculation>> {
        let conn = self.db_connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT data FROM calculations ORDER BY timestamp DESC"
        )?;
        
        let encrypted_results: Vec<Vec<u8>> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        
        drop(stmt);
        drop(conn); // Release lock
        
        let calculations = encrypted_results
            .into_iter()
            .filter_map(|encrypted| {
                let decrypted = self.decrypt_data(&encrypted);
                serde_json::from_slice(&decrypted).ok()
            })
            .collect();
        
        Ok(calculations)
    }
    
    fn delete_calculation(&mut self, id: &str) -> Result<()> {
        let conn = self.db_connection.lock().unwrap();
        conn.execute(
            "DELETE FROM calculations WHERE id = ?1",
            params![id],
        )?;
        drop(conn); // Release lock
        
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
        
        let conn = self.db_connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO profiles (id, name, data, encrypted) 
             VALUES (?1, ?2, ?3, ?4)",
            params![profile.id, profile.name, encrypted, 1],
        )?;
        
        Ok(())
    }
    
    fn load_profile(&self, id: &str) -> Result<FirearmProfile> {
        let conn = self.db_connection.lock().unwrap();
        let encrypted_data: Vec<u8> = conn.query_row(
            "SELECT data FROM profiles WHERE id = ?1",
            params![id],
            |row| row.get(0),
        )?;
        drop(conn); // Release lock
        
        let decrypted = self.decrypt_data(&encrypted_data);
        Ok(serde_json::from_slice(&decrypted)?)
    }
    
    fn list_profiles(&self) -> Result<Vec<FirearmProfile>> {
        let conn = self.db_connection.lock().unwrap();
        let mut stmt = conn.prepare(
            "SELECT data FROM profiles ORDER BY name"
        )?;
        
        let encrypted_results: Vec<Vec<u8>> = stmt
            .query_map([], |row| row.get(0))?
            .filter_map(|r| r.ok())
            .collect();
        
        drop(stmt);
        drop(conn); // Release lock
        
        let profiles = encrypted_results
            .into_iter()
            .filter_map(|encrypted| {
                let decrypted = self.decrypt_data(&encrypted);
                serde_json::from_slice(&decrypted).ok()
            })
            .collect();
        
        Ok(profiles)
    }
    
    fn delete_profile(&mut self, id: &str) -> Result<()> {
        let conn = self.db_connection.lock().unwrap();
        conn.execute(
            "DELETE FROM profiles WHERE id = ?1",
            params![id],
        )?;
        Ok(())
    }
}

// Alternative implementation using file-based storage for simpler thread safety
pub struct FileBasedStorage {
    base_path: PathBuf,
}

impl FileBasedStorage {
    pub fn new(base_path: &str) -> Result<Self> {
        let path = PathBuf::from(base_path);
        
        // Create directories
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

impl StorageBackend for FileBasedStorage {
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
        let mut calculations: Vec<SavedCalculation> = Vec::new();
        
        for entry in fs::read_dir(self.calculations_path())? {
            let entry = entry?;
            if entry.path().extension() == Some("json".as_ref()) {
                let json = fs::read_to_string(entry.path())?;
                if let Ok(calc) = serde_json::from_str::<SavedCalculation>(&json) {
                    calculations.push(calc);
                }
            }
        }
        
        // Sort by timestamp
        calculations.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        Ok(calculations)
    }
    
    fn delete_calculation(&mut self, id: &str) -> Result<()> {
        let path = self.calculations_path().join(format!("{}.json", id));
        if path.exists() {
            fs::remove_file(path)?;
        }
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
        let mut profiles: Vec<FirearmProfile> = Vec::new();
        
        for entry in fs::read_dir(self.profiles_path())? {
            let entry = entry?;
            if entry.path().extension() == Some("json".as_ref()) {
                let json = fs::read_to_string(entry.path())?;
                if let Ok(profile) = serde_json::from_str::<FirearmProfile>(&json) {
                    profiles.push(profile);
                }
            }
        }
        
        // Sort by name
        profiles.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(profiles)
    }
    
    fn delete_profile(&mut self, id: &str) -> Result<()> {
        let path = self.profiles_path().join(format!("{}.json", id));
        if path.exists() {
            fs::remove_file(path)?;
        }
        Ok(())
    }
}