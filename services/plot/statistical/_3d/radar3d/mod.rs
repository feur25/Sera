use crate::html::js_3d::render_3d_html_impl;
use crate::plot::statistical::_3d::budget::Budget;
use crate::plot::statistical::_3d::{render_blocks3d_view_html, BlockView};
use crate::plot::statistical::radar::layout3d;
use crate::plot::statistical::RadarVariant;
use crate::plot::{apply_bg3d, parse_all};

const RING_GAP_CONTROL_JS: &str = "(function(){var wrap=document.getElementById(cid);if(!wrap)return;var outer=document.createElement('div');outer.style.cssText='display:flex;flex-direction:column;align-items:center;gap:10px;max-width:100%';wrap.parentElement.insertBefore(outer,wrap);outer.appendChild(wrap);var ctrl=document.createElement('div');ctrl.style.cssText='width:'+W+'px;max-width:100%;box-sizing:border-box;display:flex;align-items:center;gap:10px;font:11px -apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,sans-serif;color:#cbd5e1;background:rgba(15,23,42,.6);padding:7px 12px;border-radius:9px';var lbl=document.createElement('span');lbl.textContent='ring gap';lbl.style.cssText='white-space:nowrap;opacity:.75';var sl=document.createElement('input');sl.type='range';sl.min='0';sl.max='200';sl.step='1';sl.value=String(Math.round(RG*200));sl.style.cssText='flex:1;cursor:pointer';var val=document.createElement('span');val.textContent=RG.toFixed(2);val.style.cssText='width:32px;text-align:right;font-weight:700;color:#f1f5f9';sl.addEventListener('input',function(){RG=parseInt(sl.value,10)/200;val.textContent=RG.toFixed(2);R();});ctrl.appendChild(lbl);ctrl.appendChild(sl);ctrl.appendChild(val);outer.appendChild(ctrl);})();";

#[allow(clippy::too_many_arguments)]
fn render_closed_loop_radar3d_html(
    title: &str,
    axes: &[String],
    series: &[(String, Vec<f64>)],
    variant_key: &str,
    axis_labels: (&str, &str, &str),
    names: &[String],
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
    ring_gap: f64,
) -> String {
    let n_axes = axes.len();
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for (si, (_, vals)) in series.iter().enumerate() {
        let max_val = vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9);
        for ai in 0..n_axes.min(vals.len()) {
            let angle = std::f64::consts::TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r);
            yv.push(si as f64);
            zv.push(angle.sin() * r);
            cv.push(si as f64);
        }
    }
    let extra_js = format!("var RG={:.4};var RVAR='{}';{}", ring_gap, variant_key, RING_GAP_CONTROL_JS);
    let html = render_3d_html_impl(3, title, &xv, &yv, &zv, axis_labels, &cv, names, w, h, bg_color, scene, extra_js.as_bytes());
    html.replacen("<body>", &format!("<body><div height=\"{}\" style=\"display:none\"></div>", h + 60), 1)
}

#[crate::chart_demo("axes=[\"Python\",\"Rust\",\"SQL\",\"ML\",\"DevOps\"], series=[[9,7,8,8,6],[5,10,6,4,9]], series_names=[\"Alice\",\"Bob\"]")]
#[crate::params(paramsList["title","axes","series","series_names","variant","scene","orientation3d","theme","zone","max_points","bg_color","width","height","x_label","y_label","z_label","ring_gap"])]
#[crate::sera_alias("radar3d", "radar_3d", "radar3d_chart", "radar3d_family")]
#[crate::sera_builder]
pub fn build_radar3d_chart(input: &str) -> String {
    let (title_s, a, o) = parse_all(input);
    let title = title_s.as_str();
    let axes = a.axes.unwrap_or_default();
    let series_flat = a.series.unwrap_or_default();
    let names: Vec<String> = o.series_names.clone().unwrap_or_else(|| (0..series_flat.len()).map(|_| String::new()).collect());
    let series: Vec<(String, Vec<f64>)> = names.iter().cloned().zip(series_flat.iter().cloned()).collect();
    let variant = RadarVariant::from_str(o.variant.as_deref().unwrap_or("basic"));
    let env = o.scene.as_deref().unwrap_or("default");
    let bg_str = o.bg_str();
    let bg_default = if env == "default" && bg_str.is_none() { Some("#090d18") } else { bg_str.as_deref() };
    use RadarVariant::*;
    let html = match variant {
        Basic | Lines | Filled | Markers | Dashed => {
            let ring_gap = o.ring_gap.unwrap_or(1.0).clamp(0.0, 1.0);
            render_closed_loop_radar3d_html(
                title,
                &axes,
                &series,
                variant.name(),
                (&o.xl(), &o.yl(), &o.zl()),
                &names,
                o.w(900),
                o.h(560),
                bg_default,
                env,
                ring_gap,
            )
        }
        PolarBar | Petal | Band | Stacked => {
            let view = BlockView::new(0.65, "jet").with_zone(o.zone.as_deref());
            let (blocks, out_names) = layout3d::layout_named(&axes, &series, variant, &Budget::new(o.max_points));
            render_blocks3d_view_html(
                title,
                &blocks,
                &view,
                (&o.xl(), &o.yl(), &o.zl()),
                &out_names,
                o.w(900),
                o.h(560),
                bg_default,
                env,
            )
        }
    };
    apply_bg3d(html, &o)
}

