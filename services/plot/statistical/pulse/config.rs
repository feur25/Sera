crate::chart_config!(PulseConfig, 560, 560;
    struct {
        pub variant:     super::variant::PulseVariant,
        pub labels:      &'a [String],
        pub values:      &'a [f64],
        pub palette:     &'a [u32],
        pub inner_r:     f64,
        pub show_labels: bool,
    }
    defaults {
        variant:     super::variant::PulseVariant::Radial,
        labels:      &[],
        values:      &[],
        palette:     &[],
        inner_r:     60.0,
        show_labels: true,
    }
);
