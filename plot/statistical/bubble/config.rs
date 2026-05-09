crate::chart_config!(BubbleConfig, 900, 500;
    struct {
        pub variant: super::variant::BubbleVariant,
        pub theme: crate::plot::statistical::theme::ChartTheme,
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub sizes: &'a [f64],
        pub categories: &'a [String],
        pub labels: &'a [String],
        pub color_values: &'a [f64],
        pub palette: &'a [u32],
        pub color_hex: u32,
        pub color_low: u32,
        pub color_high: u32,
        pub min_size: f64,
        pub max_size: f64,
        pub show_text: bool,
        pub stroke_width: f64,
    }
    defaults {
        variant: super::variant::BubbleVariant::Basic,
        theme: crate::plot::statistical::theme::ChartTheme::None,
        x_values: &[],
        y_values: &[],
        sizes: &[],
        categories: &[],
        labels: &[],
        color_values: &[],
        palette: &[],
        color_hex: 0x6366F1,
        color_low: 0x6366F1,
        color_high: 0xF43F5E,
        min_size: 4.0,
        max_size: 40.0,
        show_text: false,
        stroke_width: 1.5,
    }
);


