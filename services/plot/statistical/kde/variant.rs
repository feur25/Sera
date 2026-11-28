crate::plot_family! {
    pub enum KdeVariant default Basic family "kde" {
        Basic       => "basic" | "filled" | "default" | "single" | "multi",
        Outline     => "outline" | "line" | "stroke" | "compare" | "no_fill",
        Stepped     => "stepped" | "step" | "stair" | "stairs",
        Rug         => "rug" | "carpet" | "ticks" | "rugplot",
        Histogram   => "histogram" | "hist" | "with_hist" | "kdehist" | "distplot",
        Normalized  => "normalized" | "pdf" | "norm" | "density",
        Cumulative  => "cumulative" | "cdf" | "cum",
        Contour     => "contour" | "bivariate" | "kde2d" | "joint_density" | "smooth",
        Levels      => "levels" | "bands" | "iso_bands" | "ring_contour" | "banded",
        Stack       => "stack" | "stacked" | "layered_stack",
        Fill        => "fill" | "stack100" | "percent_stack" | "filled_stack",
    }
}
