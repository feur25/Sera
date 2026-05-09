crate::chart_config!(GaugeConfig, 400, 300;
    struct {
        pub variant: super::variant::GaugeVariant,
        pub value: f64,
        pub min_val: f64,
        pub max_val: f64,
        pub label: &'a str,
        pub thresholds: &'a [(f64, u32)],
        pub comparison: f64,
    }
    defaults {
        variant: super::variant::GaugeVariant::Basic,
        value: 0.0,
        min_val: 0.0,
        max_val: 100.0,
        label: "",
        thresholds: &[],
        comparison: 0.0,
    }
);


