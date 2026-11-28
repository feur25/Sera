pub struct VectorFieldMapConfig<'a> {
    pub variant: super::variant::VectorFieldMapVariant,
    pub title: &'a str,
    pub lats: &'a [f64],
    pub lons: &'a [f64],
    pub u: &'a [f64],
    pub v: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub color_low: u32,
    pub color_high: u32,
}
