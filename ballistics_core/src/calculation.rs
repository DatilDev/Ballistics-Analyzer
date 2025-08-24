//! Calculation engine for ballistics

use crate::models::{CalculationData, TrajectoryPoint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationResult {
    pub trajectory: Vec<TrajectoryPoint>,
    pub metadata: CalculationMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CalculationMetadata {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub input_hash: String,
    pub calculation_time_ms: u64,
}

pub fn perform_calculation(input: &CalculationData) -> CalculationResult {
    let start = std::time::Instant::now();
    
    // Simple trajectory calculation for demonstration
    let mut trajectory = Vec::new();
    for distance in (0..=1000).step_by(25) {
        trajectory.push(TrajectoryPoint {
            distance: distance as f64,
            drop: (distance as f64 * 0.01).powi(2),
            drift: 0.0,
            velocity: 2800.0 - (distance as f64 * 0.5),
            energy: 2000.0 - (distance as f64 * 0.8),
            time: distance as f64 / 2800.0,
        });
    }
    
    CalculationResult {
        trajectory,
        metadata: CalculationMetadata {
            timestamp: chrono::Utc::now(),
            input_hash: format!("{:?}", input),
            calculation_time_ms: start.elapsed().as_millis() as u64,
        },
    }
}