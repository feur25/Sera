use super::config::SankeyConfig;

#[crate::chart_demo("labels=[\"In\",\"Mid\",\"Out1\",\"Out2\"], edges_i=[0,0,1,1], edges_j=[1,2,2,3], edges_w=[20,5,12,8]")]
pub fn render(cfg: &SankeyConfig) -> String {
    let gapped = SankeyConfig {
        node_gap: cfg.node_gap * 3,
        node_width: cfg.node_width + 4,
        ..*cfg
    };
    super::basic::render(&gapped)
}