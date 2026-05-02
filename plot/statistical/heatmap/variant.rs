crate::plot_family! {
    pub enum HeatmapVariant default Basic {
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
    }
}
