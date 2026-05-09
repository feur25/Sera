crate::plot_family! {
    pub enum ViolinVariant default Box {
        Basic      => "basic" | "simple" | "kde" | "density",
        Box        => "box" | "with_box" | "default" | "vertical",
        Quartile   => "quartile" | "quartiles" | "lines" | "iqr",
        Mean       => "mean" | "meanline" | "average" | "with_mean",
        Points     => "points" | "with_points" | "raw" | "all_points",
        Strip      => "strip" | "swarm" | "jitter" | "stripplot",
        Horizontal => "horizontal" | "horiz" | "h" | "rotated",
        Split      => "split" | "paired" | "back_to_back" | "mirror",
        Half       => "half" | "asymmetric" | "one_sided" | "single",
        Rainbow    => "rainbow" | "gradient" | "spectrum" | "colorful",
        Aurora     => "aurora" | "warm" | "ember" | "fire",
        Deluxe     => "deluxe" | "premium" | "neon" | "dark",
        Crystal    => "crystal" | "glass" | "prism" | "transparent",
    }
}


