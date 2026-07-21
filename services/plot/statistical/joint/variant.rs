crate::plot_family! {
    pub enum JointVariant default HexbinMarginal family "joint" {
        HexbinMarginal      => "hexbin_marginal" | "hexbin" | "hexbin_marginals",
        HeatScatter         => "heat_scatter" | "heatscatter" | "density_heat" | "joint_histogram" | "histogram2d" | "hist2d",
        LayeredBivariate    => "layered_bivariate" | "kde_scatter" | "layered",
        JointKde            => "joint_kde" | "kde_joint" | "bivariate_kde",
        KdeSmooth           => "kde_smooth" | "smooth_bivariate_kde" | "smooth_kde",
        MultipleBivariateKde => "multiple_bivariate_kde" | "kde_multi" | "overlapping_kde",
        MarginalTicks       => "marginal_ticks" | "rug_marginal" | "rug",
        RegressionMarginals => "regression_marginals" | "regression_joint" | "joint_regression",
    }
}
