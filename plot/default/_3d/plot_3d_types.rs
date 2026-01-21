use crate::plot::controller::plot_3d_controller::{Plot3DRenderContext, Plot3DTypeBuilder, Plot3DGroupBuilder, get_group_registry};

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
    crate::plot::default::_3d::render_points_3d(crate::plot::default::_3d::Scatter3DRenderContext {
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

const RENDERERS: &[(u32, crate::plot::controller::plot_3d_controller::Plot3DRenderer)] = &[
    (0, render_lines_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer),
    (1, render_points_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer),
    (2, render_bars_3d_wrapper as crate::plot::controller::plot_3d_controller::Plot3DRenderer),
];

pub fn get_renderers() -> &'static [(u32, crate::plot::controller::plot_3d_controller::Plot3DRenderer)] {
    RENDERERS
}

pub fn register_default_3d_types() {
    let _ = Plot3DTypeBuilder::new(3)
        .with_name("line_3d")
        .with_renderer(render_lines_3d_wrapper)
        .build();

    let _ = Plot3DTypeBuilder::new(4)
        .with_name("scatter_3d")
        .with_renderer(render_points_3d_wrapper)
        .build();

    let _ = Plot3DTypeBuilder::new(5)
        .with_name("bar_3d")
        .with_renderer(render_bars_3d_wrapper)
        .build();
    
    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("default".to_string(), vec![3, 4, 5]);
    }
}
