crate::plot_family! {
    pub enum DendrogramVariant default Vertical {
        Vertical   => "vertical" | "top" | "default" | "classic",
        Horizontal => "horizontal" | "left" | "h",
        Radial     => "radial" | "circular" | "polar",
        Compact    => "compact" | "dense" | "tight",
        Elegant    => "elegant" | "smooth" | "rounded",
        Triangular => "triangular" | "diagonal" | "straight" | "angular",
    }
}
