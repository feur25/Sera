pub mod variant;
pub mod config;
pub mod common;
pub mod basic;
pub mod cleveland;
pub mod diverging;
pub mod circular;
pub mod highlight;
pub mod office;

pub use variant::LollipopVariant;
pub use config::LollipopConfig;

pub fn render_lollipop_html(cfg: &LollipopConfig) -> String {
    use variant::LollipopVariant::*;
    match cfg.variant {
        Basic     => basic::render(cfg),
        Cleveland => cleveland::render(cfg),
        Diverging => diverging::render(cfg),
        Circular  => circular::render(cfg),
        Highlight => highlight::render(cfg),
        Office    => office::render(cfg),
    }
}
