use crate::plot::controller::plot_3d_controller::{
    get_group_registry, Plot3DPositioner, Plot3DRenderContext, Plot3DTypeBuilder,
};

fn render_lines_3d_wrapper(ctx: Plot3DRenderContext) {
    crate::plot::default::_3d::render_lines_3d(crate::plot::default::_3d::Line3DRenderContext {
        painter: ctx.painter,
        plot_rect: ctx.plot_rect,
        colors: ctx.colors,
        hovered_idx: ctx.hovered_idx,
        values: ctx.values,
        max_val: ctx.max_val,
        visible_indices: ctx.visible_indices,
        camera_controller: ctx.camera_controller,
    });
}

fn render_points_3d_wrapper(ctx: Plot3DRenderContext) {
    crate::plot::default::_3d::render_points_3d(
        crate::plot::default::_3d::Scatter3DRenderContext {
            painter: ctx.painter,
            plot_rect: ctx.plot_rect,
            colors: ctx.colors,
            hovered_idx: ctx.hovered_idx,
            values: ctx.values,
            max_val: ctx.max_val,
            visible_indices: ctx.visible_indices,
            camera_controller: ctx.camera_controller,
        },
    );
}

fn render_bars_3d_wrapper(ctx: Plot3DRenderContext) {
    crate::plot::default::_3d::render_bars_3d(crate::plot::default::_3d::Bar3DRenderContext {
        painter: ctx.painter,
        plot_rect: ctx.plot_rect,
        colors: ctx.colors,
        hovered_idx: ctx.hovered_idx,
        values: ctx.values,
        max_val: ctx.max_val,
        visible_indices: ctx.visible_indices,
        camera_controller: ctx.camera_controller,
    });
}

fn get_positions_line_3d_wrapper(
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &crate::plot::containers_3d::CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    crate::plot::default::_3d::line_3d::get_3d_positions(
        values,
        max_val,
        visible_indices,
        camera_controller,
        plot_rect,
    )
}

fn get_positions_scatter_3d_wrapper(
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &crate::plot::containers_3d::CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    crate::plot::default::_3d::scatter_3d::get_3d_positions(
        values,
        max_val,
        visible_indices,
        camera_controller,
        plot_rect,
    )
}

fn get_positions_bar_3d_wrapper(
    values: &[f64],
    max_val: f64,
    visible_indices: &[usize],
    camera_controller: &crate::plot::containers_3d::CameraController,
    plot_rect: egui::Rect,
) -> Vec<(egui::Pos2, usize)> {
    crate::plot::default::_3d::bar_3d::get_3d_positions(
        values,
        max_val,
        visible_indices,
        camera_controller,
        plot_rect,
    )
}

const DEFAULT_3D_TYPES: &[(
    u8,
    &str,
    crate::plot::controller::plot_3d_controller::Plot3DRenderer,
    Plot3DPositioner,
)] = &[
    (
        3,
        "line_3d",
        render_lines_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer,
        get_positions_line_3d_wrapper as Plot3DPositioner,
    ),
    (
        4,
        "scatter_3d",
        render_points_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer,
        get_positions_scatter_3d_wrapper as Plot3DPositioner,
    ),
    (
        5,
        "bar_3d",
        render_bars_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer,
        get_positions_bar_3d_wrapper as Plot3DPositioner,
    ),
];

pub fn register_default_3d_types() {
    let mut ids = Vec::new();

    for (id, name, renderer, positioner) in DEFAULT_3D_TYPES {
        let _ = Plot3DTypeBuilder::new(*id)
            .with_name(name)
            .with_renderer(*renderer)
            .build();

        crate::plot::controller::plot_3d_controller::register_positioner_for_type(*id, *positioner);
        ids.push(*id);
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("default".to_string(), ids);
    }
}
