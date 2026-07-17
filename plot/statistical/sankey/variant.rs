crate::plot_family! {
    pub enum SankeyVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Gradient => "gradient" | "color" | "flow",
        Gapped   => "gapped" | "spaced" | "separated",
        Ribbon   => "ribbon" | "wide" | "thick",
        Minimal  => "minimal" | "thin" | "outline",
        Labeled  => "labeled" | "labelled" | "values" | "annotated",
    }
}