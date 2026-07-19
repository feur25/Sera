crate::plot_family! {
    pub enum HexbinVariant default Basic family "hexbin" {
        Basic    => "basic" | "default" | "classic" | "filled",
        Outlined => "outlined" | "outline" | "stroke" | "labeled",
        Spaced   => "spaced" | "gapped" | "confetti" | "dotted",
        Highlight => "highlight" | "top" | "hotspot" | "peak",
    }
}
