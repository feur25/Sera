use super::canvas_core::Canvas;
use super::element::{El, Layer};
use super::geometry::catmull_rom;
use pyo3::prelude::*;

#[pymethods]
impl Canvas {
    pub fn pin(&mut self, chart_ref: usize, name: &str, local_x: f64, local_y: f64) {
        self.register_native_pin(chart_ref, name, local_x, local_y);
    }

    pub fn pin_frac(&mut self, chart_ref: usize, name: &str, fx: f64, fy: f64) {
        if let Some(info) = self.placed.get(chart_ref) {
            let local_x = info.native_w * fx;
            let local_y = info.native_h * fy;
            let point = Self::map_native_point(info, local_x, local_y);
            self.pins.insert(Self::pin_key(chart_ref, name), point);
        }
    }

    pub fn pin_xy(&self, chart_ref: usize, name: &str) -> Option<(f64, f64)> {
        self.pins.get(&Self::pin_key(chart_ref, name)).copied()
    }

    #[pyo3(signature = (chart_ref, values, chart_w, chart_h,
                        has_groups = false, has_ylabel = false, has_xlabel = false))]
    pub fn attach_bar(
        &mut self,
        chart_ref: usize,
        values: Vec<f64>,
        chart_w: f64,
        chart_h: f64,
        has_groups: bool,
        has_ylabel: bool,
        has_xlabel: bool,
    ) {
        let n = values.len();
        if n == 0 {
            return;
        }
        let rects = self
            .chart_html(chart_ref)
            .map(super::anchors::bar_rects)
            .unwrap_or_default();
        let fallback = super::anchors::bar_fallback(chart_w, chart_h, has_groups, has_ylabel, has_xlabel);
        self.update_chart_space(chart_ref, chart_w, chart_h, Some(fallback));
        if !rects.is_empty() {
            for rect in rects {
                let cx = rect.x + rect.w * 0.5;
                let cy = rect.y + rect.h * 0.5;
                self.register_native_pin(chart_ref, &format!("bar:{}:top", rect.idx), cx, rect.y);
                self.register_native_pin(chart_ref, &format!("bar:{}:center", rect.idx), cx, cy);
                self.register_native_pin(
                    chart_ref,
                    &format!("bar:{}:bottom", rect.idx),
                    cx,
                    rect.y + rect.h,
                );
                self.register_native_pin(chart_ref, &format!("bar:{}:left", rect.idx), rect.x, cy);
                self.register_native_pin(
                    chart_ref,
                    &format!("bar:{}:right", rect.idx),
                    rect.x + rect.w,
                    cy,
                );
            }
            return;
        }
        let max_val = values
            .iter()
            .copied()
            .fold(f64::NEG_INFINITY, f64::max)
            .max(1.0);
        let plot = self.plot_or(chart_ref, fallback);
        let bar_w = plot.width / n as f64;
        for (i, &val) in values.iter().enumerate() {
            let bh = (val / max_val).max(0.0) * plot.height;
            let cx = plot.left + (i as f64 + 0.5) * bar_w;
            let yt = plot.top + plot.height - bh;
            let yc = plot.top + plot.height - bh * 0.5;
            let yb = plot.top + plot.height;
            self.register_native_pin(chart_ref, &format!("bar:{}:top", i), cx, yt);
            self.register_native_pin(chart_ref, &format!("bar:{}:center", i), cx, yc);
            self.register_native_pin(chart_ref, &format!("bar:{}:bottom", i), cx, yb);
        }
    }

    #[pyo3(signature = (chart_ref, x_vals, y_vals, labels, chart_w, chart_h, has_groups = false))]
    pub fn attach_scatter(
        &mut self,
        chart_ref: usize,
        x_vals: Vec<f64>,
        y_vals: Vec<f64>,
        labels: Vec<String>,
        chart_w: f64,
        chart_h: f64,
        has_groups: bool,
    ) {
        let n = x_vals.len().min(y_vals.len());
        if n == 0 {
            return;
        }
        let fallback = super::anchors::scatter_fallback(chart_w, chart_h, has_groups);
        self.update_chart_space(chart_ref, chart_w, chart_h, Some(fallback));
        let plot = self.plot_or(chart_ref, fallback);
        let Some(bounds) = super::anchors::scatter_bounds(&x_vals, &y_vals, n) else {
            return;
        };
        for (i, (&x, &y)) in x_vals.iter().zip(&y_vals).take(n).enumerate() {
            let (px, py) = super::anchors::project_scatter(plot, bounds, x, y);
            if let Some(label) = labels.get(i) {
                if !label.is_empty() {
                    self.register_native_pin(chart_ref, label, px, py);
                }
            }
            self.register_native_pin(chart_ref, &format!("point:{}", i), px, py);
        }
    }

    #[pyo3(signature = (from_ref, from_name, to_ref, to_name, color = "#ffffff",
                        width = 1.5, opacity = 0.8, bend = 0.0, layer = "fg", name = ""))]
    pub fn connect(
        &mut self,
        from_ref: usize,
        from_name: &str,
        to_ref: usize,
        to_name: &str,
        color: &str,
        width: f64,
        opacity: f64,
        bend: f64,
        layer: &str,
        name: &str,
    ) {
        let p1 = self.pins.get(&Self::pin_key(from_ref, from_name)).copied();
        let p2 = self.pins.get(&Self::pin_key(to_ref, to_name)).copied();
        if let (Some((x1, y1)), Some((x2, y2))) = (p1, p2) {
            let dx = x2 - x1;
            let dy = y2 - y1;
            let dist = (dx * dx + dy * dy).sqrt().max(1.0);
            let mx = (x1 + x2) / 2.0 + (-dy / dist) * dist * bend;
            let my = (y1 + y2) / 2.0 + (dx / dist) * dist * bend;
            let pts = vec![(x1, y1), (mx, my), (x2, y2)];
            let d = catmull_rom(&pts, 1.0);
            let element_idx = self.elements.len();
            self.register_name(name, element_idx);
            self.elements.push(El::RawPath {
                d,
                fill: "none".to_string(),
                stroke: color.to_string(),
                sw: width,
                opacity,
                layer: Layer::from_str(layer),
                name: name.to_string(),
            });
        }
    }

    #[pyo3(signature = (chart_ref, pin_name, text, offset_x = 60.0, offset_y = -30.0,
                        color = "#ffffff", size = 12.0, line_dash = "3,4",
                        line_width = 1.0, bg = "", layer = "fg", name = ""))]
    pub fn annotate_at(
        &mut self,
        chart_ref: usize,
        pin_name: &str,
        text: &str,
        offset_x: f64,
        offset_y: f64,
        color: &str,
        size: f64,
        line_dash: &str,
        line_width: f64,
        bg: &str,
        layer: &str,
        name: &str,
    ) {
        if let Some((ax, ay)) = self.pins.get(&Self::pin_key(chart_ref, pin_name)).copied() {
            let element_idx = self.elements.len();
            self.register_name(name, element_idx);
            self.elements.push(El::Annotate {
                text: text.to_string(),
                ax,
                ay,
                tx: ax + offset_x,
                ty: ay + offset_y,
                color: color.to_string(),
                size,
                line_dash: line_dash.to_string(),
                lw: line_width,
                bg: bg.to_string(),
                layer: Layer::from_str(layer),
                name: name.to_string(),
            });
        }
    }
}
