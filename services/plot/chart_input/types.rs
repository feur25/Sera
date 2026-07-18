use serde::Deserialize;

fn deser_opt_fill_opacity<'de, D>(d: D) -> Result<Option<i32>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let v: serde_json::Value = serde_json::Value::deserialize(d)?;
    Ok(match &v {
        serde_json::Value::Null => None,
        serde_json::Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i as i32)
            } else if let Some(f) = n.as_f64() {
                Some(if f > 0.0 && f <= 1.0 { (f * 100.0) as i32 } else { f as i32 })
            } else {
                None
            }
        }
        _ => None,
    })
}

#[derive(Deserialize, Default)]
pub struct ChartOpts {
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub x_label: Option<String>,
    pub y_label: Option<String>,
    pub z_label: Option<String>,
    pub gridlines: Option<bool>,
    pub sort_order: Option<String>,
    pub hover_json: Option<String>,
    pub legend_position: Option<String>,
    pub scene: Option<String>,
    pub orientation3d: Option<String>,
    pub palette: Option<Vec<u32>>,
    pub background: Option<String>,
    pub bg_color: Option<String>,
    pub no_x_axis: Option<bool>,
    pub no_y_axis: Option<bool>,
    pub color_hex: Option<u32>,
    pub orientation: Option<String>,
    pub orientation_option: Option<u8>,
    pub show_text: Option<bool>,
    pub color_groups: Option<Vec<String>>,
    pub show_points: Option<bool>,
    pub show_regression: Option<bool>,
    pub regression_type: Option<String>,
    pub bins: Option<i32>,
    pub max_points: Option<usize>,
    pub voxel_size: Option<f64>,
    pub cone_size: Option<f64>,
    pub tube_radius: Option<f64>,
    pub n_steps: Option<usize>,
    pub iso_level: Option<f64>,
    pub show_counts: Option<bool>,
    pub overlay_color_hex: Option<u32>,
    pub show_values: Option<bool>,
    pub color_low: Option<u32>,
    pub color_mid: Option<u32>,
    pub color_high: Option<u32>,
    pub col_labels: Option<Vec<String>>,
    pub show_pct: Option<bool>,
    pub inner_radius_ratio: Option<f64>,
    pub left_label: Option<String>,
    pub right_label: Option<String>,
    pub stacked: Option<bool>,
    pub series_names: Option<Vec<String>>,
    pub targets: Option<Vec<f64>>,
    pub max_vals: Option<Vec<f64>>,
    pub ranges: Option<Vec<f64>>,
    pub comparisons: Option<Vec<f64>>,
    pub comparison: Option<f64>,
    pub color_values: Option<Vec<f64>>,
    pub color_labels: Option<Vec<String>>,
    pub filled: Option<bool>,
    #[serde(default, deserialize_with = "deser_opt_fill_opacity")]
    pub fill_opacity: Option<i32>,
    pub bandwidth: Option<f64>,
    pub overlap: Option<f64>,
    pub min_font: Option<f64>,
    pub max_font: Option<f64>,
    pub min_val: Option<f64>,
    pub max_val: Option<f64>,
    pub label: Option<String>,
    pub series_name_start: Option<String>,
    pub series_name_end: Option<String>,
    pub point_labels: Option<Vec<String>>,
    pub interval_ms: Option<u32>,
    pub cols: Option<usize>,
    pub gap: Option<i32>,
    pub cell_height: Option<i32>,
    pub eps: Option<f64>,
    pub min_samples: Option<usize>,
    pub k: Option<usize>,
    pub max_iter: Option<usize>,
    pub normalize: Option<bool>,
    pub images: Option<Vec<Option<String>>>,
    pub descriptions: Option<Vec<Vec<Vec<String>>>>,
    pub variant: Option<String>,
    pub offset_groups: Option<Vec<String>>,
    pub widths: Option<Vec<f64>>,
    pub super_categories: Option<Vec<String>>,
    pub icon_size: Option<i32>,
    pub max_icons_per_column: Option<i32>,
    pub units_per_icon: Option<f64>,
    pub unit_description: Option<String>,
    pub corner_radius: Option<i32>,
    pub bar_gap: Option<f64>,
    pub bargroup_gap: Option<f64>,
    pub ring_gap: Option<f64>,
    pub line_shape: Option<String>,
    pub step_shape: Option<String>,
    pub spline_tension: Option<f64>,
    pub dash_pattern: Option<String>,
    pub stroke_width: Option<f64>,
    pub marker_size: Option<i32>,
    pub gap_threshold: Option<f64>,
    pub spark_cols: Option<usize>,
    pub spark_cell_w: Option<i32>,
    pub spark_cell_h: Option<i32>,
    pub stack_fill: Option<bool>,
    pub fill_opacity_f: Option<f64>,
    pub min_size: Option<f64>,
    pub max_size: Option<f64>,
    pub point_size: Option<f64>,
    pub symbol: Option<String>,
    pub symbols: Option<Vec<String>>,
    pub pull: Option<Vec<f64>>,
    pub subplot_titles: Option<Vec<String>>,
    pub subplot_cols: Option<usize>,
    pub proportional: Option<bool>,
    pub min_label_frac: Option<f64>,
    pub center_text: Option<String>,
    pub center_subtext: Option<String>,
    pub secondary_values: Option<Vec<f64>>,
    pub secondary_labels: Option<Vec<String>>,
    pub pattern: Option<String>,
    pub notch: Option<bool>,
    pub jitter: Option<f64>,
    pub boxen_depth: Option<usize>,
    pub violin_overlay: Option<bool>,
    pub fill_opacity_real: Option<f64>,
    pub box_stroke_width: Option<f64>,
    pub colorscale: Option<String>,
    pub colorbar_position: Option<String>,
    pub origin_lower: Option<bool>,
    pub show_box: Option<bool>,
    pub show_mean: Option<bool>,
    pub highlight_index: Option<i32>,
    pub color_axis: Option<i32>,
    pub category_indices: Option<Vec<i32>>,
    pub annotations: Option<Vec<Annotation>>,
    pub shape: Option<String>,
    pub mask: Option<Vec<i32>>,
    pub mask_width: Option<i32>,
    pub mask_height: Option<i32>,
    pub points_x: Option<Vec<f64>>,
    pub points_y: Option<Vec<f64>>,
    pub cluster_labels: Option<Vec<String>>,
    pub edges_i: Option<Vec<i32>>,
    pub edges_j: Option<Vec<i32>>,
    pub edges_w: Option<Vec<f64>>,
    pub theme: Option<String>,
    pub despine: Option<bool>,
    pub watermark_text: Option<String>,
    pub watermark_opacity: Option<f64>,
    pub caption: Option<String>,
    pub glow_color: Option<String>,
    pub highlight_at: Option<usize>,
    pub highlight_color: Option<String>,
    pub hline_value: Option<f64>,
    pub hline_color: Option<String>,
    pub hline_label: Option<String>,
    pub subtitle: Option<String>,
    pub shadow_blur: Option<i32>,
    pub shadow_color: Option<String>,
    pub pulse_duration: Option<f64>,
    pub pulse_index: Option<Vec<usize>>,
    pub pulse_above: Option<f64>,
    pub pulse_color: Option<String>,
    pub outline_color: Option<String>,
    pub outline_width: Option<f64>,
    pub value_labels: Option<bool>,
    pub value_labels_decimals: Option<i32>,
    pub value_labels_color: Option<String>,
    pub error_bars_margin: Option<f64>,
    pub error_bars_color: Option<String>,
    pub delta_labels: Option<bool>,
    pub delta_labels_pos_color: Option<String>,
    pub delta_labels_neg_color: Option<String>,
    pub cumulative_line_color: Option<String>,
    pub rank_badges_top_n: Option<usize>,
    pub rank_badges_color: Option<String>,
    pub title_color: Option<String>,
    pub export_button: Option<bool>,
    pub log_scale: Option<bool>,
    pub moving_average_window: Option<usize>,
    pub moving_average_color: Option<String>,
    pub outliers_threshold: Option<f64>,
    pub outliers_color: Option<String>,
    pub fill_between_color: Option<String>,
    pub fill_between_opacity: Option<f64>,
    pub box_annotate_color: Option<String>,
    pub pct_of_total: Option<bool>,
    pub pct_of_total_decimals: Option<i32>,
    pub pct_of_total_color: Option<String>,
    pub correlation_badge_color: Option<String>,
    pub highlight_range_low: Option<usize>,
    pub highlight_range_high: Option<usize>,
    pub highlight_range_color: Option<String>,
    pub highlight_range_opacity: Option<f64>,
    pub iqr_band_color: Option<String>,
    pub iqr_band_opacity: Option<f64>,
    pub growth_badge_color: Option<String>,
    pub zscore_heat: Option<bool>,
    pub pareto_marker_threshold: Option<f64>,
    pub pareto_marker_color: Option<String>,
    pub diff_from_mean: Option<bool>,
    pub diff_from_mean_pos_color: Option<String>,
    pub diff_from_mean_neg_color: Option<String>,
    pub scatter_regression_color: Option<String>,
    pub scatter_regression_width: Option<f64>,
    pub cluster_eps: Option<f64>,
    pub cluster_min_samples: Option<usize>,
    pub rolling_std_band_window: Option<usize>,
    pub rolling_std_band_color: Option<String>,
    pub rolling_std_band_opacity: Option<f64>,
    pub forecast_line_periods: Option<usize>,
    pub forecast_line_color: Option<String>,
    pub percentile_band_low: Option<f64>,
    pub percentile_band_high: Option<f64>,
    pub percentile_band_color: Option<String>,
    pub percentile_band_opacity: Option<f64>,
}

