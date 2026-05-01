crate::plot_family! {
    pub enum PieVariant default Basic {
        Basic        => "basic" | "pie",
        Donut        => "donut" | "ring" | "hole",
        Exploded     => "exploded" | "pulled" | "pull" | "explode",
        Subplots     => "subplots" | "grid" | "facet" | "multi",
        Proportional => "proportional" | "scaled" | "scalegroup" | "area_proportional",
    }
}
