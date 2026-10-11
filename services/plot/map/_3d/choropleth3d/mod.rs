use crate::core::math::heat_color;
use crate::plot::map::choropleth::layout3d;
use crate::plot::map::choropleth::{binned, bivariate, dot_density};
use crate::plot::map::choropleth::{ChoroplethConfig, ChoroplethVariant};
use crate::plot::map::regions;
use crate::plot::map::world_data;
use crate::plot::map::_3d::{render_globe3d_html, render_region_globe3d_html};
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::common::colorscale_color;
use crate::plot::{apply_bg3d, parse_all};
use std::collections::HashMap;

fn pack(c: (u8, u8, u8)) -> u32 {
    ((c.0 as u32) << 16) | ((c.1 as u32) << 8) | c.2 as u32
}

fn hex_of(value: f64, max_val: f64) -> u32 {
    pack(heat_color(value, max_val))
}

fn span_of(values: &[f64]) -> (f64, f64) {
    let min = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::INFINITY, f64::min);
    let max = values.iter().cloned().filter(|v| v.is_finite()).fold(f64::NEG_INFINITY, f64::max);
    if min.is_finite() && max.is_finite() {
        (min, max)
    } else {
        (0.0, 1.0)
    }
}

#[allow(clippy::too_many_arguments)]
fn render_globe_variant(
    title: &str,
    labels: &[String],
    values: &[f64],
    o: &crate::plot::chart_input::ChartOpts,
    env: &str,
    bg: Option<&str>,
    initial_pitch: Option<f64>,
    color_of: impl Fn(usize, f64) -> u32,
) -> String {
    let mut region_colors: HashMap<&str, u32> = HashMap::new();
    let (mut lons, mut lats, mut vals, mut cl) = (Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for i in 0..labels.len().min(values.len()) {
        let Some(shape) = world_data::lookup_country(&labels[i]) else { continue };
        region_colors.insert(shape.id.as_str(), color_of(i, values[i]));
        let c = world_data::shape_centroid(shape);
        let (lat, lon) = world_data::svg_to_latlon(c[0], c[1]);
        lons.push(lon);
        lats.push(lat);
        vals.push(values[i]);
        cl.push(labels[i].clone());
    }
    if lons.is_empty() {
        lons.push(0.0);
        lats.push(-89.0);
        vals.push(0.0);
        cl.push(String::new());
    }
    let cv: Vec<f64> = (0..lons.len()).map(|i| i as f64).collect();
    render_region_globe3d_html(title, &region_colors, &lons, &lats, &vals, ("Longitude", "Latitude", "Value"), &cv, &cl, o.w(1200), o.h(600), bg, env, initial_pitch)
}

fn render_dot_density_globe(title: &str, labels: &[String], values: &[f64], o: &crate::plot::chart_input::ChartOpts, env: &str, bg: Option<&str>) -> String {
    let dot_value = 400.0_f64;
    let (mut lons, mut lats, mut vals, mut cv, mut cl) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    for i in 0..labels.len().min(values.len()) {
        let Some(shape) = world_data::lookup_country(&labels[i]) else { continue };
        let polys = world_data::normalized_polygons(shape);
        let count = ((values[i] / dot_value).round() as usize).clamp(1, 400);
        let seed = (i as u32).wrapping_mul(747796405).wrapping_add(2891336453);
        for d in dot_density::scatter(&polys, count, seed) {
            let (lat, lon) = world_data::svg_to_latlon(d[0], d[1]);
            lons.push(lon);
            lats.push(lat);
            vals.push(1.0);
            cv.push(i as f64);
            cl.push(labels[i].clone());
        }
    }
    if lons.is_empty() {
        lons.push(0.0);
        lats.push(-89.0);
        vals.push(0.0);
        cv.push(0.0);
        cl.push(String::new());
    }
    render_globe3d_html(title, &lons, &lats, &vals, ("Longitude", "Latitude", "Value"), &cv, &cl, o.w(1200), o.h(600), bg, env)
}

#[allow(clippy::too_many_arguments)]
fn render_world_variant(
    title: &str,
    labels: &[String],
    values: &[f64],
    secondary_values: &[f64],
    variant: ChoroplethVariant,
    o: &crate::plot::chart_input::ChartOpts,
    env: &str,
    bg: Option<&str>,
) -> String {
    match variant {
        ChoroplethVariant::Sequential | ChoroplethVariant::Orthographic | ChoroplethVariant::DayNight => {
            let (_, max_val) = span_of(values);
            let max_val = max_val.max(1.0);
            render_globe_variant(title, labels, values, o, env, bg, None, move |_, v| hex_of(v, max_val))
        }
        ChoroplethVariant::Polar => {
            let (_, max_val) = span_of(values);
            let max_val = max_val.max(1.0);
            render_globe_variant(title, labels, values, o, env, bg, Some(1.45), move |_, v| hex_of(v, max_val))
        }
        ChoroplethVariant::Binned => {
            let bins = o.bins.map(|b| b as usize).unwrap_or(5).clamp(2, 9);
            let mut sorted: Vec<f64> = values.iter().cloned().filter(|v| v.is_finite()).collect();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let edges = binned::quantile_edges(&sorted, bins);
            render_globe_variant(title, labels, values, o, env, bg, None, move |_, value| {
                let bin = edges.iter().filter(|&&e| value >= e).count().min(bins - 1);
                let t = bin as f64 / (bins - 1).max(1) as f64;
                colorscale_color("viridis", t)
            })
        }
        ChoroplethVariant::Diverging => {
            let mid = 0.0_f64;
            let (min, max) = span_of(values);
            let spread = (max - mid).abs().max((min - mid).abs()).max(1e-9);
            render_globe_variant(title, labels, values, o, env, bg, None, move |_, value| {
                let t = 0.5 + 0.5 * (value - mid) / spread;
                colorscale_color("rdbu", t.clamp(0.0, 1.0))
            })
        }
        ChoroplethVariant::Bivariate => {
            let (a_min, a_max) = span_of(values);
            let (b_min, b_max) = span_of(secondary_values);
            let sv = secondary_values.to_vec();
            render_globe_variant(title, labels, values, o, env, bg, None, move |i, a| {
                let b = sv.get(i).copied().unwrap_or(0.0);
                pack(bivariate::color_for(a, b, a_min, a_max, b_min, b_max))
            })
        }
        ChoroplethVariant::DotDensity => render_dot_density_globe(title, labels, values, o, env, bg),
    }
}

#[crate::chart_demo("labels=[\"FR\",\"DE\",\"US\",\"BR\",\"CN\",\"AU\"], values=[42,55,88,30,95,18]")]
#[crate::params(paramsList["title","labels","values","secondary_values","map","region","bins","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label"])]
#[crate::sera_alias("choropleth3d", "choropleth_3d", "choropleth3d_chart", "geo_map3d")]
#[crate::sera_builder]
pub fn build_choropleth3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let labels = a.labels.unwrap_or_default();
    let values = a.values.unwrap_or_default();
    let secondary_values = o.secondary_values.clone().unwrap_or_default();
    let region = regions::resolve(o.map.as_deref().unwrap_or("")).or_else(regions::default_region_set).expect("world region set must be registered");
    let variant = ChoroplethVariant::from_str(o.variant.as_deref().unwrap_or("sequential"));
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    if region.key == "world" {
        let html = render_world_variant(title, &labels, &values, &secondary_values, variant, &o, env, bg_default);
        return apply_bg3d(html, &o);
    }
    let mut cfg = ChoroplethConfig::new(region);
    cfg.variant = variant;
    cfg.title = title;
    cfg.labels = &labels;
    cfg.values = &values;
    cfg.secondary_values = &secondary_values;
    cfg.group = o.region.as_deref().unwrap_or("");
    cfg.bins = o.bins.map(|b| b as usize).unwrap_or(5);
    let view = BlockView::new(0.6, "jet").with_zone(o.zone.as_deref());
    let (blocks, names) = layout3d::layout_named(&cfg, &Budget::new(o.max_points));
    let html = render_blocks3d_view_html(
        title,
        &blocks,
        &view,
        (&o.xl(), &o.yl(), &o.zl()),
        &names,
        o.w(1200),
        o.h(600),
        bg_default,
        env,
    );
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "map",
        id: 118,
        name: "choropleth_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_choropleth3d_chart;
    use crate::plot::map::choropleth::ChoroplethVariant;
    use crate::plot::statistical::_3d::twin;

    fn demos() -> twin::Demos {
        twin::variant_demos("map/choropleth/", ChoroplethVariant::keys_and_aliases(), ChoroplethVariant::default_key())
    }

    fn is_globe_variant(key: &str) -> bool {
        matches!(key, "orthographic" | "daynight" | "polar")
    }

    fn block_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_globe_variant(key)).collect()
    }

    #[test]
    fn every_choropleth_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_choropleth3d_chart, &block_demos(), ChoroplethVariant::all().len() - 3);
    }

    #[test]
    fn every_3d_plane_applies_to_every_choropleth_variant() {
        twin::check_planes(build_choropleth3d_chart, &block_demos());
    }

    #[test]
    fn every_scene_applies_to_every_choropleth_variant() {
        twin::check_scenes(build_choropleth3d_chart, &block_demos());
    }

    #[test]
    fn every_chart_theme_styles_every_choropleth_variant() {
        twin::check_themes(build_choropleth3d_chart, &block_demos());
    }

    #[test]
    fn the_registry_exposes_the_choropleth_variants_and_the_view_axes() {
        twin::check_axes("choropleth3d", ChoroplethVariant::all().len());
    }

    #[test]
    fn every_choropleth_variant_honours_an_explicit_zone() {
        twin::check_zone(build_choropleth3d_chart, &block_demos());
    }

    #[test]
    fn orthographic_daynight_and_polar_render_a_real_rotating_globe_with_colored_regions() {
        for variant in ["orthographic", "daynight", "polar"] {
            let json = format!(r#"{{"title":"t","labels":["FR","DE","US"],"values":[10.0,20.0,30.0],"variant":"{variant}"}}"#);
            let html = build_choropleth3d_chart(&json);
            assert!(html.contains("var MAP=["), "{variant} must carry the globe's country outlines");
            assert!(html.contains("var MAPCOL=["), "{variant} must carry per-country choropleth colors");
            assert!(!html.contains("var MAPCOL=[];"), "{variant} must actually color at least one matched country");
        }
        let polar_html = build_choropleth3d_chart(r#"{"title":"t","labels":["FR","DE","US"],"values":[10.0,20.0,30.0],"variant":"polar"}"#);
        assert!(polar_html.contains("var IPITCH="), "polar must start looking down from the pole");
        let sequential_html = build_choropleth3d_chart(r#"{"title":"t","labels":["FR","DE","US"],"values":[10.0,20.0,30.0],"variant":"sequential"}"#);
        assert!(!sequential_html.contains("var IPITCH="), "only polar gets the top-down default");
    }

    #[test]
    fn world_scoped_data_routes_every_variant_through_the_real_globe_renderer() {
        let base = r#"{"title":"t","labels":["FR","DE","US","BR"],"values":[10.0,20.0,30.0,40.0],"variant":"#;
        for variant in ["sequential", "binned", "diverging"] {
            let html = build_choropleth3d_chart(&format!("{base}\"{variant}\"}}"));
            assert!(html.contains("var MAP=["), "{variant} must use the real globe on world-scoped labels");
            assert!(html.contains("var MAPCOL=["), "{variant} must carry per-country choropleth colors");
        }
        let bivariate_json = r#"{"title":"t","labels":["FR","DE","US","BR"],"values":[10.0,20.0,30.0,40.0],"secondary_values":[1.0,2.0,3.0,4.0],"variant":"bivariate"}"#;
        let bivariate_html = build_choropleth3d_chart(bivariate_json);
        assert!(bivariate_html.contains("var MAP=["));
        assert!(bivariate_html.contains("var MAPCOL=["));
        let dot_html = build_choropleth3d_chart(r#"{"title":"t","labels":["FR","DE","US","BR"],"values":[4000.0,8000.0,20000.0,16000.0],"variant":"dot_density"}"#);
        assert!(dot_html.contains("var MAP=["), "dot_density must still draw the country outlines");
        assert!(dot_html.contains("var X=["), "dot_density must scatter real points across the matched countries");
    }

    #[test]
    fn an_unmatched_label_is_skipped_not_crashed() {
        let html = build_choropleth3d_chart(r#"{"title":"t","labels":["NOT_A_REAL_COUNTRY"],"values":[5.0],"variant":"orthographic"}"#);
        assert!(html.contains("var MAP=["));
    }

    #[test]
    fn empty_data_still_renders_a_bare_globe() {
        let html = build_choropleth3d_chart(r#"{"title":"t","variant":"orthographic"}"#);
        assert!(html.contains("var MAP=["));
    }

    #[test]
    fn every_choropleth_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_choropleth3d_chart, &demos());
    }

    #[test]
    fn every_choropleth_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_choropleth3d_chart, &demos(), 400);
    }

    #[test]
    fn switching_the_map_option_still_produces_a_working_3d_chart() {
        let html = build_choropleth3d_chart(r#"{"title":"t","labels":["CA","TX"],"values":[10.0,20.0],"map":"usa_states"}"#);
        assert!(html.contains("<canvas"), "expected a real 3D chart for usa_states: {html}");
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("choropleth3d", build_choropleth3d_chart, &demos(), ChoroplethVariant::default_key());
    }
}
