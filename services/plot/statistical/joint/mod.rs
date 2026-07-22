pub mod basic;
pub mod common;
pub mod variant;

pub use basic::build_joint;

#[cfg(test)]
mod tests {
    use super::build_joint as build;

    fn xy_json() -> serde_json::Value {
        let x: Vec<f64> = (0..40).map(|i| (i as f64 * 0.31).sin() * 3.0 + i as f64 * 0.12).collect();
        let y: Vec<f64> = (0..40).map(|i| (i as f64 * 0.21).cos() * 2.5 + i as f64 * 0.09).collect();
        serde_json::json!({ "x": x, "y": y })
    }

    #[test]
    fn any_registered_family_can_be_the_panel() {
        for family in ["hexbin", "scatter", "kde", "histogram", "bar"] {
            let mut input = xy_json();
            input["variant"] = serde_json::json!(family);
            let html = build(&input.to_string());
            assert!(html.contains("<iframe"), "panel={family} produced no iframe composition");
        }
    }

    #[test]
    fn any_registered_family_can_be_the_marginal() {
        for family in ["histogram", "kde", "scatter", "bar", "orbita"] {
            let mut input = xy_json();
            input["variant"] = serde_json::json!("hexbin");
            input["marginal"] = serde_json::json!(family);
            let html = build(&input.to_string());
            assert!(html.contains("<iframe"), "marginal={family} produced no iframe composition");
        }
    }

    #[test]
    fn marginal_none_disables_the_side_strips() {
        let mut input = xy_json();
        input["variant"] = serde_json::json!("hexbin");
        input["marginal"] = serde_json::json!("none");
        let html = build(&input.to_string());
        assert_eq!(html.matches("<iframe").count(), 1);
    }

    #[test]
    fn unknown_panel_family_returns_empty() {
        let mut input = xy_json();
        input["variant"] = serde_json::json!("not_a_real_family");
        assert!(build(&input.to_string()).is_empty());
    }

    #[test]
    fn legacy_preset_names_still_resolve_to_a_real_family() {
        for legacy in [
            "hexbin_marginal",
            "heat_scatter",
            "layered_bivariate",
            "joint_kde",
            "kde_smooth",
            "multiple_bivariate_kde",
            "marginal_ticks",
            "regression_marginals",
        ] {
            let mut input = xy_json();
            input["variant"] = serde_json::json!(legacy);
            let html = build(&input.to_string());
            assert!(!html.is_empty(), "legacy variant {legacy} produced empty html");
        }
    }
}
