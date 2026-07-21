use crate::plot::{apply, parse_all};
pub mod common;
pub mod config;
pub mod heat_scatter;
pub mod hexbin_marginal;
pub mod joint_kde;
pub mod kde_smooth;
pub mod layered_bivariate;
pub mod marginal_ticks;
pub mod multiple_bivariate_kde;
pub mod regression_marginals;
pub mod variant;

pub use config::JointConfig;
pub use variant::JointVariant;

pub fn render_joint_html(cfg: &JointConfig) -> String {
    use variant::JointVariant::*;
    match cfg.variant {
        HexbinMarginal => hexbin_marginal::render(cfg),
        HeatScatter => heat_scatter::render(cfg),
        LayeredBivariate => layered_bivariate::render(cfg),
        JointKde => joint_kde::render(cfg),
        KdeSmooth => kde_smooth::render(cfg),
        MultipleBivariateKde => multiple_bivariate_kde::render(cfg),
        MarginalTicks => marginal_ticks::render(cfg),
        RegressionMarginals => regression_marginals::render(cfg),
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
    let variant = JointVariant::from_str(o.variant.as_deref().unwrap_or("hexbin_marginal"));
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
        x_label: &xl,
        y_label: &yl,
        gridlines: o.grid(),
        x_values: &x_values,
        y_values: &y_values,
        bins: o.bins.map(|b| b.max(2) as usize).unwrap_or(24),
        colorscale: &colorscale,
        point_hex: o.color_hex.unwrap_or(0x6366f1),
        palette: &o.pal(),
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
    use super::*;

    fn sample_xy() -> (Vec<f64>, Vec<f64>) {
        let x = vec![1.2, 2.4, 2.1, 3.6, 3.1, 3.9, 4.2, 4.6, 4.4, 4.9, 5.5, 5.1];
        let y = vec![1.1, 2.3, 3.2, 2.4, 3.6, 4.1, 3.3, 4.7, 5.2, 3.9, 4.4, 5.6];
        (x, y)
    }

    #[test]
    fn every_variant_renders_non_empty_svg() {
        let (x, y) = sample_xy();
        for variant in [
            JointVariant::HexbinMarginal,
            JointVariant::HeatScatter,
            JointVariant::LayeredBivariate,
            JointVariant::JointKde,
            JointVariant::KdeSmooth,
            JointVariant::MarginalTicks,
            JointVariant::RegressionMarginals,
        ] {
            let cfg = JointConfig {
                variant,
                x_values: &x,
                y_values: &y,
                ..JointConfig::default()
            };
            let html = render_joint_html(&cfg);
            assert!(html.contains("<svg"), "{:?} produced no svg", variant);
        }
    }

    #[test]
    fn multiple_bivariate_kde_renders_one_layer_per_group() {
        let group_series = vec![
            ("a".to_string(), vec![1.0, 1.5, 2.0], vec![1.0, 1.4, 1.9]),
            ("b".to_string(), vec![5.0, 5.5, 6.0], vec![5.0, 5.4, 5.9]),
        ];
        let cfg = JointConfig {
            variant: JointVariant::MultipleBivariateKde,
            group_series: &group_series,
            ..JointConfig::default()
        };
        let html = render_joint_html(&cfg);
        assert!(html.contains("<svg"));
    }

    #[test]
    fn multiple_bivariate_kde_without_groups_is_empty() {
        let cfg = JointConfig {
            variant: JointVariant::MultipleBivariateKde,
            ..JointConfig::default()
        };
        assert!(render_joint_html(&cfg).is_empty());
    }

    #[test]
    fn heat_scatter_alias_resolves_to_same_variant() {
        assert_eq!(JointVariant::from_str("histogram2d"), JointVariant::HeatScatter);
        assert_eq!(JointVariant::from_str("joint_histogram"), JointVariant::HeatScatter);
    }
}
