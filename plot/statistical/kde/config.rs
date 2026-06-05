crate::chart_config!(KdeConfig, 900, 420;
    struct {
        pub variant: super::variant::KdeVariant,
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub bandwidth: f64,
        pub filled: bool,
        pub fill_opacity: u8,
        pub n_points: usize,
        pub bins: usize,
    }
    defaults {
        variant: super::variant::KdeVariant::Basic,
        series: &[],
        palette: &[],
        bandwidth: 0.0,
        filled: true,
        fill_opacity: 50,
        n_points: 80,
        bins: 30,
    }
);
