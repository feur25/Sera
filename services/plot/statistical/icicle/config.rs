crate::chart_config!(IcicleConfig, 760, 520;
    struct {
        pub variant: super::variant::IcicleVariant,
        pub labels: &'a [String],
        pub parents: &'a [String],
        pub values: &'a [f64],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::IcicleVariant::Basic,
        labels: &[],
        parents: &[],
        values: &[],
        palette: &[],
    }
);
