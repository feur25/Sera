crate::plot_family! {
    pub enum BarVariant default Basic {
        Basic           => "basic",
        Horizontal      => "horizontal" | "h" | "hbar",
        Grouped         => "grouped" | "group",
        Stacked         => "stacked" | "stack",
        Relative        => "relative" | "rel",
        GroupedStacked  => "grouped_stacked" | "groupstack" | "grouped-stacked",
        Marimekko       => "marimekko" | "mekko" | "mosaic",
        Pictogram       => "pictogram" | "icon",
        Multicategory   => "multicategory" | "multi" | "hierarchical",
        Deluxe          => "deluxe" | "premium" | "neon" | "dark",
        Prism           => "prism" | "crystal" | "glass" | "rainbow",
    }
}
