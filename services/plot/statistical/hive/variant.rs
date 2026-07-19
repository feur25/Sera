crate::plot_family! {
    pub enum HiveVariant default Basic family "hive" {
        Basic    => "basic" | "default" | "classic",
        Curved   => "curved" | "smooth" | "bezier",
        Weighted => "weighted" | "width" | "value",
        Minimal  => "minimal" | "thin" | "clean",
        Directed => "directed" | "arrows" | "flow" | "dependency",
    }
}
