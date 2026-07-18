crate::plot_family! {
    pub enum IcicleVariant default Basic {
        Basic      => "basic" | "default" | "classic" | "layers",
        Gapped     => "gapped" | "spaced" | "isolated" | "padded",
        Horizontal => "horizontal" | "h" | "sideways" | "left_to_right",
        Radial     => "radial" | "sunburst" | "polar" | "mandala",
        Rank       => "rank" | "percentile" | "peer_rank" | "standing",
    }
}
