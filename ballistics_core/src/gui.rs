//! GUI components (feature-gated)

#[cfg(feature = "gui")]
use egui::Ui;
#[cfg(feature = "gui")]
use crate::models::TrajectoryPoint;

#[cfg(feature = "gui")]
pub fn render_trajectory_plot(ui: &mut Ui, _trajectory: &[TrajectoryPoint]) {
    // Placeholder for trajectory plotting
    ui.label("Trajectory plot would go here");
}