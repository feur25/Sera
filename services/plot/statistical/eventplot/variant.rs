crate::plot_family! {
    pub enum EventplotVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "ticks",
        Density  => "density" | "kde" | "smoothed" | "rug",
        Connected => "connected" | "sequence" | "path" | "trajectory",
    }
}