impl ChartOpts {
    pub fn w(&self, default: i32) -> i32 {
        self.width.unwrap_or(default)
    }
    pub fn h(&self, default: i32) -> i32 {
        self.height.unwrap_or(default)
    }
    pub fn xl(&self) -> String {
        self.x_label.clone().unwrap_or_default()
    }
    pub fn yl(&self) -> String {
        self.y_label.clone().unwrap_or_default()
    }
    pub fn zl(&self) -> String {
        self.z_label.clone().unwrap_or_else(|| "Z".to_string())
    }
    pub fn grid(&self) -> bool {
        self.gridlines.unwrap_or(false) || crate::plot::get_global_grid()
    }
    pub fn srt(&self) -> String {
        self.sort_order
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }
    pub fn lp(&self) -> String {
        self.legend_position
            .clone()
            .unwrap_or_else(|| "none".to_string())
    }
    pub fn scene3d(&self) -> String {
        crate::plot::scene3d::Scene3DVariant::from_str(self.scene.as_deref().unwrap_or("studio"))
            .name()
            .to_string()
    }
    pub fn pal(&self) -> Vec<u32> {
        if let Some(p) = &self.palette {
            if !p.is_empty() {
                return p.clone();
            }
        }
        let g = crate::plot::get_global_pal();
        if !g.is_empty() {
            g
        } else {
            Vec::new()
        }
    }
    pub fn hj(&self) -> Vec<crate::html::hover::HoverSlot> {
        self.hover_json
            .as_ref()
            .filter(|s| !s.is_empty())
            .map(|s| crate::plot::statistical::parse_hover_json(s))
            .unwrap_or_default()
    }
    pub fn bg_str(&self) -> Option<String> {
        self.background
            .clone()
            .or_else(|| self.bg_color.clone())
            .filter(|s| !s.is_empty())
    }

