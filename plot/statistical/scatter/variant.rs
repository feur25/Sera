crate::plot_family! {
    pub enum ScatterVariant default Basic {
        Basic       => "basic" | "simple" | "default",
        Categorical => "categorical" | "grouped" | "groups" | "category",
        Gradient    => "gradient" | "colorscale" | "continuous" | "scaled",
        Symbols     => "symbols" | "marker" | "markers" | "shape" | "shapes",
        Labeled     => "labeled" | "labels" | "text" | "annotated",
        Regression  => "regression" | "trendline" | "fit" | "ols",
        Galaxy      => "galaxy" | "nebula" | "cosmic" | "glow",
        Nova        => "nova" | "burst" | "starburst" | "spike",
        Deluxe      => "deluxe" | "premium" | "stellar" | "dark",
    }
}
