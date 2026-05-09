crate::plot_family! {
    pub enum PieVariant default Basic {
        Basic        => "basic" | "pie",
        Donut        => "donut" | "ring" | "hole",
        Exploded     => "exploded" | "pulled" | "pull" | "explode",
        Subplots     => "subplots" | "grid" | "facet" | "multi",
        Proportional => "proportional" | "scaled" | "scalegroup" | "area_proportional",
        Semi         => "semi" | "semicircle" | "half" | "halfpie" | "half_pie",
        Kpi          => "kpi" | "center" | "centered" | "indicator" | "donut_kpi" | "metric",
        Nested       => "nested" | "concentric" | "rings" | "double_ring" | "multi_ring",
        Pattern      => "pattern" | "patterned" | "textured" | "hatched",
    }
}


