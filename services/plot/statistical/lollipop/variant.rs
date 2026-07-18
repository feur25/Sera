crate::plot_family! {
    pub enum LollipopVariant default Basic {
        Basic     => "basic" | "default" | "classic" | "vertical",
        Cleveland => "cleveland" | "horizontal" | "h" | "row",
        Diverging => "diverging" | "div" | "signed" | "delta",
        Circular  => "circular" | "polar" | "radial" | "round",
        Highlight => "highlight" | "spotlight" | "focus" | "dim",
        Office    => "office" | "grouped" | "season" | "panel",
    }
}
