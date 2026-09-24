use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::pulse::layout3d;
use crate::plot::statistical::{PulseConfig, PulseVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Mon\",\"Tue\",\"Wed\",\"Thu\",\"Fri\",\"Sat\",\"Sun\"], values=[0.4,0.7,0.9,0.6,0.8,0.3,0.5]")]
#[crate::params(paramsList["title","labels","values","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("pulse3d", "pulse_3d", "pulse3d_chart", "radial_bar3d", "rhythm3d")]
#[crate::sera_builder]
pub fn build_pulse3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let variant = PulseVariant::from_str(o.variant.as_deref().unwrap_or("radial"));
    let cfg = PulseConfig { variant, title, labels: &labels, values: &values, ..PulseConfig::default() };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    let view = BlockView::new(0.7, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(560),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 117,
        name: "pulse_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_pulse3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::PulseVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/pulse/", PulseVariant::keys_and_aliases(), PulseVariant::default_key())
    }

    #[test]
    fn every_pulse_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_pulse3d_chart, &demos(), PulseVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_pulse_variant() {
        twin::check_planes(build_pulse3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_pulse_variant() {
        twin::check_scenes(build_pulse3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_pulse_variant() {
        twin::check_themes(build_pulse3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_pulse_variants_and_the_view_axes() {
        twin::check_axes("pulse3d", PulseVariant::all().len());
    }

    #[test]
    fn every_pulse_variant_honours_an_explicit_zone() {
        twin::check_zone(build_pulse3d_chart, &demos());
    }

    #[test]
    fn every_pulse_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_pulse3d_chart, &demos());
    }

    #[test]
    fn every_pulse_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_pulse3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("pulse3d", build_pulse3d_chart, &demos(), PulseVariant::default_key());
    }
}
