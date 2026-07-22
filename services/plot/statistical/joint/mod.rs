use crate::plot::{apply, parse_all};
pub mod common;
pub mod config;
pub mod heat_grid;
pub mod hexbin_panel;
pub mod kde_contour;
pub mod kde_heat;
pub mod scatter_panel;
pub mod variant;

pub use config::JointConfig;
pub use variant::{JointMarginal, JointVariant};

pub fn render_joint_html(cfg: &JointConfig) -> String {
    use variant::JointVariant::*;
    match cfg.variant {
        Hexbin => hexbin_panel::render(cfg),
        HeatGrid => heat_grid::render(cfg),
        KdeHeat => kde_heat::render(cfg),
        KdeContour => kde_contour::render(cfg),
        Scatter => scatter_panel::render(cfg),
    }
}

pub use build as build_joint;

#[crate::sera_alias("joint", "jointplot", "joint_plot", "bivariate")]
#[crate::sera_builder("build_joint")]
pub fn build(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x_values = a.x.unwrap_or_default();
    let y_values = a.y.unwrap_or_default();
    use crate::plot::statistical::{render_joint_html, JointConfig, JointVariant};
    let hover = o.hj();
    let variant_name = o.variant.as_deref().unwrap_or("hexbin");
    let preset = variant::resolve_preset(variant_name);
    let panel_name = preset.as_ref().map(|p| p.panel).unwrap_or(variant_name);
    let variant = JointVariant::from_str(panel_name);
    let marginal = o
        .marginal
        .as_deref()
        .map(JointMarginal::from_str)
        .or_else(|| preset.as_ref().map(|p| p.marginal))
        .unwrap_or(if variant == JointVariant::KdeContour {
            JointMarginal::None
        } else {
            JointMarginal::Histogram
        });
    let show_points = o
        .show_points
        .or_else(|| preset.as_ref().map(|p| p.show_points))
        .unwrap_or(false);
    let show_regression = o
        .show_regression
        .or_else(|| preset.as_ref().map(|p| p.show_regression))
        .unwrap_or(false);
    let panel_variant = o.panel_variant.clone().unwrap_or_else(|| "basic".to_string());
    let colorscale = o.colorscale.clone().unwrap_or_default();
    let xl = o.xl();
    let yl = o.yl();
    let group_series: Vec<(String, Vec<f64>, Vec<f64>)> = if let Some(cats) = a.categories.as_ref() {
        let mut order: Vec<String> = Vec::new();
        let mut xs_by: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        let mut ys_by: std::collections::HashMap<String, Vec<f64>> = std::collections::HashMap::new();
        let n = x_values.len().min(y_values.len()).min(cats.len());
        for i in 0..n {
            let c = &cats[i];
            xs_by.entry(c.clone()).or_default().push(x_values[i]);
            ys_by.entry(c.clone()).or_default().push(y_values[i]);
            if !order.contains(c) {
                order.push(c.clone());
            }
        }
        order
            .into_iter()
            .map(|k| {
                let xs = xs_by.remove(&k).unwrap_or_default();
                let ys = ys_by.remove(&k).unwrap_or_default();
                (k, xs, ys)
            })
            .collect()
    } else {
        Vec::new()
    };
    let html = render_joint_html(&JointConfig {
        title,
        variant,
        marginal,
        panel_variant: &panel_variant,
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_values: &x_values,
        y_values: &y_values,
        bins: o.bins.map(|b| b.max(2) as usize).unwrap_or(24),
        colorscale: &colorscale,
        point_hex: o.color_hex.unwrap_or(0x6366f1),
        palette: &o.pal(),
        show_points,
        show_regression,
        group_series: &group_series,
        width: o.w(760),
        height: o.h(760),
        hover: &hover,
        ..JointConfig::default()
    });
    apply(html, &o)
}

#[cfg(test)]
mod tests {
    use super::build;

