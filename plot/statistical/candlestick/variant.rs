crate::plot_family! {
    pub enum CandlestickVariant default Basic {
        Basic    => "basic" | "default" | "classic" | "filled",
        Hollow   => "hollow" | "empty" | "japanese" | "white_up",
        Ohlc     => "ohlc" | "western" | "bar" | "tick",
        Heikin   => "heikin" | "heikin_ashi" | "ha" | "smoothed",
        Outlined => "outlined" | "outline" | "stroke" | "wireframe",
        Line     => "line" | "close" | "lineplot" | "trend",
        Mountain => "mountain" | "area" | "filled_area" | "shade",
        Range    => "range" | "hl" | "highlow" | "spread",
    }
}
