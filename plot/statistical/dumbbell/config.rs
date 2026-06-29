crate::chart_config!(DumbbellConfig, 1000, 500;
    struct {
        pub variant: super::variant::DumbbellVariant,
        pub labels: &'a [String],
        pub values_start: &'a [f64],
        pub values_end: &'a [f64],
        pub series_names: (&'a str, &'a str),
        pub palette: &'a [u32],
        pub show_text: bool,
    }
    defaults {
        variant: super::variant::DumbbellVariant::Basic,
        labels: &[],
        values_start: &[],
        values_end: &[],
        series_names: ("Start", "End"),
        palette: &[],
        show_text: false,
    }
);
