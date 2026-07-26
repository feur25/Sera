use crate::plot::{apply, parse_all};
pub mod basic;
pub mod chevron;
pub mod common;
pub mod compare;
pub mod config;
pub mod grouped;
pub mod conversion;
pub mod inverted;
pub mod pyramid;
pub mod rounded;
pub mod stepped;
pub mod variant;

pub use config::FunnelConfig;
pub use variant::FunnelVariant;

pub fn render_funnel_html(cfg: &FunnelConfig) -> String {
    use variant::FunnelVariant::*;
    let v = if cfg.variant == Basic && cfg.series.len() > 1 {
        if cfg.stage_labels.is_empty() {
            Grouped
        } else {
            Compare
        }
    } else {
        cfg.variant
    };
    match v {
        Basic => basic::render(cfg),
        Stepped => stepped::render(cfg),
        Rounded => rounded::render(cfg),
        Chevron => chevron::render(cfg),
        Pyramid => pyramid::render(cfg),
        Inverted => inverted::render(cfg),
        Conversion => conversion::render(cfg),
        Compare => compare::render(cfg),
        Grouped => grouped::render(cfg),
    }
}

pub use build as build_funnel;

#[crate::sera_alias("funnel", "funnels", "funnel_chart", "funnel_family", "funnel_unified")]
#[crate::sera_builder("build_funnel")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_funnel_html, FunnelConfig, FunnelVariant};
    let series: Vec<(String, Vec<f64>)> = {
        let sn = o.series_names.clone().unwrap_or_default();
        a.series
            .clone()
            .unwrap_or_default()
            .into_iter()
            .enumerate()
            .map(|(si, vals)| {
                (
                    sn.get(si)
                        .cloned()
                        .unwrap_or_else(|| format!("S{}", si + 1)),
                    vals,
                )
            })
            .collect()
    };
    let stage_labels = a.category_series.clone().unwrap_or_default();
    let text_info = o
        .text_info
        .clone()
        .unwrap_or_else(|| "value+percent_initial".to_string());
    let hover = o.hj();
    let variant = FunnelVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_funnel_html(&FunnelConfig {
        title,
        variant,
        labels: &labels,
        values: &values,
        palette: &o.pal(),
        show_text: o.show_text.unwrap_or(true),
        series: &series,
        stage_labels: &stage_labels,
        text_info: &text_info,
        width: o.w(800),
        height: o.h(480),
        sort_order: &o.srt(),
        hover: &hover,
        ..FunnelConfig::default()
    });
    apply(html, &o)
}
