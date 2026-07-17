crate::plot_family! {
    pub enum ArcDiagramVariant default Basic {
        Basic      => "basic" | "default" | "classic",
        Bilateral  => "bilateral" | "both" | "dual",
        Weighted   => "weighted" | "width" | "value",
        Gradient   => "gradient" | "color",
        Minimal    => "minimal" | "thin" | "clean",
        Directed   => "directed" | "arrows" | "flow" | "dependency",
    }
}
