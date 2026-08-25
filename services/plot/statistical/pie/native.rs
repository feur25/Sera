use crate::plot::default::PlotRenderContext;

pub fn render_pie_native(ctx: PlotRenderContext) {
    let total: f64 = ctx
        .visible_indices
        .iter()
        .map(|&i| ctx.values[i].max(0.0))
        .sum();
    if total <= 0.0 {
        return;
    }

    let center = ctx.plot_rect.center();
    let radius = ctx.plot_rect.width().min(ctx.plot_rect.height()) * 0.5 - 12.0;
    if radius <= 0.0 {
        return;
    }

    let mut start_angle = -std::f32::consts::FRAC_PI_2;
    for &actual_idx in ctx.visible_indices {
        let value = ctx.values[actual_idx].max(0.0);
        if value <= 0.0 {
            continue;
        }
        let sweep = (value / total) as f32 * std::f32::consts::TAU;
        let end_angle = start_angle + sweep;

        let color = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            color
        };

        let steps = ((sweep.abs() / (std::f32::consts::TAU / 96.0)).ceil() as usize).max(1);
        let mut points = Vec::with_capacity(steps + 2);
        points.push(center);
        for step in 0..=steps {
            let t = start_angle + sweep * (step as f32 / steps as f32);
            points.push(center + egui::vec2(t.cos(), t.sin()) * radius);
        }
        ctx.painter.add(egui::Shape::convex_polygon(
            points,
            display_color,
            egui::Stroke::new(1.0, egui::Color32::WHITE),
        ));

        let mid_angle = start_angle + sweep * 0.5;
        let label_pos = center + egui::vec2(mid_angle.cos(), mid_angle.sin()) * (radius + 14.0);
        let label_text = ctx
            .labels
            .get(actual_idx)
            .map(|s| s.as_str())
            .unwrap_or("");
        if !label_text.is_empty() {
            ctx.painter.text(
                label_pos,
                egui::Align2::CENTER_CENTER,
                label_text,
                egui::FontId::proportional(10.0),
                egui::Color32::from_gray(100),
            );
        }

        start_angle = end_angle;
    }
}
