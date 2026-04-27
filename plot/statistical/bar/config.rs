use crate::html::hover::HoverSlot;

pub struct BarConfig<'a> {
    pub variant: super::variant::BarVariant,

    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub width: i32,
    pub height: i32,
    pub gridlines: bool,
    pub sort_order: &'a str,
    pub legend_position: &'a str,
    pub hover: &'a [HoverSlot],
    pub palette: &'a [u32],

    pub labels: &'a [String],
    pub values: &'a [f64],
    pub color_hex: u32,
    pub color_groups: &'a [String],

    pub category_labels: &'a [String],
    pub series: &'a [(String, Vec<f64>)],

    pub offset_groups: &'a [String],
    pub widths: &'a [f64],
    pub super_categories: &'a [String],

    pub icon_size: i32,
    pub max_icons_per_column: i32,
    pub units_per_icon: f64,
    pub unit_description: &'a str,

    pub show_text: bool,
    pub corner_radius: i32,
    pub bar_gap: f64,
    pub bargroup_gap: f64,
}

impl<'a> Default for BarConfig<'a> {
    fn default() -> Self {
        Self {
            variant: super::variant::BarVariant::Basic,
            title: "", x_label: "", y_label: "",
            width: 900, height: 480,
            gridlines: false, sort_order: "none",
            legend_position: "right",
            hover: &[], palette: &[],
            labels: &[], values: &[], color_hex: 0, color_groups: &[],
            category_labels: &[], series: &[],
            offset_groups: &[], widths: &[], super_categories: &[],
            icon_size: 24, max_icons_per_column: 10, units_per_icon: 1.0,
            unit_description: "",
            show_text: false, corner_radius: 0, bar_gap: 0.2, bargroup_gap: 0.1,
        }
    }
}
