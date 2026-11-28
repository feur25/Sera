use super::common::render_html;
use super::config::FlowMapConfig;

#[crate::chart_demo(
    "labels=[\"US\",\"CN\",\"DE\",\"GB\",\"JP\",\"IN\",\"BR\",\"FR\",\"CA\",\"MX\",\"KR\",\"IT\",\"RU\",\"AU\",\"ES\",\"NL\",\"CH\",\"SA\",\"SG\",\"ZA\"], edges_i=[0,1,0,0,0,0,0,2,2,2,1,1,4,3,3,7,7,11,5,5,6,6,8,9,12,12,13,13,14,16,17,17,18,18,19,10,10], edges_j=[1,0,8,9,2,4,3,1,7,15,4,10,10,2,0,2,11,2,1,0,1,0,1,1,1,2,1,4,7,2,1,0,1,0,1,0,1], edges_w=[420,380,580,490,190,210,140,200,170,200,300,280,90,130,120,160,90,140,95,110,100,65,70,85,190,45,150,60,55,100,65,40,80,55,30,95,170], title=\"Trade Flow (USD billions)\", variant=\"straight\""
)]

pub fn render(cfg: &FlowMapConfig) -> String {
    render_html(cfg, false)
}
