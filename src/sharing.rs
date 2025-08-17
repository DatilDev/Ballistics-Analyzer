use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};

use crate::auth;
use crate::SavedCalculation;  // This is re-exported from lib.rs

#[derive(Default)]
pub struct SharingManager {
    pub include_photos: bool,
    pub include_profile: bool,
    pub include_weather: bool,
    pub import_event_id: String,
    pub recent_shares: Vec<RecentShare>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct RecentShare {
    pub event_id: String,
    pub timestamp: String,
}

impl SharingManager {
    pub fn share_calculation(
        &mut self,
        auth: &auth::NostrAuth,
        calculation: &SavedCalculation,
    ) -> Option<String> {
        let (client, keys) = match (auth.get_client(), auth.get_keys()) {
            (Some(c), Some(k)) => (c, k),
            _ => return None,
        };

        // Build content (you may want to conditionally strip fields based on include_* flags)
        let content = serde_json::to_string(calculation).ok()?;

        let mut tags = vec![
            Tag::hashtag("ballistics"), // Use Tag::hashtag instead of Tag::Hashtag
            Tag::hashtag("shooting"),
        ];
        
        // Add caliber tag if parsing succeeds
        if let Ok(caliber_tag) = Tag::parse(&[
            "caliber".to_string(),
            calculation.calculation.projectile_data.caliber.clone(),
        ]) {
            tags.push(caliber_tag);
        }

        let event = EventBuilder::new(
            Kind::Custom(30078), // Use Kind::Custom instead of Kind::from
            content,
            tags, // Use Vec<Tag> instead of &[Tag; 3]
        )
        .to_event(keys)
        .ok()?;

        // Publish event (simple local runtime; replace with your app's runtime if available)
        let send_ok = tokio::runtime::Runtime::new()
            .ok()?
            .block_on(async { client.send_event(event.clone()).await.ok() })
            .is_some();

        if send_ok {
            let id = event.id.to_string();
            self.recent_shares.push(RecentShare {
                event_id: id.clone(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            });
            Some(id)
        } else {
            None
        }
    }

    pub fn import_calculation(&self, _event_id: &str) -> Option<SavedCalculation> {
        // TODO: fetch and parse from relays via client; stubbed for now
        None
    }
}