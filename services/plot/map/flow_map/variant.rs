crate::plot_family! {
    pub enum FlowMapVariant default Arc family "flow_map" kind "map" {
        Arc      => "arc" | "basic" | "default" | "curved" | "great_circle",
        Straight => "straight" | "line" | "direct",
    }
}
