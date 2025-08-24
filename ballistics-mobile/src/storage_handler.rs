//! Storage handler for mobile application

use anyhow::Result;
use ballistics_core::storage::{StorageManager, StorageEntry};
use std::path::PathBuf;

pub struct MobileStorage {
    manager: StorageManager,
}

impl MobileStorage {
    /// Create new mobile storage
    #[cfg(target_os = "android")]
    pub fn new(context: &android_activity::AndroidApp) -> Result<Self> {
        let data_dir = context.internal_data_path()
            .ok_or_else(|| anyhow::anyhow!("No data path"))?;
        
        let db_path = data_dir.join("calculations.db");
        
        let manager = StorageManager::sqlite(
            db_path.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?
        )?;
        
        Ok(Self { manager })
    }

    /// Create new mobile storage (iOS)
    #[cfg(target_os = "ios")]
    pub fn new() -> Result<Self> {
        let data_dir = Self::get_documents_dir()?;
        let db_path = data_dir.join("calculations.db");
        
        std::fs::create_dir_all(&data_dir)?;
        
        let manager = StorageManager::sqlite(
            db_path.to_str()
                .ok_or_else(|| anyhow::anyhow!("Invalid path"))?
        )?;
        
        Ok(Self { manager })
    }

    #[cfg(target_os = "ios")]
    fn get_documents_dir() -> Result<PathBuf> {
        // iOS-specific path
        let home = std::env::var("HOME")?;
        Ok(PathBuf::from(home).join("Documents"))
    }

    /// Save calculation with sync flag for cloud backup
    pub fn save_with_sync(&self, id: &str, data: &str, sync: bool) -> Result<()> {
        // Save locally first
        self.manager.save(id, data)?;
        
        if sync {
            // Mark for cloud sync (platform-specific)
            self.mark_for_sync(id)?;
        }
        
        Ok(())
    }

    fn mark_for_sync(&self, _id: &str) -> Result<()> {
        // Platform-specific sync implementation
        // For Android: Could use Android Backup Service
        // For iOS: Could use iCloud
        Ok(())
    }

    pub fn save(&self, id: &str, data: &str) -> Result<()> {
        self.manager.save(id, data)
    }

    pub fn load(&self, id: &str) -> Result<Option<String>> {
        self.manager.load(id)
    }

    pub fn list(&self) -> Result<Vec<StorageEntry>> {
        self.manager.list()
    }

    pub fn delete(&self, id: &str) -> Result<()> {
        self.manager.delete(id)
    }
}