crate::plot_family! {
    pub enum SankeyVariant default Basic family "sankey" {
        Basic    => "basic" | "default" | "classic",
        Gapped   => "gapped" | "spaced" | "separated",
        Ribbon   => "ribbon" | "wide" | "thick",
        Minimal  => "minimal" | "thin" | "outline",
        Sorted   => "sorted" | "reordered" | "by_flow" | "ranked",
        Hourglass => "hourglass" | "radiant_flow" | "nutrient_flow" | "braided" | "flow_bloom",
        Matrix    => "matrix" | "mosaic" | "dot_matrix" | "grid_flow" | "big_data",
        Beacon    => "beacon" | "flight_radar" | "route_wheel" | "departure_board" | "hub_wheel",
    }
}