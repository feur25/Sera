use super::common::render_html;
use super::config::FlowMapConfig;

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"DE\",\"GB\",\"JP\",\"IN\",\"BR\"], edges_i=[0,1,0,5,6,2], edges_j=[1,0,2,0,0,3], edges_w=[420,380,290,140,95,210], title=\"Trade Flow (USD billions)\", variant=\"straight\""
)]

pub fn render(cfg: &FlowMapConfig) -> String {
    render_html(cfg, false)
}
