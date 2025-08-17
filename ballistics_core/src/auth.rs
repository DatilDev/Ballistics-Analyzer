#[cfg(feature = "nostr")]
use nostr::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct NostrAuth {
    secret_key: Option<String>,
    public_key: Option<String>,
    pub amber_endpoint: String,
    
    #[serde(skip)]
    #[cfg(feature = "nostr")]
    keys: Option<nostr::Keys>,
}

impl NostrAuth {
    pub fn authenticate(&mut self) -> bool {
        #[cfg(feature = "nostr")]
        {
            let keys = nostr::Keys::generate();
            self.secret_key = Some(keys.secret_key().to_bech32().unwrap());
            self.public_key = Some(keys.public_key().to_string());
            self.keys = Some(keys);
            true
        }
        
        #[cfg(not(feature = "nostr"))]
        {
            // Fallback for platforms without nostr
            self.secret_key = Some(format!("mock_secret_{}", uuid::Uuid::new_v4()));
            self.public_key = Some(format!("mock_public_{}", uuid::Uuid::new_v4()));
            true
        }
    }
    
    pub fn get_pubkey(&self) -> String {
        self.public_key.clone().unwrap_or_default()
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.public_key.is_some()
    }
    
    pub fn logout(&mut self) {
        self.secret_key = None;
        self.public_key = None;
        #[cfg(feature = "nostr")]
        {
            self.keys = None;
        }
    }
}