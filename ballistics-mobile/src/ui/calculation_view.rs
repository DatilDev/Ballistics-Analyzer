use crate::app::{MobileApp, ViewType};
use egui::{Ui, ScrollArea, Grid, CollapsingHeader, Button, RichText, Color32, plot};

pub fn render(app: &mut MobileApp, ui: &mut Ui) {
    // Show main view for input, but with results emphasis
    super::main_view::render(app, ui);
    
    // Additional calculation-specific features
    if !app.calculation_data.trajectory.is_empty() {
        ui.separator();
        render_trajectory_plot(app, ui);
        render_advanced_results(app, ui);
    }
}

fn render_trajectory_plot(app: &mut MobileApp, ui: &mut Ui) {
    ui.heading("📈 Trajectory Plot");
    
    let trajectory_points: plot::PlotPoints = app.calculation_data.trajectory
        .iter()
        .map(|p| [p.distance, -p.drop])
        .collect();
    
    let windage_points: plot::PlotPoints = app.calculation_data.trajectory
        .iter()
        .map(|p| [p.distance, p.windage])
        .collect();
    
    plot::Plot::new("trajectory_plot")
        .height(200.0)
        .allow_zoom(true)
        .allow_drag(true)
        .show(ui, |plot_ui| {
            plot_ui.line(plot::Line::new(trajectory_points)
                .name("Drop")
                .color(Color32::from_rgb(200, 50, 50)));
            
            plot_ui.line(plot::Line::new(windage_points)
                .name("Windage")
                .color(Color32::from_rgb(50, 50, 200)));
        });
}

fn render_advanced_results(app: &mut MobileApp, ui: &mut Ui) {
    CollapsingHeader::new("🔬 Advanced Results")
        .default_open(false)
        .show(ui, |ui| {
            // Stability calculation
            let stability = ballistics_core::BallisticsCalculator::calculate_stability(&app.calculation_data);
            ui.label(format!("Gyroscopic Stability: {:.2}", stability));
            
            if stability < 1.0 {
                ui.colored_label(Color32::RED, "⚠ Unstable - bullet may tumble");
            } else if stability < 1.3 {
                ui.colored_label(Color32::YELLOW, "⚠ Marginally stable");
            } else if stability > 2.5 {
                ui.colored_label(Color32::YELLOW, "⚠ Over-stabilized");
            } else {
                ui.colored_label(Color32::GREEN, "✓ Optimal stability");
            }
            
            ui.separator();
            
            // Spin drift
            if let Some(point) = app.calculation_data.trajectory.last() {
                let spin_drift = ballistics_core::BallisticsCalculator::calculate_spin_drift(
                    &app.calculation_data,
                    point.distance
                );
                ui.label(format!("Spin Drift at {:.0} yd: {:.2} in", point.distance, spin_drift));
            }
            
            // Coriolis effect
            if app.calculation_data.latitude != 0.0 {
                if let Some(point) = app.calculation_data.trajectory.last() {
                    let (horiz, vert) = ballistics_core::BallisticsCalculator::calculate_coriolis(
                        &app.calculation_data,
                        point.distance
                    );
                    ui.label(format!("Coriolis (Horizontal): {:.2} in", horiz));
                    ui.label(format!("Coriolis (Vertical): {:.2} in", vert));
                }
            }
            
            ui.separator();
            
            // Danger space
            render_danger_space(app, ui);
            
            // MPBR calculation
            render_mpbr(app, ui);
        });
}

fn render_danger_space(app: &mut MobileApp, ui: &mut Ui) {
    ui.label("Danger Space (6\" vital zone):");
    
    let mut danger_spaces = Vec::new();
    let vital_zone_height = 6.0; // inches
    
    for i in 1..app.calculation_data.trajectory.len() {
        let prev = &app.calculation_data.trajectory[i - 1];
        let curr = &app.calculation_data.trajectory[i];
        
        if prev.drop.abs() <= vital_zone_height / 2.0 && 
           curr.drop.abs() <= vital_zone_height / 2.0 {
            danger_spaces.push((prev.distance, curr.distance));
        }
    }
    
    for (start, end) in danger_spaces {
        ui.label(format!("  {:.0} - {:.0} yd", start, end));
    }
}

fn render_mpbr(app: &mut MobileApp, ui: &mut Ui) {
    // Maximum Point Blank Range calculation
    let vital_zone = 6.0; // inches
    let mut mpbr = 0.0;
    
    for point in &app.calculation_data.trajectory {
        if point.drop.abs() <= vital_zone / 2.0 {
            mpbr = point.distance;
        } else {
            break;
        }
    }
    
    ui.label(format!("Maximum Point Blank Range: {:.0} yd", mpbr));
    ui.small(format!("(for {}\" vital zone)", vital_zone));
}