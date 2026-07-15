crate::plot_family! {
    pub enum OrbitaVariant default Classic {
        Classic  => "classic" | "default" | "basic",
        Bubble   => "bubble" | "sized" | "area",
        Trail    => "trail" | "line" | "connected",
        Glow     => "glow" | "neon" | "light",
        Minimal  => "minimal" | "thin" | "clean",
    }
}
