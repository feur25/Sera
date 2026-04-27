use crate::html::hover::HoverSlot;

pub struct LineConfig<'a> {
    pub variant: super::variant::LineVariant,

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
    pub show_points: bool,

    pub series: &'a [(String, Vec<f64>)],
    pub x_labels: &'a [String],

    pub step_shape: &'a str,
    pub spline_tension: f64,
    pub fill_opacity: f64,
    pub stack_fill: bool,

    pub dash_pattern: &'a str,
    pub stroke_width: f64,
    pub marker_size: i32,

    pub gap_threshold: f64,
    pub spark_cols: usize,
    pub spark_cell_h: i32,
    pub spark_cell_w: i32,
}

impl<'a> Default for LineConfig<'a> {
    fn default() -> Self {
        Self {
            variant: super::variant::LineVariant::Basic,
            title: "", x_label: "", y_label: "",
            width: 900, height: 480,
            gridlines: false, sort_order: "none",
            legend_position: "right",
            hover: &[], palette: &[],
            labels: &[], values: &[], color_hex: 0, show_points: true,
            series: &[], x_labels: &[],
            step_shape: "hv", spline_tension: 0.5, fill_opacity: 0.3, stack_fill: false,
            dash_pattern: "8,4", stroke_width: 2.0, marker_size: 4,
            gap_threshold: f64::NAN,
            spark_cols: 3, spark_cell_h: 60, spark_cell_w: 220,
        }
    }
}