    pub fn no_x(&self) -> bool {
        self.no_x_axis.unwrap_or(false)
    }
    pub fn no_y(&self) -> bool {
        self.no_y_axis.unwrap_or(false)
    }
    pub fn no_background(&self) -> bool {
        self.background.is_none() && self.bg_color.is_none()
    }

    pub fn is_horiz(&self) -> bool {
        match self.orientation.as_deref() {
            Some(s) => {
                let l = s.to_ascii_lowercase();
                l == "h"
                    || l == "horiz"
                    || l == "horizontal"
                    || l == "rotated"
                    || l == "hbar"
                    || l == "hbox"
                    || l == "barh"
            }
            None => false,
        }
    }
    pub fn orient_byte(&self) -> u8 {
        if self.is_horiz() {
            b'h'
        } else {
            b'v'
        }
    }
    pub fn rotation_deg(&self) -> i32 {
        if let Some(opt) = self.orientation_option {
            return match opt {
                2 => 90,
                3 => 180,
                4 => 270,
                _ => 0,
            };
        }
        if self.is_horiz() {
            90
        } else {
            0
        }
    }
    pub fn rotation_deg_native(&self) -> i32 {
        match self.orientation_option.unwrap_or(0) {
            2 => 90,
            3 => 180,
            4 => 270,
            _ => 0,
        }
    }
}

