crate::chart_config!(CandlestickConfig, 1100, 500;
    struct {
        pub variant: super::variant::CandlestickVariant,
        pub labels: &'a [String],
        pub open: &'a [f64],
        pub high: &'a [f64],
        pub low: &'a [f64],
        pub close: &'a [f64],
        pub palette: &'a [u32],
    }
    defaults {
        variant: super::variant::CandlestickVariant::Basic,
        labels: &[],
        open: &[],
        high: &[],
        low: &[],
        close: &[],
        palette: &[],
    }
);


