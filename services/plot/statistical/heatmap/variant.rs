crate::plot_family! {
    pub enum HeatmapVariant default Basic family "heatmap" {
        Basic       => "basic" | "simple" | "default" | "matrix",
        Annotated   => "annotated" | "annotate" | "labeled" | "values",
        Categorical => "categorical" | "category" | "discrete_labels" | "cat",
        Unequal     => "unequal" | "irregular" | "weighted" | "uneven",
        Log         => "log" | "logarithmic" | "log_scale" | "log10",
        Discrete    => "discrete" | "binned" | "stepped" | "bands",
        Correlation => "correlation" | "corr" | "diverging" | "pearson",
        Density     => "density" | "imshow" | "viridis" | "smooth",
        Contour     => "contour" | "iso" | "isolines" | "level",
        Temporal    => "temporal" | "calendar" | "time" | "date" | "timeseries",
        Cluster     => "cluster" | "clustermap" | "dendrogram" | "reorder",
        Bubble      => "bubble" | "size_scaled" | "circle_heatmap" | "punchcard",
        Marginal    => "marginal" | "with_marginals" | "histograms" | "side_bars",
        Confusion   => "confusion" | "confusion_matrix" | "classifier" | "cm",
        Pivot       => "pivot" | "pivot_table" | "totals" | "summary",
        Polar       => "polar" | "wheel" | "clock" | "radial_heat" | "carbon_wheel",
        RadialCluster => "radial_cluster" | "circular_cluster" | "circos" | "radial_dendrogram" | "circular_dendrogram",
        HexGrid => "hex_grid" | "hexbin_grid" | "hex_calendar" | "honeycomb" | "hex_matrix",
        Horizon => "horizon" | "horizon_chart" | "stock_ridge" | "banded_horizon" | "sentiment_bands",
        Moods => "moods" | "mood_matrix" | "punchcard_grouped" | "library" | "sentiment_grid",
    }
}
