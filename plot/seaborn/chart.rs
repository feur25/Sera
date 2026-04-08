use crate::plot::controller::chart_controller::*;

const SEABORN_RENDERERS: &[(u8, &str, ChartRenderer)] = &[
    (40, "seaborn_line", crate::plot::default::render_lines as ChartRenderer),
    (41, "seaborn_scatter", crate::plot::default::render_points as ChartRenderer),
    (42, "seaborn_bar", crate::plot::default::render_bars as ChartRenderer),
];

const SEABORN_SVG_RENDERERS: &[(u8, SvgChartRenderer)] = &[
    (40, crate::plot::default::line::render_svg_lines as SvgChartRenderer),
    (41, crate::plot::default::scatter::render_svg_scatter as SvgChartRenderer),
    (42, crate::plot::default::bar::render_svg_bars as SvgChartRenderer),
];

const SEABORN_COLORS: &[(u8, u32)] = &[
    (40, 0x4C72B0),
    (41, 0x55A868),
    (42, 0xC44E52),
];

pub fn register_seaborn_types() {
    let mut ids = Vec::new();

    for (id, name, renderer) in SEABORN_RENDERERS {
        let _ = ChartTypeBuilder::new(*id)
            .with_name(name)
            .with_renderer(*renderer)
            .build();
        ids.push(*id);
    }

    for (id, svg_renderer) in SEABORN_SVG_RENDERERS {
        if let Ok(mut reg) = get_registry().lock() {
            reg.register_svg(*id, *svg_renderer);
        }
    }

    for (id, color) in SEABORN_COLORS {
        if let Ok(mut reg) = get_registry().lock() {
            reg.register_color(*id, *color);
        }
    }

    if let Ok(mut grp_reg) = get_group_registry().lock() {
        grp_reg.register_group("seaborn".to_string(), ids);
    }
}
