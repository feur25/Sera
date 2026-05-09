crate::plot_family! {
    pub enum HistogramVariant default Basic {
        Basic       => "basic" | "simple" | "default" | "vertical",
        Horizontal  => "horizontal" | "h" | "barh" | "hbar",
        Normalized  => "normalized" | "probability" | "density" | "norm" | "pdf",
        Cumulative  => "cumulative" | "cdf" | "cum",
        Stacked     => "stacked" | "stack" | "stack_by",
        Overlay     => "overlay" | "overlapping" | "compare" | "ab",
        Step        => "step" | "outline" | "stair",
        Deluxe      => "deluxe" | "premium" | "neon" | "dark",
    }
}


