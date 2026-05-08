crate::plot_family! {
    pub enum BubbleVariant default Basic {
        Basic       => "basic" | "simple",
        Categorical => "categorical" | "grouped" | "groups" | "category",
        Gradient    => "gradient" | "colorscale" | "continuous" | "scaled",
        Labeled     => "labeled" | "labels" | "text" | "annotated",
        Outlined    => "outlined" | "hollow" | "ring" | "open",
        Negative    => "negative" | "signed" | "diverging",
        Plasma      => "plasma" | "orb" | "sphere" | "3d" | "glow",
    }
}
