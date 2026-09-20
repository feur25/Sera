use crate::html::js_3d::render_3d_html;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::{colormap_3d, heatmap_layout_3d, HeatmapConfig, HeatmapVariant, HEIGHT_RATIO_3D};
use crate::plot::{apply_bg3d, parse_all};

pub fn render_heatmap3d_html(
    title: &str,
    x: &[f64],
    y: &[f64],
    z: &[f64],
    axis_labels: (&str, &str, &str),
    colors: &[f64],
    color_labels: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
) -> String {
    render_3d_html(
        9,
        title,
        x,
        y,
        z,
        axis_labels,
        colors,
        color_labels,
        w,
        h,
        bg_color,
        scene,
    )
}

#[crate::chart_demo("labels=[\"R1\",\"R2\"], categories=[\"C1\",\"C2\"], matrix=[[1,2],[3,4]]")]
#[crate::params(paramsList["title","labels","col_labels","categories","matrix","values","x_labels","variant","widths","ranges","bins","sort_order","scene","orientation3d","theme","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("heatmap3d", "heatmap_3d", "heatmap3d_chart", "heatmaps3d")]
#[crate::sera_builder]
pub fn build_heatmap3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let column_names = o.col_labels.clone().or_else(|| a.x_labels.clone());
    let (row_labels, col_labels) = match a.categories.clone() {
        Some(categories) => (categories, column_names.or_else(|| a.labels.clone()).unwrap_or_default()),
        None => (a.labels.clone().unwrap_or_default(), column_names.unwrap_or_default()),
    };
    let flat_matrix: Vec<f64> = match a.matrix {
        Some(rows) => rows.into_iter().flatten().collect(),
        None => a.values.unwrap_or_default(),
    };
    let variant = HeatmapVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let widths = o.widths.clone().unwrap_or_default();
    let heights = o.ranges.clone().unwrap_or_default();
    let cfg = HeatmapConfig {
        title,
        variant,
        row_labels: &row_labels,
        col_labels: &col_labels,
        flat_matrix: &flat_matrix,
        x_widths: &widths,
        y_heights: &heights,
        discrete_steps: o.bins.unwrap_or(0).max(0) as usize,
        ..HeatmapConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView {
        height_ratio: HEIGHT_RATIO_3D,
        cmap: colormap_3d(variant),
    };
    let html = render_blocks3d_view_html(
        title,
        &heatmap_layout_3d(&cfg),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &[],
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 76,
        name: "heatmap_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}
