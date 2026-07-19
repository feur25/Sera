crate::plot_family! {
    pub enum BoxplotVariant default Basic family "boxplot" {
        Basic       => "basic" | "simple" | "vertical",
        Horizontal  => "horizontal" | "hbox" | "horiz" | "h",
        Notched     => "notched" | "notch" | "ci" | "confidence",
        Grouped     => "grouped" | "group" | "side_by_side" | "multi",
        Points      => "points" | "all_points" | "scatter" | "raw",
        Outliers    => "outliers" | "outlier" | "fliers" | "anomalies",
        Strip       => "strip" | "swarm" | "jitter" | "stripplot",
        Violin      => "violin" | "kde_overlay" | "density",
        LetterValue => "letter_value" | "boxen" | "lv" | "tukey",
    }
}
