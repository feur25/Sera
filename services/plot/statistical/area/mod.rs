use crate::plot::{apply, parse_all};
pub mod basic;
pub mod common;
pub mod config;
pub mod gradient;
pub mod leader;
pub mod percent;
pub mod ribbon;
pub mod spline;
pub mod stacked;
pub mod step;
pub mod variant;
pub mod wave;

pub use config::AreaConfig;
pub use variant::AreaVariant;

const CANVAS_FALLBACK_THRESHOLD: usize = 3_000;

pub fn render_area_html(cfg: &AreaConfig) -> String {
    use variant::AreaVariant::*;
    if matches!(cfg.variant, Basic | Spline | Step)
        && cfg.series.len() == 1
        && cfg.series[0].1.len() > CANVAS_FALLBACK_THRESHOLD
    {
        let values = &cfg.series[0].1;
        let x_values: Vec<f64> = (0..values.len()).map(|i| i as f64).collect();
        let spec = crate::plot::canvas_points::CanvasPlotSpec {
            title: cfg.title,
            width: cfg.width,
            height: cfg.height,
            x_label: cfg.x_label,
            y_label: cfg.y_label,
            gridlines: cfg.gridlines,
            mode: crate::plot::canvas_points::MODE_LINE,
            color_hex: cfg.palette.first().copied().unwrap_or(0x636EFA),
        };
        return crate::plot::canvas_points::render_canvas_points_html(&spec, &x_values, values);
    }
    match cfg.variant {
        Basic => basic::render(cfg),
        Stacked => stacked::render(cfg),
        Percent => percent::render(cfg),
        Spline => spline::render(cfg),
        Step => step::render(cfg),
        Gradient => gradient::render(cfg),
        Ribbon => ribbon::render(cfg),
        Wave => wave::render(cfg),
        Leader => leader::render(cfg),
    }
}

pub use build as build_area_chart;

#[crate::sera_alias("area", "area_chart", "area_family", "area_unified")]
#[crate::sera_builder("build_area_chart")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_labels = a.x_labels.or(a.labels).unwrap_or_else(|| {
        a.x.as_ref()
            .map(|xs| xs.iter().map(|&v| crate::plot::statistical::common::format_axis_label(v)).collect())
            .unwrap_or_default()
    });
    let values_fallback = a.values.or(a.y);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_area_chart_accepts_x_y_as_an_alias_for_labels_values() {
        let via_xy = build(r#"{"title":"t","x":[2010.0,2011.0,2012.0],"y":[10.0,20.0,15.0]}"#);
        for needle in ["2010", "2011", "2012"] {
            assert!(via_xy.contains(needle), "x/y input should carry the year {needle} through as a visible tick label: {via_xy}");
        }
    }

    #[test]
    fn build_area_chart_prefers_explicit_labels_values_over_x_y_when_both_are_present() {
        let out = build(r#"{"title":"t","x":[8172.0,8173.0],"y":[8172.0,8173.0],"labels":["Alpha","Beta"],"values":[10.0,20.0]}"#);
        assert!(!out.contains("8172") && !out.contains("8173"), "explicit labels/values must take priority over x/y when both are present, but the x-derived label leaked through: {out}");
        assert!(out.contains("Alpha") && out.contains("Beta"), "the explicit labels must appear when labels/values win over x/y: {out}");
    }
}
