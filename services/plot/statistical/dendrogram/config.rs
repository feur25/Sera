crate::chart_config!(DendrogramConfig, 820, 480;
    struct {
        pub variant:     super::variant::DendrogramVariant,
        pub labels:      &'a [String],
        pub parents:     &'a [String],
        pub values:      &'a [Vec<f64>],
        pub clusters:    usize,
        pub palette:     &'a [u32],
        pub show_labels: bool,
        pub line_width:  f64,
    }
    defaults {
        variant:     super::variant::DendrogramVariant::Vertical,
        labels:      &[],
        parents:     &[],
        values:      &[],
        clusters:    3,
        palette:     &[],
        show_labels: true,
        line_width:  1.5,
    }
);
