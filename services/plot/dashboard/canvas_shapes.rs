use super::canvas_core::Canvas;
use super::element::{El, Layer};
use super::geometry::{parse_pts, polar_xy, voronoi_cells};
use pyo3::prelude::*;

#[pymethods]
impl Canvas {
    #[pyo3(signature = (content, x, y, size = 24.0, color = "#ffffff", weight = "normal",
                        anchor = "start", rotation = 0.0, letter_spacing = 0.0,
                        font = "sans-serif", opacity = 1.0, layer = "fg", name = ""))]
    pub fn text(
        &mut self,
        content: &str,
        x: f64,
        y: f64,
        size: f64,
        color: &str,
        weight: &str,
        anchor: &str,
        rotation: f64,
        letter_spacing: f64,
        font: &str,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Text {
            content: content.to_string(),
            x,
            y,
            size,
            color: color.to_string(),
            opacity,
            rotation,
            anchor: anchor.to_string(),
            weight: weight.to_string(),
            ls: letter_spacing,
            font: font.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
            group: String::new(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5, dash = "",
                        opacity = 1.0, cap = "round", layer = "fg", hover_group = "", name = ""))]
    pub fn line(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        dash: &str,
        opacity: f64,
        cap: &str,
        layer: &str,
        hover_group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Line {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            dash: dash.to_string(),
            opacity,
            cap: cap.to_string(),
            layer: Layer::from_str(layer),
            group: hover_group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (points, color = "#ffffff", width = 1.5, opacity = 1.0,
                        tension = 1.0, fill = "none", layer = "fg", name = ""))]
    pub fn curve(
        &mut self,
        points: Vec<Vec<f64>>,
        color: &str,
        width: f64,
        opacity: f64,
        tension: f64,
        fill: &str,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Curve {
            pts: parse_pts(points),
            color: color.to_string(),
            width,
            opacity,
            tension,
            fill: fill.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5, opacity = 1.0,
                        bend = 0.5, layer = "fg", name = ""))]
    pub fn connector(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        opacity: f64,
        bend: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Connector {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            opacity,
            bend,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, r, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", hover_group = "", name = ""))]
    pub fn circle(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        hover_group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Circle {
            cx,
            cy,
            r,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            group: hover_group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, inner_r, outer_r, fill = "#ffffff", stroke = "none",
                        stroke_width = 0.0, opacity = 1.0, layer = "fg", name = ""))]
    pub fn ring(
        &mut self,
        cx: f64,
        cy: f64,
        inner_r: f64,
        outer_r: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Ring {
            cx,
            cy,
            r_inner: inner_r,
            r_outer: outer_r,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, r, start_deg, end_deg, color = "#ffffff", width = 1.5,
                        opacity = 1.0, cap = "round", layer = "fg", name = ""))]
    pub fn arc(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        start_deg: f64,
        end_deg: f64,
        color: &str,
        width: f64,
        opacity: f64,
        cap: &str,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Arc {
            cx,
            cy,
            r,
            start_deg,
            end_deg,
            color: color.to_string(),
            width,
            opacity,
            cap: cap.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, r_inner, r_outer, start_deg, end_deg, fill = "#ffffff",
                        stroke = "none", stroke_width = 0.0, opacity = 1.0, layer = "fg", group = "", name = ""))]
    pub fn wedge(
        &mut self,
        cx: f64,
        cy: f64,
        r_inner: f64,
        r_outer: f64,
        start_deg: f64,
        end_deg: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Wedge {
            cx,
            cy,
            r_inner,
            r_outer,
            start_deg,
            end_deg,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            group: group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (cx, cy, r, a_start, a_end, b_start, b_end, fill = "#ffffff",
                        opacity = 0.7, layer = "fg", name = ""))]
    pub fn ribbon(
        &mut self,
        cx: f64,
        cy: f64,
        r: f64,
        a_start: f64,
        a_end: f64,
        b_start: f64,
        b_end: f64,
        fill: &str,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Ribbon {
            cx,
            cy,
            r,
            a_start,
            a_end,
            b_start,
            b_end,
            fill: fill.to_string(),
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x, y, w, h, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, rx = 0.0, opacity = 1.0, rotation = 0.0,
                        layer = "fg", name = ""))]
    pub fn rect(
        &mut self,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        rx: f64,
        opacity: f64,
        rotation: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Rect {
            x,
            y,
            w,
            h,
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            rx,
            opacity,
            rotation,
            layer: Layer::from_str(layer),
            name: name.to_string(),
            group: String::new(),
        });
        element_idx
    }

    #[pyo3(signature = (points, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", hover_group = "", name = ""))]
    pub fn polygon(
        &mut self,
        points: Vec<Vec<f64>>,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        hover_group: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Polygon {
            pts: parse_pts(points),
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            group: hover_group.to_string(),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (d, fill = "none", stroke = "#ffffff",
                        stroke_width = 1.5, opacity = 1.0, layer = "fg", name = ""))]
    pub fn path(
        &mut self,
        d: &str,
        fill: &str,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::RawPath {
            d: d.to_string(),
            fill: fill.to_string(),
            stroke: stroke.to_string(),
            sw: stroke_width,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (x1, y1, x2, y2, color = "#ffffff", width = 1.5,
                        head_size = 4.0, opacity = 1.0, layer = "fg", name = ""))]
    pub fn arrow(
        &mut self,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
        color: &str,
        width: f64,
        head_size: f64,
        opacity: f64,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Arrow {
            x1,
            y1,
            x2,
            y2,
            color: color.to_string(),
            width,
            head_size,
            opacity,
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (text, ax, ay, tx, ty, color = "#ffffff", size = 13.0,
                        line_dash = "", line_width = 1.0, bg = "", layer = "fg", name = ""))]
    pub fn annotate(
        &mut self,
        text: &str,
        ax: f64,
        ay: f64,
        tx: f64,
        ty: f64,
        color: &str,
        size: f64,
        line_dash: &str,
        line_width: f64,
        bg: &str,
        layer: &str,
        name: &str,
    ) -> usize {
        let element_idx = self.elements.len();
        self.register_name(name, element_idx);
        self.elements.push(El::Annotate {
            text: text.to_string(),
            ax,
            ay,
            tx,
            ty,
            color: color.to_string(),
            size,
            line_dash: line_dash.to_string(),
            lw: line_width,
            bg: bg.to_string(),
            layer: Layer::from_str(layer),
            name: name.to_string(),
        });
        element_idx
    }

    #[pyo3(signature = (id, from_color, to_color, x1 = 0.0, y1 = 0.0, x2 = 1.0, y2 = 0.0))]
    pub fn gradient(
        &mut self,
        id: &str,
        from_color: &str,
        to_color: &str,
        x1: f64,
        y1: f64,
        x2: f64,
        y2: f64,
    ) {
        self.elements.push(El::GradDef {
            id: id.to_string(),
            from_color: from_color.to_string(),
            to_color: to_color.to_string(),
            x1,
            y1,
            x2,
            y2,
        });
    }

    #[pyo3(signature = (id, from_color, to_color, cx = 0.5, cy = 0.5, r = 0.5))]
    pub fn radial_gradient(&mut self, id: &str, from_color: &str, to_color: &str, cx: f64, cy: f64, r: f64) {
        self.elements.push(El::RadialGradDef {
            id: id.to_string(),
            from_color: from_color.to_string(),
            to_color: to_color.to_string(),
            cx,
            cy,
            r,
        });
    }

    #[pyo3(signature = (cx, cy, r, deg))]
    pub fn polar(&self, cx: f64, cy: f64, r: f64, deg: f64) -> (f64, f64) {
        polar_xy(cx, cy, r, deg)
    }

    #[pyo3(signature = (sites, x, y, w, h, fills = None, stroke = "#0d1117",
                        stroke_width = 1.0, opacity = 1.0, layer = "fg", name_prefix = "cell"))]
    pub fn voronoi(
        &mut self,
        sites: Vec<Vec<f64>>,
        x: f64,
        y: f64,
        w: f64,
        h: f64,
        fills: Option<Vec<String>>,
        stroke: &str,
        stroke_width: f64,
        opacity: f64,
        layer: &str,
        name_prefix: &str,
    ) -> Vec<usize> {
        let pts = parse_pts(sites);
        let cells = voronoi_cells(&pts, x, y, w, h);
        let fills = fills.unwrap_or_default();
        let mut out = Vec::with_capacity(cells.len());
        for (i, cell) in cells.into_iter().enumerate() {
            if cell.len() < 3 {
                continue;
            }
            let fill = fills.get(i).cloned().unwrap_or_else(|| "none".to_string());
            let name = format!("{}{}", name_prefix, i);
            let element_idx = self.elements.len();
            self.register_name(&name, element_idx);
            self.elements.push(El::Polygon {
                pts: cell,
                fill,
                stroke: stroke.to_string(),
                sw: stroke_width,
                opacity,
                layer: Layer::from_str(layer),
                group: String::new(),
                name,
            });
            out.push(element_idx);
        }
        out
    }
}
