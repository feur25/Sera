crate::chart_config!(DendrogramConfig, 820, 480;
    struct {
        pub variant:     super::variant::DendrogramVariant,
        pub labels:      &'a [String],
        pub parents:     &'a [String],
        pub palette:     &'a [u32],
        pub show_labels: bool,
        pub line_width:  f64,
    }
    defaults {
        variant:     super::variant::DendrogramVariant::Vertical,
        labels:      &[],
        parents:     &[],
        palette:     &[],
        show_labels: true,
        line_width:  1.5,
    }
);
