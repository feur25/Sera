use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod gradient;
pub mod percent;
pub mod spline;
pub mod stacked;
pub mod step;
pub mod variant;

pub use config::AreaConfig;
pub use variant::AreaVariant;

pub fn render_area_html(cfg: &AreaConfig) -> String {
    use variant::AreaVariant::*;
    match cfg.variant {
        Basic => basic::render(cfg),
        Stacked => stacked::render(cfg),
        Percent => percent::render(cfg),
        Spline => spline::render(cfg),
        Step => step::render(cfg),
        Gradient => gradient::render(cfg),
    }
}

pub use build as build_area_chart;

#[crate::sera_alias("area", "area_chart", "area_family", "area_unified")]
#[crate::sera_builder("build_area_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_default();
    let values_fallback = a.values;
    let series_flat = a
        .series
        .unwrap_or_else(|| values_fallback.map(|v| vec![v]).unwrap_or_default());
    let hover = o.hj();
    let sn = o.series_names.clone().unwrap_or_default();
    let names: Vec<String> = if sn.is_empty() {
        (0..series_flat.len()).map(|_| String::new()).collect()
    } else {
        sn
    };
    let series: Vec<(String, Vec<f64>)> = names.into_iter().zip(series_flat.into_iter()).collect();

    let dec = crate::plot::decimate::Decimator::for_series(o.max_points, &series);
    let x_labels = dec.apply(x_labels);
    let series = dec.apply_each(series);

    let variant = AreaVariant::from_str(
        o.variant
            .as_deref()
            .unwrap_or(if o.stacked.unwrap_or(false) { "stacked" } else { "basic" }),
    );

    let html = render_area_html(&AreaConfig {
        title,
        variant,
        x_labels: &x_labels,
        series: &series,
        palette: &o.pal(),
        x_label: &o.xl(),
        y_label: &o.yl(),
        gridlines: o.grid(),
        width: o.w(1100),
        height: o.h(480),
        hover: &hover,
        sort_order: &o.srt(),
        ..AreaConfig::default()
    });
    apply(html, &o)
}
