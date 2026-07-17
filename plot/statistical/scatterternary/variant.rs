crate::plot_family! {
    pub enum ScatterTernaryVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "dots",
        Gradient => "gradient" | "color" | "colorscale" | "continuous",
        Bubble   => "bubble" | "sized" | "weighted" | "proportional",
        Labeled  => "labeled" | "labelled" | "annotated" | "named",
    }
}
