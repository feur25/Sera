crate::plot_family! {
    pub enum ContourMapVariant default Filled family "contour_map" kind "map" {
        Filled   => "filled" | "bands" | "heat" | "basic" | "default",
        Isolines => "isolines" | "lines" | "contour_lines" | "iso",
    }
}
