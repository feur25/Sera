crate::chart_config!(OrbitaConfig, 580, 580;
    struct {
        pub variant:      super::variant::OrbitaVariant,
        pub series_names: &'a [String],
        pub labels:       &'a [String],
        pub matrix:       &'a [f64],
        pub palette:      &'a [u32],
        pub inner_r:      f64,
        pub orbit_gap:    f64,
        pub show_labels:  bool,
    }
    defaults {
        variant:      super::variant::OrbitaVariant::Classic,
        series_names: &[],
        labels:       &[],
        matrix:       &[],
        palette:      &[],
        inner_r:      40.0,
        orbit_gap:    36.0,
        show_labels:  true,
    }
);
