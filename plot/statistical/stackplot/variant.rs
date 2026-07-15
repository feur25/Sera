crate::plot_family! {
    pub enum StackplotVariant default Basic {
        Basic       => "basic" | "default" | "classic" | "stacked",
        Streamgraph => "streamgraph" | "stream" | "silhouette" | "themeriver",
    }
}
