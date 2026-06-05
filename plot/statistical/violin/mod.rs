use crate::plot::{apply, apply_h, parse_all};
pub mod aurora;
pub mod basic;
pub mod common;
pub mod config;
pub mod crystal;
pub mod deluxe;
pub mod half;
pub mod horizontal;
pub mod mean;
pub mod points;
pub mod quartile;
pub mod rainbow;
pub mod split;
pub mod strip;
pub mod variant;
pub mod with_box;

pub use config::ViolinConfig;
pub use variant::ViolinVariant;

pub fn render_violin_html(cfg: &ViolinConfig) -> String {
    use ViolinVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Box => with_box::render(cfg),
        Quartile => quartile::render(cfg),
        Mean => mean::render(cfg),
        Points => points::render(cfg),
        Strip => strip::render(cfg),
        Horizontal => horizontal::render(cfg),
        Split => split::render(cfg),
        Half => half::render(cfg),
        Rainbow => rainbow::render(cfg),
        Aurora => aurora::render(cfg),
        Deluxe => deluxe::render(cfg),
        Crystal => crystal::render(cfg),
    }
}

pub use build as build_violin;

#[crate::sera_alias("violin", "violins", "violin_chart", "violin_family", "violin_unified")]
#[crate::sera_builder("build_violin")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let categories = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_violin_html, ViolinConfig, ViolinVariant};
    let hover = o.hj();
    let mut variant = ViolinVariant::from_str(o.variant.as_deref().unwrap_or("box"));
    if o.is_horiz()
        && matches!(
            variant,
            ViolinVariant::Basic
                | ViolinVariant::Box
                | ViolinVariant::Quartile
                | ViolinVariant::Mean
        )
    {
        variant = ViolinVariant::Horizontal;
    }
    let html = render_violin_html(&ViolinConfig {
        title,
        variant,
        categories: &categories,
        values: &values,
        x_label: &o.xl(),
        y_label: &o.yl(),
        palette: &o.pal(),
        gridlines: o.grid(),
        width: o.w(900),
        height: o.h(500),
        sort_order: &o.srt(),
        hover: &hover,
        legend_position: &o.lp(),
        bandwidth: o.bandwidth.unwrap_or(1.0),
        fill_opacity: o.fill_opacity_real.unwrap_or(0.55),
        stroke_width: o.box_stroke_width.unwrap_or(1.4),
        show_box: o.show_box.unwrap_or(false),
        show_points: o.show_points.unwrap_or(false),
        show_mean: o.show_mean.unwrap_or(false),
        jitter: o.jitter.unwrap_or(0.35),
        kde_steps: 32,
    });
    let native = matches!(variant, ViolinVariant::Horizontal);
    if native {
        apply_h(html, &o)
    } else {
        apply(html, &o)
    }
}
