crate::plot_family! {
    pub enum SunburstVariant default Basic {
        Basic     => "basic" | "default" | "classic" | "ring",
        Donut     => "donut" | "hole" | "ring_hole" | "donut_ring",
        Outlined  => "outlined" | "outline" | "stroke" | "wireframe",
        Gapped    => "gapped" | "spaced" | "isolated" | "petals",
        DepthFade => "depth_fade" | "fade" | "fading" | "depth",
        Mono      => "mono" | "monochrome" | "single" | "uniform",
        Zoomable  => "zoomable" | "zoom" | "animated" | "interactive" | "drill",
    }
}