inventory::submit! {
    crate::plot::controller::plot_3d_controller::Plot3DTypeEntry {
        group: "statistical",
        id: 70,
        name: "radar_3d",
        renderer: crate::plot::controller::plot_3d_controller::noop_3d_renderer,
        positioner: crate::plot::controller::plot_3d_controller::noop_3d_positioner,
    }
}

#[cfg(test)]
mod tests {
    use super::build_radar3d_chart;
    use crate::plot::statistical::_3d::twin;
    use crate::plot::statistical::RadarVariant;

    fn demos() -> twin::Demos {
        let base = r#"{"axes":["Python","Rust","SQL","ML","DevOps"],"series":[[9,7,8,8,6],[5,10,6,4,9]],"series_names":["Alice","Bob"]}"#;
        RadarVariant::keys_and_aliases().iter().map(|(key, _)| (*key, twin::set_field(base, "variant", key))).collect()
    }

    fn is_block_variant(key: &str) -> bool {
        matches!(key, "polar_bar" | "petal" | "band" | "stacked")
    }

    fn block_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| is_block_variant(key)).collect()
    }

    fn point_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_block_variant(key)).collect()
    }

    #[test]
    fn every_radar_variant_has_a_working_3d_counterpart() {
        twin::check_variants(build_radar3d_chart, &block_demos(), 4);
        for (key, json) in &point_demos() {
            let html = build_radar3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(!html.contains("var BN="), "{key} must render as a closed loop, not Bar3DBlock cuboids");
            assert!(html.contains("var yaw="), "{key} must carry the shared drag-to-rotate camera state");
        }
    }

    #[test]
    fn every_3d_plane_applies_to_every_radar_variant() {
        twin::check_planes(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_scene_applies_to_every_radar_variant() {
        twin::check_scenes(build_radar3d_chart, &block_demos());
        for (key, json) in &point_demos() {
            for (scene_key, _) in crate::plot::scene3d::Scene3DVariant::keys_and_aliases() {
                let html = build_radar3d_chart(&twin::set_field(json, "scene", scene_key));
                assert!(!html.is_empty(), "{key} under {scene_key} must render");
            }
        }
    }

    #[test]
    fn every_chart_theme_styles_every_radar_variant() {
        twin::check_themes(build_radar3d_chart, &demos());
    }

    #[test]
    fn the_registry_exposes_the_radar_variants_and_the_view_axes() {
        twin::check_axes("radar3d", RadarVariant::all().len());
    }

    #[test]
    fn every_radar_variant_honours_an_explicit_zone() {
        twin::check_zone(build_radar3d_chart, &block_demos());
    }

    #[test]
    fn the_ring_gap_control_is_only_injected_for_closed_loop_variants() {
        let closed_loop = build_radar3d_chart(r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"basic"}"#);
        assert!(closed_loop.contains("ring gap"), "closed-loop variants must carry the ring-gap gauge");
        let polar_bar = build_radar3d_chart(r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"polar_bar"}"#);
        assert!(!polar_bar.contains("ring gap"), "block-based variants have their own radial spacing, no ring-gap gauge");
    }

    #[test]
    fn every_closed_loop_variant_carries_its_own_2d_styling_flag() {
        for key in ["basic", "lines", "filled", "markers", "dashed"] {
            let json = format!(r#"{{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"{key}"}}"#);
            let html = build_radar3d_chart(&json);
            assert!(html.contains(&format!("RVAR='{key}'")), "{key} must carry its own RVAR flag");
        }
    }

    #[test]
    fn every_radar_variant_survives_empty_single_and_extreme_inputs() {
        twin::check_robust(build_radar3d_chart, &demos());
    }

    #[test]
    fn every_radar_variant_stays_within_the_block_budget_on_big_data() {
        twin::check_big(build_radar3d_chart, &demos(), 400);
    }

    #[test]
    #[ignore]
    fn write_preview_assets() {
        twin::write_previews("radar3d", build_radar3d_chart, &demos(), RadarVariant::default_key());
    }
}
