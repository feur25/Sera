use crate::plot::{apply_h, parse_all};
pub mod basic;
pub mod circular;
pub mod cleveland;
pub mod common;
pub mod config;
pub mod diverging;
pub mod highlight;
pub mod office;
pub mod variant;

pub use config::LollipopConfig;
pub use variant::LollipopVariant;

pub fn render_lollipop_html(cfg: &LollipopConfig) -> String {
    use variant::LollipopVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Cleveland => cleveland::render(cfg),
        Diverging => diverging::render(cfg),
        Circular => circular::render(cfg),
        Highlight => highlight::render(cfg),
        Office => office::render(cfg),
    }
}

pub use build as build_lollipop_chart;

#[crate::sera_alias(
    "lollipop",
    "lollipops",
    "lollipop_chart",
    "lollipop_family",
    "lollipop_unified"
)]
#[crate::sera_builder("build_lollipop_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let groups = o.color_groups.clone().unwrap_or_default();
    let dec = crate::plot::decimate::Decimator::new(o.max_points, &values);
    let labels = dec.apply(labels);
    let values = dec.apply(values);
    let groups = dec.apply(groups);
    use crate::plot::statistical::{render_lollipop_html, LollipopConfig, LollipopVariant};
    let mut variant = LollipopVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let orient = o.orient_byte();
    if o.variant.is_none() && orient == b'h' {
        variant = LollipopVariant::Cleveland;
    }
    let hover = o.hj();
    let html = render_lollipop_html(&LollipopConfig {
        variant,
        title,
        labels: &labels,
        values: &values,
        groups: &groups,
        x_label: &o.xl(),
        y_label: &o.yl(),
        palette: &o.pal(),
        color_hex: o.color_hex.unwrap_or(0),
        gridlines: o.grid(),
        show_values: o.show_values.unwrap_or(false),
        highlight_index: o.highlight_index.unwrap_or(-1),
        sort_order: &o.srt(),
        width: o.w(900),
        height: o.h(480),
        hover: &hover,
        legend_position: &o.lp(),
    });
    apply_h(html, &o)
}
