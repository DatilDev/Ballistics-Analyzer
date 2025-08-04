use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct ProjectileData {
    pub caliber: String,
    pub mass: f64,         // grains
    pub velocity: f64,     // ft/s
    pub bc: f64,           // ballistic coefficient
    pub zero_range: f64,   // yards
    pub sight_height: f64, // inches

    // Environmental
    pub temperature: f64, // Fahrenheit
    pub pressure: f64,    // inHg
    pub humidity: f64,    // percentage
    pub altitude: f64,    // feet
    pub wind_speed: f64,  // mph
    pub wind_angle: f64,  // degrees
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct CalculationNote {
    pub text: String,
    pub timestamp: String,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrajectoryPoint {
    pub distance: f64,       // yards
    pub drop: f64,           // inches
    pub drift: f64,          // inches
    pub velocity: f64,       // ft/s
    pub energy: f64,         // ft-lbs
    pub time: f64,           // seconds
    pub moa_adjustment: f64, // MOA
    pub mil_adjustment: f64, // MILS
}

#[derive(Clone, Serialize, Deserialize)]
pub struct TrajectoryResult {
    pub trajectory_points: Vec<TrajectoryPoint>,
    pub max_range: f64,
    pub max_ordinate: f64,
    pub zero_offset: f64,
}

pub struct BallisticsCalculator;

impl Default for BallisticsCalculator {
    fn default() -> Self {
        Self
    }
}

impl BallisticsCalculator {
    pub fn calculate(&self, data: &ProjectileData) -> TrajectoryResult {
        let mut points = Vec::new();

        // Air density calculation based on conditions
        let air_density = self.calculate_air_density(
            data.temperature,
            data.pressure,
            data.humidity,
            data.altitude,
        );

        // Convert units
        let mass_lb = data.mass / 7000.0; // grains to pounds
        let wind_fps = data.wind_speed * 1.467; // mph to ft/s

        // Calculate sight angle for zero
        let zero_angle = self.calculate_zero_angle(data);

        // Calculate trajectory for various ranges
        let ranges = vec![
            0, 25, 50, 75, 100, 125, 150, 175, 200, 225, 250, 275, 300, 350, 400, 450, 500, 600,
            700, 800, 900, 1000,
        ];

        for &range_yards in &ranges {
            let range_feet = range_yards as f64 * 3.0;

            let point = self.calculate_point(
                data,
                range_feet,
                range_yards as f64,
                air_density,
                wind_fps,
                zero_angle,
                mass_lb,
            );

            points.push(point);
        }

        // Max ordinate as max absolute drop above zero reference
        let max_ordinate = points
            .iter()
            .map(|p| p.drop.abs())
            .fold(0.0, f64::max);

        TrajectoryResult {
            trajectory_points: points,
            max_range: 1000.0,
            max_ordinate,
            zero_offset: zero_angle,
        }
    }

    fn calculate_air_density(
        &self,
        temp_f: f64,
        pressure: f64,
        humidity: f64,
        altitude: f64,
    ) -> f64 {
        let _ = humidity; // placeholder, not used in this simplified model

        // Convert to absolute temperature
        let temp_r = temp_f + 459.67;

        // Standard sea level values
        let std_temp = 518.67; // Rankine
        let std_pressure = 29.92; // inHg

        // Altitude adjustment
        let pressure_alt = pressure * (1.0 - 0.0000068756 * altitude).powf(5.2559);

        // Calculate density ratio
        let density_ratio = (pressure_alt / std_pressure) * (std_temp / temp_r);

        0.076474 * density_ratio // lb/ft³
    }

    fn calculate_zero_angle(&self, data: &ProjectileData) -> f64 {
        // Simplified zero calculation
        let zero_feet = data.zero_range * 3.0;
        if zero_feet <= 0.0 || data.velocity <= 0.0 {
            return 0.0;
        }
        let drop_at_zero = 0.5 * 32.174 * (zero_feet / data.velocity).powf(2.0);
        ((drop_at_zero + data.sight_height / 12.0) / zero_feet).atan()
    }

    fn calculate_point(
        &self,
        data: &ProjectileData,
        range_feet: f64,
        range_yards: f64,
        air_density: f64,
        wind_fps: f64,
        zero_angle: f64,
        mass_lb: f64,
    ) -> TrajectoryPoint {
        let tof = if data.velocity > 0.0 {
            range_feet / data.velocity
        } else {
            0.0
        };

        // Velocity at range (with drag, simplified)
        let drag_factor = 1.0 - (0.0001 * data.bc * air_density * range_yards);
        let velocity_at_range = (data.velocity * drag_factor.max(0.3)).max(0.0);

        // Drop calculation
        let total_drop = 0.5 * 32.174 * tof * tof * 12.0; // inches
        let sight_adjustment = range_feet * zero_angle.tan() * 12.0;
        let apparent_drop = total_drop - sight_adjustment - data.sight_height;

        // Wind drift
        let drift = if data.bc.abs() > f64::EPSILON {
            wind_fps * tof * data.wind_angle.to_radians().sin() * 12.0 / data.bc
        } else {
            0.0
        };

        // Energy (approx)
        let energy = if velocity_at_range > 0.0 {
            0.5 * mass_lb * velocity_at_range * velocity_at_range / 32.174
        } else {
            0.0
        };

        // Angular adjustments
        let moa_adjustment = if range_yards > 0.0 {
            apparent_drop / (range_yards / 100.0 * 1.047)
        } else {
            0.0
        };

        let mil_adjustment = if range_yards > 0.0 {
            apparent_drop / (range_yards / 100.0 * 3.6)
        } else {
            0.0
        };

        TrajectoryPoint {
            distance: range_yards,
            drop: apparent_drop,
            drift,
            velocity: velocity_at_range,
            energy,
            time: tof,
            moa_adjustment,
            mil_adjustment,
        }
    }
}