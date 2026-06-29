use crate::plot::{apply, apply_h, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod cumulative;
pub mod deluxe;
pub mod horizontal;
pub mod normalized;
pub mod overlay;
pub mod stacked;
pub mod step;
pub mod variant;

pub use common::{bin_to_edges, compute_bins};
pub use config::HistogramConfig;
pub use variant::HistogramVariant;

pub struct Histogram;

pub fn render_histogram_html(cfg: &HistogramConfig) -> String {
    use crate::plot::statistical::theme::ChartTheme;
    if cfg.theme == ChartTheme::Deluxe {
        return deluxe::render(cfg);
    }
    if matches!(
        cfg.theme,
        ChartTheme::Aurora | ChartTheme::Inferno | ChartTheme::Frost | ChartTheme::Prism
    ) {}
    let v = if cfg.orientation == b'h' && cfg.variant != HistogramVariant::Horizontal {
        HistogramVariant::Horizontal
    } else if cfg.variant == HistogramVariant::Basic && cfg.overlay_values.is_some() {
        HistogramVariant::Overlay
    } else {
        cfg.variant
    };
    match v {
        HistogramVariant::Basic => basic::render(cfg),
        HistogramVariant::Horizontal => horizontal::render(cfg),
        HistogramVariant::Normalized => normalized::render(cfg),
        HistogramVariant::Cumulative => cumulative::render(cfg),
        HistogramVariant::Stacked => stacked::render(cfg),
        HistogramVariant::Overlay => overlay::render(cfg),
        HistogramVariant::Step => step::render(cfg),
        HistogramVariant::Deluxe => deluxe::render(cfg),
    }
}

pub use build as build_histogram;

#[crate::sera_alias(
    "hist",
    "histogram",
    "histograms",
    "histogram_family",
    "histogram_unified"
)]
#[crate::sera_builder("build_histogram")]
pub fn build(input: &str) -> String {
    use crate::plot::statistical::{render_histogram_html, HistogramConfig, HistogramVariant};
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.clone().unwrap_or_default();
    let cats = o.color_groups.clone().unwrap_or_default();
    let palette = o.pal();
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names = if sn.len() >= 2 {
        Some((sn[0].as_str(), sn[1].as_str()))
    } else {
        None
    };
    let ref_names: Option<(&str, &str)> = names.as_ref().map(|(a, b)| (*a, *b));
    let mut variant = HistogramVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let overlay_opt: Option<&[f64]> = if overlay.is_empty() {
        None
    } else {
        Some(&overlay)
    };
    let orient = o.orient_byte();
    if orient == b'h' && matches!(variant, HistogramVariant::Basic) {
        variant = HistogramVariant::Horizontal;
    }
    let color = match o.color_hex {
        Some(0) | None => palette.get(0).copied().unwrap_or(0x636EFA),
        Some(c) => c,
    };
    let html = render_histogram_html(&HistogramConfig {
        title,
        variant,
        values: &values,
        bins: o.bins.unwrap_or(0) as usize,
        color,
        overlay_color: o.overlay_color_hex.unwrap_or(0xF43F5E),
        overlay_values: overlay_opt,
        categories: &cats,
        palette: &palette,
        stroke_width: o.stroke_width.unwrap_or(1.0),
        gap: o.gap.unwrap_or(2),
        orientation: orient,
        series_names: ref_names,
        x_label: &o.xl(),
        y_label: &o.yl(),
        show_counts: o.show_counts.unwrap_or(false),
        gridlines: o.grid(),
        width: o.w(860),
        height: o.h(380),
        hover: &hover,
        sort_order: &o.srt(),
        theme: {
            use crate::plot::statistical::ChartTheme;
            ChartTheme::from_str(o.theme.as_deref().unwrap_or("none"))
        },
        ..HistogramConfig::default()
    });
    use crate::plot::statistical::HistogramVariant::*;
    let native = matches!(variant, Basic | Horizontal);
    if native {
        apply_h(html, &o)
    } else {
        apply(html, &o)
    }
}

pub fn build_histogram_overlay(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    let overlay = a.overlay.unwrap_or_default();
    use crate::plot::statistical::{render_histogram_html, HistogramConfig, HistogramVariant};
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names = if sn.len() >= 2 {
        Some((sn[0].as_str(), sn[1].as_str()))
    } else {
        None
    };
    let ref_names: Option<(&str, &str)> = names.as_ref().map(|(a, b)| (*a, *b));
    let html = render_histogram_html(&HistogramConfig {
        title,
        variant: HistogramVariant::Overlay,
        values: &values,
        bins: o.bins.unwrap_or(0) as usize,
        color: o.color_hex.unwrap_or(0x636EFA),
        overlay_color: o.overlay_color_hex.unwrap_or(0xF43F5E),
        overlay_values: Some(&overlay),
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        width: o.w(860),
        height: o.h(380),
        hover: &hover,
        series_names: ref_names,
        sort_order: &o.srt(),
        ..HistogramConfig::default()
    });
    apply(html, &o)
}
