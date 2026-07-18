use crate::plot::controller::chart_controller::*;

const MAP_RENDERERS: &[(u8, &str, ChartRenderer)] = &[
    (20, "choropleth", super::render_choropleth as ChartRenderer),
    (21, "bubble_map", super::render_bubble_map as ChartRenderer),
];

const MAP_SVG_RENDERERS: &[(u8, SvgChartRenderer)] = &[
    (
        20,
        super::choropleth::render_svg_choropleth as SvgChartRenderer,
    ),
    (
        21,
        super::bubble_map::render_svg_bubble_map as SvgChartRenderer,
    ),
];

const MAP_COLORS: &[(u8, u32)] = &[(20, 0xF43F5E), (21, 0x636EFA)];

pub fn register_map_types() {
    let mut ids = Vec::new();
    for (id, name, renderer) in MAP_RENDERERS {
        if let Err(e) = ChartTypeBuilder::new(*id)
            .with_name(name)
            .with_renderer(*renderer)
            .build()
        {
            eprintln!("seraplot: failed to register map chart type '{name}' (id {id}): {e}");
        }
        ids.push(*id);
    }

    for (id, svg_renderer) in MAP_SVG_RENDERERS {
        with_registry_mut("register_map_types/svg", |reg| reg.register_svg(*id, *svg_renderer));
    }

    for (id, color) in MAP_COLORS {
        with_registry_mut("register_map_types/color", |reg| reg.register_color(*id, *color));
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("map".to_string(), ids);
    } else {
        eprintln!("seraplot: failed to register 'map' chart group, group registry lock poisoned");
    }
}
