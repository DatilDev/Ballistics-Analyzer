//! Utility functions

//use anyhow::Result;

/// Calculate standard atmosphere
pub fn standard_atmosphere(altitude: f64) -> (f64, f64, f64) {
    // Returns (pressure, temperature, density)
    let pressure = 29.92 * (1.0 - altitude * 0.0000068756).powf(5.2559);
    let temperature = 59.0 - (altitude * 0.00356616);
    let density = 1.0; // Simplified
    (pressure, temperature, density)
}

/// Calculate density altitude
pub fn density_altitude(pressure: f64, temperature: f64, humidity: f64) -> f64 {
    // Simplified calculation
    let std_pressure = 29.92;
    let std_temp = 59.0;
    
    let pressure_alt = (1.0 - (pressure / std_pressure).powf(0.190284)) * 145366.45;
    let temp_correction = (temperature - std_temp) * 120.0;
    
    pressure_alt + temp_correction
}

/// Format time duration
pub fn format_duration(seconds: f64) -> String {
    if seconds < 1.0 {
        format!("{:.3}s", seconds)
    } else if seconds < 60.0 {
        format!("{:.1}s", seconds)
    } else {
        let minutes = (seconds / 60.0) as u32;
        let secs = seconds % 60.0;
        format!("{}m {:.1}s", minutes, secs)
    }
}

/// Generate unique ID
pub fn generate_id() -> String {
    uuid::Uuid::new_v4().to_string()
}