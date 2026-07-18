crate::plot_family! {
    pub enum GaugeVariant default Basic {
        Basic      => "basic" | "default" | "half" | "classic",
        Radial     => "radial" | "donut" | "ring" | "full",
        Arc270     => "arc270" | "three_quarter" | "arc" | "wide",
        Sleek      => "sleek" | "minimal" | "clean" | "flat",
        Tick       => "tick" | "tickmarks" | "scaled" | "ruler",
        Segmented  => "segmented" | "battery" | "signal" | "chunked",
        Glow       => "glow" | "neon" | "halo" | "luminous",
        Concentric => "concentric" | "rings" | "target" | "dual",
    }
}
