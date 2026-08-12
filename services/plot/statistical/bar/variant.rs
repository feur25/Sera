crate::plot_family! {
    pub enum BarVariant default Basic family "bar" {
        Basic           => "basic",
        Horizontal      => "horizontal" | "h" | "hbar",
        Grouped         => "grouped" | "group",
        Stacked         => "stacked" | "stack",
        Relative        => "relative" | "rel",
        GroupedStacked  => "grouped_stacked" | "groupstack" | "grouped-stacked",
        Marimekko       => "marimekko" | "mekko" | "mosaic",
        Pictogram       => "pictogram" | "icon",
        Multicategory   => "multicategory" | "multi" | "hierarchical",
        Circular        => "circular" | "circular_basic" | "radial_bar" | "polar_bar",
        CircularGrouped => "circular_grouped" | "radial_grouped" | "circular_groups",
        Pyramid         => "population_pyramid" | "pyramid" | "age_pyramid",
        Diverging       => "diverging" | "signed" | "delta" | "bidirectional",
        Distribution    => "distribution" | "bar_box" | "boxbar" | "bar_boxplot",
        Spiral          => "spiral" | "spiral_bar" | "nautilus" | "radial_spiral" | "growth_spiral",
        Hedgehog        => "hedgehog" | "flow_fan" | "quill" | "spike_flow" | "relocation_fan",
    }
}
