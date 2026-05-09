crate::chart_config!(SunburstConfig, 700, 700;
    struct {
        pub variant: super::variant::SunburstVariant,
        pub labels: &'a [String],
        pub parents: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::SunburstVariant::Basic,
        labels: &[],
        parents: &[],
        values: &[],
        palette: &[],
    }
);


