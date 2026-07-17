crate::plot_family! {
    pub enum StackplotVariant default Basic {
        Basic       => "basic" | "default" | "classic" | "stacked",
        Streamgraph => "streamgraph" | "stream" | "silhouette" | "themeriver",
        Normalized  => "normalized" | "percent" | "hundred_percent" | "share",
        Gradient    => "gradient" | "fade" | "depth" | "shaded",
    }
}
