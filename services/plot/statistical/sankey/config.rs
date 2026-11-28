crate::chart_config!(SankeyConfig, 900, 520;
    struct {
        pub variant: super::variant::SankeyVariant,
        pub labels:  &'a [String],
        pub sources: &'a [i32],
        pub targets: &'a [i32],
        pub weights: &'a [f64],
        pub palette: &'a [u32],
        pub node_width: i32,
        pub node_gap:   i32,
    }
    defaults {
        variant:    super::variant::SankeyVariant::Basic,
        labels:     &[],
        sources:    &[],
        targets:    &[],
        weights:    &[],
        palette:    &[],
        node_width: 18,
        node_gap:   10,
    }
);