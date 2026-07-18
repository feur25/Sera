crate::chart_config!(StackplotConfig, 1100, 480;
    struct {
        pub variant: super::variant::StackplotVariant,
        pub x_labels: &'a [String],
        pub series: &'a [(String, Vec<f64>)],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::StackplotVariant::Basic,
        x_labels: &[],
        series: &[],
        palette: &[],
    }
);
