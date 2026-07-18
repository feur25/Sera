use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod cumulative;
pub mod histogram;
pub mod normalized;
pub mod outline;
pub mod rug;
pub mod stepped;
pub mod variant;

pub use common::{kde_eval, scott_bw};
pub use config::KdeConfig;
pub use variant::KdeVariant;

pub fn render_kde_html(cfg: &KdeConfig) -> String {
    use variant::KdeVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Outline => outline::render(cfg),
        Stepped => stepped::render(cfg),
        Rug => rug::render(cfg),
        Histogram => histogram::render(cfg),
        Normalized => normalized::render(cfg),
        Cumulative => cumulative::render(cfg),
    }
}

pub use build as build_kde_chart;

#[crate::sera_alias("kde", "kdes", "kde_chart", "kde_family", "density", "density_plot")]
#[crate::sera_builder("build_kde_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let values = a.values.unwrap_or_default();
    use crate::plot::statistical::{render_kde_html, KdeConfig, KdeVariant};
    let series: Vec<(String, Vec<f64>)> = if let Some(cats) = a.categories {
        let mut group_order: Vec<String> = Vec::new();
        let mut group_vals: std::collections::HashMap<String, Vec<f64>> =
            std::collections::HashMap::new();
        for (v, c) in values.iter().zip(cats.iter()) {
            group_vals.entry(c.clone()).or_default().push(*v);
            if !group_order.contains(c) {
                group_order.push(c.clone());
            }
        }
        group_order
            .into_iter()
            .map(|k| {
                let v = group_vals.remove(&k).unwrap_or_default();
                (k, v)
            })
            .collect()
    } else {
        vec![("Series".to_string(), values)]
    };
    let hover = o.hj();
    let xl = o.xl();
    let yl = if o.y_label.is_none() {
        "Density".to_string()
    } else {
        o.yl()
    };
    let variant = KdeVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let html = render_kde_html(&KdeConfig {
        title,
        variant,
        series: &series,
        palette: &o.pal(),
        x_label: &xl,
        y_label: &yl,
        bandwidth: o.bandwidth.unwrap_or(0.0),
        filled: o.filled.unwrap_or(true),
        fill_opacity: o.fill_opacity.unwrap_or(50) as u8,
        gridlines: o.grid(),
        bins: o.bins.unwrap_or(0) as usize,
        width: o.w(900),
        height: o.h(420),
        sort_order: &o.srt(),
        hover: &hover,
        ..KdeConfig::default()
    });
    apply(html, &o)
}
