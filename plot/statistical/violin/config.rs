crate::chart_config!(ViolinConfig, 900, 500;
    struct {
        pub variant: super::variant::ViolinVariant,
        pub categories: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
        pub bandwidth: f64,
        pub fill_opacity: f64,
        pub stroke_width: f64,
        pub show_box: bool,
        pub show_points: bool,
        pub show_mean: bool,
        pub jitter: f64,
        pub kde_steps: usize,
    }
    defaults {
        variant: super::variant::ViolinVariant::Box,
        categories: &[],
        values: &[],
        palette: &[],
        bandwidth: 1.0,
        fill_opacity: 0.55,
        stroke_width: 1.4,
        show_box: false,
        show_points: false,
        show_mean: false,
        jitter: 0.35,
        kde_steps: 32,
    }
);


