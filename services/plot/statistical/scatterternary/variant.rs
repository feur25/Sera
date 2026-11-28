crate::plot_family! {
    pub enum ScatterTernaryVariant default Basic family "scatterternary" {
        Basic    => "basic" | "default" | "classic" | "dots",
        Bubble   => "bubble" | "sized" | "weighted" | "proportional",
        Labeled  => "labeled" | "labelled" | "annotated" | "named",
    }
}
