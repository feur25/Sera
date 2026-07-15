crate::chart_config!(CorrelogramConfig, 560, 520;
    struct {
        pub variant:     super::variant::CorrelogramVariant,
        pub labels:      &'a [String],
        pub matrix:      &'a [f64],
        pub palette:     &'a [u32],
        pub show_values: bool,
    }
    defaults {
        variant:     super::variant::CorrelogramVariant::Circle,
        labels:      &[],
        matrix:      &[],
        palette:     &[],
        show_values: false,
    }
);
