crate::chart_config!(TreemapConfig, 1100, 520;
    struct {
        pub variant: super::variant::TreemapVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub parents: &'a [String],
        pub palette: &'a [u32],
        pub show_text: bool,
    }
    defaults {
        variant: super::variant::TreemapVariant::Basic,
        labels: &[],
        values: &[],
        parents: &[],
        palette: &[],
        show_text: false,
    }
);
