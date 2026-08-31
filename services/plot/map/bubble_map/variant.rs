crate::plot_family! {
    pub enum BubbleMapVariant default Proportional family "bubble_map" kind "map" {
        Proportional => "proportional" | "bubble" | "graduated" | "sized" | "basic" | "default",
        Filled       => "filled" | "regions",
        Globe        => "globe" | "orthographic" | "sphere" | "space",
        Ring         => "ring" | "donut" | "outline" | "hollow",
        Pulse        => "pulse" | "ripple" | "radar" | "ping",
        Hexbin       => "hexbin" | "hex" | "density" | "honeycomb",
    }
}
