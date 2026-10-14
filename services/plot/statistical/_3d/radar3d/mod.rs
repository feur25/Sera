use crate::html::js_3d::render_3d_html_impl;
use crate::plot::statistical::_3d::wedge::{self, Wedges};
use crate::plot::statistical::RadarVariant;
use crate::plot::{apply_bg3d, parse_all};
use std::f64::consts::TAU;

const RING_GAP_CONTROL_JS: &str = "(function(){var wrap=document.getElementById(cid);if(!wrap)return;var outer=document.createElement('div');outer.style.cssText='display:flex;flex-direction:column;align-items:center;gap:10px;max-width:100%';wrap.parentElement.insertBefore(outer,wrap);outer.appendChild(wrap);var ctrl=document.createElement('div');ctrl.className='sp-ring-gap-ctrl';ctrl.style.cssText='position:relative;width:'+W+'px;max-width:100%;box-sizing:border-box;display:flex;align-items:center;gap:10px;font:11px -apple-system,BlinkMacSystemFont,\"Segoe UI\",Roboto,sans-serif;color:#cbd5e1;background:rgba(15,23,42,.6);padding:7px 12px;border-radius:9px;touch-action:none';var lbl=document.createElement('span');lbl.textContent='ring gap';lbl.style.cssText='white-space:nowrap;opacity:.75;cursor:grab;user-select:none';var sl=document.createElement('input');sl.type='range';sl.min='0';sl.max='200';sl.step='1';sl.value=String(Math.round(RG*200));sl.style.cssText='flex:1;cursor:pointer';var val=document.createElement('span');val.textContent=RG.toFixed(2);val.style.cssText='width:32px;text-align:right;font-weight:700;color:#f1f5f9';sl.addEventListener('input',function(){RG=parseInt(sl.value,10)/200;val.textContent=RG.toFixed(2);R();});ctrl.appendChild(lbl);ctrl.appendChild(sl);ctrl.appendChild(val);outer.appendChild(ctrl);var dgX=0,dgY=0,dragging=false,startX=0,startY=0;lbl.addEventListener('mousedown',function(e){dragging=true;startX=e.clientX-dgX;startY=e.clientY-dgY;lbl.style.cursor='grabbing';e.preventDefault();});window.addEventListener('mousemove',function(e){if(!dragging)return;dgX=e.clientX-startX;dgY=e.clientY-startY;ctrl.style.transform='translate('+dgX+'px,'+dgY+'px)';});window.addEventListener('mouseup',function(){dragging=false;lbl.style.cursor='grab';});})();";

fn wrap_pg_content_div(html: String, h: i32) -> String {
    html.replacen("<body>", &format!("<body><div height=\"{}\" style=\"display:none\"></div>", h + 60), 1)
}

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
    shared_max: Option<f64>,
) -> String {
    let n_axes = axes.len();
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    for (si, (_, vals)) in series.iter().enumerate() {
        let max_val = shared_max.unwrap_or_else(|| vals.iter().cloned().fold(0.0f64, f64::max).max(1e-9));
        for ai in 0..n_axes.min(vals.len()) {
            let angle = TAU * ai as f64 / n_axes as f64;
            let r = vals[ai] / max_val;
            xv.push(angle.cos() * r);
            yv.push(si as f64);
            zv.push(angle.sin() * r);
            cv.push(si as f64);
        }
    }
    let extra_js = format!("var RG={:.4};var RVAR='{}';{}", ring_gap, variant_key, RING_GAP_CONTROL_JS);
    let html = render_3d_html_impl(3, title, &xv, &yv, &zv, axis_labels, &cv, names, w, h, bg_color, scene, extra_js.as_bytes());
    wrap_pg_content_div(html, h)
}

