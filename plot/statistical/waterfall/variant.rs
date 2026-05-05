crate::plot_family! {
    pub enum WaterfallVariant default Basic {
        Basic      => "basic" | "default" | "classic" | "bars",
        Stepped    => "stepped" | "step" | "staircase" | "stairs",
        Lollipop   => "lollipop" | "stick" | "popsicle" | "lolly",
        Arrowed    => "arrowed" | "arrow" | "directional" | "tipped",
        Delta      => "delta" | "percent" | "annotated" | "pct",
        Horizontal => "horizontal" | "rows" | "sideways" | "h",
    }
}
