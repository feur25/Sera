use crate::plot::controller::chart_controller::*;
use std::ffi::CStr;
use std::os::raw::c_char;

inventory::submit! {
    ChartTypeEntry {
        group: "default",
        id: 0,
        name: "line",
        renderer: super::render_lines as ChartRenderer,
        svg_renderer: Some(super::line::render_svg_lines as SvgChartRenderer),
        color: 0x50c878,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "default",
        id: 1,
        name: "scatter",
        renderer: super::render_points as ChartRenderer,
        svg_renderer: Some(super::scatter::render_svg_scatter as SvgChartRenderer),
        color: 0xf39c12,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "default",
        id: 2,
        name: "bar",
        renderer: super::render_bars as ChartRenderer,
        svg_renderer: Some(super::bar::render_svg_bars as SvgChartRenderer),
        color: 0x4a90e2,
    }
}

pub fn register_default_types() {
    register_group_from_inventory("default");
}

#[no_mangle]
pub extern "C" fn sera_register_chart_type(id: u8, name: *const c_char, renderer_id: u32) -> bool {
    if name.is_null() {
        return false;
    }

    let name_str = unsafe { CStr::from_ptr(name).to_string_lossy().into_owned() };

    let renderer: ChartRenderer = match renderer_id {
        0 => super::render_lines,
        1 => super::render_points,
        2 => super::render_bars,
        _ => return false,
    };

    ChartTypeBuilder::new(id)
        .with_name(&name_str)
        .with_renderer(renderer)
        .build()
        .is_ok()
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::chart_controller::test_support::*;

    #[test]
    fn default_group_is_well_formed() {
        assert_group_well_formed("default");
    }

    #[test]
    fn register_default_types_matches_inventory() {
        assert_registered_group_matches_inventory("default", super::register_default_types);
    }
}
