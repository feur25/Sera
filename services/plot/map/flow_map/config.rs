use crate::plot::map::regions::RegionSetEntry;

pub struct FlowMapConfig<'a> {
    pub variant: super::variant::FlowMapVariant,
    pub title: &'a str,
    pub labels: &'a [String],
    pub sources: &'a [i32],
    pub targets: &'a [i32],
    pub weights: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub hover: &'a [crate::html::hover::HoverSlot],
    pub region: &'static RegionSetEntry,
    pub group: &'a str,
    pub min_width: f64,
    pub max_width: f64,
    pub lats: &'a [f64],
    pub lons: &'a [f64],
    pub track_values: &'a [f64],
}

impl<'a> FlowMapConfig<'a> {
    pub fn new(region: &'static RegionSetEntry) -> Self {
        Self {
            variant: super::variant::FlowMapVariant::Arc,
            title: "",
            labels: &[],
            sources: &[],
            targets: &[],
            weights: &[],
            width: 1200,
            height: 600,
            hover: &[],
            region,
            group: "",
            min_width: 1.0,
            max_width: 7.0,
            lats: &[],
            lons: &[],
            track_values: &[],
        }
    }
}
