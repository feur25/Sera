crate::plot_family! {
    pub enum CorrelogramVariant default Circle family "correlogram" {
        Circle   => "circle" | "default" | "classic",
        Heatmap  => "heatmap" | "heat" | "square",
        Text     => "text" | "number" | "value",
        Mixed    => "mixed" | "combo" | "both",
        Sorted   => "sorted" | "clustered" | "reordered",
    }
}
