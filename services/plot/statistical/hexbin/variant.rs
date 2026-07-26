crate::plot_family! {
    pub enum HexbinVariant default Basic family "hexbin" {
        Basic     => "basic" | "default" | "classic" | "filled",
        Outlined  => "outlined" | "outline" | "stroke" | "labeled",
        Spaced    => "spaced" | "gapped" | "confetti",
        Highlight => "highlight" | "top" | "hotspot" | "peak",
        Mincnt    => "mincnt" | "threshold" | "sparse",
        Nested    => "nested" | "magnitude" | "rings" | "centroids",
        LogCounts => "log_counts" | "log" | "log_scale" | "logarithmic",
        Weighted  => "weighted" | "mean" | "aggregate" | "reduce_mean",
        Dotted    => "dotted" | "dashed" | "styled" | "magma",
        Marginals => "marginals" | "joint" | "with_histograms" | "density_marginals",
    }
}
