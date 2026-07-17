use super::config::ChordConfig;

#[crate::chart_demo("labels=[\"A\",\"B\",\"C\",\"D\"], matrix=[[0,15,3,6],[4,0,9,2],[8,5,0,11],[7,3,4,0]]")]
pub fn render(cfg: &ChordConfig) -> String {
    super::basic::render_directed(cfg)
}
