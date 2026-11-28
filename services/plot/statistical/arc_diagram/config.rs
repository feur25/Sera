crate::chart_config!(ArcDiagramConfig, 900, 400;
    struct {
        pub variant: super::variant::ArcDiagramVariant,
        pub labels:  &'a [String],
        pub sources: &'a [i32],
        pub targets: &'a [i32],
        pub weights: &'a [f64],
        pub palette: &'a [u32],
        pub node_r:  f64,
    }
    defaults {
        variant: super::variant::ArcDiagramVariant::Basic,
        labels:  &[],
        sources: &[],
        targets: &[],
        weights: &[],
        palette: &[],
        node_r:  7.0,
    }
);
