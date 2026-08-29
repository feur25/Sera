use crate::plot::map::regions::RegionSetEntry;

pub struct ChoroplethConfig<'a> {
    pub variant: super::variant::ChoroplethVariant,
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub hover: &'a [crate::html::hover::HoverSlot],
    pub region: &'static RegionSetEntry,
    pub group: &'a str,
    pub bins: usize,
    pub diverging_midpoint: f64,
    pub center_lat: Option<f64>,
    pub center_lon: Option<f64>,
}

impl<'a> ChoroplethConfig<'a> {
    pub fn new(region: &'static RegionSetEntry) -> Self {
        Self {
            variant: super::variant::ChoroplethVariant::Sequential,
            title: "",
            labels: &[],
            values: &[],
            width: 1200,
            height: 600,
            hover: &[],
            region,
            group: "",
            bins: 5,
            diverging_midpoint: 0.0,
            center_lat: None,
            center_lon: None,
        }
    }
}
