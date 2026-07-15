use crate::plot::controller::plot_3d_controller::Plot3DRenderContext;

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

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "default",
        id: 3,
        name: "line_3d",
        renderer: render_lines_3d_wrapper,
        positioner: get_positions_line_3d_wrapper,
    }
}
inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "default",
        id: 4,
        name: "scatter_3d",
        renderer: render_points_3d_wrapper,
        positioner: get_positions_scatter_3d_wrapper,
    }
}
inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "default",
        id: 5,
        name: "bar_3d",
        renderer: render_bars_3d_wrapper,
        positioner: get_positions_bar_3d_wrapper,
    }
}

pub fn register_default_3d_types() {
    crate::plot::controller::plot_3d_controller::register_group_from_inventory("default");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::plot_3d_controller::test_support::*;

    #[test]
    fn default_3d_group_is_well_formed() {
        assert_group_well_formed("default");
    }

    #[test]
    fn register_default_3d_types_matches_inventory() {
        assert_registered_group_matches_inventory("default", super::register_default_3d_types);
    }
}
