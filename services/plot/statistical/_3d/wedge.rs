use super::super::bar::Bar3DBlock;
use super::zone::{fit as zone_fit, Fit};

pub const HEIGHT: f64 = 0.24;
pub const MODE: u8 = 19;

#[derive(Default)]
pub struct Wedges {
    pub a0: Vec<f64>,
    pub a1: Vec<f64>,
    pub inner_r: Vec<f64>,
    pub outer_r: Vec<f64>,
    pub cx: Vec<f64>,
    pub cy: Vec<f64>,
    pub height: Vec<f64>,
    pub value: Vec<f64>,
    pub color_idx: Vec<f64>,
    pub names: Vec<String>,
}

impl Wedges {
    pub fn is_empty(&self) -> bool {
        self.a0.is_empty()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn push(&mut self, a0: f64, a1: f64, inner_r: f64, outer_r: f64, cx: f64, cy: f64, height: f64, value: f64, color_idx: f64, name: String) {
        self.a0.push(a0);
        self.a1.push(a1);
        self.inner_r.push(inner_r);
        self.outer_r.push(outer_r);
        self.cx.push(cx);
        self.cy.push(cy);
        self.height.push(height);
        self.value.push(value);
        self.color_idx.push(color_idx);
        self.names.push(name);
    }
}

fn bounding_blocks(w: &Wedges) -> Vec<Bar3DBlock> {
    (0..w.a0.len())
        .map(|i| {
            let r = w.outer_r[i].max(1e-6);
            Bar3DBlock::new(w.cx[i], w.cy[i], 0.0, w.height[i].max(1e-6), r, r, i)
        })
        .collect()
}

fn js_str_escape(out: &mut String, s: &str) {
    for ch in s.chars() {
        match ch {
            '\'' => out.push_str("\\'"),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            c => out.push(c),
        }
    }
}

fn nums(values: &[f64], places: usize) -> String {
    values.iter().map(|v| format!("{v:.places$}")).collect::<Vec<_>>().join(",")
}

#[allow(clippy::too_many_arguments)]
pub fn render_html(
    title: &str,
    w: &Wedges,
    axis_labels: (&str, &str, &str),
    width: i32,
    height: i32,
    bg: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    if w.is_empty() {
        return crate::html::js_3d::render_3d_html_impl(MODE, title, &[0.0], &[0.0], &[0.0], axis_labels, &[], &[], width, height, bg, scene, b"");
    }
    let blocks = bounding_blocks(w);
    let explicit = zone.and_then(|p| <[f64; 3]>::try_from(p).ok());
    let fitted = zone_fit(&blocks, 0.55, explicit, Fit::Uniform);
    let x = w.cx.clone();
    let y = w.cy.clone();
    let z: Vec<f64> = w.height.iter().map(|h| h / 2.0).collect();
    let mut extra = format!(
        "var BFIT={};var A0=[{}],A1=[{}],IR=[{}],OR=[{}],CX=[{}],CY=[{}],RH=[{}],VAL=[{}];",
        fitted.to_js(),
        nums(&w.a0, 5),
        nums(&w.a1, 5),
        nums(&w.inner_r, 4),
        nums(&w.outer_r, 4),
        nums(&w.cx, 4),
        nums(&w.cy, 4),
        nums(&w.height, 4),
        nums(&w.value, 3),
    );
    extra.push_str("var NM=[");
    for (i, name) in w.names.iter().enumerate() {
        if i > 0 {
            extra.push(',');
        }
        extra.push('\'');
        js_str_escape(&mut extra, name);
        extra.push('\'');
    }
    extra.push_str("];");
    crate::html::js_3d::render_3d_html_impl(MODE, title, &x, &y, &z, axis_labels, &w.color_idx, &[], width, height, bg, scene, extra.as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Wedges {
        let mut w = Wedges::default();
        w.push(0.0, 1.0, 0.0, 1.0, 0.0, 0.0, HEIGHT, 10.0, 0.0, "A".into());
        w.push(1.0, 2.0, 0.0, 1.0, 0.0, 0.0, HEIGHT, 20.0, 1.0, "B".into());
        w
    }

    #[test]
    fn empty_wedges_render_nothing_but_do_not_panic() {
        let html = render_html("t", &Wedges::default(), ("x", "y", "z"), 400, 300, None, "default", None);
        assert!(!html.is_empty());
        assert!(!html.contains("var A0="));
    }

    #[test]
    fn a_real_wedge_set_carries_a_fitted_zone_and_every_array() {
        let html = render_html("t", &sample(), ("x", "y", "z"), 400, 300, None, "default", None);
        for key in ["var BFIT=", "var A0=", "A1=[", "IR=[", "OR=[", "CX=[", "CY=[", "RH=[", "VAL=[", "var NM="] {
            assert!(html.contains(key), "missing {key}");
        }
        assert!(!html.contains("var BN="));
    }

    #[test]
    fn many_wedges_at_a_growing_radius_still_produce_a_finite_fitted_zone() {
        let mut w = Wedges::default();
        for i in 0..40 {
            w.push(0.0, 0.2, 0.0, 1.0 + i as f64 * 0.3, i as f64 * 2.0, 0.0, HEIGHT, 1.0, i as f64, format!("n{i}"));
        }
        let html = render_html("t", &w, ("x", "y", "z"), 400, 300, None, "default", None);
        assert!(html.contains("var BFIT="));
    }
}
