crate::chart_config!(ScatterConfig, 900, 500;
    struct {
        pub variant: super::variant::ScatterVariant,
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub categories: &'a [String],
        pub labels: &'a [String],
        pub color_values: &'a [f64],
        pub symbols: &'a [String],
        pub palette: &'a [u32],
        pub color_hex: u32,
        pub color_low: u32,
        pub color_high: u32,
        pub point_size: f64,
        pub stroke_width: f64,
        pub show_text: bool,
        pub symbol: &'a str,
        pub regression_type: &'a str,
    }
    defaults {
        variant: super::variant::ScatterVariant::Basic,
        x_values: &[],
        y_values: &[],
        categories: &[],
        labels: &[],
        color_values: &[],
        symbols: &[],
        palette: &[],
        color_hex: 0x6366F1,
        color_low: 0x6366F1,
        color_high: 0xF43F5E,
        point_size: 5.0,
        stroke_width: 1.0,
        show_text: false,
        symbol: "circle",
        regression_type: "linear",
    }
);