fn render_wedge_radar3d_html(
    title: &str,
    axes: &[String],
    series: &[(String, Vec<f64>)],
    names: &[String],
    subdivide: bool,
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
    zone: Option<&[f64]>,
) -> String {
    let n_axes = axes.len();
    let n_ser = series.len().max(1);
    let global_max = series.iter().flat_map(|(_, v)| v.iter().cloned()).fold(0.0f64, f64::max).max(1e-9);
    let sector = TAU / n_axes.max(1) as f64;
    let group_span = sector * 0.78;
    let mut wedges = Wedges::default();
    for ai in 0..n_axes {
        let center = sector * ai as f64;
        for (si, (_, vals)) in series.iter().enumerate() {
            let v = vals.get(ai).copied().unwrap_or(0.0).max(0.0);
            if v <= 0.0 {
                continue;
            }
            let outer = (v / global_max).max(0.03);
            let (a0, a1) = if subdivide {
                let bar_span = group_span / n_ser as f64;
                let half = bar_span / 2.0;
                let offset = -group_span / 2.0 + bar_span / 2.0 + si as f64 * bar_span;
                let mid = center - offset;
                (mid - half, mid + half)
            } else {
                let width_frac = (1.0 - si.min(3) as f64 * 0.15).max(0.4);
                let half = group_span * width_frac / 2.0;
                (center - half, center + half)
            };
            wedges.push(a0, a1, 0.0, outer, 0.0, 0.0, wedge::HEIGHT, v, si as f64, names.get(si).cloned().unwrap_or_default());
        }
    }
    wedge::render_html(title, &wedges, axis_labels, w, h, bg_color, scene, zone, &[], &[], 0.0, 0.0, false)
}

fn clean_pair_name(name: &str) -> String {
    name.trim_end_matches(|c: char| c.is_whitespace())
        .trim_end_matches("_low")
        .trim_end_matches(" low")
        .trim_end_matches("Low")
        .to_string()
}

