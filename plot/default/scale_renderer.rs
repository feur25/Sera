pub fn render_scale_labels(
    painter: &egui::Painter,
    plot_rect: egui::Rect,
    max_val: f64,
) {
    let center = plot_rect.center();
    let width = plot_rect.width();
    let height = plot_rect.height();
    
    let label_font = egui::FontId::monospace(11.0);
    let label_color = egui::Color32::from_rgb(180, 180, 200);
    
    let scale_marks = vec![0.0, 0.25, 0.5, 0.75, 1.0];
    
    for mark in scale_marks {
        let val = (max_val * mark as f64).max(0.0);
        let val_str = format!("{:.1}", val);
        
        let label_x = center.x - width * 0.15;
        let label_y = center.y - height * 0.2 * (mark as f32 - 0.3);
        
        painter.text(
            egui::pos2(label_x, label_y),
            egui::Align2::RIGHT_CENTER,
            &val_str,
            label_font.clone(),
            label_color,
        );
    }
}
