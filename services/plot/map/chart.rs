use crate::plot::controller::chart_controller::*;

inventory::submit! {
    ChartTypeEntry {
        group: "map",
        id: 20,
        name: "choropleth",
        renderer: super::render_choropleth as ChartRenderer,
        svg_renderer: Some(super::choropleth::render_svg_choropleth as SvgChartRenderer),
        color: 0xF43F5E,
    }
}
inventory::submit! {
    ChartTypeEntry {
        group: "map",
        id: 21,
        name: "bubble_map",
        renderer: super::render_bubble_map as ChartRenderer,
        svg_renderer: Some(super::bubble_map::render_svg_bubble_map as SvgChartRenderer),
        color: 0x636EFA,
    }
}

pub fn register_map_types() {
    register_group_from_inventory("map");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::chart_controller::test_support::*;

    #[test]
    fn map_group_is_well_formed() {
        assert_group_well_formed("map");
    }

    #[test]
    fn register_map_types_matches_inventory() {
        assert_registered_group_matches_inventory("map", super::register_map_types);
    }
}
