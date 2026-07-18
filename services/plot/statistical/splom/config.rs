crate::chart_config!(SplomConfig, 900, 900;
    struct {
        pub variant: super::variant::SplomVariant,
        pub axes: &'a [String],
        pub series_values: &'a [Vec<f64>],
        pub palette: &'a [u32],
        pub colorscale: &'a str,
        pub point_size: f64,
    }
    defaults {
        variant: super::variant::SplomVariant::Basic,
        axes: &[],
        series_values: &[],
        palette: &[],
        colorscale: "",
        point_size: 2.2,
    }
);
