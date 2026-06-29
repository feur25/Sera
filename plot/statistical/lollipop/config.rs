crate::chart_config!(LollipopConfig, 900, 480;
    struct {
        pub variant: super::variant::LollipopVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub groups: &'a [String],
        pub palette: &'a [u32],
        pub color_hex: u32,
        pub show_values: bool,
        pub highlight_index: i32,
    }
    defaults {
        variant: super::variant::LollipopVariant::Basic,
        labels: &[],
        values: &[],
        groups: &[],
        palette: &[],
        color_hex: 0x636EFA,
        show_values: false,
        highlight_index: -1,
    }
);
