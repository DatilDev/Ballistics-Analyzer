//! Data card generation for field use
use crate::models::{CalculationData, TrajectoryPoint, EnvironmentalConditions};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataCard {
    pub title: String,
    pub firearm: String,
    pub ammunition: String,
    pub conditions: EnvironmentalConditions,
    pub data_points: Vec<DataPoint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
    pub range: f64,
    pub elevation_moa: f64,
    pub windage_10mph: f64,
    pub velocity: f64,
    pub energy: f64,
}

pub fn generate_data_card(
    input: &CalculationData,
    trajectory: &[TrajectoryPoint],
) -> DataCard {
    let data_points = trajectory
        .iter()
        .step_by(4) // Every 100 yards for 25-yard steps
        .map(|point| {
            // Calculate MOA from drop (1 MOA = 1.047 inches at 100 yards)
            let elevation_moa = if point.distance > 0.0 {
                point.drop / (1.047 * point.distance / 100.0)
            } else {
                0.0
            };
            
            DataPoint {
                range: point.distance,
                elevation_moa,
                windage_10mph: point.drift / 10.0, // Simplified wind calculation
                velocity: point.velocity,
                energy: point.energy,
            }
        })
        .collect();
    
    DataCard {
        title: "Field Data Card".to_string(),
        // FIX: Use firearm.name and ammunition.name from the actual profiles
        firearm: input.firearm.name.clone(),
        ammunition: input.ammunition.name.clone(),
        conditions: input.environment.clone(),  // Direct clone since types match
        data_points,
    }
}