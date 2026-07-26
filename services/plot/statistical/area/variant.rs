crate::plot_family! {
    pub enum AreaVariant default Basic family "area" {
        Basic    => "basic" | "overlay" | "default" | "simple",
        Stacked  => "stacked" | "stack",
        Percent  => "percent" | "percent_stacked" | "normalized" | "stream100",
        Spline   => "spline" | "smooth" | "curved",
        Step     => "step" | "stepped" | "stairs",
        Gradient => "gradient" | "glow" | "fade",
        Ribbon   => "ribbon" | "outlined" | "bordered" | "ggplot",
        Wave     => "wave" | "signed" | "oscillating" | "stackplot",
    }
}
