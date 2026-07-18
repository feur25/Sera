crate::chart_config!(RidgelineConfig, 900, 520;
    struct {
        pub variant: super::variant::RidgelineVariant,
        pub values: &'a [f64],
        pub categories: &'a [String],
        pub palette: &'a [u32],
        pub overlap: f64,
        pub bandwidth: f64,
        pub n_points: usize,
        pub fill_opacity: u8,
    }
    defaults {
        variant: super::variant::RidgelineVariant::Basic,
        values: &[],
        categories: &[],
        palette: &[],
        overlap: 0.5,
        bandwidth: 0.0,
        n_points: 60,
        fill_opacity: 56,
    }
);
