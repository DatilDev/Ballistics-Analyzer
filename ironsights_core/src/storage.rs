//! Storage backend for ballistics calculations
//! Uses r2d2 connection pool for thread-safe SQLite access

use anyhow::{Result, Context};
use r2d2::{Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::{params, OptionalExtension};
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

/// Trait for storage backends
pub trait StorageBackend: Send + Sync {
    fn save(&self, id: &str, data: &str) -> Result<()>;
    fn load(&self, id: &str) -> Result<Option<String>>;
    fn list(&self) -> Result<Vec<StorageEntry>>;
    fn delete(&self, id: &str) -> Result<()>;
    fn clear_all(&self) -> Result<()>;
}

/// Storage entry metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageEntry {
    pub id: String,
    pub created_at: DateTime<Utc>,
    pub size: usize,
}

/// SQLite storage backend with connection pooling
pub struct SqliteStorage {
    pool: Pool<SqliteConnectionManager>,
}

impl SqliteStorage {
    /// Create new SQLite storage with connection pool
    pub fn new(path: &str) -> Result<Self> {
        let manager = SqliteConnectionManager::file(path);
        let pool = Pool::builder()
            .max_size(10)  // Maximum 10 connections
            .min_idle(Some(1))  // Keep at least 1 connection alive
            .build(manager)
            .context("Failed to create connection pool")?;
        
        // Initialize schema
        let conn = pool.get()?;
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS calculations (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_created_at ON calculations(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_updated_at ON calculations(updated_at DESC);"
        ).context("Failed to create schema")?;
        
        Ok(Self { pool })
    }

    /// Create in-memory storage (useful for testing)
    pub fn memory() -> Result<Self> {
        let manager = SqliteConnectionManager::memory();
        let pool = Pool::new(manager)?;
        
        let conn = pool.get()?;
        conn.execute_batch(
            "CREATE TABLE calculations (
                id TEXT PRIMARY KEY,
                data TEXT NOT NULL,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            );
            CREATE INDEX idx_created_at ON calculations(created_at DESC);
            CREATE INDEX idx_updated_at ON calculations(updated_at DESC);"
        )?;
        
        Ok(Self { pool })
    }

    /// Get a connection from the pool
    fn get_conn(&self) -> Result<PooledConnection<SqliteConnectionManager>> {
        self.pool.get().context("Failed to get connection from pool")
    }

    /// Get storage statistics
    pub fn stats(&self) -> Result<StorageStats> {
        let conn = self.get_conn()?;
        
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM calculations",
            [],
            |row| row.get(0)
        )?;
        
        let total_size: Option<i64> = conn.query_row(
            "SELECT SUM(LENGTH(data)) FROM calculations",
            [],
            |row| row.get(0)
        ).optional()?;
        
        Ok(StorageStats {
            total_entries: count as usize,
            total_size: total_size.unwrap_or(0) as usize,
        })
    }
}

impl StorageBackend for SqliteStorage {
    fn save(&self, id: &str, data: &str) -> Result<()> {
        let conn = self.get_conn()?;
        let now = Utc::now().timestamp();
        
        conn.execute(
            "INSERT INTO calculations (id, data, created_at, updated_at) 
             VALUES (?1, ?2, ?3, ?3)
             ON CONFLICT(id) DO UPDATE SET 
                data = excluded.data,
                updated_at = excluded.updated_at",
            params![id, data, now],
        ).context("Failed to save calculation")?;
        
        Ok(())
    }

    fn load(&self, id: &str) -> Result<Option<String>> {
        let conn = self.get_conn()?;
        
        let result = conn.query_row(
            "SELECT data FROM calculations WHERE id = ?1",
            params![id],
            |row| row.get(0)
        ).optional()
        .context("Failed to load calculation")?;
        
        Ok(result)
    }

    fn list(&self) -> Result<Vec<StorageEntry>> {
        let conn = self.get_conn()?;
        
        let mut stmt = conn.prepare(
            "SELECT id, created_at, LENGTH(data) as size 
             FROM calculations 
             ORDER BY created_at DESC 
             LIMIT 100"
        )?;
        
        let entries = stmt.query_map([], |row| {
            Ok(StorageEntry {
                id: row.get(0)?,
                created_at: DateTime::from_timestamp(row.get(1)?, 0)
                    .unwrap_or_else(|| Utc::now()),
                size: row.get(2)?,
            })
        })?
        .collect::<Result<Vec<_>, _>>()
        .context("Failed to list calculations")?;
        
        Ok(entries)
    }

    fn delete(&self, id: &str) -> Result<()> {
        let conn = self.get_conn()?;
        
        let rows_affected = conn.execute(
            "DELETE FROM calculations WHERE id = ?1",
            params![id]
        )?;
        
        if rows_affected == 0 {
            anyhow::bail!("Entry not found: {}", id);
        }
        
        Ok(())
    }

    fn clear_all(&self) -> Result<()> {
        let conn = self.get_conn()?;
        conn.execute("DELETE FROM calculations", [])?;
        Ok(())
    }
}

/// Storage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageStats {
    pub total_entries: usize,
    pub total_size: usize,
}

/// Storage manager for handling multiple backends
pub struct StorageManager {
    backend: Box<dyn StorageBackend>,
}

impl StorageManager {
    /// Create with SQLite backend
    pub fn sqlite(path: &str) -> Result<Self> {
        Ok(Self {
            backend: Box::new(SqliteStorage::new(path)?),
        })
    }

    /// Create with in-memory backend
    pub fn memory() -> Result<Self> {
        Ok(Self {
            backend: Box::new(SqliteStorage::memory()?),
        })
    }

    /// Save data
    pub fn save(&self, id: &str, data: &str) -> Result<()> {
        self.backend.save(id, data)
    }

    /// Load data
    pub fn load(&self, id: &str) -> Result<Option<String>> {
        self.backend.load(id)
    }

    /// List all entries
    pub fn list(&self) -> Result<Vec<StorageEntry>> {
        self.backend.list()
    }

    /// Delete entry
    pub fn delete(&self, id: &str) -> Result<()> {
        self.backend.delete(id)
    }

    /// Clear all data
    pub fn clear_all(&self) -> Result<()> {
        self.backend.clear_all()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_storage() {
        let storage = SqliteStorage::memory().unwrap();
        
        // Test save and load
        storage.save("test1", "data1").unwrap();
        let loaded = storage.load("test1").unwrap();
        assert_eq!(loaded, Some("data1".to_string()));
        
        // Test list
        storage.save("test2", "data2").unwrap();
        let entries = storage.list().unwrap();
        assert_eq!(entries.len(), 2);
        
        // Test delete
        storage.delete("test1").unwrap();
        let loaded = storage.load("test1").unwrap();
        assert_eq!(loaded, None);
        
        // Test clear all
        storage.clear_all().unwrap();
        let entries = storage.list().unwrap();
        assert_eq!(entries.len(), 0);
    }

    #[test]
    fn test_storage_manager() {
        let manager = StorageManager::memory().unwrap();
        
        manager.save("calc1", r#"{"velocity": 2800}"#).unwrap();
        let data = manager.load("calc1").unwrap();
        assert!(data.is_some());
        
        let entries = manager.list().unwrap();
        assert!(!entries.is_empty());
    }
}