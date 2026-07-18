crate::plot_family! {
    pub enum VennVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Euler    => "euler" | "proportional" | "area",
        Filled   => "filled" | "solid" | "opaque",
        Minimal  => "minimal" | "outline" | "thin",
        Exclusive => "exclusive" | "unique" | "distinct",
    }
}
