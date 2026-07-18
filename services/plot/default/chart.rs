use crate::plot::controller::chart_controller::*;
use std::ffi::CStr;
use std::os::raw::c_char;

const DEFAULT_RENDERERS: &[(u8, &str, ChartRenderer)] = &[
    (0, "line", super::render_lines as ChartRenderer),
    (1, "scatter", super::render_points as ChartRenderer),
    (2, "bar", super::render_bars as ChartRenderer),
];

const DEFAULT_SVG_RENDERERS: &[(
    u8,
    crate::plot::controller::chart_controller::SvgChartRenderer,
)] = &[
    (
        0,
        super::line::render_svg_lines
            as crate::plot::controller::chart_controller::SvgChartRenderer,
    ),
    (
        1,
        super::scatter::render_svg_scatter
            as crate::plot::controller::chart_controller::SvgChartRenderer,
    ),
    (
        2,
        super::bar::render_svg_bars as crate::plot::controller::chart_controller::SvgChartRenderer,
    ),
];

const DEFAULT_COLORS: &[(u8, u32)] = &[(0, 0x50c878), (1, 0xf39c12), (2, 0x4a90e2)];

pub fn register_default_types() {
    let mut ids = Vec::new();
    for (id, name, renderer) in DEFAULT_RENDERERS {
        if let Err(e) = ChartTypeBuilder::new(*id)
            .with_name(name)
            .with_renderer(*renderer)
            .build()
        {
            eprintln!("seraplot: failed to register default chart type '{name}' (id {id}): {e}");
        }
        ids.push(*id);
    }

    for (id, svg_renderer) in DEFAULT_SVG_RENDERERS {
        with_registry_mut("register_default_types/svg", |reg| reg.register_svg(*id, *svg_renderer));
    }

    for (id, color) in DEFAULT_COLORS {
        with_registry_mut("register_default_types/color", |reg| reg.register_color(*id, *color));
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("default".to_string(), ids);
    } else {
        eprintln!("seraplot: failed to register 'default' chart group, group registry lock poisoned");
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
