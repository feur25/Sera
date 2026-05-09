use super::config::WordCloudConfig;

pub fn render(cfg: &WordCloudConfig) -> String {
    super::common::render_bubble(cfg)
}


