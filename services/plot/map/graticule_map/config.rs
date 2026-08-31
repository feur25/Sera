pub struct GraticuleMapConfig<'a> {
    pub variant: super::variant::GraticuleMapVariant,
    pub title: &'a str,
    pub width: i32,
    pub height: i32,
    pub step: f64,
    pub center_lat: Option<f64>,
    pub center_lon: Option<f64>,
    pub color_low: u32,
    pub color_high: u32,
}

impl<'a> GraticuleMapConfig<'a> {
    pub fn new() -> Self {
        Self {
            variant: super::variant::GraticuleMapVariant::Lines,
            title: "",
            width: 1200,
            height: 650,
            step: 15.0,
            center_lat: None,
            center_lon: None,
            color_low: 0x38bdf8,
            color_high: 0xf97316,
        }
    }
}
