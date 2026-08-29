use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::plot::statistical::common::colorscale_color;

#[crate::chart_demo(
    "labels=[\"CA\",\"TX\",\"NY\",\"FL\",\"WA\",\"CO\",\"IL\",\"OH\",\"GA\",\"AZ\"], values=[2.1,-1.4,0.8,-3.2,4.5,-0.6,1.9,-2.1,3.0,-4.8], title=\"Unemployment Change YoY (pts)\", map=\"usa_states\", variant=\"diverging\""
)]

pub fn render(cfg: &ChoroplethConfig) -> String {
    let mid = cfg.diverging_midpoint;
    render_html(cfg, move |value, min, max| {
        let spread = (max - mid).abs().max((min - mid).abs()).max(1e-9);
        let t = 0.5 + 0.5 * (value - mid) / spread;
        unpack(colorscale_color("rdbu", t.clamp(0.0, 1.0)))
    })
}

fn unpack(c: u32) -> (u8, u8, u8) {
    (((c >> 16) & 0xFF) as u8, ((c >> 8) & 0xFF) as u8, (c & 0xFF) as u8)
}
