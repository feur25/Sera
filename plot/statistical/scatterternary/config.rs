crate::chart_config!(ScatterTernaryConfig, 700, 640;
    struct {
        pub variant: super::variant::ScatterTernaryVariant,
        pub a_values: &'a [f64],
        pub b_values: &'a [f64],
        pub c_values: &'a [f64],
        pub labels: &'a [String],
        pub color_values: &'a [f64],
        pub palette: &'a [u32],
        pub colorscale: &'a str,
        pub point_size: f64,
        pub a_label: &'a str,
        pub b_label: &'a str,
        pub c_label: &'a str,
    }
    defaults {
        variant: super::variant::ScatterTernaryVariant::Basic,
        a_values: &[],
        b_values: &[],
        c_values: &[],
        labels: &[],
        color_values: &[],
        palette: &[],
        colorscale: "",
        point_size: 4.0,
        a_label: "A",
        b_label: "B",
        c_label: "C",
    }
);
