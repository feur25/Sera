crate::chart_config!(BulletConfig, 800, 300;
    struct {
        pub variant: super::variant::BulletVariant,
        pub labels: &'a [String],
        pub values: &'a [f64],
        pub targets: &'a [f64],
        pub max_vals: &'a [f64],
        pub ranges: &'a [f64],
        pub comparisons: &'a [f64],
    }
    defaults {
        variant: super::variant::BulletVariant::Basic,
        labels: &[],
        values: &[],
        targets: &[],
        max_vals: &[],
        ranges: &[],
        comparisons: &[],
    }
);
