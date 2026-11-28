use crate::plot::controller::chart_controller::*;

inventory::submit! {
    ChartTypeEntry {
        group: "seaborn",
        id: 40,
        name: "seaborn_line",
        renderer: crate::plot::default::render_lines as ChartRenderer,
        svg_renderer: Some(crate::plot::default::line::render_svg_lines as SvgChartRenderer),
        color: 0x636EFA,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "seaborn",
        id: 41,
        name: "seaborn_scatter",
        renderer: crate::plot::default::render_points as ChartRenderer,
        svg_renderer: Some(crate::plot::default::scatter::render_svg_scatter as SvgChartRenderer),
        color: 0x10B981,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "seaborn",
        id: 42,
        name: "seaborn_bar",
        renderer: crate::plot::default::render_bars as ChartRenderer,
        svg_renderer: Some(crate::plot::default::bar::render_svg_bars as SvgChartRenderer),
        color: 0xF43F5E,
    }
}

pub fn register_seaborn_types() {
    register_group_from_inventory("seaborn");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::chart_controller::test_support::*;

    #[test]
    fn seaborn_group_is_well_formed() {
        assert_group_well_formed("seaborn");
    }

    #[test]
    fn register_seaborn_types_matches_inventory() {
        assert_registered_group_matches_inventory("seaborn", super::register_seaborn_types);
    }
}
