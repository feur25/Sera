crate::plot_family! {
    pub enum RadarVariant default Basic family "radar" {
        Basic    => "basic" | "default" | "classic",
        Lines    => "lines" | "outline" | "stroke" | "no_fill",
        Filled   => "filled" | "fill" | "solid" | "area",
        Markers  => "markers" | "dots" | "points" | "marker",
        Dashed   => "dashed" | "dash" | "dotted",
        Stacked  => "stacked" | "stack" | "cumulative",
        PolarBar => "polar_bar" | "polar" | "bar" | "radial_bar",
    }
}
