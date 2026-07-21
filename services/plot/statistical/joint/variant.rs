crate::plot_family! {
    pub enum JointVariant default HexbinMarginal family "joint" {
        HexbinMarginal   => "hexbin_marginal" | "hexbin" | "hexbin_marginals",
        HeatScatter      => "heat_scatter" | "heatscatter" | "density_heat",
        LayeredBivariate => "layered_bivariate" | "kde_scatter" | "layered",
    }
}
