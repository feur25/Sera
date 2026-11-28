crate::plot_family! {
    pub enum EventplotVariant default Basic family "eventplot" {
        Basic    => "basic" | "default" | "classic" | "ticks",
        Density  => "density" | "kde" | "smoothed" | "rug",
        Connected => "connected" | "sequence" | "path" | "trajectory",
    }
}
