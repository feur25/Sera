crate::plot_family! {
    pub enum GanttVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "flat",
        Progress => "progress" | "percent" | "completion" | "filled",
        Gradient => "gradient" | "duration" | "colorscale" | "heat",
    }
}
