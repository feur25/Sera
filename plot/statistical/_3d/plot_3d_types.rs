use crate::plot::controller::plot_3d_controller::{
    get_group_registry, Plot3DRenderContext, Plot3DTypeBuilder,
};
use crate::plot::{apply_bg3d, parse_all};

fn noop_3d_renderer(_ctx: Plot3DRenderContext) {}

fn noop_3d_positioner(
    _values: &[f64],
    _max_val: f64,
    _visible_indices: &[usize],
    _camera_controller: &crate::plot::containers_3d::CameraController,
    _plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    Vec::new()
}

const STATISTICAL_3D_TYPES: &[(u8, &str)] = &[
    (70, "radar_3d"),
    (71, "lollipop_3d"),
    (72, "kde_3d"),
    (73, "ridgeline_3d"),
    (74, "pie_3d"),
    (75, "violin_3d"),
    (76, "heatmap_3d"),
    (77, "candlestick_3d"),
    (78, "dumbbell_3d"),
    (79, "funnel_3d"),
    (80, "sunburst_3d"),
    (81, "stacked_bar_3d"),
];

pub fn register_statistical_3d_types() {
    let mut ids = Vec::new();

    for &(id, name) in STATISTICAL_3D_TYPES {
        let _ = Plot3DTypeBuilder::new(id)
            .with_name(name)
            .with_renderer(noop_3d_renderer)
            .build();

        crate::plot::controller::plot_3d_controller::register_positioner_for_type(
            id,
            noop_3d_positioner,
        );
        ids.push(id);
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("statistical".to_string(), ids);
    }
}

#[crate::chart_demo("x=[1,2,3], y=[1,2,3], z=[1,2,3], sizes=[10,20,30]")]
#[crate::params(paramsList["title","x","y","z","sizes","palette","bg_color","scene","orientation3d","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias(
    "bubble3d",
    "bubble_3d",
    "bubble3d_chart",
    "bubble3d_family",
    "bubbles3d"
)]
#[crate::sera_builder]
pub fn build_bubble3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let x = a.x.unwrap_or_default();
    let y = a.y.unwrap_or_default();
    let z = a.z.unwrap_or_default();
    let size_values = a.size.or(a.sizes).unwrap_or_default();
    let cv = o.color_values.clone().unwrap_or_default();
    let cl = o.color_labels.clone().unwrap_or_default();
    let n = x.len().min(y.len()).min(z.len()).min(size_values.len());
    let smn = size_values[..n]
        .iter()
        .cloned()
        .fold(f64::INFINITY, f64::min);
    let smx = size_values[..n]
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let sr = (smx - smn).max(1e-9);
    let size_js = format!(
        "var S=[{}];",
        size_values[..n]
            .iter()
            .map(|&s| format!("{:.3}", (s - smn) / sr))
            .collect::<Vec<_>>()
            .join(",")
    );
    let bg_str = o.bg_str();
    let html = crate::html::js_3d::render_3d_html_impl(
        16,
        title,
        &x[..n],
        &y[..n],
        &z[..n],
        (&o.xl(), &o.yl(), &o.zl()),
        &cv,
        &cl,
        o.w(900),
        o.h(560),
        bg_str.as_deref(),
        &o.scene3d(),
        size_js.as_bytes(),
    );
    apply_bg3d(html, &o)
}
