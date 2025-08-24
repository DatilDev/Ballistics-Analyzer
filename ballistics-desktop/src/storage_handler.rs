//! Storage handler for desktop application

use anyhow::Result;
use ballistics_core::storage::{StorageManager, StorageEntry};
use std::path::PathBuf;

pub struct DesktopStorage {
    manager: StorageManager,
}

impl DesktopStorage {
    /// Create new desktop storage
    pub fn new() -> Result<Self> {
        let data_dir = Self::get_data_dir()?;
        let db_path = data_dir.join("calculations.db");
        
        // Ensure directory exists
        std::fs::create_dir_all(&data_dir)?;
        
        let manager = StorageManager::sqlite(
            db_path.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?
        )?;
        
        Ok(Self { manager })
    }

    /// Get data directory for the application
    fn get_data_dir() -> Result<PathBuf> {
        let dir = dirs::data_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find data directory"))?
            .join("ballistics-analyzer");
        Ok(dir)
    }

    /// Save a calculation
    pub fn save_calculation(&self, id: &str, data: &str) -> Result<()> {
        self.manager.save(id, data)
    }

    /// Load a calculation
    pub fn load_calculation(&self, id: &str) -> Result<Option<String>> {
        self.manager.load(id)
    }

    /// List all calculations
    pub fn list_calculations(&self) -> Result<Vec<StorageEntry>> {
        self.manager.list()
    }

    /// Delete a calculation
    pub fn delete_calculation(&self, id: &str) -> Result<()> {
        self.manager.delete(id)
    }

    /// Export all calculations
    pub fn export_all(&self) -> Result<Vec<(String, String)>> {
        let entries = self.list_calculations()?;
        let mut results = Vec::new();
        
        for entry in entries {
            if let Some(data) = self.load_calculation(&entry.id)? {
                results.push((entry.id, data));
            }
        }
        
        Ok(results)
    }

    /// Import calculations
    pub fn import_calculations(&self, data: Vec<(String, String)>) -> Result<()> {
        for (id, content) in data {
            self.save_calculation(&id, &content)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_desktop_storage() {
        // Use in-memory storage for testing
        let manager = StorageManager::memory().unwrap();
        
        // Test operations
        manager.save("test", "data").unwrap();
        let loaded = manager.load("test").unwrap();
        assert_eq!(loaded, Some("data".to_string()));
    }
}