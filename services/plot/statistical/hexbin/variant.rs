crate::plot_family! {
    pub enum HexbinVariant default Basic family "hexbin" {
        Basic     => "basic" | "default" | "classic" | "filled",
        Outlined  => "outlined" | "outline" | "stroke" | "labeled",
        Spaced    => "spaced" | "gapped" | "confetti" | "dotted",
        Highlight => "highlight" | "top" | "hotspot" | "peak",
        Mincnt    => "mincnt" | "threshold" | "sparse",
        Nested    => "nested" | "magnitude" | "rings" | "centroids",
    }
}
