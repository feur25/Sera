crate::chart_config!(CirclePackConfig, 700, 700;
    struct {
        pub variant:    super::variant::CirclePackVariant,
        pub labels:     &'a [String],
        pub parents:    &'a [String],
        pub values:     &'a [f64],
        pub palette:    &'a [u32],
        pub show_labels: bool,
        pub padding:    f64,
    }
    defaults {
        variant:     super::variant::CirclePackVariant::Basic,
        labels:      &[],
        parents:     &[],
        values:      &[],
        palette:     &[],
        show_labels: true,
        padding:     2.0,
    }
);
