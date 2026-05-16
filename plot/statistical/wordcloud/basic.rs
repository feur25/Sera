use super::config::WordCloudConfig;

pub const DEMO_KWARGS: &str = "words=[\"rust\",\"python\",\"wasm\",\"plot\",\"data\",\"viz\",\"chart\",\"graph\",\"fast\",\"native\",\"async\",\"macro\",\"trait\",\"enum\",\"crate\"], frequencies=[42,38,30,28,25,22,18,15,12,10,9,8,7,6,5]";
pub fn render(cfg: &WordCloudConfig) -> String {
    super::common::render_basic(cfg)
}