    fn xy_json() -> serde_json::Value {
        let x: Vec<f64> = (0..40).map(|i| (i as f64 * 0.31).sin() * 3.0 + i as f64 * 0.12).collect();
        let y: Vec<f64> = (0..40).map(|i| (i as f64 * 0.21).cos() * 2.5 + i as f64 * 0.09).collect();
        serde_json::json!({ "x": x, "y": y })
    }

    #[test]
    fn every_panel_renders_with_every_marginal() {
        for panel in ["hexbin", "heat_grid", "kde_heat", "scatter"] {
            for marginal in ["histogram", "kde", "rug", "none"] {
                let mut input = xy_json();
                input["variant"] = serde_json::json!(panel);
                input["marginal"] = serde_json::json!(marginal);
                let html = build(&input.to_string());
                assert!(html.contains("<svg"), "panel={panel} marginal={marginal} produced no svg");
            }
        }
    }

    #[test]
    fn hexbin_panel_variant_changes_cell_styling() {
        let mut basic = xy_json();
        basic["variant"] = serde_json::json!("hexbin");
        basic["panel_variant"] = serde_json::json!("basic");
        let basic_html = build(&basic.to_string());

        let mut outlined = xy_json();
        outlined["variant"] = serde_json::json!("hexbin");
        outlined["panel_variant"] = serde_json::json!("outlined");
        let outlined_html = build(&outlined.to_string());

        assert!(outlined_html.contains("stroke=\"#fff\" stroke-width=\"1\""));
        assert!(!basic_html.contains("stroke=\"#fff\" stroke-width=\"1\""));
        assert_ne!(basic_html, outlined_html);
    }

    #[test]
    fn show_points_and_show_regression_toggle_overlays() {
        let mut with_points = xy_json();
        with_points["variant"] = serde_json::json!("kde_heat");
        with_points["show_points"] = serde_json::json!(true);
        assert!(build(&with_points.to_string()).contains("<circle"));

        let mut without_points = xy_json();
        without_points["variant"] = serde_json::json!("kde_heat");
        assert!(!build(&without_points.to_string()).contains("<circle"));

        let mut with_reg = xy_json();
        with_reg["variant"] = serde_json::json!("scatter");
        with_reg["show_regression"] = serde_json::json!(true);
        assert!(build(&with_reg.to_string()).matches("<path").count() >= 1);
    }

    #[test]
    fn legacy_presets_still_resolve_to_expected_shape() {
        for legacy in [
            "hexbin_marginal",
            "heat_scatter",
            "layered_bivariate",
            "joint_kde",
            "kde_smooth",
            "marginal_ticks",
            "regression_marginals",
        ] {
            let mut input = xy_json();
            input["variant"] = serde_json::json!(legacy);
            let html = build(&input.to_string());
            assert!(!html.is_empty(), "legacy variant {legacy} produced empty html");
            assert!(html.contains("<svg"), "legacy variant {legacy} produced no svg");
        }

        let mut layered = xy_json();
        layered["variant"] = serde_json::json!("layered_bivariate");
        assert!(build(&layered.to_string()).contains("<circle"));

        let mut kde_smooth = xy_json();
        kde_smooth["variant"] = serde_json::json!("kde_smooth");
        assert!(!build(&kde_smooth.to_string()).contains("<circle"));
    }

    #[test]
    fn legacy_multiple_bivariate_kde_still_uses_categories_grouping() {
        let input = serde_json::json!({
            "variant": "multiple_bivariate_kde",
            "x": [1.2, 2.4, 2.1, 3.6, 3.1, 3.9, 2.2, 2.6, 4.2, 4.6, 4.4, 4.9, 5.5, 5.1, 5.8, 5.2],
            "y": [1.1, 2.3, 3.2, 2.4, 3.6, 4.1, 2.1, 3.1, 3.3, 4.7, 5.2, 3.9, 4.4, 5.6, 6.1, 4.5],
            "categories": ["a", "a", "a", "a", "a", "a", "a", "a", "b", "b", "b", "b", "b", "b", "b", "b"],
        });
        let html = build(&input.to_string());
        assert!(html.contains("<svg"));
    }
}
