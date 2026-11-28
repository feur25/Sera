crate::plot_family! {
    pub enum ArcDiagramVariant default Basic family "arc_diagram" {
        Basic      => "basic" | "default" | "classic",
        Bilateral  => "bilateral" | "both" | "dual",
        Weighted   => "weighted" | "width" | "value",
        Minimal    => "minimal" | "thin" | "clean",
        Directed   => "directed" | "arrows" | "flow" | "dependency",
    }
}
