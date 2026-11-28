crate::chart_config!(EventplotConfig, 900, 500;
    struct {
        pub variant: super::variant::EventplotVariant,
        pub x_values: &'a [f64],
        pub categories: &'a [String],
        pub palette: &'a [u32],
        pub colorscale: &'a str,
    }
    defaults {
        variant: super::variant::EventplotVariant::Basic,
        x_values: &[],
        categories: &[],
        palette: &[],
        colorscale: "",
    }
);
