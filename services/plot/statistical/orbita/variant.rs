crate::plot_family! {
    pub enum OrbitaVariant default Classic family "orbita" {
        Classic  => "classic" | "default" | "basic",
        Bubble   => "bubble" | "sized" | "area",
        Trail    => "trail" | "line" | "connected",
        Glow     => "glow" | "neon" | "light",
        Minimal  => "minimal" | "thin" | "clean",
        Delta    => "delta" | "change" | "trend" | "momentum",
    }
}
