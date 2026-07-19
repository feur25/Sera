crate::plot_family! {
    pub enum ChordVariant default Basic family "chord" {
        Basic    => "basic" | "default" | "classic",
        Ribbon   => "ribbon" | "wide" | "thick",
        Arc      => "arc" | "ring" | "outer",
        Mono     => "mono" | "monochrome" | "single",
        Directed => "directed" | "asymmetric" | "flow_direction" | "arrows",
    }
}
