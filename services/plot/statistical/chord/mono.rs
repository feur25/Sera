use super::config::ChordConfig;
use crate::plot::statistical::common::palette_color;

#[crate::chart_demo("labels=[\"P\",\"Q\",\"R\",\"S\"], matrix=[[0,8,4,6],[8,0,5,2],[4,5,0,9],[6,2,9,0]]")]
pub fn render(cfg: &ChordConfig) -> String {
    let mono_palette = vec![0x6366F1u32; cfg.labels.len()];
    let mono = ChordConfig {
        palette: &mono_palette,
        ..*cfg
    };
    super::basic::render(&mono)
}
