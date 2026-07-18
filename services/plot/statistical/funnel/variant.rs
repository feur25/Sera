crate::plot_family! {
    pub enum FunnelVariant default Basic {
        Basic      => "basic" | "default" | "trapezoid" | "classic",
        Stepped    => "stepped" | "bar" | "rect" | "rectangle",
        Rounded    => "rounded" | "round" | "pill" | "smooth",
        Chevron    => "chevron" | "arrow" | "pipeline" | "pointer",
        Pyramid    => "pyramid" | "triangle" | "cone" | "point",
        Inverted   => "inverted" | "inverse" | "reverse" | "upside_down",
        Conversion => "conversion" | "dropoff" | "rate" | "steps",
    }
}
