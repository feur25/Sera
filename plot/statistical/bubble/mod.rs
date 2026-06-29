use crate::plot::{apply, parse_all};
pub mod basic;
pub mod categorical;
pub mod common;
pub mod config;
pub mod deluxe;
pub mod gradient;
pub mod labeled;
pub mod negative;
pub mod outlined;
pub mod plasma;
pub mod variant;

pub use config::BubbleConfig;
pub use variant::BubbleVariant;

pub fn render_bubble_html(cfg: &BubbleConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    match cfg.theme {
        ChartTheme::Deluxe => return deluxe::render(cfg),
        ChartTheme::Prism => return plasma::render(cfg),
        _ => {}
    }
    use BubbleVariant::*;
    let v = if cfg.variant == Basic && !cfg.categories.is_empty() {
        Categorical
    } else {
        cfg.variant
    };
    match v {
        Basic => basic::render(cfg),
        Categorical => categorical::render(cfg),
        Gradient => gradient::render(cfg),
        Labeled => labeled::render(cfg),
        Outlined => outlined::render(cfg),
        Negative => negative::render(cfg),
        Plasma => plasma::render(cfg),
        Deluxe => deluxe::render(cfg),
    }
}

pub use build as build_bubble;

#[crate::sera_alias("bubble", "bubble_family", "bubble_unified", "bubbles")]
#[crate::sera_builder("build_bubble")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::bubble::{render_bubble_html, BubbleConfig, BubbleVariant};
    use crate::plot::statistical::ChartTheme;
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let variant = BubbleVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    let sizes = a.sizes.or(a.size).unwrap_or_default();
    let categories = a
        .categories
        .clone()
        .or_else(|| o.color_groups.clone())
        .unwrap_or_default();
    let labels = a.labels.clone().unwrap_or_default();
    let color_values = o.color_values.clone().unwrap_or_default();
    let palette = o.pal();
    let hover = o.hj();
    let xl = o.xl();
    let yl = o.yl();
    let srt = o.srt();
    let lp = o.lp();
    let cfg = BubbleConfig {
        variant,
        title,
        x_label: &xl,
        y_label: &yl,
        width: o.w(900),
        height: o.h(500),
        gridlines: o.grid(),
        sort_order: &srt,
        legend_position: &lp,
        hover: &hover,
        palette: &palette,
        x_values: &x_values,
        y_values: &y_values,
        sizes: &sizes,
        categories: &categories,
        labels: &labels,
        color_values: &color_values,
        color_hex: o.color_hex.unwrap_or(0),
        color_low: o.color_low.unwrap_or(0x636EFA),
        color_high: o.color_high.unwrap_or(0xF43F5E),
        min_size: o.min_size.unwrap_or(4.0),
        max_size: o.max_size.unwrap_or(40.0),
        show_text: o.show_values.or(o.show_text).unwrap_or(false),
        stroke_width: o.stroke_width.unwrap_or(1.5),
        theme: ChartTheme::from_str(o.theme.as_deref().unwrap_or("none")),
    };
    let html = render_bubble_html(&cfg);
    apply(html, &o)
}
