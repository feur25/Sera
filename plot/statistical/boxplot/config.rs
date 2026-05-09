crate::chart_config!(BoxplotConfig, 900, 500;
    struct {
        pub variant: super::variant::BoxplotVariant,
        pub category_labels: &'a [String],
        pub values: &'a [f64],
        pub series: &'a [Vec<f64>],
        pub series_names: &'a [String],
        pub palette: &'a [u32],
        pub notch: bool,
        pub show_points: bool,
        pub jitter: f64,
        pub violin_overlay: bool,
        pub boxen_depth: usize,
        pub fill_opacity: f64,
        pub stroke_width: f64,
    }
    defaults {
        variant: super::variant::BoxplotVariant::Basic,
        category_labels: &[],
        values: &[],
        series: &[],
        series_names: &[],
        palette: &[],
        notch: false,
        show_points: false,
        jitter: 0.35,
        violin_overlay: false,
        boxen_depth: 4,
        fill_opacity: 0.28,
        stroke_width: 1.6,
    }
);


