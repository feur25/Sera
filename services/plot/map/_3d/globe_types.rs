use crate::plot::controller::plot_3d_controller::Plot3DRenderContext;

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
    super::globe::get_globe_3d_positions(
        values,
        max_val,
        visible_indices,
        camera_controller,
        plot_rect,
    )
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 23,
        name: "globe_3d",
        renderer: render_globe_wrapper,
        positioner: get_positions_globe_wrapper,
    }
}

pub fn register_map_3d_types() {
    crate::plot::controller::plot_3d_controller::register_group_from_inventory("map");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::plot_3d_controller::test_support::*;

    #[test]
    fn map_3d_group_is_well_formed() {
        assert_group_well_formed("map");
    }

    #[test]
    fn register_map_3d_types_matches_inventory() {
        assert_registered_group_matches_inventory("map", super::register_map_3d_types);
    }
}
