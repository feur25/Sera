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
        CircularLabeled => "circular_labeled" | "circular_labels" | "radial_labeled",
        CircularGrouped => "circular_grouped" | "radial_grouped" | "circular_groups",
        CircularGrid    => "circular_grid" | "circular_final" | "radial_grid",
        Pyramid         => "population_pyramid" | "pyramid" | "age_pyramid",
    }
}
