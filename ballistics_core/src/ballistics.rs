use crate::models::{CalculationData, TrajectoryPoint};

pub struct BallisticsCalculator;

impl BallisticsCalculator {
    const GRAVITY: f64 = 32.174; // ft/s²
    #[allow(dead_code)]
    const AIR_DENSITY_SEA_LEVEL: f64 = 0.076474; // lb/ft³
    
    pub fn calculate_trajectory(data: &mut CalculationData) {
        let mut trajectory = Vec::new();
        let max_distance = data.target_distance.max(2000.0);
        let step = if max_distance <= 500.0 { 10.0 } else { 25.0 };
        
        // Environmental corrections
        let density_correction = Self::calculate_density_correction(
            data.temperature,
            data.pressure,
            data.humidity,
            data.altitude,
        );
        
        let corrected_bc = data.bc * density_correction;
        
        // Calculate trajectory points
        let mut current_distance = 0.0;
        let mut max_ordinate: f64 = 0.0;  // Fixed: Use f64 throughout
        
        while current_distance <= max_distance {
            let point = Self::calculate_point(
                data,
                current_distance,
                corrected_bc,
            );
            
            // Fixed: Both values are now f64
            if point.drop.abs() > max_ordinate.abs() {
                max_ordinate = point.drop;
            }
            
            trajectory.push(point);
            current_distance += step;
        }
        
        data.trajectory = trajectory;
        data.max_ordinate = max_ordinate;
        data.max_range = Self::calculate_max_range(data, corrected_bc);
    }
    
    fn calculate_point(
        data: &CalculationData,
        distance: f64,
        _corrected_bc: f64,
    ) -> TrajectoryPoint {
        // Simplified ballistic calculation
        let time_of_flight = if data.muzzle_velocity > 0.0 {
            distance / data.muzzle_velocity
        } else {
            0.0
        };
        
        let velocity = (data.muzzle_velocity - (distance * 0.5)).max(0.0); // Ensure non-negative
        
        // Drop calculation (simplified)
        let drop = if distance == 0.0 {
            0.0
        } else {
            let zero_drop = Self::calculate_drop_at_zero(data);
            let actual_drop = 0.5 * Self::GRAVITY * time_of_flight.powi(2) * 12.0;
            actual_drop - zero_drop - (distance * data.shooting_angle.to_radians().tan())
        };
        
        // Wind drift (simplified)
        let wind_drift = if distance == 0.0 {
            0.0
        } else {
            let wind_component = data.wind_speed * (data.wind_angle.to_radians()).sin();
            wind_component * time_of_flight * 12.0
        };
        
        // Convert to angular measurements
        let drop_moa = Self::inches_to_moa(drop, distance);
        let drop_mil = Self::inches_to_mil(drop, distance);
        let windage_moa = Self::inches_to_moa(wind_drift, distance);
        let windage_mil = Self::inches_to_mil(wind_drift, distance);
        
        // Energy calculation
        let energy = if data.bullet_weight > 0.0 && velocity > 0.0 {
            0.5 * (data.bullet_weight / 7000.0) * velocity.powi(2) / Self::GRAVITY
        } else {
            0.0
        };
        
        TrajectoryPoint {
            distance,
            drop,
            drop_moa,
            drop_mil,
            windage: wind_drift,
            windage_moa,
            windage_mil,
            velocity,
            energy,
            time: time_of_flight,
        }
    }
    
    fn calculate_drop_at_zero(data: &CalculationData) -> f64 {
        if data.muzzle_velocity > 0.0 {
            let time = data.zero_range / data.muzzle_velocity;
            0.5 * Self::GRAVITY * time.powi(2) * 12.0
        } else {
            0.0
        }
    }
    
    fn calculate_density_correction(
        temperature: f64,
        pressure: f64,
        humidity: f64,
        altitude: f64,
    ) -> f64 {
        // Standard conditions
        let std_temp = 59.0;
        let std_pressure = 29.92;
        
        // Temperature correction
        let temp_ratio = (459.67 + std_temp) / (459.67 + temperature);
        
        // Pressure correction
        let pressure_ratio = if std_pressure > 0.0 {
            pressure / std_pressure
        } else {
            1.0
        };
        
        // Humidity correction (simplified)
        let humidity_factor = 1.0 - (humidity * 0.001);
        
        // Altitude correction
        let altitude_factor = (1.0 - altitude * 0.00002).max(0.5);
        
        temp_ratio * pressure_ratio * humidity_factor * altitude_factor
    }
    
    fn calculate_max_range(data: &CalculationData, _corrected_bc: f64) -> f64 {
        // Simplified max range calculation
        if data.muzzle_velocity > 0.0 {
            let launch_angle = 35.0_f64.to_radians(); // Optimal angle (simplified)
            let v0 = data.muzzle_velocity;
            
            (v0.powi(2) * (2.0 * launch_angle).sin()) / Self::GRAVITY
        } else {
            0.0
        }
    }
    
    fn inches_to_moa(inches: f64, distance_yards: f64) -> f64 {
        if distance_yards > 0.0 {
            (inches / distance_yards) * 95.49
        } else {
            0.0
        }
    }
    
    fn inches_to_mil(inches: f64, distance_yards: f64) -> f64 {
        if distance_yards > 0.0 {
            (inches / distance_yards) * 27.78
        } else {
            0.0
        }
    }
    
    pub fn calculate_spin_drift(
        data: &CalculationData,
        distance: f64,
    ) -> f64 {
        if data.muzzle_velocity > 0.0 {
            // Miller's formula for spin drift
            let sg = Self::calculate_stability(data);
            let time = distance / data.muzzle_velocity;
            
            1.25 * (sg + 1.2) * time.powi(2)
        } else {
            0.0
        }
    }
    
    pub fn calculate_stability(data: &CalculationData) -> f64 {
        // Greenhill formula
        let caliber_value = data.caliber.parse::<f64>().unwrap_or(0.308);
        
        if caliber_value > 0.0 && data.barrel_twist > 0.0 && data.bullet_length > 0.0 {
            let twist_calibers = data.barrel_twist / caliber_value;
            let length_calibers = data.bullet_length / caliber_value;
            
            if length_calibers > 0.0 {
                30.0 / (twist_calibers / length_calibers)
            } else {
                1.0
            }
        } else {
            1.0
        }
    }
    
    pub fn calculate_coriolis(
        data: &CalculationData,
        distance: f64,
    ) -> (f64, f64) {
        if data.muzzle_velocity > 0.0 {
            let omega = 7.292e-5; // Earth's rotation rate (rad/s)
            let time = distance / data.muzzle_velocity;
            let lat_rad = data.latitude.to_radians();
            let az_rad = data.azimuth.to_radians();
            
            let horizontal = 2.0 * omega * time * distance * lat_rad.sin() * az_rad.sin();
            let vertical = 2.0 * omega * time * distance * lat_rad.cos();
            
            (horizontal, vertical)
        } else {
            (0.0, 0.0)
        }
    }
}
