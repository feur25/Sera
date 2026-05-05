crate::chart_config!(FunnelConfig, 800, 480;
    struct {
        pub variant: super::variant::FunnelVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
        pub show_text: bool,
    }
    defaults {
        variant: super::variant::FunnelVariant::Basic,
        labels: &[],
        values: &[],
        palette: &[],
        show_text: true,
    }
);
