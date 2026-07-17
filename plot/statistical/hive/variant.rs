crate::plot_family! {
    pub enum HiveVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Curved   => "curved" | "smooth" | "bezier",
        Gradient => "gradient" | "color",
        Weighted => "weighted" | "width" | "value",
        Minimal  => "minimal" | "thin" | "clean",
        Labeled  => "labeled" | "labelled" | "values" | "annotated",
    }
}