#[derive(Deserialize, Default, Clone)]
pub struct Annotation {
    pub kind: String,
    pub x: Option<f64>,
    pub y: Option<f64>,
    pub x2: Option<f64>,
    pub y2: Option<f64>,
    pub text: Option<String>,
    pub color: Option<String>,
    pub stroke_width: Option<f64>,
    pub dash: Option<String>,
    pub frac: Option<bool>,
    pub font_size: Option<f64>,
    pub fill: Option<String>,
    pub opacity: Option<f64>,
}

#[derive(Deserialize, Default)]
pub struct ChartArgs {
    pub labels: Option<Vec<String>>,
    pub values: Option<Vec<f64>>,
    #[serde(alias = "x_values")]
    pub x: Option<Vec<f64>>,
    #[serde(alias = "y_values")]
    pub y: Option<Vec<f64>>,
    #[serde(alias = "z_values")]
    pub z: Option<Vec<f64>>,
    pub x_labels: Option<Vec<String>>,
    pub series: Option<Vec<Vec<f64>>>,
    pub matrix: Option<Vec<Vec<f64>>>,
    pub parents: Option<Vec<String>>,
    pub categories: Option<Vec<String>>,
    pub category_series: Option<Vec<Vec<String>>>,
    pub open: Option<Vec<f64>>,
    pub high: Option<Vec<f64>>,
    pub low: Option<Vec<f64>>,
    pub close: Option<Vec<f64>>,
    pub sizes: Option<Vec<f64>>,
    pub overlay: Option<Vec<f64>>,
    pub left: Option<Vec<f64>>,
    pub right: Option<Vec<f64>>,
    pub start: Option<Vec<f64>>,
    pub end: Option<Vec<f64>>,
    pub size: Option<Vec<f64>>,
    pub charts: Option<Vec<String>>,
    pub value: Option<f64>,
    pub lats: Option<Vec<f64>>,
    pub lons: Option<Vec<f64>>,
    pub axes: Option<Vec<String>>,
    pub words: Option<Vec<String>>,
    pub frequencies: Option<Vec<f64>>,
    pub data: Option<Vec<Vec<f64>>>,
    pub mesh_i: Option<Vec<i64>>,
    pub mesh_j: Option<Vec<i64>>,
    pub mesh_k: Option<Vec<i64>>,
    pub u: Option<Vec<f64>>,
    pub v: Option<Vec<f64>>,
    pub w: Option<Vec<f64>>,
    pub field: Option<Vec<f64>>,
}
