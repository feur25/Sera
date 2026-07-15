crate::plot_family! {
    pub enum ChordVariant default Basic {
        Basic    => "basic" | "default" | "classic",
        Gradient => "gradient" | "color" | "flow",
        Ribbon   => "ribbon" | "wide" | "thick",
        Arc      => "arc" | "ring" | "outer",
        Mono     => "mono" | "monochrome" | "single",
    }
}