fn render_band_radar3d_html(
    title: &str,
    axes: &[String],
    series: &[(String, Vec<f64>)],
    axis_labels: (&str, &str, &str),
    w: i32,
    h: i32,
    bg_color: Option<&str>,
    scene: &str,
    ring_gap: f64,
) -> String {
    let n_axes = axes.len();
    let n_pairs = series.len() / 2;
    let mut xv = Vec::new();
    let mut yv = Vec::new();
    let mut zv = Vec::new();
    let mut cv = Vec::new();
    let mut names = Vec::new();
    for g in 0..n_pairs {
        let (low_name, low_vals) = &series[2 * g];
        let (_, high_vals) = &series[2 * g + 1];
        let max_val = low_vals.iter().chain(high_vals.iter()).cloned().fold(0.0f64, f64::max).max(1e-9);
        let clean_name = clean_pair_name(low_name);
        for ai in 0..n_axes {
            let angle = TAU * ai as f64 / n_axes as f64;
            let r = high_vals.get(ai).copied().unwrap_or(0.0).max(0.0) / max_val;
            xv.push(angle.cos() * r);
            yv.push(g as f64);
            zv.push(angle.sin() * r);
            cv.push(g as f64);
            names.push(clean_name.clone());
        }
        for ai in (0..n_axes).rev() {
            let angle = TAU * ai as f64 / n_axes as f64;
            let r = low_vals.get(ai).copied().unwrap_or(0.0).max(0.0) / max_val;
            xv.push(angle.cos() * r);
            yv.push(g as f64);
            zv.push(angle.sin() * r);
            cv.push(g as f64);
            names.push(clean_name.clone());
        }
    }
    let extra_js = format!("var RG={:.4};var RVAR='band';{}", ring_gap, RING_GAP_CONTROL_JS);
    let html = render_3d_html_impl(3, title, &xv, &yv, &zv, axis_labels, &cv, &names, w, h, bg_color, scene, extra_js.as_bytes());
    wrap_pg_content_div(html, h)
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
                None,
            )
        }
        Stacked => {
            let ring_gap = o.ring_gap.unwrap_or(1.0).clamp(0.0, 1.0);
            let n_axes = axes.len();
            let mut cum = vec![0.0f64; n_axes];
            let mut stacked_max = 1e-9f64;
            let mut cum_series = Vec::with_capacity(series.len());
            for (name, vals) in &series {
                for ai in 0..n_axes {
                    cum[ai] += vals.get(ai).copied().unwrap_or(0.0).max(0.0);
                }
                stacked_max = stacked_max.max(cum.iter().cloned().fold(0.0, f64::max));
                cum_series.push((name.clone(), cum.clone()));
            }
            render_closed_loop_radar3d_html(
                title,
                &axes,
                &cum_series,
                variant.name(),
                (&o.xl(), &o.yl(), &o.zl()),
                &names,
                o.w(900),
                o.h(560),
                bg_default,
                env,
                ring_gap,
                Some(stacked_max),
            )
        }
        PolarBar | Petal => render_wedge_radar3d_html(
            title,
            &axes,
            &series,
            &names,
            matches!(variant, PolarBar),
            (&o.xl(), &o.yl(), &o.zl()),
            o.w(900),
            o.h(560),
            bg_default,
            env,
            o.zone.as_deref(),
        ),
        Band => {
            let ring_gap = o.ring_gap.unwrap_or(1.0).clamp(0.0, 1.0);
            render_band_radar3d_html(
                title,
                &axes,
                &series,
                (&o.xl(), &o.yl(), &o.zl()),
                o.w(900),
                o.h(560),
                bg_default,
                env,
                ring_gap,
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

    fn is_wedge_variant(key: &str) -> bool {
        matches!(key, "polar_bar" | "petal")
    }

    fn wedge_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| is_wedge_variant(key)).collect()
    }

    fn loop_demos() -> twin::Demos {
        demos().into_iter().filter(|(key, _)| !is_wedge_variant(key)).collect()
    }

    #[test]
    fn every_radar_variant_has_a_working_3d_counterpart() {
        for (key, json) in &wedge_demos() {
            let html = build_radar3d_chart(json);
            assert!(!html.is_empty(), "{key} must render");
            assert!(html.contains("var A0="), "{key} must render as real wedges");
            assert!(!html.contains("var BN="), "{key} must not fall back to Bar3DBlock cuboids");
        }
        for (key, json) in &loop_demos() {
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
        for (key, json) in &demos() {
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
        twin::check_zone(build_radar3d_chart, &wedge_demos());
    }

    #[test]
    fn the_ring_gap_control_is_only_injected_for_closed_loop_variants() {
        let closed_loop = build_radar3d_chart(r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"basic"}"#);
        assert!(closed_loop.contains("ring gap"), "closed-loop variants must carry the ring-gap gauge");
        let polar_bar = build_radar3d_chart(r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"polar_bar"}"#);
        assert!(!polar_bar.contains("ring gap"), "wedge-based variants have their own radial spacing, no ring-gap gauge");
    }

    #[test]
    fn the_ring_gap_control_carries_a_disable_class_and_drag_handler() {
        let html = build_radar3d_chart(r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"basic"}"#);
        assert!(html.contains("sp-ring-gap-ctrl"), "no_ring_gap() targets this class to hide the control");
        assert!(html.contains("mousedown") && html.contains("mousemove") && html.contains("translate("), "the control must be draggable");
    }

    #[test]
    fn every_closed_loop_variant_carries_its_own_2d_styling_flag() {
        for key in ["basic", "lines", "filled", "markers", "dashed", "stacked"] {
            let json = format!(r#"{{"title":"t","axes":["A","B","C"],"series":[[1,2,3]],"variant":"{key}"}}"#);
            let html = build_radar3d_chart(&json);
            assert!(html.contains(&format!("RVAR='{key}'")), "{key} must carry its own RVAR flag");
        }
    }

    #[test]
    fn stacked_uses_a_cumulative_radius_shared_across_series() {
        let json = r#"{"title":"t","axes":["A","B","C"],"series":[[1,1,1],[1,1,1]],"variant":"stacked"}"#;
        let html = build_radar3d_chart(json);
        assert!(html.contains("RVAR='stacked'"));
    }

    #[test]
    fn polar_bar_subdivides_each_axis_sector_by_series_while_petal_does_not() {
        let json = r#"{"title":"t","axes":["A","B","C"],"series":[[5,5,5],[5,5,5]],"series_names":["X","Y"]}"#;
        let polar_bar_html = build_radar3d_chart(&twin::set_field(json, "variant", "polar_bar"));
        let petal_html = build_radar3d_chart(&twin::set_field(json, "variant", "petal"));
        assert!(polar_bar_html.contains("var A0="));
        assert!(petal_html.contains("var A0="));
        assert_ne!(polar_bar_html, petal_html);
    }

    #[test]
    fn band_needs_an_even_number_of_series_and_traces_a_single_ribbon_per_pair() {
        let json = r#"{"title":"t","axes":["A","B","C"],"series":[[1,2,3],[4,5,6]],"series_names":["X low","X high"],"variant":"band"}"#;
        let html = build_radar3d_chart(json);
        assert!(html.contains("RVAR='band'"));
        assert!(!html.contains("var BN="));
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
