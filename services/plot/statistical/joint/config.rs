crate::chart_config!(JointConfig, 760, 760;
    struct {
        pub variant: super::variant::JointVariant,
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub bins: usize,
        pub colorscale: &'a str,
        pub point_hex: u32,
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::JointVariant::HexbinMarginal,
        x_values: &[],
        y_values: &[],
        bins: 24,
        colorscale: "viridis",
        point_hex: 0x6366f1,
        palette: &[],
    }
);
