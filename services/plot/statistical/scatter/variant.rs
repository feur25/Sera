crate::plot_family! {
    pub enum ScatterVariant default Basic family "scatter" {
        Basic       => "basic" | "simple" | "default",
        Categorical => "categorical" | "grouped" | "groups" | "category",
        Symbols     => "symbols" | "marker" | "markers" | "shape" | "shapes",
        Labeled     => "labeled" | "labels" | "text" | "annotated",
        Regression  => "regression" | "trendline" | "fit" | "ols",
    }
}
