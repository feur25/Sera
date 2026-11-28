crate::plot_family! {
    pub enum PulseVariant default Radial family "pulse" {
        Radial   => "radial" | "default" | "classic",
        Wave     => "wave" | "sine" | "smooth",
        Dot      => "dot" | "scatter" | "bubble",
        Filled   => "filled" | "area" | "solid",
        Outlined => "outlined" | "outline" | "stroke" | "clean",
    }
}
