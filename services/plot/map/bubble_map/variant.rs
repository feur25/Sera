crate::plot_family! {
    pub enum BubbleMapVariant default Filled family "bubble_map" kind "map" {
        Filled       => "filled" | "basic" | "default" | "regions",
        Proportional => "proportional" | "bubble" | "graduated" | "sized",
    }
}
