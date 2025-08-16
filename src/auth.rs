use nostr_sdk::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Default, Serialize, Deserialize, Clone)]
pub struct NostrAuth {
    #[serde(skip)]
    client: Option<Client>,
    #[serde(skip)]
    keys: Option<Keys>,
    // For Amber remote signer configuration (if used)
    pub amber_endpoint: String,
}

impl NostrAuth {
    pub fn authenticate(&mut self) -> bool {
        let keys = Keys::generate();
        self.keys = Some(keys.clone());
        let client = Client::new(&keys);
        self.client = Some(client);
        true
    }

    // For NIP-07 extension on web; here just delegate to authenticate()
    pub fn authenticate_with_extension(&mut self) -> bool {
        self.authenticate()
    }

    pub fn generate_new_keys(&mut self) -> bool {
        self.authenticate()
    }

    pub fn import_key(&mut self, key: &str) -> bool {
        // Accept bech32 nsec or hex-encoded secret key
        let parsed = if key.starts_with("nsec") {
            // bech32
            Keys::parse(key)
        } else {
            // hex secret
            SecretKey::parse(key).map(Keys::new)
        };

        match parsed {
            Ok(keys) => {
                self.keys = Some(keys.clone());
                let client = Client::new(&keys);
                self.client = Some(client);
                true
            }
            Err(_) => false,
        }
    }

    pub fn logout(&mut self) {
        self.client = None;
        self.keys = None;
    }

    pub fn get_pubkey(&self) -> String {
        self.keys
            .as_ref()
            .map(|k| k.public_key().to_string())
            .unwrap_or_default()
    }

    pub fn get_display_name(&self) -> String {
        self.keys
            .as_ref()
            .map(|k| {
                let pk = k.public_key().to_string();
                if pk.len() > 8 {
                    format!("{}...", &pk[..8])
                } else {
                    pk
                }
            })
            .unwrap_or_else(|| "Not logged in".to_string())
    }

    pub fn is_authenticated(&self) -> bool {
        self.keys.is_some()
    }

    pub fn get_client(&self) -> Option<&Client> {
        self.client.as_ref()
    }

    pub fn get_keys(&self) -> Option<&Keys> {
        self.keys.as_ref()
    }

    // Persistence for wasm storage
    pub fn serialize(&self) -> String {
        // Only persist public data you need; here we just store pubkey as example
        serde_json::json!({
            "pubkey": self.get_pubkey(),
            "amber_endpoint": self.amber_endpoint,
        })
        .to_string()
    }

    pub fn restore_from_string(&mut self, s: &str) {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(s) {
            if let Some(ep) = v.get("amber_endpoint").and_then(|x| x.as_str()) {
                self.amber_endpoint = ep.to_string();
            }
            // Do not restore secret keys from storage here for security.
        }
    }

    // Amber remote signer hook (stub – integrate with your Amber/NIP-46 flow)
    pub fn login_with_amber(&mut self) -> bool {
        // TODO: implement actual handshake with Amber remote signer using amber_endpoint
        // For now, just generate ephemeral keys as a placeholder:
        self.authenticate()
    }
}