//! Export functionality for calculations

use crate::data_card::DataCard;
use anyhow::Result;

pub enum ExportFormat {
    Json,
    Csv,
    Text,
}

pub fn export_data_card(card: &DataCard, format: ExportFormat) -> Result<String> {
    match format {
        ExportFormat::Json => {
            Ok(serde_json::to_string_pretty(card)?)
        }
        ExportFormat::Csv => {
            let mut csv = String::from("Range,Elevation_MOA,Windage_10mph,Velocity,Energy\n");
            for point in &card.data_points {
                csv.push_str(&format!(
                    "{},{:.2},{:.2},{:.0},{:.0}\n",
                    point.range,
                    point.elevation_moa,
                    point.windage_10mph,
                    point.velocity,
                    point.energy
                ));
            }
            Ok(csv)
        }
        ExportFormat::Text => {
            let mut text = format!("# {}\n", card.title);
            text.push_str(&format!("Firearm: {}\n", card.firearm));
            text.push_str(&format!("Ammunition: {}\n\n", card.ammunition));
            text.push_str("Range | Elev | Wind | Vel  | Energy\n");
            text.push_str("------|------|------|------|-------\n");
            for point in &card.data_points {
                text.push_str(&format!(
                    "{:5.0} | {:4.1} | {:4.1} | {:4.0} | {:5.0}\n",
                    point.range,
                    point.elevation_moa,
                    point.windage_10mph,
                    point.velocity,
                    point.energy
                ));
            }
            Ok(text)
        }
    }
}