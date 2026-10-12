use super::renderers::TouchedRow;
use crate::plot::push_registry::{self, PushMeta};
use image::RgbaImage;
use resvg::{tiny_skia, usvg};
use std::sync::{Arc, OnceLock};

enum AnimatedFields {
    Scalar(&'static str),
    Point(&'static str, &'static str),
    Vector(&'static [&'static str]),
    NestedVector(&'static str),
}

fn animated_fields_for(family: &str, scale: &PushMeta) -> Option<AnimatedFields> {
    match (family, scale) {
        ("build_bar" | "build_heatmap" | "build_icicle", PushMeta::Scalar(_)) => Some(AnimatedFields::Scalar("values")),
        ("build_line" | "build_line_chart" | "build_area_chart" | "build_scatter_chart" | "build_bubble", PushMeta::Point(_)) => Some(AnimatedFields::Point("x", "y")),
        ("build_line" | "build_line_chart", PushMeta::Vector(_)) => Some(AnimatedFields::NestedVector("series")),
        ("build_candlestick", PushMeta::Vector(_)) => Some(AnimatedFields::Vector(&["open", "high", "low", "close"])),
        _ => None,
    }
}

pub(super) fn invert_scalar(pixel: i16, min_val: f64, range_val: f64, axis_px: i32) -> f64 {
    min_val + (pixel as f64 / axis_px.max(1) as f64) * range_val
}

pub(super) fn invert_point(px: i16, py: i16, meta: &push_registry::PointMeta) -> (f64, f64) {
    let x = meta.min_x + (px as f64 / meta.plot_w.max(1) as f64) * meta.range_x;
    let y = meta.min_y + ((meta.plot_h as f64 - py as f64) / meta.plot_h.max(1) as f64) * meta.range_y;
    (x, y)
}

fn set_array_slot(root: &mut serde_json::Value, field: &str, index: usize, v: f64) {
    if let Some(slot) = root.get_mut(field).and_then(|arr| arr.as_array_mut()).and_then(|arr| arr.get_mut(index)) {
        *slot = serde_json::json!(v);
    }
}

fn anchor_len(root: &serde_json::Value, fields: &AnimatedFields) -> usize {
    let array_len = |field: &str| root.get(field).and_then(|v| v.as_array()).map(|a| a.len()).unwrap_or(0);
    match fields {
        AnimatedFields::Scalar(field) => array_len(field),
        AnimatedFields::Point(x_field, _) => array_len(x_field),
        AnimatedFields::Vector(component_fields) => array_len(component_fields[0]),
        AnimatedFields::NestedVector(field) => root
            .get(*field)
            .and_then(|v| v.as_array())
            .and_then(|arr| arr.first())
            .and_then(|inner| inner.as_array())
            .map(|a| a.len())
            .unwrap_or(0),
    }
}

fn compact_reorder_flat(arr: &[serde_json::Value], order: &[usize]) -> serde_json::Value {
    serde_json::Value::Array(order.iter().map(|&i| arr.get(i).cloned().unwrap_or(serde_json::Value::Null)).collect())
}

fn compact_and_reorder(root: &mut serde_json::Value, anchor_len: usize, order: &[usize]) {
    let Some(obj) = root.as_object_mut() else { return };
    for value in obj.values_mut() {
        let Some(arr) = value.as_array() else { continue };
        if arr.len() == anchor_len {
            *value = compact_reorder_flat(arr, order);
            continue;
        }
        if !arr.is_empty() && arr.iter().all(|inner| inner.as_array().is_some_and(|ia| ia.len() == anchor_len)) {
            let reordered_outer: Vec<serde_json::Value> = arr.iter().map(|inner| compact_reorder_flat(inner.as_array().unwrap(), order)).collect();
            *value = serde_json::Value::Array(reordered_outer);
        }
    }
}

fn write_animated(root: &mut serde_json::Value, fields: &AnimatedFields, scale: &PushMeta, slot: usize, values: &[i16]) -> Result<(), String> {
    match (fields, scale) {
        (AnimatedFields::Scalar(field), PushMeta::Scalar(meta)) => {
            let px = values.first().copied().unwrap_or(0);
            let v = invert_scalar(px, meta.min_val, meta.range_val, meta.axis_px);
            set_array_slot(root, field, slot, v);
        }
        (AnimatedFields::Point(x_field, y_field), PushMeta::Point(meta)) => {
            let (Some(&px), Some(&py)) = (values.first(), values.get(1)) else { return Ok(()) };
            let (x, y) = invert_point(px, py, meta);
            set_array_slot(root, x_field, slot, x);
            set_array_slot(root, y_field, slot, y);
        }
        (AnimatedFields::Vector(component_fields), PushMeta::Vector(meta)) => {
            for (k, field) in component_fields.iter().enumerate() {
                let Some(&px) = values.get(k) else { continue };
                let v = invert_scalar(px, meta.min_val, meta.range_val, meta.axis_px);
                set_array_slot(root, field, slot, v);
            }
        }
        (AnimatedFields::NestedVector(field), PushMeta::Vector(meta)) => {
            let Some(series) = root.get_mut(*field).and_then(|v| v.as_array_mut()) else { return Ok(()) };
            for (k, &px) in values.iter().enumerate() {
                let v = invert_scalar(px, meta.min_val, meta.range_val, meta.axis_px);
                if let Some(dest) = series.get_mut(k).and_then(|s| s.as_array_mut()).and_then(|s| s.get_mut(slot)) {
                    *dest = serde_json::json!(v);
                }
            }
        }
        _ => return Err("the recording's push shape does not match this family's animated-field mapping".to_string()),
    }
    Ok(())
}

fn stabilize_palette_by_identity(root: &mut serde_json::Value, order: &[usize]) {
    let mut distinct: Vec<usize> = order.to_vec();
    distinct.sort_unstable();
    distinct.dedup();
    let palette: Vec<serde_json::Value> = order
        .iter()
        .map(|idx| {
            let slot = distinct.iter().position(|d| d == idx).unwrap_or(0);
            serde_json::json!(crate::plot::statistical::common::palette_color(&[], slot))
        })
        .collect();
    if let Some(obj) = root.as_object_mut() {
        obj.insert("palette".to_string(), serde_json::Value::Array(palette));
    }
}

fn patch_json(json: &str, fields: &AnimatedFields, touched: &[TouchedRow], scale: &PushMeta, race: bool) -> Result<(String, Vec<u32>), String> {
    let mut root: serde_json::Value = serde_json::from_str(json).map_err(|e| format!("failed to parse recorded chart source: {e}"))?;
    let anchor_len = anchor_len(&root, fields);
    let order: Vec<u32>;
    if race {
        let raw_order: Vec<usize> = touched.iter().map(|(idx, _)| *idx as usize).collect();
        compact_and_reorder(&mut root, anchor_len, &raw_order);
        for (slot, (_, values)) in touched.iter().enumerate() {
            write_animated(&mut root, fields, scale, slot, values)?;
        }
        if matches!(fields, AnimatedFields::Scalar(_)) {
            stabilize_palette_by_identity(&mut root, &raw_order);
        }
        order = raw_order.into_iter().map(|i| i as u32).collect();
    } else {
        for (idx, values) in touched {
            write_animated(&mut root, fields, scale, *idx as usize, values)?;
        }
        order = (0..anchor_len as u32).collect();
    }
    Ok((root.to_string(), order))
}

fn invoke_builder(family: &str, json: &str) -> Option<String> {
    crate::bindings::fn_registry::iter_entries().find(|e| e.name == family).map(|e| (e.invoke)(json))
}

fn preceding_style_blocks(prefix: &str) -> String {
    let mut out = String::new();
    let mut rest = prefix;
    while let Some(open) = rest.find("<style") {
        let Some(tag_close) = rest[open..].find('>') else { break };
        let body_start = open + tag_close + 1;
        let Some(close_rel) = rest[body_start..].find("</style>") else { break };
        let body_end = body_start + close_rel + "</style>".len();
        out.push_str(&rest[open..body_end]);
        rest = &rest[body_end..];
    }
    out
}

fn extract_svg(html: &str) -> Option<String> {
    let start = html.find("<svg")?;
    let end = html.rfind("</svg>")? + 6;
    let open_tag_end = start + html[start..].find('>')? + 1;
    let styles = preceding_style_blocks(&html[..start]);
    let mut svg = String::with_capacity(end - start + styles.len());
    svg.push_str(&html[start..open_tag_end]);
    svg.push_str(&styles);
    svg.push_str(&html[open_tag_end..end]);
    Some(svg)
}

pub(crate) fn mask_svg_from_x_fraction(svg: &str, frac: f64) -> String {
    let frac = frac.clamp(0.0, 1.0);
    let Some(attr_start) = svg.find("data-sp=\"") else { return svg.to_string() };
    let rest = &svg[attr_start + "data-sp=\"".len()..];
    let Some(attr_end) = rest.find('"') else { return svg.to_string() };
    let parts: Vec<f64> = rest[..attr_end].split(',').filter_map(|s| s.parse::<f64>().ok()).collect();
    let [pad_l, pad_t, plot_w, plot_h] = parts[..] else { return svg.to_string() };
    let mask_x = pad_l + plot_w * frac;
    let mask_w = (pad_l + plot_w - mask_x).max(0.0);
    if mask_w <= 0.0 {
        return svg.to_string();
    }
    let Some(close) = svg.rfind("</svg>") else { return svg.to_string() };
    let mut out = String::with_capacity(svg.len() + 96);
    out.push_str(&svg[..close]);
    out.push_str(&format!("<rect x=\"{mask_x}\" y=\"{pad_t}\" width=\"{mask_w}\" height=\"{plot_h}\" fill=\"#ffffff\"/>"));
    out.push_str(&svg[close..]);
    out
}

fn shared_fontdb() -> Arc<usvg::fontdb::Database> {
    static FONTDB: OnceLock<Arc<usvg::fontdb::Database>> = OnceLock::new();
    FONTDB
        .get_or_init(|| {
            let mut db = usvg::fontdb::Database::new();
            db.load_system_fonts();
            Arc::new(db)
        })
        .clone()
}

pub(crate) fn rasterize(svg: &str, width: u32, height: u32) -> Result<RgbaImage, String> {
    let opts = usvg::Options { fontdb: shared_fontdb(), ..usvg::Options::default() };
    let tree = usvg::Tree::from_str(svg, &opts).map_err(|e| format!("replayed SVG failed to parse: {e}"))?;
    let mut pixmap = tiny_skia::Pixmap::new(width, height).ok_or_else(|| "invalid target pixmap size".to_string())?;
    pixmap.fill(tiny_skia::Color::WHITE);
    let size = tree.size();
    let sx = width as f32 / size.width().max(1.0);
    let sy = height as f32 / size.height().max(1.0);
    resvg::render(&tree, tiny_skia::Transform::from_scale(sx, sy), &mut pixmap.as_mut());
    RgbaImage::from_raw(width, height, pixmap.data().to_vec()).ok_or_else(|| "rasterized pixmap size mismatch".to_string())
}

pub(crate) struct ReplaySvg {
    pub svg: String,
    pub order: Vec<u32>,
}

pub(crate) fn render_svg(family: &str, json: &str, scale: &PushMeta, touched: &[TouchedRow], race: bool) -> Result<ReplaySvg, String> {
    let fields = animated_fields_for(family, scale).ok_or_else(|| format!("no animated-field mapping registered for family '{family}'"))?;
    let (patched, order) = patch_json(json, &fields, touched, scale, race)?;
    let html = invoke_builder(family, &patched).ok_or_else(|| format!("no registered builder found for family '{family}'"))?;
    let svg = extract_svg(&html).ok_or_else(|| "the replayed chart produced no SVG content".to_string())?;
    Ok(ReplaySvg { svg, order })
}

pub(crate) fn render_frame(family: &str, json: &str, scale: &PushMeta, touched: &[TouchedRow], width: u32, height: u32, race: bool) -> Result<RgbaImage, String> {
    let ReplaySvg { svg, .. } = render_svg(family, json, scale, touched, race)?;
    rasterize(&svg, width, height)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::push_registry::{PointMeta, ScalarMeta, VectorMeta};

    fn point_meta() -> PushMeta {
        PushMeta::Point(PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 400, plot_h: 300 })
    }

