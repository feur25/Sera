crate::plot_family! {
    pub enum GanttVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "flat",
        Progress => "progress" | "percent" | "completion" | "filled",
        Milestone => "milestone" | "diamonds" | "checkpoints" | "markers",
    }
}
