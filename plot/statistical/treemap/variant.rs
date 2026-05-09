crate::plot_family! {
    pub enum TreemapVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "rounded",
        Flat     => "flat" | "mosaic" | "edge" | "tiled",
        Gradient => "gradient" | "shaded" | "vgrad" | "fade",
        Outlined => "outlined" | "outline" | "stroke" | "wireframe",
        Gapped   => "gapped" | "spaced" | "isolated" | "padded",
        Nested   => "nested" | "grouped" | "framed" | "parent",
        Heat     => "heat" | "ramp" | "magnitude" | "intensity",
        Mono     => "mono" | "monochrome" | "single" | "uniform",
    }
}


