use crate::plot::map::regions::RegionSetEntry;

pub struct BubbleMapConfig<'a> {
    pub variant: super::variant::BubbleMapVariant,
    pub title: &'a str,
    pub labels: &'a [String],
    pub values: &'a [f64],
    pub width: i32,
    pub height: i32,
    pub hover: &'a [crate::html::hover::HoverSlot],
    pub region: &'static RegionSetEntry,
    pub group: &'a str,
    pub min_bubble_size: f64,
    pub max_bubble_size: f64,
    pub center_lat: Option<f64>,
    pub center_lon: Option<f64>,
}

impl<'a> BubbleMapConfig<'a> {
    pub fn new(region: &'static RegionSetEntry) -> Self {
        Self {
            variant: super::variant::BubbleMapVariant::Proportional,
            title: "",
            labels: &[],
            values: &[],
            width: 1200,
            height: 600,
            hover: &[],
            region,
            group: "",
            min_bubble_size: 5.0,
            max_bubble_size: 42.0,
            center_lat: None,
            center_lon: None,
        }
    }
}
