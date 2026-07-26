crate::chart_config!(FunnelConfig, 800, 480;
    struct {
        pub variant: super::variant::FunnelVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
        pub show_text: bool,
        pub series: &'a [(String, Vec<f64>)],
        pub stage_labels: &'a [Vec<String>],
        pub text_info: &'a str,
    }
    defaults {
        variant: super::variant::FunnelVariant::Basic,
        labels: &[],
        values: &[],
        palette: &[],
        show_text: true,
        series: &[],
        stage_labels: &[],
        text_info: "value+percent_initial",
    }
);
