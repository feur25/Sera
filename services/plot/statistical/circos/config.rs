crate::chart_config!(CircosConfig, 960, 960;
    struct {
        pub variant:         super::variant::CircosVariant,
        pub item_labels:     &'a [String],
        pub item_groups:     &'a [String],
        pub bar_series:      &'a [(String, Vec<f64>)],
        pub heat_categories: &'a [String],
        pub heat_matrix:     &'a [Vec<f64>],
        pub link_sources:    &'a [i32],
        pub link_targets:    &'a [i32],
        pub palette:         &'a [u32],
    }
    defaults {
        variant:         super::variant::CircosVariant::Basic,
        item_labels:     &[],
        item_groups:     &[],
        bar_series:      &[],
        heat_categories: &[],
        heat_matrix:     &[],
        link_sources:    &[],
        link_targets:    &[],
        palette:         &[],
    }
);
