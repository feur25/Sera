use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::waterfall::layout3d;
use crate::plot::statistical::{WaterfallConfig, WaterfallVariant};
use crate::plot::{apply_bg3d, parse_all};

#[crate::chart_demo("labels=[\"Start\",\"Q1\",\"Q2\",\"Q3\",\"End\"], values=[100,30,-15,40,155]")]
#[crate::params(paramsList["title","labels","values","variant","sort_order","x_label","y_label","z_label","bg_color","scene","orientation3d","theme","zone","max_points","width","height"])]
#[crate::sera_alias("waterfall3d", "waterfall_3d", "waterfall3d_chart")]
#[crate::sera_builder]
pub fn build_waterfall3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let sort_order = o.srt();
    let variant = WaterfallVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let cfg = WaterfallConfig {
        title,
        variant,
        labels: &labels,
        values: &values,
        sort_order: &sort_order,
        ..WaterfallConfig::default()
    };
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() {
        Some("#090d18")
    } else {
        bg_str.as_deref()
    };
    let view = BlockView::new(layout3d::HEIGHT_RATIO, layout3d::COLORMAP).with_zone(o.zone.as_deref());
    let html = render_blocks3d_view_html(
        title,
        &layout3d::layout_3d(&cfg, &Budget::new(o.max_points)),
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &labels,
        o.w(900),
        o.h(560),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 90,
        name: "waterfall_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use crate::plot::build_waterfall3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::WaterfallVariant;

    fn demos() -> twin::Demos {
        twin::variant_demos("statistical/waterfall/", WaterfallVariant::keys_and_aliases(), WaterfallVariant::default_key())
    }

    #[test]
    fn every_waterfall_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_waterfall3d_chart, &demos(), WaterfallVariant::all().len());
    }

    #[test]
    fn every_3d_plane_applies_to_every_waterfall_variant() {
        twin::check_planes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_waterfall_variant() {
        twin::check_scenes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn every_chart_theme_styles_every_waterfall_variant() {
        twin::check_themes(build_waterfall3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_waterfall_variants_and_the_view_axes() {
        twin::check_axes("waterfall3d", WaterfallVariant::all().len());
    }

    #[test]
    fn every_waterfall_variant_honours_an_explicit_zone() {
        twin::check_zone(build_waterfall3d_chart, &demos());
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("waterfall3d", build_waterfall3d_chart, &demos(), WaterfallVariant::default_key());
    }
}
