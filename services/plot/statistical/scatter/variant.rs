crate::plot_family! {
    pub enum ScatterVariant default Basic family "scatter" {
        Basic         => "basic" | "simple" | "default",
        Categorical   => "categorical" | "grouped" | "groups" | "category",
        Symbols       => "symbols" | "marker" | "markers" | "shape" | "shapes",
        Labeled       => "labeled" | "labels" | "text" | "annotated",
        Regression    => "regression" | "trendline" | "fit" | "ols",
        Residual      => "residual" | "residuals" | "residplot",
        DualStyle     => "dual_style" | "hue_style" | "two_way",
        ContinuousHue => "continuous_hue" | "numeric_hue" | "colormap",
        Facet         => "facet" | "facets" | "small_multiples" | "relplot",
        Sized         => "sized" | "size_scale" | "bubble_scatter" | "magnitude_size",
        WideForm      => "wide_form" | "wide" | "multi_series" | "columns",
        Rug           => "rug" | "rugplot" | "marginal_ticks" | "carpet_ticks",
    }
}
