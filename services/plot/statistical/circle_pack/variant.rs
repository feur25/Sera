crate::plot_family! {
    pub enum CirclePackVariant default Basic family "circle_pack" {
        Basic    => "basic" | "default" | "classic",
        Flat     => "flat" | "single" | "packed",
        Outlined => "outlined" | "outline" | "stroke",
        Bubble   => "bubble" | "bubbles" | "plain",
        LeafFocus => "leaf_focus" | "leaves" | "leaves_only" | "focus",
        Swarm => "swarm" | "commit_swarm" | "commit_history" | "dev_swarm" | "orca",
        Matrix => "matrix" | "grid" | "swarm_matrix" | "category_grid" | "space_wars",
    }
}
