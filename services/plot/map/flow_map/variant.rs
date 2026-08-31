crate::plot_family! {
    pub enum FlowMapVariant default Arc family "flow_map" kind "map" {
        Arc      => "arc" | "basic" | "default" | "curved" | "great_circle",
        Straight => "straight" | "line" | "direct",
        Animated => "animated" | "dashed" | "moving" | "flow_dash",
        Ribbon   => "ribbon" | "tapered" | "band" | "river",
        Track    => "track" | "storm_track" | "path_track" | "hurricane",
    }
}
