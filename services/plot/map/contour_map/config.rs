pub struct ContourMapConfig<'a> {
    pub variant: super::variant::ContourMapVariant,
    pub title: &'a str,
    pub lats: &'a [f64],
    pub lons: &'a [f64],
    pub field: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub levels: usize,
    pub color_low: u32,
    pub color_high: u32,
}
