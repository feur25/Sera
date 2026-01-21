use std::ffi::CStr;
use std::os::raw::c_char;
use crate::plot::controller::chart_controller::*;

pub fn register_default_types() {
    let _ = ChartTypeBuilder::new(0)
        .with_name("line")
        .with_renderer(super::render_lines)
        .build();

    let _ = ChartTypeBuilder::new(1)
        .with_name("scatter")
        .with_renderer(super::render_points)
        .build();

    let _ = ChartTypeBuilder::new(2)
        .with_name("bar")
        .with_renderer(super::render_bars)
        .build();
    
    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("default".to_string(), vec![0, 1, 2]);
    }
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