use super::config::SankeyConfig;

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], edges_i=[0,1,0,1], edges_j=[2,2,3,3], edges_w=[5,15,3,2]")]
pub fn render(cfg: &SankeyConfig) -> String {
    super::basic::render_sorted(cfg)
}
