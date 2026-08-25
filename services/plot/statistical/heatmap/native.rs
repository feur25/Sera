use crate::plot::default::PlotRenderContext;

fn lerp_color32(a: egui::Color32, b: egui::Color32, t: f32) -> egui::Color32 {
    let t = t.clamp(0.0, 1.0);
    egui::Color32::from_rgb(
        (a.r() as f32 + (b.r() as f32 - a.r() as f32) * t) as u8,
        (a.g() as f32 + (b.g() as f32 - a.g() as f32) * t) as u8,
        (a.b() as f32 + (b.b() as f32 - a.b() as f32) * t) as u8,
    )
}

pub fn render_heatmap_native(ctx: PlotRenderContext) {
    let n = ctx.visible_indices.len();
    if n == 0 {
        return;
    }
    let cols = (n as f64).sqrt().ceil().max(1.0) as usize;
    let rows = n.div_ceil(cols);

    let cell_w = ctx.plot_rect.width() / cols as f32;
    let cell_h = ctx.plot_rect.height() / rows as f32;

    for (pos, &actual_idx) in ctx.visible_indices.iter().enumerate() {
        let row = pos / cols;
        let col = pos % cols;

        let min = egui::pos2(
            ctx.plot_rect.left() + col as f32 * cell_w,
            ctx.plot_rect.top() + row as f32 * cell_h,
        );
        let rect = egui::Rect::from_min_size(min, egui::vec2(cell_w, cell_h)).shrink(1.0);

        let norm_val = (ctx.values[actual_idx] / ctx.max_val.max(1.0)).clamp(0.0, 1.0) as f32;
        let base = ctx.colors[actual_idx % ctx.colors.len()];
        let is_hovered = ctx.hovered_idx.map(|h| h == actual_idx).unwrap_or(false);
        let display_color = if is_hovered {
            egui::Color32::from_rgb(255, 200, 0)
        } else {
            lerp_color32(egui::Color32::from_gray(245), base, 0.15 + 0.85 * norm_val)
        };

        ctx.painter.rect_filled(rect, 2.0, display_color);

        let label_text = ctx
            .labels
            .get(actual_idx)
            .map(|s| s.as_str())
            .unwrap_or("");
        if !label_text.is_empty() && n <= 64 {
            ctx.painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                label_text,
                egui::FontId::proportional(9.0),
                if norm_val > 0.6 {
                    egui::Color32::WHITE
                } else {
                    egui::Color32::from_gray(60)
                },
            );
        }
    }
}