    fn scalar_meta() -> PushMeta {
        PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 })
    }

    fn vector_meta(vpp: u32) -> PushMeta {
        PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 100.0, axis_px: 100, vpp })
    }

    #[test]
    fn animated_fields_covers_every_currently_pushable_family() {
        for family in ["build_bar", "build_heatmap", "build_icicle"] {
            assert!(animated_fields_for(family, &scalar_meta()).is_some(), "family {family} should have a scalar animated-field mapping");
        }
        for family in ["build_line", "build_line_chart", "build_area_chart", "build_scatter_chart", "build_bubble"] {
            assert!(animated_fields_for(family, &point_meta()).is_some(), "family {family} should have a point animated-field mapping");
        }
        assert!(animated_fields_for("build_candlestick", &vector_meta(4)).is_some(), "candlestick should have a vector animated-field mapping");
    }

    #[test]
    fn animated_fields_covers_multi_series_line_when_pushed_as_a_vector() {
        for family in ["build_line", "build_line_chart"] {
            assert!(
                matches!(animated_fields_for(family, &vector_meta(3)), Some(AnimatedFields::NestedVector("series"))),
                "family {family} pushed with a vector scale should map to the nested-vector 'series' field, not fall back to point x/y"
            );
        }
    }

    #[test]
    fn animated_fields_is_none_for_an_unmapped_family() {
        assert!(animated_fields_for("build_correlogram", &point_meta()).is_none());
        assert!(animated_fields_for("build_sankey", &scalar_meta()).is_none());
    }

    #[test]
    fn animated_fields_is_none_when_the_family_and_push_shape_are_mismatched() {
        assert!(animated_fields_for("build_bar", &point_meta()).is_none(), "bar is scalar-shaped, a point scale should not match");
        assert!(animated_fields_for("build_scatter_chart", &scalar_meta()).is_none(), "scatter is point-shaped, a scalar scale should not match");
    }

    #[test]
    fn invert_scalar_round_trips_the_forward_pixel_formula() {
        let min_val = 10.0;
        let range_val = 90.0;
        let axis_px = 300;
        let value = 55.0;
        let px = ((value - min_val) / range_val * axis_px as f64) as i16;
        let back = invert_scalar(px, min_val, range_val, axis_px);
        assert!((back - value).abs() < 1.0, "expected ~{value}, got {back}");
    }

    #[test]
    fn invert_point_round_trips_the_forward_pixel_formulas() {
        let meta = PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 400, plot_h: 300 };
        let x = 42.0;
        let y = 71.0;
        let px = ((x - meta.min_x) / meta.range_x * meta.plot_w as f64) as i16;
        let py = (meta.plot_h as f64 - (y - meta.min_y) / meta.range_y * meta.plot_h as f64) as i16;
        let (back_x, back_y) = invert_point(px, py, &meta);
        assert!((back_x - x).abs() < 1.0, "expected x~{x}, got {back_x}");
        assert!((back_y - y).abs() < 1.0, "expected y~{y}, got {back_y}");
    }

    #[test]
    fn invert_point_extrapolates_a_y_value_pushed_beyond_the_originally_established_range() {
        let meta = PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 65.0, plot_w: 400, plot_h: 300 };
        let y = 96.5;
        let py = (meta.plot_h as f64 - (y - meta.min_y) / meta.range_y * meta.plot_h as f64) as i16;
        assert!(py < 0, "a y value beyond the established range must encode to a negative pixel, got {py}");
        let (_, back_y) = invert_point(0, py, &meta);
        assert!(
            (back_y - y).abs() < 1.0,
            "a point pushed beyond the chart's originally established y-range must extrapolate correctly to ~{y}, not clamp to the old max ({}), got {back_y}",
            meta.min_y + meta.range_y
        );
    }

    #[test]
    fn invert_point_extrapolates_an_x_value_pushed_below_the_originally_established_range() {
        let meta = PointMeta { min_x: 10.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 400, plot_h: 300 };
        let x = -5.0;
        let px = ((x - meta.min_x) / meta.range_x * meta.plot_w as f64) as i16;
        assert!(px < 0, "an x value below the established range must encode to a negative pixel, got {px}");
        let (back_x, _) = invert_point(px, 0, &meta);
        assert!(
            (back_x - x).abs() < 1.0,
            "a point pushed below the chart's originally established x-range must extrapolate correctly to ~{x}, not clamp to min_x ({}), got {back_x}",
            meta.min_x
        );
    }

    #[test]
    fn invert_scalar_extrapolates_a_value_pushed_below_the_originally_established_range() {
        let min_val = 10.0;
        let range_val = 90.0;
        let axis_px = 300;
        let value = -20.0;
        let px = ((value - min_val) / range_val * axis_px as f64) as i16;
        assert!(px < 0, "a value below min_val must encode to a negative pixel, got {px}");
        let back = invert_scalar(px, min_val, range_val, axis_px);
        assert!((back - value).abs() < 1.0, "a value pushed below the originally established range must extrapolate to ~{value}, not clamp to min_val, got {back}");
    }

    #[test]
    fn patch_json_updates_only_the_touched_scalar_indices() {
        let json = r#"{"title":"t","values":[1.0,2.0,3.0,4.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(1, vec![50])];
        let (patched, order) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, false).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        let arr = value["values"].as_array().unwrap();
        assert_eq!(arr[0].as_f64().unwrap(), 1.0, "untouched index 0 must survive unchanged");
        assert!((arr[1].as_f64().unwrap() - 50.0).abs() < 1.0, "touched index 1 should reflect the inverted pushed pixel value");
        assert_eq!(arr[2].as_f64().unwrap(), 3.0);
        assert_eq!(arr[3].as_f64().unwrap(), 4.0);
        assert_eq!(order, vec![0, 1, 2, 3], "non-race mode must report an identity slot-to-original-index mapping");
    }

    #[test]
    fn patch_json_updates_both_x_and_y_fields_for_point_shape() {
        let json = r#"{"x":[0.0,0.0],"y":[0.0,0.0]}"#;
        let scale = PushMeta::Point(PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 400, plot_h: 300 });
        let touched: Vec<TouchedRow> = vec![(0, vec![200, 150])];
        let (patched, _) = patch_json(json, &AnimatedFields::Point("x", "y"), &touched, &scale, false).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert!(value["x"][0].as_f64().unwrap() > 0.0);
        assert!(value["y"][0].as_f64().unwrap() > 0.0);
    }

    #[test]
    fn patch_json_updates_all_four_vector_component_fields() {
        let json = r#"{"open":[10.0],"high":[10.0],"low":[10.0],"close":[10.0]}"#;
        let scale = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 100.0, axis_px: 100, vpp: 4 });
        let touched: Vec<TouchedRow> = vec![(0, vec![20, 80, 10, 70])];
        let (patched, _) = patch_json(json, &AnimatedFields::Vector(&["open", "high", "low", "close"]), &touched, &scale, false).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert!((value["open"][0].as_f64().unwrap() - 20.0).abs() < 1.0);
        assert!((value["high"][0].as_f64().unwrap() - 80.0).abs() < 1.0);
        assert!((value["low"][0].as_f64().unwrap() - 10.0).abs() < 1.0);
        assert!((value["close"][0].as_f64().unwrap() - 70.0).abs() < 1.0);
    }

    #[test]
    fn patch_json_rejects_malformed_source_json_with_a_clear_error() {
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let err = patch_json("not json", &AnimatedFields::Scalar("values"), &[], &scale, false).unwrap_err();
        assert!(err.contains("failed to parse"));
    }

    #[test]
    fn patch_json_in_race_mode_compacts_to_only_the_touched_rows() {
        let json = r#"{"labels":["a","b","c","d","e"],"values":[1.0,2.0,3.0,4.0,5.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(3, vec![90]), (1, vec![40])];
        let (patched, order) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, true).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert_eq!(value["values"].as_array().unwrap().len(), 2, "race mode should compact down to only the racing rows");
        assert_eq!(value["labels"].as_array().unwrap().len(), 2, "labels must be compacted in lockstep with values");
        assert_eq!(order, vec![3, 1], "the order mapping must report which original index landed at each compacted slot");
    }

    #[test]
    fn patch_json_in_race_mode_assigns_each_row_a_color_tied_to_its_original_identity() {
        let json = r#"{"labels":["a","b","c"],"values":[0.0,0.0,0.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });

        let touched_before: Vec<TouchedRow> = vec![(0, vec![10]), (2, vec![90])];
        let (patched_before, _) = patch_json(json, &AnimatedFields::Scalar("values"), &touched_before, &scale, true).unwrap();
        let before: serde_json::Value = serde_json::from_str(&patched_before).unwrap();

        let touched_after: Vec<TouchedRow> = vec![(2, vec![95]), (0, vec![20])];
        let (patched_after, _) = patch_json(json, &AnimatedFields::Scalar("values"), &touched_after, &scale, true).unwrap();
        let after: serde_json::Value = serde_json::from_str(&patched_after).unwrap();

        let color_of = |root: &serde_json::Value, slot: usize| root["palette"][slot].as_u64().unwrap();
        assert_eq!(color_of(&before, 0), color_of(&after, 1), "original index 0 must keep the same color whether it is at slot 0 or slot 1");
        assert_eq!(color_of(&before, 1), color_of(&after, 0), "original index 2 must keep the same color whether it is at slot 1 or slot 0");
    }

    #[test]
    fn stabilize_palette_by_identity_never_collides_for_up_to_ten_simultaneous_racers() {
        let json = r#"{"labels":["a","b","c","d","e","f"],"values":[0.0,0.0,0.0,0.0,0.0,0.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let indices = [15u32, 120, 260, 400, 510, 45];
        let touched: Vec<TouchedRow> = indices.iter().map(|&i| (i, vec![50])).collect();
        let (patched, _) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, true).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        let colors: std::collections::HashSet<u64> = value["palette"].as_array().unwrap().iter().map(|v| v.as_u64().unwrap()).collect();
        assert_eq!(
            colors.len(),
            indices.len(),
            "evenly-spaced round-number indices (a natural choice for demo/race data, and the exact set that collided under a plain idx % palette_len scheme) must all get distinct colors - got {} distinct colors for {} indices",
            colors.len(),
            indices.len()
        );
    }

    #[test]
    fn stabilize_palette_by_identity_is_stable_regardless_of_the_order_field_appears_in() {
        let json = r#"{"labels":["a","b","c"],"values":[0.0,0.0,0.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });

        let forward: Vec<TouchedRow> = vec![(3, vec![50]), (7, vec![50])];
        let (patched_forward, _) = patch_json(json, &AnimatedFields::Scalar("values"), &forward, &scale, true).unwrap();
        let forward_colors: Vec<u64> = serde_json::from_str::<serde_json::Value>(&patched_forward).unwrap()["palette"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();

        let backward: Vec<TouchedRow> = vec![(7, vec![50]), (3, vec![50])];
        let (patched_backward, _) = patch_json(json, &AnimatedFields::Scalar("values"), &backward, &scale, true).unwrap();
        let backward_colors: Vec<u64> = serde_json::from_str::<serde_json::Value>(&patched_backward).unwrap()["palette"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect();

        assert_eq!(forward_colors[0], backward_colors[1], "idx 3's color must be the same regardless of which rank slot it currently occupies");
        assert_eq!(forward_colors[1], backward_colors[0], "idx 7's color must be the same regardless of which rank slot it currently occupies");
    }

    #[test]
    fn patch_json_in_non_race_mode_does_not_inject_a_palette_override() {
        let json = r#"{"labels":["a","b"],"values":[1.0,2.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(0, vec![50])];
        let (patched, _) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, false).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert!(value.get("palette").is_none(), "stable (non-race) mode never reorders slots, so no palette override is needed");
    }

    #[test]
    fn patch_json_in_race_mode_carries_the_original_label_with_its_row_in_rank_order() {
        let json = r#"{"labels":["Item 0","Item 1","Item 2"],"values":[0.0,0.0,0.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(2, vec![90]), (0, vec![10])];
        let (patched, order) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, true).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert_eq!(value["labels"][0].as_str().unwrap(), "Item 2", "the rank-1 slot must carry the label that originally belonged to index 2");
        assert_eq!(value["labels"][1].as_str().unwrap(), "Item 0", "the rank-2 slot must carry the label that originally belonged to index 0");
        assert!((value["values"][0].as_f64().unwrap() - 90.0).abs() < 1.0);
        assert!((value["values"][1].as_f64().unwrap() - 10.0).abs() < 1.0);
        assert_eq!(order, vec![2, 0], "order[slot] must be the original index now occupying that slot");
    }

    #[test]
    fn patch_json_in_race_mode_leaves_unrelated_length_fields_untouched() {
        let json = r#"{"labels":["a","b","c"],"values":[1.0,2.0,3.0],"col_labels":["x","y"]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(0, vec![50])];
        let (patched, _) = patch_json(json, &AnimatedFields::Scalar("values"), &touched, &scale, true).unwrap();
        let value: serde_json::Value = serde_json::from_str(&patched).unwrap();
        assert_eq!(value["col_labels"].as_array().unwrap().len(), 2, "a field whose length does not match the anchor array must not be reordered or resized");
    }

    #[test]
    fn extract_svg_finds_the_svg_substring_and_ignores_surrounding_html() {
        let html = "<html><body><div>x</div><svg width=\"1\"><rect/></svg></body></html>";
        let svg = extract_svg(html).unwrap();
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn extract_svg_returns_none_when_there_is_no_svg_tag() {
        assert!(extract_svg("<html><body>no svg here</body></html>").is_none());
    }

    const MASK_TEST_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" data-sp="50,40,700,360"><rect class="sp-bg" width="100%" height="100%"/><polyline points="50,200 750,80"/></svg>"#;

    #[test]
    fn mask_svg_from_x_fraction_at_zero_covers_the_entire_plot_width() {
        let masked = mask_svg_from_x_fraction(MASK_TEST_SVG, 0.0);
        assert!(masked.contains("x=\"50\""), "at frac=0 the mask should start exactly at the plot's left edge: {masked}");
        assert!(masked.contains("width=\"700\""), "at frac=0 the mask should cover the entire plot width: {masked}");
    }

    #[test]
    fn mask_svg_from_x_fraction_at_one_adds_no_mask_since_everything_is_revealed() {
        let masked = mask_svg_from_x_fraction(MASK_TEST_SVG, 1.0);
        assert_eq!(masked, MASK_TEST_SVG, "a fully-revealed frame should be returned unchanged, with no masking rect appended");
    }

    #[test]
    fn mask_svg_from_x_fraction_at_half_covers_the_right_half_of_the_plot() {
        let masked = mask_svg_from_x_fraction(MASK_TEST_SVG, 0.5);
        assert!(masked.contains("x=\"400\""), "at frac=0.5 over a plot starting at x=50 spanning 700px, the mask should start at x=400: {masked}");
        assert!(masked.contains("width=\"350\""), "at frac=0.5 the mask should cover the remaining half of the plot width: {masked}");
    }

    #[test]
    fn mask_svg_from_x_fraction_inserts_the_mask_immediately_before_the_closing_svg_tag() {
        let masked = mask_svg_from_x_fraction(MASK_TEST_SVG, 0.5);
        let rect_pos = masked.find("fill=\"#ffffff\"").expect("mask rect should be present");
        let close_pos = masked.rfind("</svg>").expect("closing svg tag should be present");
        assert!(rect_pos < close_pos, "the mask rect must be drawn on top of, meaning after, all existing chart content: {masked}");
    }

    #[test]
    fn mask_svg_from_x_fraction_leaves_svg_unchanged_when_it_has_no_data_sp_attribute() {
        let svg = "<svg xmlns=\"http://www.w3.org/2000/svg\"><rect/></svg>";
        assert_eq!(mask_svg_from_x_fraction(svg, 0.5), svg, "a chart family whose SVG never exposes plot-area bounds must degrade gracefully to an unmasked frame, not panic or corrupt the SVG");
    }

    #[test]
    fn mask_svg_from_x_fraction_clamps_an_out_of_range_fraction() {
        let low = mask_svg_from_x_fraction(MASK_TEST_SVG, -0.5);
        let high = mask_svg_from_x_fraction(MASK_TEST_SVG, 1.5);
        assert!(low.contains("x=\"50\"") && low.contains("width=\"700\""), "a negative fraction should clamp to 0.0 (full mask): {low}");
        assert_eq!(high, MASK_TEST_SVG, "a fraction above 1.0 should clamp to 1.0 (no mask)");
    }

    #[test]
    fn extract_svg_carries_preceding_style_blocks_into_the_extracted_fragment() {
        let html = r#"<html><head><style>.sp-bg{fill:#112233}</style></head><body><svg width="1"><rect class="sp-bg"/></svg></body></html>"#;
        let svg = extract_svg(html).unwrap();
        assert!(svg.contains(".sp-bg{fill:#112233}"), "a style block defined before the svg tag must be spliced into the extracted fragment, got: {svg}");
        assert!(svg.starts_with("<svg"));
        assert!(svg.ends_with("</svg>"));
    }

    #[test]
    fn extract_svg_carries_multiple_preceding_style_blocks_in_order() {
        let html = r#"<head><style>.a{fill:#111111}</style><style>.b{fill:#222222}</style></head><svg width="1"><rect/></svg>"#;
        let svg = extract_svg(html).unwrap();
        let pos_a = svg.find(".a{fill:#111111}").expect("first style block must be present");
        let pos_b = svg.find(".b{fill:#222222}").expect("second style block must be present");
        assert!(pos_a < pos_b, "style blocks must be carried over in their original order");
    }

    #[test]
    fn rasterize_produces_a_pixmap_of_the_requested_dimensions() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100" viewBox="0 0 200 100"><rect x="0" y="0" width="200" height="100" fill="#ffffff"/><rect x="10" y="10" width="50" height="50" fill="#ff0000"/></svg>"##;
        let img = rasterize(svg, 400, 200).expect("should rasterize");
        assert_eq!((img.width(), img.height()), (400, 200));
    }

    #[test]
    fn rasterize_resolves_a_class_based_fill_when_its_style_block_was_spliced_in_by_extract_svg() {
        let html = r##"<html><head><style>.sp-bg{fill:#224488}</style></head><body><svg xmlns="http://www.w3.org/2000/svg" width="40" height="40" viewBox="0 0 40 40"><rect class="sp-bg" width="40" height="40"/></svg></body></html>"##;
        let svg = extract_svg(html).unwrap();
        let img = rasterize(&svg, 40, 40).expect("should rasterize");
        let center = img.get_pixel(20, 20);
        assert_eq!((center.0[0], center.0[1], center.0[2]), (0x22, 0x44, 0x88), "the sp-bg class fill must resolve via the spliced-in style block instead of falling back to SVG's default black fill, got {:?}", center.0);
    }

    #[test]
    fn shared_fontdb_is_loaded_once_and_reused_across_calls() {
        let a = shared_fontdb();
        let b = shared_fontdb();
        assert!(Arc::ptr_eq(&a, &b), "the font database must be cached and shared, not rescanned from disk on every frame");
    }

    #[test]
    fn rasterize_of_a_real_svg_actually_paints_the_expected_color() {
        let svg = r##"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100" viewBox="0 0 100 100"><rect x="0" y="0" width="100" height="100" fill="#ffffff"/><rect x="20" y="20" width="60" height="60" fill="#ff0000"/></svg>"##;
        let img = rasterize(svg, 100, 100).expect("should rasterize");
        let center = img.get_pixel(50, 50);
        assert!(center.0[0] > 200 && center.0[1] < 60 && center.0[2] < 60, "center pixel should be red, got {:?}", center.0);
    }

    #[test]
    fn render_frame_end_to_end_produces_a_family_correct_bar_svg_bitmap() {
        let json = r#"{"title":"t","labels":["a","b","c"],"values":[10.0,10.0,10.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(1, vec![90])];
        let img = render_frame("build_bar", json, &scale, &touched, 400, 300, false).expect("bar replay should succeed");
        assert_eq!((img.width(), img.height()), (400, 300));
        let corner = img.get_pixel(2, 2);
        assert!(
            corner.0[0] > 200 && corner.0[1] > 200 && corner.0[2] > 200,
            "the chart's real .sp-bg background must render near-white, not fall back to SVG's default black fill when its defining style block lives outside <svg>, got {:?}",
            corner.0
        );
    }

    #[test]
    fn render_frame_end_to_end_produces_a_family_correct_connected_scatter_line_bitmap() {
        let json = r#"{"title":"t","x":[0.0,1.0,2.0,3.0],"y":[5.0,9.0,3.0,7.0],"variant":"connected_scatter","show_points":true}"#;
        let scale = PushMeta::Point(PointMeta { min_x: 0.0, range_x: 3.0, min_y: 0.0, range_y: 10.0, plot_w: 400, plot_h: 300 });
        let touched: Vec<TouchedRow> = vec![(2, vec![200, 100])];
        let img = render_frame("build_line", json, &scale, &touched, 400, 300, false).expect("connected_scatter line replay should succeed");
        assert_eq!((img.width(), img.height()), (400, 300));
        let non_white = img.pixels().filter(|p| !(p.0[0] > 250 && p.0[1] > 250 && p.0[2] > 250)).count();
        assert!(
            non_white > 100,
            "a connected_scatter line chart constructed from x/y and replayed with a patched touched row must paint real line/marker pixels, not render as an empty chart, got {non_white} non-white pixels"
        );
    }

    #[test]
    fn render_frame_end_to_end_produces_a_family_correct_area_chart_bitmap_from_x_y() {
        let json = r#"{"title":"t","x":[0.0,1.0,2.0,3.0],"y":[5.0,9.0,3.0,7.0]}"#;
        let scale = PushMeta::Point(PointMeta { min_x: 0.0, range_x: 3.0, min_y: 0.0, range_y: 10.0, plot_w: 400, plot_h: 300 });
        let touched: Vec<TouchedRow> = vec![(2, vec![200, 100])];
        let img = render_frame("build_area_chart", json, &scale, &touched, 400, 300, false).expect("area replay from x/y should succeed");
        assert_eq!((img.width(), img.height()), (400, 300));
        let non_white = img.pixels().filter(|p| !(p.0[0] > 250 && p.0[1] > 250 && p.0[2] > 250)).count();
        assert!(
            non_white > 100,
            "an area chart constructed from x/y and replayed with a patched touched row must paint real fill/line pixels, not render as an empty chart, got {non_white} non-white pixels"
        );
    }

    #[test]
    fn render_frame_end_to_end_produces_a_family_correct_multi_series_line_bitmap_from_pushed_vectors() {
        let json = r#"{"title":"t","x_labels":["2020","2021","2022"],"series":[[10.0,20.0,15.0],[5.0,8.0,12.0]],"series_names":["A","B"],"variant":"multi"}"#;
        let scale = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 30.0, axis_px: 300, vpp: 2 });
        let touched: Vec<TouchedRow> = vec![(1, vec![250, 100])];
        let img = render_frame("build_line", json, &scale, &touched, 400, 300, false).expect("multi-series line replay should succeed");
        assert_eq!((img.width(), img.height()), (400, 300));
        let non_white = img.pixels().filter(|p| !(p.0[0] > 250 && p.0[1] > 250 && p.0[2] > 250)).count();
        assert!(
            non_white > 100,
            "a multi-series line chart replayed from a pushed vector touched row must paint real per-series line pixels, not render as an empty chart, got {non_white} non-white pixels"
        );
    }

    #[test]
    fn render_svg_for_a_multi_series_line_carries_a_visible_legend_naming_each_series() {
        let json = r#"{"title":"t","x_labels":["2020","2021","2022"],"series":[[10.0,20.0,15.0],[5.0,8.0,12.0]],"series_names":["Nord","Sud"],"variant":"multi"}"#;
        let scale = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 30.0, axis_px: 300, vpp: 2 });
        let touched: Vec<TouchedRow> = vec![(1, vec![250, 100])];
        let svg = render_svg("build_line", json, &scale, &touched, false).expect("multi-series line replay should succeed").svg;
        assert!(svg.contains("Nord") && svg.contains("Sud"), "a replayed multi-series legend must still name every series: {svg}");
        assert!(
            !svg.contains("data-legend=\"1\" style=\"display:none\""),
            "a replayed chart's legend items must not be display:none - the click-to-toggle JS that would normally reveal them never runs during headless GIF/video rasterization: {svg}"
        );
    }

    #[test]
    fn write_animated_updates_only_the_touched_series_component_leaving_the_other_series_untouched() {
        let json = r#"{"title":"t","x_labels":["2020","2021"],"series":[[10.0,20.0],[5.0,8.0]]}"#;
        let mut root: serde_json::Value = serde_json::from_str(json).unwrap();
        let scale = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 100.0, axis_px: 100, vpp: 2 });
        write_animated(&mut root, &AnimatedFields::NestedVector("series"), &scale, 0, &[90, 10]).unwrap();
        let series = root["series"].as_array().unwrap();
        assert!((series[0][0].as_f64().unwrap() - 90.0).abs() < 1.0, "the touched series-0 value at slot 0 should update");
        assert!((series[1][0].as_f64().unwrap() - 10.0).abs() < 1.0, "the touched series-1 value at slot 0 should update");
        assert_eq!(series[0][1].as_f64().unwrap(), 20.0, "the untouched slot 1 of series 0 must survive unchanged");
        assert_eq!(series[1][1].as_f64().unwrap(), 8.0, "the untouched slot 1 of series 1 must survive unchanged");
    }

    #[test]
    fn render_frame_rejects_a_family_with_no_animated_field_mapping() {
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let err = render_frame("build_sankey", "{}", &scale, &[], 100, 100, false).unwrap_err();
        assert!(err.contains("no animated-field mapping"));
    }

    #[test]
    fn render_frame_in_race_mode_produces_a_compact_family_correct_bitmap() {
        let n = 20;
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let values = vec![0.0; n];
        let json = serde_json::json!({"title": "t", "labels": labels, "values": values}).to_string();
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(15, vec![90]), (3, vec![60]), (9, vec![30])];
        let img = render_frame("build_bar", &json, &scale, &touched, 400, 300, true).expect("race-mode bar replay should succeed");
        assert_eq!((img.width(), img.height()), (400, 300));
    }

    #[test]
    fn perf_cold_system_font_scan_shows_the_cost_the_fix_now_pays_only_once() {
        let started = std::time::Instant::now();
        let mut db = usvg::fontdb::Database::new();
        db.load_system_fonts();
        let elapsed = started.elapsed();
        println!("cold system font scan (paid once per process now, was paid every frame before the fix): {elapsed:?}, {} faces", db.len());
        assert!(elapsed.as_millis() < 10_000, "a cold font scan took implausibly long: {elapsed:?}");
    }

    #[test]
    fn perf_replay_render_frame_on_a_600_row_bar_source_race_mode() {
        let n = 600;
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let values: Vec<f64> = (0..n).map(|i| (i % 100) as f64).collect();
        let json = serde_json::json!({"title": "SeraStudio 60fps demo - bar chart race (pictogram variant)", "labels": labels, "values": values, "variant": "pictogram"}).to_string();
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(15, vec![90]), (120, vec![70]), (260, vec![60]), (400, vec![50]), (45, vec![80]), (510, vec![40])];

        render_frame("build_bar", &json, &scale, &touched, 1280, 720, true).expect("warm-up call should succeed");

        let iterations = 10u32;
        let started = std::time::Instant::now();
        for _ in 0..iterations {
            render_frame("build_bar", &json, &scale, &touched, 1280, 720, true).expect("replay should succeed");
        }
        let elapsed = started.elapsed();
        println!(
            "replay render_frame (race mode, 600-row pictogram bar source, 1280x720) x{iterations}: {elapsed:?} ({:?}/call)",
            elapsed / iterations
        );
        assert!(elapsed.as_millis() < 60_000, "replay render_frame got pathologically slow: {elapsed:?} for {iterations} calls (bound is loose to tolerate an unoptimized debug build)");
    }

    #[test]
    fn perf_replay_render_frame_on_a_2500_cell_heatmap_source() {
        let size = 50;
        let labels: Vec<String> = (0..size).map(|i| format!("V{i}")).collect();
        let values: Vec<f64> = (0..size * size).map(|i| ((i * 37 + 11) % 100) as f64 / 100.0).collect();
        let json = serde_json::json!({
            "title": "SeraStudio 60fps demo - correlogram-style heatmap",
            "labels": labels, "col_labels": labels, "values": values,
            "color_low": 0xF5F3FF, "color_mid": 0xA78BFA, "color_high": 0x4C1D95
        })
        .to_string();
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 1.0, axis_px: 100 });
        let touched: Vec<TouchedRow> = vec![(200, vec![50]), (850, vec![70]), (1400, vec![30]), (1900, vec![90]), (90, vec![10]), (2340, vec![60])];

        render_frame("build_heatmap", &json, &scale, &touched, 900, 900, false).expect("warm-up call should succeed");

        let iterations = 5u32;
        let started = std::time::Instant::now();
        for _ in 0..iterations {
            render_frame("build_heatmap", &json, &scale, &touched, 900, 900, false).expect("replay should succeed");
        }
        let elapsed = started.elapsed();
        println!("replay render_frame (2500-cell heatmap source, 900x900) x{iterations}: {elapsed:?} ({:?}/call)", elapsed / iterations);
        assert!(elapsed.as_millis() < 60_000, "replay render_frame got pathologically slow: {elapsed:?} for {iterations} calls (bound is loose to tolerate an unoptimized debug build)");
    }

    #[test]
    fn perf_replay_render_frame_on_a_3200_point_connected_scatter_line_source_full_cloud() {
        let n = 3200;
        let x: Vec<f64> = (0..n).map(|i| 2000.0 + i as f64 * (24.0 / n as f64)).collect();
        let y: Vec<f64> = (0..n).map(|i| ((i * 37 + 11) % 100) as f64).collect();
        let json = serde_json::json!({"title": "t", "x": x, "y": y, "variant": "connected_scatter", "show_points": true}).to_string();
        let scale = PushMeta::Point(PointMeta { min_x: 2000.0, range_x: 24.0, min_y: 0.0, range_y: 100.0, plot_w: 1000, plot_h: 750 });
        let touched: Vec<TouchedRow> = vec![(1600, vec![500, 300])];

        render_frame("build_line", &json, &scale, &touched, 1000, 750, false).expect("warm-up call should succeed");

        let iterations = 5u32;
        let started = std::time::Instant::now();
        for _ in 0..iterations {
            render_frame("build_line", &json, &scale, &touched, 1000, 750, false).expect("replay should succeed");
        }
        let elapsed = started.elapsed();
        println!("replay render_frame (3200-point connected_scatter line source, 1000x750) x{iterations}: {elapsed:?} ({:?}/call)", elapsed / iterations);
        assert!(elapsed.as_millis() < 60_000, "replay render_frame got pathologically slow: {elapsed:?} for {iterations} calls (bound is loose to tolerate an unoptimized debug build)");
    }

    #[test]
    fn perf_replay_render_frame_on_a_3200_point_scatter_regression_source_full_cloud() {
        let n = 3200;
        let x: Vec<f64> = (0..n).map(|i| (i % 100) as f64).collect();
        let y: Vec<f64> = (0..n).map(|i| 0.6 * (i % 100) as f64 + (i % 7) as f64).collect();
        let json = serde_json::json!({
            "title": "SeraStudio smooth demo - regression with orbiting points",
            "x": x, "y": y, "variant": "regression"
        })
        .to_string();
        let scale = PushMeta::Point(PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 70.0, plot_w: 800, plot_h: 600 });
        let touched: Vec<TouchedRow> = (0..n as u32).map(|i| (i, vec![(i % 800) as i16, (i % 600) as i16])).collect();

        render_frame("build_scatter_chart", &json, &scale, &touched, 1000, 750, false).expect("warm-up call should succeed");

        let iterations = 3u32;
        let started = std::time::Instant::now();
        for _ in 0..iterations {
            render_frame("build_scatter_chart", &json, &scale, &touched, 1000, 750, false).expect("replay should succeed");
        }
        let elapsed = started.elapsed();
        println!(
            "replay render_frame (3200-point scatter regression source, full cloud touched, 1000x750) x{iterations}: {elapsed:?} ({:?}/call)",
            elapsed / iterations
        );
        assert!(elapsed.as_millis() < 120_000, "replay render_frame got pathologically slow: {elapsed:?} for {iterations} calls (bound is loose to tolerate an unoptimized debug build)");
    }
}
