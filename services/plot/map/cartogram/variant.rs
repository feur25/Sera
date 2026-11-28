crate::plot_family! {
    pub enum CartogramVariant default Dorling family "cartogram" kind "map" {
        Dorling => "dorling" | "circles" | "basic" | "default" | "bubble_relax",
        Demers  => "demers" | "squares" | "grid_relax" | "boxes",
    }
}
