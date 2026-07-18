crate::chart_config!(ChordConfig, 700, 700;
    struct {
        pub variant:    super::variant::ChordVariant,
        pub labels:     &'a [String],
        pub matrix:     &'a [Vec<f64>],
        pub palette:    &'a [u32],
        pub gap_deg:    f64,
        pub arc_width:  f64,
        pub show_labels: bool,
    }
    defaults {
        variant:    super::variant::ChordVariant::Basic,
        labels:     &[],
        matrix:     &[],
        palette:    &[],
        gap_deg:    2.0,
        arc_width:  22.0,
        show_labels: true,
    }
);
