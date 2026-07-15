crate::chart_config!(HexbinConfig, 900, 520;
    struct {
        pub variant: super::variant::HexbinVariant,
        pub x_values: &'a [f64],
        pub y_values: &'a [f64],
        pub gridsize: usize,
        pub colorscale: &'a str,
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::HexbinVariant::Basic,
        x_values: &[],
        y_values: &[],
        gridsize: 20,
        colorscale: "",
        palette: &[],
    }
);
