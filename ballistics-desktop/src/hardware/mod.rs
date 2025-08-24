#[cfg(feature = "hardware")]
pub struct HardwareManager {
    // Hardware management implementation
}

#[cfg(feature = "hardware")]
impl Default for HardwareManager {
    fn default() -> Self {
        Self {}
    }
}