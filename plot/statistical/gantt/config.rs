crate::chart_config!(GanttConfig, 1000, 520;
    struct {
        pub variant: super::variant::GanttVariant,
        pub labels: &'a [String],
        pub values_start: &'a [f64],
        pub values_end: &'a [f64],
        pub categories: &'a [String],
        pub progress: &'a [f64],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::GanttVariant::Basic,
        labels: &[],
        values_start: &[],
        values_end: &[],
        categories: &[],
        progress: &[],
        palette: &[],
    }
);
