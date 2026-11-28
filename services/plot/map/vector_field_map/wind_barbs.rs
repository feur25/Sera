use super::common::{field_bounds, lerp_rgb, project, push_base_outlines, svg_open, to_html};
use super::config::VectorFieldMapConfig;

const SHAFT_LEN: f64 = 34.0;
const BARB_LEN: f64 = 11.0;
const BARB_SPACING: f64 = 6.5;
const PENNANT_BASE: f64 = 7.5;
const PENNANT_HEIGHT: f64 = 9.5;
const TILT_COS: f64 = 0.9396926;
const TILT_SIN: f64 = 0.3420201;

struct Barb {
    station: (f64, f64),
    from_dir: (f64, f64),
    perp: (f64, f64),
}

impl Barb {
    fn slot(&self, k: f64) -> (f64, f64) {
        (
            self.station.0 + self.from_dir.0 * (SHAFT_LEN - k * BARB_SPACING),
            self.station.1 + self.from_dir.1 * (SHAFT_LEN - k * BARB_SPACING),
        )
    }

    fn tilted(&self, len: f64) -> (f64, f64) {
        (
            self.perp.0 * len * TILT_COS - self.from_dir.0 * len * TILT_SIN,
            self.perp.1 * len * TILT_COS - self.from_dir.1 * len * TILT_SIN,
        )
    }
}

fn push_barb_glyph(svg: &mut String, station: (f64, f64), from_dir: (f64, f64), speed_kt: f64, r: u8, g: u8, b: u8, idx: usize) {
    let color = format!("rgb({r},{g},{b})");
    if speed_kt < 2.5 {
        svg.push_str(&format!(
            "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\" fill=\"none\" stroke=\"{color}\" stroke-width=\"1.6\" data-index=\"{idx}\"/>",
            station.0, station.1,
        ));
        return;
    }

    let barb = Barb { station, from_dir, perp: (-from_dir.1, from_dir.0) };
    let outer = (
        station.0 + from_dir.0 * SHAFT_LEN,
        station.1 + from_dir.1 * SHAFT_LEN,
    );
    svg.push_str(&format!(
        "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"1.8\" stroke-linecap=\"round\" data-index=\"{idx}\"/>",
        station.0, station.1, outer.0, outer.1,
    ));
    svg.push_str(&format!(
        "<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"2\" fill=\"{color}\"/>",
        station.0, station.1,
    ));

    let rounded = ((speed_kt / 5.0).round() * 5.0).max(5.0);
    let mut remaining = rounded;
    let mut slot = 0.0_f64;

    let pennants = (remaining / 50.0) as i32;
    remaining -= pennants as f64 * 50.0;
    for _ in 0..pennants {
        let a = barb.slot(slot);
        let base = barb.slot(slot + PENNANT_BASE / BARB_SPACING);
        let (tx, ty) = barb.tilted(PENNANT_HEIGHT);
        let apex = ((a.0 + base.0) / 2.0 + tx, (a.1 + base.1) / 2.0 + ty);
        svg.push_str(&format!(
            "<polygon points=\"{:.1},{:.1} {:.1},{:.1} {:.1},{:.1}\" fill=\"{color}\"/>",
            a.0, a.1, base.0, base.1, apex.0, apex.1,
        ));
        slot += PENNANT_BASE / BARB_SPACING + 0.3;
    }

    let full_barbs = (remaining / 10.0) as i32;
    remaining -= full_barbs as f64 * 10.0;
    for _ in 0..full_barbs {
        let p = barb.slot(slot);
        let (tx, ty) = barb.tilted(BARB_LEN);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"1.8\" stroke-linecap=\"round\"/>",
            p.0, p.1, p.0 + tx, p.1 + ty,
        ));
        slot += 1.0;
    }

    if remaining >= 5.0 - 1e-6 {
        let p = barb.slot(slot);
        let (tx, ty) = barb.tilted(BARB_LEN * 0.55);
        svg.push_str(&format!(
            "<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"{color}\" stroke-width=\"1.8\" stroke-linecap=\"round\"/>",
            p.0, p.1, p.0 + tx, p.1 + ty,
        ));
    }
}

#[crate::chart_demo(
    "lats=[40.7,51.5,35.7,-33.9,-33.9,-22.9], lons=[-74.0,-0.12,139.7,151.2,18.4,-43.2], u=[8,45,0.01,-15,20,-35], v=[3,-30,0.01,8,20,25], title=\"Station Wind Barbs\", variant=\"wind_barbs\""
)]
pub fn render(cfg: &VectorFieldMapConfig) -> String {
    let n = cfg.lats.len().min(cfg.lons.len()).min(cfg.u.len()).min(cfg.v.len());
    if n == 0 {
        return String::new();
    }
    let bounds = field_bounds(cfg);
    let mut svg = svg_open(cfg.width, cfg.height);
    push_base_outlines(&mut svg, cfg.width, cfg.height);

    for i in 0..n {
        let station = project(cfg.lats[i], cfg.lons[i], cfg.width, cfg.height);
        let station = (station.0 as f64, station.1 as f64);
        let mag = (cfg.u[i] * cfg.u[i] + cfg.v[i] * cfg.v[i]).sqrt();
        let from_dir = if mag > 1e-9 {
            (-cfg.u[i] / mag, cfg.v[i] / mag)
        } else {
            (0.0, -1.0)
        };
        let t = mag / bounds.max_mag;
        let (r, g, b) = lerp_rgb(cfg.color_low, cfg.color_high, t);
        push_barb_glyph(&mut svg, station, from_dir, mag, r, g, b, i);
    }

    to_html(cfg, svg)
}
