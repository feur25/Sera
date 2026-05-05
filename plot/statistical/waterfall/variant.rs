crate::plot_family! {
    pub enum WaterfallVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "bars",
        Stepped  => "stepped" | "step" | "staircase" | "stairs",
        Gradient => "gradient" | "shaded" | "fade" | "vgrad",
        Rounded  => "rounded" | "round" | "pill" | "soft",
        Lollipop => "lollipop" | "stick" | "popsicle" | "lolly",
        Arrowed  => "arrowed" | "arrow" | "directional" | "tipped",
        Delta    => "delta" | "percent" | "annotated" | "pct",
        Outlined => "outlined" | "outline" | "stroke" | "wireframe",
    }
}
