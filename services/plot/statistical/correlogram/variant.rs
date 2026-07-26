crate::plot_family! {
    pub enum CorrelogramVariant default Circle family "correlogram" {
        Circle       => "circle" | "default" | "classic",
        Heatmap      => "heatmap" | "heat" | "square",
        Text         => "text" | "number" | "value",
        Ellipse      => "ellipse" | "oval",
        Mixed        => "mixed" | "combo" | "both",
        PieSquare    => "pie_square" | "pie" | "mixed_pie",
        CircleLegend => "circle_legend" | "legend" | "scale",
    }
}
