crate::plot_family! {
    pub enum SankeyVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Gapped   => "gapped" | "spaced" | "separated",
        Ribbon   => "ribbon" | "wide" | "thick",
        Minimal  => "minimal" | "thin" | "outline",
        Sorted   => "sorted" | "reordered" | "by_flow" | "ranked",
    }
}