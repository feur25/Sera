crate::plot_family! {
    pub enum RidgelineVariant default Basic {
        Basic     => "basic" | "filled" | "default" | "single" | "multi",
        Gradient  => "gradient" | "shade" | "fade" | "vgrad",
        Lines     => "lines" | "outline" | "stroke" | "no_fill",
        Quartiles => "quartiles" | "q" | "qrt" | "iqr",
        Mean      => "mean" | "average" | "avg" | "mean_dot",
        Rug       => "rug" | "ticks" | "carpet" | "rugplot",
        Heatmap   => "heatmap" | "heat" | "rainbow" | "colored",
        Spaced    => "spaced" | "separated" | "no_overlap" | "split",
        Deluxe    => "deluxe" | "premium" | "aurora" | "neon",
    }
}


