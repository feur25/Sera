pub mod variant;
pub mod config;
pub mod basic;
pub mod grouped;
pub mod relative;
pub mod grouped_stacked;
pub mod marimekko;
pub mod pictogram;
pub mod multicategory;
pub mod deluxe;
pub mod prism;

pub use variant::BarVariant;
pub use config::BarConfig;

pub fn render_bar_html(cfg: &BarConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    match cfg.theme {
        ChartTheme::Deluxe => return deluxe::render(cfg, cfg.orientation),
        ChartTheme::Prism  => return prism::render(cfg),
        _                  => {}
    }
    use BarVariant::*;
    match cfg.variant {
        Basic           => basic::render(cfg, cfg.orientation),
        Horizontal      => basic::render(cfg, b'h'),
        Grouped         => grouped::render(cfg, false),
        Stacked         => grouped::render(cfg, true),
        Relative        => relative::render(cfg),
        GroupedStacked  => grouped_stacked::render(cfg),
        Marimekko       => marimekko::render(cfg),
        Pictogram       => pictogram::render(cfg),
        Multicategory   => multicategory::render(cfg),
        Deluxe          => deluxe::render(cfg, b'v'),
        Prism           => prism::render(cfg),
    }
}


