crate::chart_config!(WaterfallConfig, 900, 480;
    struct {
        pub variant: super::variant::WaterfallVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub show_text: bool,
        pub orientation: u8,
    }
    defaults {
        variant: super::variant::WaterfallVariant::Basic,
        labels: &[],
        values: &[],
        show_text: true,
        orientation: b'v',
    }
);


