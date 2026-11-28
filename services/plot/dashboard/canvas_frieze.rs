use super::canvas_core::Canvas;
use pyo3::prelude::*;

impl Canvas {
    fn glow_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, color: &str, width: f64, opacity: f64) {
        self.line(x1, y1, x2, y2, color, width * 3.2, "", opacity * 0.28, "round", "bg", "", "");
        self.line(x1, y1, x2, y2, color, width, "", opacity, "round", "bg", "", "");
    }

    fn glow_curve(&mut self, points: Vec<Vec<f64>>, color: &str, width: f64, opacity: f64, tension: f64) {
        self.curve(points.clone(), color, width * 3.2, opacity * 0.24, tension, "none", "bg", "");
        self.curve(points, color, width, opacity, tension, "none", "bg", "");
    }

    fn frieze_impl(
        &mut self,
        labels: Vec<String>,
        weights: Vec<f64>,
        cols: usize,
        cell_w: f64,
        cell_h: f64,
        x0: f64,
        y0: f64,
        line_color: &str,
        line_width: f64,
        ring_color: &str,
        ring_width: f64,
        label_color: &str,
        label_size: f64,
        name_prefix: &str,
    ) -> Vec<(f64, f64, f64)> {
        let n = labels.len();
        if n == 0 {
            return Vec::new();
        }
        let cols = cols.max(1);
        let cell_r = cell_w.min(cell_h) * 0.44;
        let has_w = weights.len() == n;
        let max_w = if has_w {
            weights.iter().cloned().fold(0.0_f64, f64::max).max(1e-9)
        } else {
            1.0
        };

        let mut anchors: Vec<(f64, f64, f64)> = Vec::with_capacity(n);
        for i in 0..n {
            let row = i / cols;
            let col = i % cols;
            let cidx = if row % 2 == 0 { col } else { cols - 1 - col };
            let ax = x0 + cidx as f64 * cell_w;
            let ay = y0 + row as f64 * cell_h;
            let r = if has_w {
                let t = (weights[i] / max_w).clamp(0.0, 1.0).powf(0.7);
                cell_r * (0.3 + 0.7 * t)
            } else {
                cell_r
            };
            anchors.push((ax, ay, r));
        }

        for i in 0..n.saturating_sub(1) {
            let (ax, ay, _) = anchors[i];
            let (bx, by, _) = anchors[i + 1];
            let same_row = i / cols == (i + 1) / cols;
            if same_row {
                self.glow_line(ax, ay, bx, by, line_color, line_width, 0.85);
            } else {
                let col_here = i % cols;
                let dir = if col_here == 0 { -1.0 } else { 1.0 };
                let bulge = cell_w * 0.62 * dir;
                let midx = (ax + bx) / 2.0 + bulge;
                let midy = (ay + by) / 2.0;
                self.glow_curve(
                    vec![vec![ax, ay], vec![ax + bulge * 0.55, ay], vec![midx, midy], vec![bx + bulge * 0.55, by], vec![bx, by]],
                    line_color,
                    line_width,
                    0.85,
                    0.75,
                );
            }
        }

        for (i, &(ax, ay, r)) in anchors.iter().enumerate() {
            let outer_name = format!("{name_prefix}-ring-{i}");
            self.circle(ax, ay, r, "none", ring_color, ring_width * 0.55, 0.9, "fg", "", &outer_name);
            let inner_r = (r - ring_width * 2.2).max(1.0);
            self.circle(ax, ay, inner_r, "none", ring_color, ring_width * 0.55, 0.55, "fg", "", "");

            let stem_y0 = ay + r;
            let stem_y1 = stem_y0 + 12.0;
            self.line(ax, stem_y0, ax, stem_y1, ring_color, 1.4, "", 0.7, "round", "fg", "", "");

            if let Some(lbl) = labels.get(i) {
                if !lbl.is_empty() {
                    let lbl_name = format!("{name_prefix}-lbl-{i}");
                    self.text(
                        lbl, ax, stem_y1 + label_size + 4.0, label_size, label_color, "600", "middle", 0.0, 0.0,
                        "sans-serif", 1.0, "fg", &lbl_name,
                    );
                }
            }
        }

        anchors
    }
}

#[pymethods]
impl Canvas {
    #[pyo3(signature = (labels, weights = vec![], cols = 8, cell_w = 160.0, cell_h = 160.0,
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 2.6,
                        ring_color = "#7dd3fc", ring_width = 2.6, label_color = "#334155",
                        label_size = 11.0, name_prefix = "frz"))]
    pub fn frieze(
        &mut self,
        labels: Vec<String>,
        weights: Vec<f64>,
        cols: usize,
        cell_w: f64,
        cell_h: f64,
        x0: f64,
        y0: f64,
        line_color: &str,
        line_width: f64,
        ring_color: &str,
        ring_width: f64,
        label_color: &str,
        label_size: f64,
        name_prefix: &str,
    ) -> Vec<(f64, f64, f64)> {
        self.frieze_impl(
            labels, weights, cols, cell_w, cell_h, x0, y0, line_color, line_width, ring_color, ring_width,
            label_color, label_size, name_prefix,
        )
    }

    #[pyo3(signature = (labels, weights = vec![], cols = 8, cell_w = 160.0, cell_h = 160.0,
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 2.6,
                        ring_color = "#7dd3fc", ring_width = 2.6, label_color = "#334155",
                        label_size = 11.0, name_prefix = "tml"))]
    pub fn timeline(
        &mut self,
        labels: Vec<String>,
        weights: Vec<f64>,
        cols: usize,
        cell_w: f64,
        cell_h: f64,
        x0: f64,
        y0: f64,
        line_color: &str,
        line_width: f64,
        ring_color: &str,
        ring_width: f64,
        label_color: &str,
        label_size: f64,
        name_prefix: &str,
    ) -> Vec<(f64, f64, f64)> {
        self.frieze_impl(
            labels, weights, cols, cell_w, cell_h, x0, y0, line_color, line_width, ring_color, ring_width,
            label_color, label_size, name_prefix,
        )
    }

    #[pyo3(signature = (labels, weights = vec![], cols = 8, cell_w = 160.0, cell_h = 160.0,
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 2.6,
                        ring_color = "#7dd3fc", ring_width = 2.6, label_color = "#334155",
                        label_size = 11.0, name_prefix = "chr"))]
    pub fn chronology(
        &mut self,
        labels: Vec<String>,
        weights: Vec<f64>,
        cols: usize,
        cell_w: f64,
        cell_h: f64,
        x0: f64,
        y0: f64,
        line_color: &str,
        line_width: f64,
        ring_color: &str,
        ring_width: f64,
        label_color: &str,
        label_size: f64,
        name_prefix: &str,
    ) -> Vec<(f64, f64, f64)> {
        self.frieze_impl(
            labels, weights, cols, cell_w, cell_h, x0, y0, line_color, line_width, ring_color, ring_width,
            label_color, label_size, name_prefix,
        )
    }
}
