crate::chart_config!(JointConfig, 760, 760;
    struct {
        pub variant: super::variant::JointVariant,
        pub marginal: super::variant::JointMarginal,
        pub panel_variant: &'a str,
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub bins: usize,
        pub colorscale: &'a str,
        pub point_hex: u32,
        pub palette: &'a [u32],
        pub show_points: bool,
        pub show_regression: bool,
        pub group_series: &'a [(String, Vec<f64>, Vec<f64>)],
    }
    defaults {
        variant: super::variant::JointVariant::Hexbin,
        marginal: super::variant::JointMarginal::Histogram,
        panel_variant: "basic",
        x_values: &[],
        y_values: &[],
        bins: 24,
        colorscale: "viridis",
        point_hex: 0x6366f1,
        palette: &[],
        show_points: false,
        show_regression: false,
        group_series: &[],
    }
);
