use crate::html::hover::HoverSlot;

pub struct PieConfig<'a> {
    pub variant: super::variant::PieVariant,

    pub title: &'a str,
    pub x_label: &'a str,
    pub y_label: &'a str,
    pub gridlines: bool,
    pub sort_order: &'a str,
    pub hover: &'a [HoverSlot],
    pub legend_position: &'a str,
    pub width: i32,
    pub height: i32,

    pub labels: &'a [String],
    pub values: &'a [f64],

    pub donut: f64,
    pub show_pct: bool,
    pub min_label_frac: f64,
    pub palette: &'a [u32],

    pub pull: &'a [f64],

    pub series: &'a [Vec<f64>],
    pub subplot_titles: &'a [String],
    pub subplot_cols: usize,
    pub proportional: bool,
}

impl<'a> Default for PieConfig<'a> {
    fn default() -> Self {
        Self {
            variant: super::variant::PieVariant::Basic,
            title: "",
            x_label: "",
            y_label: "",
            gridlines: false,
            sort_order: "none",
            hover: &[],
            legend_position: "right",
            width: 720,
            height: 440,
            labels: &[],
            values: &[],
            donut: 0.0,
            show_pct: true,
            min_label_frac: 0.04,
            palette: &[],
            pull: &[],
            series: &[],
            subplot_titles: &[],
            subplot_cols: 0,
            proportional: false,
        }
    }
}

pub struct Pie;
