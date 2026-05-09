crate::chart_config!(RadarConfig, 700, 560;
    struct {
        pub variant: super::variant::RadarVariant,
        pub axes: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
        pub filled: bool,
        pub fill_opacity: u8,
    }
    defaults {
        variant: super::variant::RadarVariant::Basic,
        axes: &[],
        series: &[],
        palette: &[],
        filled: true,
        fill_opacity: 50,
    }
);


