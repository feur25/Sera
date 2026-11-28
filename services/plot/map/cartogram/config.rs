use crate::plot::map::regions::RegionSetEntry;

pub struct CartogramConfig<'a> {
    pub variant: super::variant::CartogramVariant,
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub lats: &'a [f64],
    pub lons: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub hover: &'a [crate::html::hover::HoverSlot],
    pub region: &'static RegionSetEntry,
    pub group: &'a str,
    pub min_radius: f64,
    pub max_radius: f64,
    pub color_low: u32,
    pub color_high: u32,
    pub iterations: u32,
}

impl<'a> CartogramConfig<'a> {
    pub fn new(region: &'static RegionSetEntry) -> Self {
        Self {
            variant: super::variant::CartogramVariant::Dorling,
            title: "",
            labels: &[],
            values: &[],
            lats: &[],
            lons: &[],
            width: 1200,
            height: 650,
            hover: &[],
            region,
            group: "",
            min_radius: 6.0,
            max_radius: 46.0,
            color_low: 0x1e3a8a,
            color_high: 0xf59e0b,
            iterations: 180,
        }
    }
}
