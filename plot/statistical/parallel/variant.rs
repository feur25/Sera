crate::plot_family! {
    pub enum ParallelVariant default Basic {
        Basic       => "basic" | "default" | "classic" | "lines",
        Smooth      => "smooth" | "curved" | "bezier" | "spline",
        Categorical => "categorical" | "category" | "groups" | "colored",
        Highlight   => "highlight" | "spotlight" | "focus" | "dim",
        Density     => "density" | "fade" | "translucent" | "alpha",
        Gradient    => "gradient" | "value" | "ramp" | "shaded",
        Arc         => "arc" | "bezier_color" | "smooth_color" | "colored_bezier",
        Ribbon      => "ribbon" | "flow" | "band" | "filled_bezier",
    }
}
