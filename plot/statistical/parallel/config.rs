crate::chart_config!(ParallelConfig, 1000, 500;
    struct {
        pub variant: super::variant::ParallelVariant,
        pub theme: crate::plot::statistical::theme::ChartTheme,
        pub axes: &'a [String],
        pub series_names: &'a [String],
        pub series_values: &'a [Vec<f64>],
        pub categories: &'a [i32],
        pub palette: &'a [u32],
        pub highlight_index: i32,
        pub color_axis: i32,
    }
    defaults {
        variant: super::variant::ParallelVariant::Basic,
        theme: crate::plot::statistical::theme::ChartTheme::None,
        axes: &[],
        series_names: &[],
        series_values: &[],
        categories: &[],
        palette: &[],
        highlight_index: -1,
        color_axis: -1,
    }
);


