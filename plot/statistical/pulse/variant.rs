crate::plot_family! {
    pub enum PulseVariant default Radial {
        Radial   => "radial" | "default" | "classic",
        Wave     => "wave" | "sine" | "smooth",
        Dot      => "dot" | "scatter" | "bubble",
        Filled   => "filled" | "area" | "solid",
        Gradient => "gradient" | "color" | "heatring",
        Outlined => "outlined" | "outline" | "stroke" | "clean",
    }
}
