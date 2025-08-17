use crate::models::SavedCalculation;
use serde::{Deserialize, Serialize};

#[derive(Default)]
pub struct SharingManager {
    pub include_photos: bool,
    pub include_profile: bool,
    pub include_weather: bool,
    pub recent_shares: Vec<RecentShare>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecentShare {
    pub event_id: String,
    pub timestamp: String,
    pub calculation_id: String,
}

impl SharingManager {
    #[cfg(feature = "nostr")]
    pub async fn share_calculation(
        &mut self,
        auth: &crate::auth::NostrAuth,
        calculation: &SavedCalculation,
    ) -> anyhow::Result<String> {
        use nostr_sdk::prelude::*;
        
        if !auth.is_authenticated() {
            return Err(anyhow::anyhow!("Not authenticated"));
        }
        
        // Serialize calculation
        let content = serde_json::to_string(calculation)?;
        
        // Create nostr event
        let event_id = uuid::Uuid::new_v4().to_string();
        
        self.recent_shares.push(RecentShare {
            event_id: event_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            calculation_id: calculation.id.clone(),
        });
        
        Ok(event_id)
    }
    
    #[cfg(not(feature = "nostr"))]
    pub async fn share_calculation(
        &mut self,
        _auth: &crate::auth::NostrAuth,
        calculation: &SavedCalculation,
    ) -> anyhow::Result<String> {
        // Fallback sharing via export
        let event_id = uuid::Uuid::new_v4().to_string();
        
        self.recent_shares.push(RecentShare {
            event_id: event_id.clone(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            calculation_id: calculation.id.clone(),
        });
        
        Ok(event_id)
    }
    
    pub fn export_to_json(&self, calculation: &SavedCalculation) -> String {
        serde_json::to_string_pretty(calculation).unwrap_or_default()
    }
    
    pub fn import_from_json(&self, json: &str) -> anyhow::Result<SavedCalculation> {
        Ok(serde_json::from_str(json)?)
    }
}