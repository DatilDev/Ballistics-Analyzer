#[cfg(feature = "nostr")]
use nostr_sdk::prelude::*;
use anyhow::Result;

pub struct NostrAuth {
    #[cfg(feature = "nostr")]
    client: Client,
    keys: Option<Keys>,
}

impl NostrAuth {
    pub async fn new() -> Result<Self> {
        #[cfg(feature = "nostr")]
        {
            let client = Client::new(&Keys::generate());
            Ok(Self {
                client,
                keys: None,
            })
        }
        
        #[cfg(not(feature = "nostr"))]
        Ok(Self {
            keys: None,
        })
    }
    
    pub fn is_authenticated(&self) -> bool {
        self.keys.is_some()
    }
}