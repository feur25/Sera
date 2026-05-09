crate::plot_family! {
    pub enum BulletVariant default Basic {
        Basic     => "basic" | "default" | "classic" | "standard",
        Stacked   => "stacked" | "stacked_ranges" | "zones" | "qualitative",
        Thermo    => "thermo" | "thermometer" | "vertical" | "column",
        Segmented => "segmented" | "traffic" | "rag" | "zones_color",
        Minimal   => "minimal" | "sparkline" | "clean" | "naked",
        Dot       => "dot" | "point" | "marker" | "pip",
        Progress  => "progress" | "pill" | "bar" | "percent",
        Compare   => "compare" | "vs" | "ghost" | "prior",
    }
}


