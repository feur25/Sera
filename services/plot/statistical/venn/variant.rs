crate::plot_family! {
    pub enum VennVariant default Basic family "venn" {
        Basic    => "basic" | "default" | "classic",
        Euler    => "euler" | "proportional" | "area",
        Filled   => "filled" | "solid" | "opaque",
        Minimal  => "minimal" | "outline" | "thin",
        Exclusive => "exclusive" | "unique" | "distinct",
    }
}
