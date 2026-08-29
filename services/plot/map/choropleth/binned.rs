use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::plot::statistical::common::colorscale_color;

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"WA\",\"CO\",\"IL\",\"OH\",\"GA\",\"AZ\",\"NV\",\"UT\",\"OR\",\"NC\",\"MA\"], values=[38.9,30.5,19.6,22.6,7.8,5.9,12.6,11.8,11.0,7.4,3.2,3.4,4.2,10.8,7.0], title=\"Population by State (binned)\", map=\"usa_states\", variant=\"binned\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    let mut sorted: Vec<f64> = cfg.values.iter().cloned().filter(|v| v.is_finite()).collect();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let bins = cfg.bins.clamp(2, 9);
    let edges = quantile_edges(&sorted, bins);
    render_html(cfg, move |value, _min, _max| {
        let bin = edges.iter().filter(|&&e| value >= e).count().min(bins - 1);
        let t = bin as f64 / (bins - 1).max(1) as f64;
        unpack(colorscale_color("viridis", t))
    })
}

fn quantile_edges(sorted: &[f64], bins: usize) -> Vec<f64> {
    if sorted.is_empty() {
        return Vec::new();
    }
    let last = sorted.len() - 1;
    (1..bins)
        .map(|i| {
            let pos = i as f64 / bins as f64 * last as f64;
            let lo = pos.floor() as usize;
            let hi = (pos.ceil() as usize).min(last);
            let frac = pos - lo as f64;
            sorted[lo] * (1.0 - frac) + sorted[hi] * frac
        })
        .collect()
}

fn unpack(c: u32) -> (u8, u8, u8) {
    (((c >> 16) & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, (c & 0xFF) as u8)
}
