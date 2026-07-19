crate::plot_family! {
    pub enum DumbbellVariant default Basic family "dumbbell" {
        Basic    => "basic" | "default" | "classic" | "dot",
        Arrow    => "arrow" | "directional" | "delta_arrow" | "flow",
        Delta    => "delta" | "change" | "diff" | "signed",
        Barbell  => "barbell" | "thick" | "weighted" | "editorial",
        Glow     => "glow" | "halo" | "neon" | "soft",
        Dotted   => "dotted" | "dashed" | "minimal" | "thin",
        Ranked   => "ranked" | "ranking" | "ordered" | "numbered",
    }
}
