crate::plot_family! {
    pub enum SlopeVariant default Basic family "slope" {
        Basic       => "basic" | "default" | "direction" | "classic",
        Monochrome  => "monochrome" | "mono" | "uniform" | "single_color",
        Highlighted => "highlighted" | "highlight" | "top" | "movers",
        Bumps       => "bumps" | "bumpchart" | "rank" | "ranking",
        Curved      => "curved" | "curve" | "bezier" | "smooth",
        Thick       => "thick" | "magnitude" | "weighted" | "weight",
        Diverging   => "diverging" | "delta" | "change" | "centered",
        Stepped     => "stepped" | "step" | "elbow" | "rectilinear",
    }
}
