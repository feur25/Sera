pub fn resolve_legacy_panel(name: &str) -> Option<&'static str> {
    Some(match name {
        "hexbin_marginal" | "hexbin_marginals" => "hexbin",
        "heat_scatter" | "heatscatter" | "density_heat" | "joint_histogram" | "histogram2d" | "hist2d" => "hexbin",
        "layered_bivariate" | "kde_scatter" | "layered" | "joint_kde" | "kde_joint" | "bivariate_kde" | "kde_smooth"
        | "smooth_bivariate_kde" | "smooth_kde" => "kde",
        "multiple_bivariate_kde" | "kde_multi" | "overlapping_kde" => "kde",
        "marginal_ticks" | "rug_marginal" | "rug" => "scatter",
        "regression_marginals" | "regression_joint" | "joint_regression" => "scatter",
        _ => return None,
    })
}
