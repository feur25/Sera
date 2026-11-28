crate::plot_family! {
    pub enum GraticuleMapVariant default Lines family "graticule_map" kind "map" {
        Lines  => "lines" | "grid" | "basic" | "default" | "meridians",
        Globe  => "globe" | "orthographic" | "sphere" | "space",
        Tissot => "tissot" | "indicatrix" | "distortion",
    }
}
