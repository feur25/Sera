crate::chart_config!(AreaConfig, 1100, 480;
    struct {
        pub variant: super::variant::AreaVariant,
        pub x_labels: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::AreaVariant::Basic,
        x_labels: &[],
        series: &[],
        palette: &[],
    }
);
