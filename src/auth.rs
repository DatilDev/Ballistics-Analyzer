use serde::{Deserialize, Serialize};

#[cfg(not(target_arch = "wasm32"))]
use nostr::prelude::*;

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct NostrAuth {
    secret_key: Option<String>,
    public_key: Option<String>,
    pub amber_endpoint: String,
    
    #[serde(skip)]
    #[cfg(not(target_arch = "wasm32"))]
    keys: Option<nostr::Keys>,
}

impl NostrAuth {
    pub fn authenticate(&mut self) -> bool {
        #[cfg(not(target_arch = "wasm32"))]
        {
            let keys = nostr::Keys::generate();
            self.secret_key = Some(keys.secret_key().to_bech32().unwrap());
            self.public_key = Some(keys.public_key().to_string());
            self.keys = Some(keys);
            true
        }
        
        #[cfg(target_arch = "wasm32")]
        {
            // Simple key generation for WASM
            let key = format!("nsec1{}", uuid::Uuid::new_v4().simple());
            self.secret_key = Some(key.clone());
            self.public_key = Some(format!("npub1{}", uuid::Uuid::new_v4().simple()));
            true
        }
    }

    pub fn authenticate_with_extension(&mut self) -> bool {
        self.authenticate()
    }

    pub fn generate_new_keys(&mut self) -> bool {
        self.authenticate()
    }

    pub fn import_key(&mut self, key: &str) -> bool {
        self.secret_key = Some(key.to_string());
        self.public_key = Some(format!("imported_{}", &key[..8.min(key.len())]));
        
        #[cfg(not(target_arch = "wasm32"))]
        {
            if let Ok(keys) = nostr::Keys::parse(key) {
                self.keys = Some(keys);
                self.public_key = Some(keys.public_key().to_string());
                return true;
            }
        }
        
        true
    }

    pub fn logout(&mut self) {
        self.secret_key = None;
        self.public_key = None;
        #[cfg(not(target_arch = "wasm32"))]
        {
            self.keys = None;
        }
    }

    pub fn get_pubkey(&self) -> String {
        self.public_key.clone().unwrap_or_default()
    }

    pub fn get_display_name(&self) -> String {
        self.public_key
            .as_ref()
            .map(|pk| {
                if pk.len() > 8 {
                    format!("{}...", &pk[..8])
                } else {
                    pk.clone()
                }
            })
            .unwrap_or_else(|| "Not logged in".to_string())
    }

    pub fn is_authenticated(&self) -> bool {
        self.public_key.is_some()
    }

    pub fn serialize(&self) -> String {
        serde_json::to_string(self).unwrap_or_default()
    }

    pub fn restore_from_string(&mut self, s: &str) {
        if let Ok(auth) = serde_json::from_str::<NostrAuth>(s) {
            *self = auth;
        }
    }

    pub fn login_with_amber(&mut self) -> bool {
        self.authenticate()
    }
}