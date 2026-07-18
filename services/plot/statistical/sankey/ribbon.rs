use super::config::SankeyConfig;

#[crate::chart_demo("labels=[\"Source\",\"A\",\"B\",\"Sink\"], edges_i=[0,0,1,2], edges_j=[1,2,3,3], edges_w=[15,10,15,10]")]
pub fn render(cfg: &SankeyConfig) -> String {
    let ribbon = SankeyConfig {
        node_width: cfg.node_width + 8,
        node_gap: (cfg.node_gap as f64 * 1.5) as i32,
        ..*cfg
    };
    super::basic::render(&ribbon)
}