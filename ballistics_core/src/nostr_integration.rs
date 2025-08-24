//! Nostr integration (feature-gated)

#[cfg(feature = "nostr")]
use anyhow::Result;

#[cfg(feature = "nostr")]
pub fn share_to_nostr(_data: &str) -> Result<()> {
    // Placeholder implementation
    Ok(())
}

#[cfg(not(feature = "nostr"))]
pub fn share_to_nostr(_data: &str) -> anyhow::Result<()> {
    anyhow::bail!("Nostr feature not enabled")
}