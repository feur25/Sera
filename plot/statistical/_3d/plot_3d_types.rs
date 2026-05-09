use crate::plot::controller::plot_3d_controller::{Plot3DRenderContext, Plot3DTypeBuilder, get_group_registry};

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


