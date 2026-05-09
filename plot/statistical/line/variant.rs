crate::plot_family! {
    pub enum LineVariant default Basic {
        Basic            => "basic",
        Multi            => "multi" | "multiline" | "multiple",
        Stepped          => "stepped" | "step" | "hv" | "vh" | "hvh" | "vhv",
        Spline           => "spline" | "smooth" | "curved",
        Filled           => "filled" | "area" | "fill",
        Sparkline        => "sparkline" | "spark" | "tiny",
        Dashed           => "dashed" | "dotted" | "styled",
        ConnectedScatter => "connected_scatter" | "markers" | "lines+markers",
        Gapped           => "gapped" | "gaps" | "missing",
        Neon             => "neon" | "glow" | "electric" | "cyber",
    }
}


