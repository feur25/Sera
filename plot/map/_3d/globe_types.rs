use crate::plot::controller::plot_3d_controller::{Plot3DRenderContext, Plot3DTypeBuilder, get_group_registry, Plot3DPositioner};

fn render_globe_wrapper(ctx: Plot3DRenderContext) {
    let labels: Vec<String> = ctx.labels.to_vec();

    super::globe::render_globe_3d(super::globe::Globe3DRenderContext {
        painter: ctx.painter,
        plot_rect: ctx.plot_rect,
        colors: ctx.colors,
        hovered_idx: ctx.hovered_idx,
        values: ctx.values,
        max_val: ctx.max_val,
        visible_indices: ctx.visible_indices,
        camera_controller: ctx.camera_controller,
        labels: &labels,
    });
}

fn get_positions_globe_wrapper(
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &crate::plot::containers_3d::CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    super::globe::get_globe_3d_positions(values, max_val, visible_indices, camera_controller, plot_rect)
}

const MAP_3D_TYPES: &[(u8, &str, crate::plot::controller::plot_3d_controller::Plot3DRenderer, Plot3DPositioner)] = &[
    (23, "globe_3d", render_globe_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer, get_positions_globe_wrapper as Plot3DPositioner),
];

pub fn register_map_3d_types() {
    let mut ids = Vec::new();

    for (id, name, renderer, positioner) in MAP_3D_TYPES {
        let _ = Plot3DTypeBuilder::new(*id)
            .with_name(name)
            .with_renderer(*renderer)
            .build();

        crate::plot::controller::plot_3d_controller::register_positioner_for_type(*id, *positioner);
        ids.push(*id);
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("map".to_string(), ids);
    }
}
