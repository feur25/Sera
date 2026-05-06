crate::chart_config!(WordCloudConfig, 900, 500;
    struct {
        pub variant: super::variant::WordCloudVariant,
        pub words: &'a [String],
        pub frequencies: &'a [f64],
        pub palette: &'a [u32],
        pub min_font: f64,
        pub max_font: f64,
        pub bg_color: Option<&'a str>,
    }
    defaults {
        variant: super::variant::WordCloudVariant::Basic,
        words: &[],
        frequencies: &[],
        palette: &[],
        min_font: 12.0,
        max_font: 72.0,
        bg_color: None,
    }
);
