use std::path::PathBuf;

use egui_extras::RetainedImage;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

use crate::app::{AttachedImage, SavedCalculation};
use crate::firearm_profiles::FirearmProfile;

#[derive(Default)]
pub struct LocalStorage {
    db_path: Option<PathBuf>,
}

impl LocalStorage {
    pub fn init_user_storage(&mut self, user_pubkey: &str) {
        let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
        path.push("ballistics_analyzer");
        let _ = std::fs::create_dir_all(&path);
        let short = if user_pubkey.len() >= 8 {
            &user_pubkey[..8]
        } else {
            user_pubkey
        };
        path.push(format!("{}.db", short));

        self.db_path = Some(path);
        self.init_db();
    }

    fn init_db(&self) {
        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();

            conn.execute(
                "CREATE TABLE IF NOT EXISTS calculations (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL,
                    timestamp TEXT NOT NULL
                )",
                [],
            )
            .unwrap();

            conn.execute(
                "CREATE TABLE IF NOT EXISTS images (
                    id TEXT PRIMARY KEY,
                    name TEXT NOT NULL,
                    data BLOB NOT NULL,
                    calculation_id TEXT,
                    FOREIGN KEY(calculation_id) REFERENCES calculations(id)
                )",
                [],
            )
            .unwrap();

            conn.execute(
                "CREATE TABLE IF NOT EXISTS profiles (
                    id TEXT PRIMARY KEY,
                    data TEXT NOT NULL
                )",
                [],
            )
            .unwrap();
        }
    }

    pub fn save_calculation(&self, calculation: &SavedCalculation, images: &[AttachedImage]) {
        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();

            let data = serde_json::to_string(calculation).unwrap();
            conn.execute(
                "INSERT OR REPLACE INTO calculations (id, data, timestamp) VALUES (?1, ?2, ?3)",
                params![&calculation.id, &data, &calculation.calculation.timestamp],
            )
            .unwrap();

            for img in images {
                conn.execute(
                    "INSERT OR REPLACE INTO images (id, name, data, calculation_id) VALUES (?1, ?2, ?3, ?4)",
                    params![&img.id, &img.name, &img.data, &calculation.id],
                )
                .unwrap();
            }
        }
    }

    pub fn load_calculations(&self) -> Vec<SavedCalculation> {
        let mut calculations = Vec::new();

        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();
            let mut stmt =
                conn.prepare("SELECT data FROM calculations ORDER BY timestamp DESC").unwrap();

            let rows = stmt
                .query_map([], |row| {
                    let data: String = row.get(0)?;
                    let calc: SavedCalculation = serde_json::from_str(&data).unwrap();
                    Ok(calc)
                })
                .unwrap();

            for r in rows {
                if let Ok(c) = r {
                    calculations.push(c);
                }
            }
        }

        calculations
    }

    pub fn load_images(&self, image_ids: &[String]) -> Vec<AttachedImage> {
        let mut images = Vec::new();

        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();

            for id in image_ids {
                let mut stmt = conn
                    .prepare("SELECT name, data FROM images WHERE id = ?1")
                    .unwrap();

                let mut rows = stmt.query([id]).unwrap();
                if let Some(row) = rows.next().unwrap() {
                    let name: String = row.get(0).unwrap();
                    let data: Vec<u8> = row.get(1).unwrap();

                    let thumbnail = if let Ok(img) = image::load_from_memory(&data) {
                        let thumb = img.thumbnail(100, 100);
                        let mut buf = Vec::new();
                        if thumb
                            .write_to(&mut std::io::Cursor::new(&mut buf), image::ImageFormat::Png)
                            .is_ok()
                        {
                            RetainedImage::from_image_bytes(id, &buf).ok()
                        } else {
                            None
                        }
                    } else {
                        None
                    };

                    images.push(AttachedImage {
                        id: id.clone(),
                        name,
                        data,
                        thumbnail,
                    });
                }
            }
        }

        images
    }

    pub fn save_profiles(&self, profiles: &[FirearmProfile]) {
        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();

            for profile in profiles {
                let data = serde_json::to_string(profile).unwrap();
                conn.execute(
                    "INSERT OR REPLACE INTO profiles (id, data) VALUES (?1, ?2)",
                    params![&profile.id, &data],
                )
                .unwrap();
            }
        }
    }

    pub fn load_profiles(&self) -> Vec<FirearmProfile> {
        let mut profiles = Vec::new();

        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();
            let mut stmt = conn.prepare("SELECT data FROM profiles").unwrap();

            let rows = stmt
                .query_map([], |row| {
                    let data: String = row.get(0)?;
                    let p: FirearmProfile = serde_json::from_str(&data).unwrap();
                    Ok(p)
                })
                .unwrap();

            for r in rows {
                if let Ok(p) = r {
                    profiles.push(p);
                }
            }
        }

        profiles
    }

    pub fn delete_calculation(&self, id: &str) {
        if let Some(path) = &self.db_path {
            let conn = Connection::open(path).unwrap();
            conn.execute("DELETE FROM calculations WHERE id = ?1", params![id])
                .unwrap();
            conn.execute(
                "DELETE FROM images WHERE calculation_id = ?1",
                params![id],
            )
            .unwrap();
        }
    }

    pub fn clear_all(&self) {
        if let Some(path) = &self.db_path {
            let _ = std::fs::remove_file(path);
        }
    }
}