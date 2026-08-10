crate::plot_family! {
    pub enum BubbleVariant default Basic family "bubble" {
        Basic       => "basic" | "simple",
        Categorical => "categorical" | "grouped" | "groups" | "category",
        Labeled     => "labeled" | "labels" | "text" | "annotated",
        Outlined    => "outlined" | "hollow" | "ring" | "open",
        Negative    => "negative" | "signed" | "diverging",
        Split       => "split" | "half" | "binary" | "categorical_split",
        Burst       => "burst" | "radial_burst" | "spiral" | "transition" | "topic_burst",
    }
}
