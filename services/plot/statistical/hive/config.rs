crate::chart_config!(HiveConfig, 620, 580;
    struct {
        pub variant:    super::variant::HiveVariant,
        pub axes:       &'a [String],
        pub labels:     &'a [String],
        pub categories: &'a [String],
        pub values:     &'a [f64],
        pub sources:    &'a [i32],
        pub targets:    &'a [i32],
        pub weights:    &'a [f64],
        pub palette:    &'a [u32],
        pub inner_r:    f64,
        pub outer_r:    f64,
    }
    defaults {
        variant:    super::variant::HiveVariant::Basic,
        axes:       &[],
        labels:     &[],
        categories: &[],
        values:     &[],
        sources:    &[],
        targets:    &[],
        weights:    &[],
        palette:    &[],
        inner_r:    40.0,
        outer_r:    220.0,
    }
);
