crate::plot_family! {
    pub enum VectorFieldMapVariant default Arrows family "vector_field_map" kind "map" {
        Arrows      => "arrows" | "quiver" | "wind" | "basic" | "default",
        Streamlines => "streamlines" | "streamline" | "flow" | "particles",
    }
}
