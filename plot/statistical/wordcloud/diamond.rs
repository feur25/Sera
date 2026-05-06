use super::config::WordCloudConfig;
use super::variant::WordCloudVariant;

pub fn render(cfg: &WordCloudConfig) -> String {
    super::common::render_with(cfg, WordCloudVariant::Diamond)
}
