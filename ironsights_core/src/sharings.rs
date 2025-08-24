// Minimal sharing module for android-arch branch
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShareData {
    pub content: String,
    pub format: ShareFormat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ShareFormat {
    Text,
    Json,
    Csv,
}

pub fn create_share_text(data: &str) -> ShareData {
    ShareData {
        content: data.to_string(),
        format: ShareFormat::Text,
    }
}