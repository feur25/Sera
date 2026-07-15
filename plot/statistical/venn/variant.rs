crate::plot_family! {
    pub enum VennVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Euler    => "euler" | "proportional" | "area",
        Filled   => "filled" | "solid" | "opaque",
        Gradient => "gradient" | "color",
        Minimal  => "minimal" | "outline" | "thin",
    }
}
