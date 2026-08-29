crate::plot_family! {
    pub enum ChoroplethVariant default Sequential family "choropleth" kind "map" {
        Sequential   => "sequential" | "basic" | "default" | "heat",
        Binned       => "binned" | "quantile" | "classed" | "steps",
        Diverging    => "diverging" | "delta" | "change" | "rdbu",
        Orthographic => "orthographic" | "globe" | "sphere" | "space",
        Polar        => "polar" | "azimuthal" | "pole",
    }
}
