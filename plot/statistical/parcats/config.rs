crate::chart_config!(ParcatsConfig, 900, 520;
    struct {
        pub variant: super::variant::ParcatsVariant,
        pub axes: &'a [String],
        pub category_series: &'a [Vec<String>],
        pub palette: &'a [u32],
        pub node_width: i32,
        pub node_gap: i32,
    }
    defaults {
        variant: super::variant::ParcatsVariant::Basic,
        axes: &[],
        category_series: &[],
        palette: &[],
        node_width: 90,
        node_gap: 4,
    }
);
