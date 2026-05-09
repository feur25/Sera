crate::plot_family! {
    pub enum KdeVariant default Basic {
        Basic       => "basic" | "filled" | "default" | "single" | "multi",
        Outline     => "outline" | "line" | "stroke" | "compare" | "no_fill",
        Stepped     => "stepped" | "step" | "stair" | "stairs",
        Rug         => "rug" | "carpet" | "ticks" | "rugplot",
        Histogram   => "histogram" | "hist" | "with_hist" | "kdehist" | "distplot",
        Normalized  => "normalized" | "pdf" | "norm" | "density",
        Cumulative  => "cumulative" | "cdf" | "cum",
        Gradient    => "gradient" | "shade" | "fade" | "ridge",
    }
}


