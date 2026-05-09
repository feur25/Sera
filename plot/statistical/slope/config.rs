crate::chart_config!(SlopeConfig, 700, 500;
    struct {
        pub variant: super::variant::SlopeVariant,
        pub labels: &'a [String],
        pub values_left: &'a [f64],
        pub values_right: &'a [f64],
        pub left_label: &'a str,
        pub right_label: &'a str,
        pub palette: &'a [u32],
        pub show_text: bool,
    }
    defaults {
        variant: super::variant::SlopeVariant::Basic,
        labels: &[],
        values_left: &[],
        values_right: &[],
        left_label: "Before",
        right_label: "After",
        palette: &[],
        show_text: true,
    }
);


