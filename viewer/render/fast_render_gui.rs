pub struct BatchRenderer {
    circles: Vec<(egui::Pos2, f32, egui::Color32)>,
    lines: Vec<(egui::Pos2, egui::Pos2, egui::Color32, f32)>,
    text_items: Vec<(egui::Pos2, String, egui::Color32)>,
}

impl BatchRenderer {
    pub fn new() -> Self {
        Self {
            circles: Vec::with_capacity(10000),
            lines: Vec::with_capacity(5000),
            text_items: Vec::with_capacity(500),
        }
    }

    pub fn add_circle(&mut self, pos: egui::Pos2, radius: f32, color: egui::Color32) {
        self.circles.push((pos, radius, color));
    }

    pub fn add_line(
        &mut self,
        from: egui::Pos2,
        to: egui::Pos2,
        color: egui::Color32,
        width: f32,
    ) {
        self.lines.push((from, to, color, width));
    }

    pub fn add_text(&mut self, pos: egui::Pos2, text: String, color: egui::Color32) {
        self.text_items.push((pos, text, color));
    }

    pub fn flush(&self, painter: &egui::Painter) {
        for (pos, radius, color) in &self.circles {
            painter.circle_filled(*pos, *radius, *color);
        }

        for (from, to, color, width) in &self.lines {
            painter.line_segment([*from, *to], egui::Stroke::new(*width, *color));
        }

        for (pos, text, color) in &self.text_items {
            painter.text(
                *pos,
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::proportional(10.0),
                *color,
            );
        }
    }

    pub fn clear(&mut self) {
        self.circles.clear();
        self.lines.clear();
        self.text_items.clear();
    }
}
