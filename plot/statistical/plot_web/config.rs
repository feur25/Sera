crate::chart_config!(PlotWebConfig, 1440, 580;
    struct {
        pub variant:    super::variant::PlotWebVariant,
        pub x_values:   &'a [f64],
        pub y_values:   &'a [f64],
        pub sizes:      &'a [f64],
        pub labels:     &'a [String],
        pub groups:     &'a [String],
        pub palette:    &'a [u32],
        pub size_label: &'a str,
        pub x_log:      bool,
        pub min_r:      f64,
        pub max_r:      f64,
    }
    defaults {
        variant:    super::variant::PlotWebVariant::Scatter,
        x_values:   &[],
        y_values:   &[],
        sizes:      &[],
        labels:     &[],
        groups:     &[],
        palette:    &[],
        size_label: "",
        x_log:      false,
        min_r:      6.0,
        max_r:      38.0,
    }
);
