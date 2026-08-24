use super::canvas_core::Canvas;
use pyo3::prelude::*;

impl Canvas {
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
        let cell_r = cell_w.min(cell_h) * 0.42;
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
                let t = (weights[i] / max_w).clamp(0.0, 1.0).sqrt();
                cell_r * (0.6 + 0.4 * t)
            } else {
                cell_r
            };
            anchors.push((ax, ay, r));
        }

        for i in 0..n.saturating_sub(1) {
            let (ax, ay, _) = anchors[i];
            let (bx, by, _) = anchors[i + 1];
            let same_row = i / cols == (i + 1) / cols;
            let name = format!("{name_prefix}-ln-{i}");
            if same_row {
                self.line(ax, ay, bx, by, line_color, line_width, "", 0.75, "round", "bg", "", &name);
            } else {
                self.connector(ax, ay, bx, by, line_color, line_width, 0.75, 0.62, "bg", &name);
            }
        }

        for (i, &(ax, ay, r)) in anchors.iter().enumerate() {
            let ring_name = format!("{name_prefix}-ring-{i}");
            self.ring(ax, ay, (r - ring_width).max(0.0), r, ring_color, "none", 0.0, 0.9, "fg", &ring_name);
            if let Some(lbl) = labels.get(i) {
                if !lbl.is_empty() {
                    let lbl_name = format!("{name_prefix}-lbl-{i}");
                    self.text(
                        lbl, ax, ay + r + label_size + 8.0, label_size, label_color, "600", "middle", 0.0, 0.0,
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
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 1.6,
                        ring_color = "#7dd3fc", ring_width = 2.4, label_color = "#334155",
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
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 1.6,
                        ring_color = "#7dd3fc", ring_width = 2.4, label_color = "#334155",
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
                        x0 = 100.0, y0 = 100.0, line_color = "#60a5fa", line_width = 1.6,
                        ring_color = "#7dd3fc", ring_width = 2.4, label_color = "#334155",
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
