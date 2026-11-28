crate::chart_config!(VennConfig, 560, 420;
    struct {
        pub variant:     super::variant::VennVariant,
        pub labels:      &'a [String],
        pub values:      &'a [f64],
        pub palette:     &'a [u32],
        pub show_labels: bool,
        pub opacity:     f64,
    }
    defaults {
        variant:     super::variant::VennVariant::Basic,
        labels:      &[],
        values:      &[],
        palette:     &[],
        show_labels: true,
        opacity:     0.38,
    }
);
