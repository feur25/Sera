crate::plot_family! {
    pub enum ParallelVariant default Basic {
        Basic       => "basic" | "default" | "classic" | "lines",
        Smooth      => "smooth" | "curved" | "bezier" | "spline",
        Categorical => "categorical" | "category" | "groups" | "colored",
        Highlight   => "highlight" | "spotlight" | "focus" | "dim",
        Density     => "density" | "fade" | "translucent" | "alpha",
        Gradient    => "gradient" | "value" | "ramp" | "shaded",
    }
}
