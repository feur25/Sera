pub fn register_statistical_3d_types() {
    crate::plot::controller::plot_3d_controller::register_group_from_inventory("statistical");
}

#[cfg(test)]
mod inventory_tests {
    use crate::plot::controller::plot_3d_controller::test_support::*;

    #[test]
    fn statistical_3d_group_is_well_formed() {
        assert_group_well_formed("statistical");
    }

    #[test]
    fn register_statistical_3d_types_matches_inventory() {
        assert_registered_group_matches_inventory("statistical", super::register_statistical_3d_types);
    }
}
