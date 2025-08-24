//! Unit conversion utilities

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum LengthUnit {
    Inches,
    Feet,
    Yards,
    Meters,
    Centimeters,
    Millimeters,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VelocityUnit {
    FPS,  // Feet per second
    MPS,  // Meters per second
    MPH,  // Miles per hour
    KPH,  // Kilometers per hour
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum WeightUnit {
    Grains,
    Grams,
    Ounces,
    Pounds,
    Kilograms,
}

// Conversion functions
pub fn convert_length(value: f64, from: LengthUnit, to: LengthUnit) -> f64 {
    let meters = match from {
        LengthUnit::Inches => value * 0.0254,
        LengthUnit::Feet => value * 0.3048,
        LengthUnit::Yards => value * 0.9144,
        LengthUnit::Meters => value,
        LengthUnit::Centimeters => value * 0.01,
        LengthUnit::Millimeters => value * 0.001,
    };
    
    match to {
        LengthUnit::Inches => meters / 0.0254,
        LengthUnit::Feet => meters / 0.3048,
        LengthUnit::Yards => meters / 0.9144,
        LengthUnit::Meters => meters,
        LengthUnit::Centimeters => meters * 100.0,
        LengthUnit::Millimeters => meters * 1000.0,
    }
}

pub fn convert_velocity(value: f64, from: VelocityUnit, to: VelocityUnit) -> f64 {
    let mps = match from {
        VelocityUnit::FPS => value * 0.3048,
        VelocityUnit::MPS => value,
        VelocityUnit::MPH => value * 0.44704,
        VelocityUnit::KPH => value * 0.27778,
    };
    
    match to {
        VelocityUnit::FPS => mps / 0.3048,
        VelocityUnit::MPS => mps,
        VelocityUnit::MPH => mps / 0.44704,
        VelocityUnit::KPH => mps / 0.27778,
    }
}

pub fn convert_weight(value: f64, from: WeightUnit, to: WeightUnit) -> f64 {
    let grams = match from {
        WeightUnit::Grains => value * 0.06479891,
        WeightUnit::Grams => value,
        WeightUnit::Ounces => value * 28.34952,
        WeightUnit::Pounds => value * 453.59237,
        WeightUnit::Kilograms => value * 1000.0,
    };
    
    match to {
        WeightUnit::Grains => grams / 0.06479891,
        WeightUnit::Grams => grams,
        WeightUnit::Ounces => grams / 28.34952,
        WeightUnit::Pounds => grams / 453.59237,
        WeightUnit::Kilograms => grams / 1000.0,
    }
}

// MOA/MIL conversions
pub fn moa_to_inches(moa: f64, range_yards: f64) -> f64 {
    moa * 1.047 * range_yards / 100.0
}

pub fn mils_to_inches(mils: f64, range_yards: f64) -> f64 {
    mils * 0.36 * range_yards
}