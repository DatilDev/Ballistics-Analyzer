use eframe::egui;
use crate::ballistics::TrajectoryResult;

/// Draw a trajectory graph visualization with zoom controls and inverted display
pub fn draw_trajectory_graph(ui: &mut egui::Ui, results: &TrajectoryResult) {
    // Add zoom controls
    static mut ZOOM_LEVEL: f32 = 1.0;
    
    ui.horizontal(|ui| {
        ui.label("Zoom:");
        if ui.button("➖").clicked() {
            unsafe { ZOOM_LEVEL = (ZOOM_LEVEL * 1.5).min(10.0); }
        }
        if ui.button("🔄").clicked() {
            unsafe { ZOOM_LEVEL = 1.0; }
        }
        if ui.button("➕").clicked() {
            unsafe { ZOOM_LEVEL = (ZOOM_LEVEL / 1.5).max(0.1); }
        }
        ui.separator();
        unsafe {
            ui.label(format!("Range: 0-{:.0} yards", results.max_range * ZOOM_LEVEL as f64));
        }
    });
    
    ui.separator();
    
    let desired_size = egui::vec2(ui.available_width(), 350.0);
    let (response, painter) = ui.allocate_painter(desired_size, egui::Sense::hover());
    let rect = response.rect;
    
    // Draw background
    painter.rect_filled(rect, 4.0, egui::Color32::from_gray(30));
    
    // Draw border
    painter.rect_stroke(rect, 4.0, egui::Stroke::new(1.0, egui::Color32::from_gray(60)));
    
    // Get trajectory points and find bounds with zoom
    let points = &results.trajectory_points;
    if points.is_empty() {
        ui.label("No trajectory data available");
        return;
    }
    
    let zoom = unsafe { ZOOM_LEVEL as f64 };
    let max_display_distance = results.max_range * zoom;
    
    // Filter points within zoom range
    let visible_points: Vec<_> = points
        .iter()
        .filter(|p| p.distance <= max_display_distance)
        .collect();
    
    if visible_points.is_empty() {
        ui.label("No data in current zoom range");
        return;
    }
    
    // Find drop bounds for visible points
    let min_drop = visible_points.iter().map(|p| p.drop).fold(0.0_f64, f64::min);
    let max_drop = visible_points.iter().map(|p| p.drop).fold(0.0_f64, f64::max);
    let drop_range = (max_drop - min_drop).max(10.0);
    
    // Calculate margins and plot area
    let margin_left = 50.0;
    let margin_right = 20.0;
    let margin_top = 30.0;
    let margin_bottom = 40.0;
    
    let plot_rect = egui::Rect::from_min_max(
        rect.min + egui::vec2(margin_left, margin_top),
        rect.max - egui::vec2(margin_right, margin_bottom),
    );
    
    // Draw grid lines
    let grid_color = egui::Color32::from_gray(50);
    let text_color = egui::Color32::from_gray(180);
    
    // Draw zero reference line (horizontal line at y=0)
    let zero_y = plot_rect.top() + (((0.0 - max_drop).abs() / drop_range) as f32) * plot_rect.height();
    painter.line_segment(
        [egui::pos2(plot_rect.left(), zero_y), egui::pos2(plot_rect.right(), zero_y)],
        egui::Stroke::new(1.5, egui::Color32::from_rgb(150, 150, 150)),
    );
    painter.text(
        egui::pos2(plot_rect.left() - 35.0, zero_y),
        egui::Align2::RIGHT_CENTER,
        "0",
        egui::FontId::proportional(10.0),
        egui::Color32::from_rgb(150, 150, 150),
    );
    
    // Vertical grid lines (distance)
    let num_grid_lines = 8;
    for i in 0..=num_grid_lines {
        let x_norm = i as f32 / num_grid_lines as f32;
        let x = plot_rect.left() + x_norm * plot_rect.width();
        
        // Draw grid line
        painter.line_segment(
            [egui::pos2(x, plot_rect.top()), egui::pos2(x, plot_rect.bottom())],
            egui::Stroke::new(0.5, grid_color),
        );
        
        // Distance labels
        let distance = (max_display_distance * (x_norm as f64)) as i32;
        painter.text(
            egui::pos2(x, rect.bottom() - 20.0),
            egui::Align2::CENTER_TOP,
            format!("{}", distance),
            egui::FontId::proportional(10.0),
            text_color,
        );
    }
    
    // Horizontal grid lines (drop) - now showing negative values going down
    for i in 0..=5 {
        let y_norm = i as f32 / 5.0;
        let y = plot_rect.top() + y_norm * plot_rect.height();
        
        painter.line_segment(
            [egui::pos2(plot_rect.left(), y), egui::pos2(plot_rect.right(), y)],
            egui::Stroke::new(0.5, grid_color),
        );
        
        // Drop labels - showing trajectory dropping (negative values)
        let drop_value = max_drop - (drop_range * (y_norm as f64));
        if (drop_value - 0.0).abs() > 0.1 {  // Don't duplicate zero label
            painter.text(
                egui::pos2(rect.left() + 25.0, y),
                egui::Align2::LEFT_CENTER,
                format!("{:.0}", drop_value),
                egui::FontId::proportional(10.0),
                text_color,
            );
        }
    }
    
    // Convert trajectory points to screen coordinates (inverted to show drop)
    let screen_points: Vec<egui::Pos2> = visible_points
        .iter()
        .map(|p| {
            let x_norm = ((p.distance / max_display_distance) as f32).clamp(0.0, 1.0);
            let y_norm = (((max_drop - p.drop) / drop_range) as f32).clamp(0.0, 1.0);
            
            egui::pos2(
                plot_rect.left() + x_norm * plot_rect.width(),
                plot_rect.top() + y_norm * plot_rect.height(),
            )
        })
        .collect();
    
    // Draw trajectory line (should curve downward as distance increases)
    if screen_points.len() >= 2 {
        for i in 0..screen_points.len() - 1 {
            painter.line_segment(
                [screen_points[i], screen_points[i + 1]],
                egui::Stroke::new(2.5, egui::Color32::from_rgb(100, 200, 255)),
            );
        }
    }
    
    // Draw points with hover information
    for (i, point) in screen_points.iter().enumerate() {
        let hover_rect = egui::Rect::from_center_size(*point, egui::vec2(10.0, 10.0));
        let is_hovered = hover_rect.contains(response.hover_pos().unwrap_or(egui::Pos2::ZERO));
        
        let color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 100)
        } else {
            egui::Color32::from_rgb(150, 220, 255)
        };
        
        painter.circle_filled(*point, 3.5, color);
        
        // Show tooltip on hover
        if is_hovered {
            if let Some(p) = visible_points.get(i) {
                let tooltip_pos = *point + egui::vec2(10.0, -30.0);
                let tooltip_text = format!(
                    "{:.0} yds\nDrop: {:.1}\"\nVel: {:.0} fps",
                    p.distance, p.drop, p.velocity
                );
                
                // Draw tooltip background
                let galley = painter.layout_no_wrap(
                    tooltip_text.clone(),
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
                let tooltip_rect = egui::Rect::from_min_size(
                    tooltip_pos,
                    galley.size() + egui::vec2(8.0, 6.0),
                );
                painter.rect_filled(
                    tooltip_rect,
                    3.0,
                    egui::Color32::from_rgba_premultiplied(50, 50, 50, 230),
                );
                painter.text(
                    tooltip_pos + egui::vec2(4.0, 3.0),
                    egui::Align2::LEFT_TOP,
                    tooltip_text,
                    egui::FontId::proportional(11.0),
                    egui::Color32::WHITE,
                );
            }
        }
    }
    
    // Draw axis labels
    painter.text(
        egui::pos2(rect.center().x, rect.bottom() - 5.0),
        egui::Align2::CENTER_BOTTOM,
        "Distance (yards)",
        egui::FontId::proportional(12.0),
        text_color,
    );
    
    // Rotated Y-axis label
    painter.text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        "Bullet\nDrop\n(inches)",
        egui::FontId::proportional(12.0),
        text_color,
    );
    
    // Draw title with current zoom level
    painter.text(
        egui::pos2(rect.center().x, rect.top() + 10.0),
        egui::Align2::CENTER_TOP,
        format!("📈 Trajectory (Zoom: {:.1}x)", 1.0 / zoom),
        egui::FontId::proportional(14.0),
        egui::Color32::from_gray(220),
    );
    
    // Show key stats below the graph
    ui.separator();
    ui.horizontal(|ui| {
        ui.label(format!("📏 Max Range: {} yards", results.max_range));
        ui.separator();
        ui.label(format!("📉 Max Drop: {:.1} inches", 
            visible_points.iter().map(|p| p.drop.abs()).fold(0.0_f64, f64::max)));
        ui.separator();
        ui.label(format!("🎯 Zero: {:.2} MOA", results.zero_offset));
        ui.separator();
        
        // Show point under cursor if hovering
        if let Some(hover_pos) = response.hover_pos() {
            let x_norm = ((hover_pos.x - plot_rect.left()) / plot_rect.width()).clamp(0.0, 1.0);
            let approx_distance = max_display_distance * (x_norm as f64);
            ui.label(format!("📍 {:.0} yds", approx_distance));
        }
    });
}