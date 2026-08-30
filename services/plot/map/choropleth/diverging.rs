use super::common::render_html;
use super::config::ChoroplethConfig;
use crate::plot::statistical::common::colorscale_color;

#[crate::chart_demo(
    "labels=[\"AL\",\"AK\",\"AZ\",\"AR\",\"CA\",\"CO\",\"CT\",\"DE\",\"FL\",\"GA\",\"HI\",\"ID\",\"IL\",\"IN\",\"IA\",\"KS\",\"KY\",\"LA\",\"ME\",\"MD\",\"MA\",\"MI\",\"MN\",\"MS\",\"MO\",\"MT\",\"NE\",\"NV\",\"NH\",\"NJ\",\"NM\",\"NY\",\"NC\",\"ND\",\"OH\",\"OK\",\"OR\",\"PA\",\"RI\",\"SC\",\"SD\",\"TN\",\"TX\",\"UT\",\"VT\",\"VA\",\"WA\",\"WV\",\"WI\",\"WY\",\"DC\"], values=[0.8,-0.4,2.1,0.3,-1.4,1.6,-0.2,1.1,3.2,1.9,-0.6,2.8,-1.9,0.5,-0.1,0.2,-0.3,-1.1,0.9,0.1,-0.5,-0.8,0.4,-0.9,0.3,1.4,0.2,2.3,1.0,-0.7,0.6,-2.1,2.2,0.5,-0.6,0.4,0.7,-0.9,-0.3,1.8,0.6,1.5,2.9,2.5,0.2,0.8,1.2,-1.3,0.3,-0.5,-1.6], title=\"Unemployment Change YoY (pts)\", map=\"usa_states\", variant=\"diverging\""
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
