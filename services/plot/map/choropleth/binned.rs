use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::plot::statistical::common::colorscale_color;

#[crate::chart_demo(
    "labels=[\"AL\",\"AK\",\"AZ\",\"AR\",\"CA\",\"CO\",\"CT\",\"DE\",\"FL\",\"GA\",\"HI\",\"ID\",\"IL\",\"IN\",\"IA\",\"KS\",\"KY\",\"LA\",\"ME\",\"MD\",\"MA\",\"MI\",\"MN\",\"MS\",\"MO\",\"MT\",\"NE\",\"NV\",\"NH\",\"NJ\",\"NM\",\"NY\",\"NC\",\"ND\",\"OH\",\"OK\",\"OR\",\"PA\",\"RI\",\"SC\",\"SD\",\"TN\",\"TX\",\"UT\",\"VT\",\"VA\",\"WA\",\"WV\",\"WI\",\"WY\",\"DC\"], values=[5.1,0.73,7.4,3.0,38.9,5.9,3.6,1.0,22.6,11.0,1.4,2.0,12.6,6.8,3.2,2.9,4.5,4.6,1.4,6.2,7.0,10.0,5.7,2.9,6.2,1.1,2.0,3.2,1.4,9.3,2.1,19.6,10.8,0.78,11.8,4.0,4.2,12.9,1.1,5.4,0.92,7.1,30.5,3.4,0.65,8.7,7.8,1.8,5.9,0.58,0.68], title=\"Population by State (binned)\", map=\"usa_states\", variant=\"binned\""
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
