crate::plot_family! {
    pub enum CorrelogramVariant default Circle {
        Circle   => "circle" | "default" | "classic",
        Heatmap  => "heatmap" | "heat" | "square",
        Text     => "text" | "number" | "value",
        Mixed    => "mixed" | "combo" | "both",
        Gradient => "gradient" | "color" | "diverging",
    }
}
