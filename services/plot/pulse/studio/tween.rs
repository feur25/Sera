use std::collections::{HashMap, VecDeque};

fn is_attr_name_byte(b: u8) -> bool {
    b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b':'
}

fn numeric_attrs(tag: &str) -> Vec<(&str, usize, usize, f64)> {
    let bytes = tag.as_bytes();
    let mut out = Vec::new();
    let mut i = 0;
    while i + 1 < bytes.len() {
        if bytes[i] == b'=' && bytes[i + 1] == b'"' {
            let mut name_start = i;
            while name_start > 0 && is_attr_name_byte(bytes[name_start - 1]) {
                name_start -= 1;
            }
            let name = &tag[name_start..i];
            let value_start = i + 2;
            if let Some(rel_end) = tag[value_start..].find('"') {
                let value_end = value_start + rel_end;
                let value = &tag[value_start..value_end];
                if !name.is_empty() && !name.starts_with("data-") && name != "id" && name != "class" && name != "points" {
                    if let Ok(f) = value.parse::<f64>() {
                        out.push((name, value_start, value_end, f));
                    }
                }
                i = value_end + 1;
                continue;
            }
        }
        i += 1;
    }
    out
}

fn points_attr(tag: &str) -> Option<(usize, usize, Vec<(f64, f64)>)> {
    let key = "points=\"";
    let rel = tag.find(key)?;
    let value_start = rel + key.len();
    let rel_end = tag[value_start..].find('"')?;
    let value_end = value_start + rel_end;
    let raw = &tag[value_start..value_end];
    let mut pairs = Vec::new();
    for tok in raw.split_whitespace() {
        let (xs, ys) = tok.split_once(',')?;
        pairs.push((xs.parse::<f64>().ok()?, ys.parse::<f64>().ok()?));
    }
    Some((value_start, value_end, pairs))
}

fn tag_name(tag: &str) -> &str {
    let rest = tag.trim_start_matches('<');
    let end = rest.find(|c: char| c.is_whitespace() || c == '>' || c == '/').unwrap_or(rest.len());
    &rest[..end]
}

fn lerp_tag(to_tag: &str, from_tag: &str, t: f64) -> String {
    let from_numeric = numeric_attrs(from_tag);
    let mut edits: Vec<(usize, usize, String)> = Vec::new();

    for (name, v_start, v_end, to_val) in numeric_attrs(to_tag) {
        if let Some(&(_, _, _, from_val)) = from_numeric.iter().find(|(n, ..)| *n == name) {
            let lerped = from_val + (to_val - from_val) * t;
            edits.push((v_start, v_end, format!("{lerped:.3}")));
        }
    }

    if let Some((v_start, v_end, to_pairs)) = points_attr(to_tag) {
        if let Some((_, _, from_pairs)) = points_attr(from_tag) {
            if from_pairs.len() == to_pairs.len() && !to_pairs.is_empty() {
                let mut s = String::new();
                for (i, &(tx, ty)) in to_pairs.iter().enumerate() {
                    let (fx, fy) = from_pairs[i];
                    if i > 0 {
                        s.push(' ');
                    }
                    s.push_str(&format!("{:.3},{:.3}", fx + (tx - fx) * t, fy + (ty - fy) * t));
                }
                edits.push((v_start, v_end, s));
            }
        }
    }

    edits.sort_by_key(|(start, _, _)| *start);
    let mut out = String::with_capacity(to_tag.len());
    let mut cursor = 0usize;
    for (start, end, replacement) in edits {
        out.push_str(&to_tag[cursor..start]);
        out.push_str(&replacement);
        cursor = end;
    }
    out.push_str(&to_tag[cursor..]);
    out
}

fn immediate_child_span(svg: &str, after: usize) -> Option<(usize, usize)> {
    let rest = &svg[after..];
    let rel_start = rest.find('<')?;
    let start = after + rel_start;
    if svg[start..].starts_with("</") {
        return None;
    }
    let rel_end = svg[start..].find('>')?;
    Some((start, start + rel_end + 1))
}

fn find_elements_by_slot(svg: &str) -> HashMap<i64, Vec<(usize, usize)>> {
    let mut out: HashMap<i64, Vec<(usize, usize)>> = HashMap::new();
    let mut pos = 0usize;
    while let Some(rel) = svg[pos..].find("data-idx=\"") {
        let marker_start = pos + rel;
        let idx_val_start = marker_start + "data-idx=\"".len();
        let Some(rel_end) = svg[idx_val_start..].find('"') else { break };
        let idx_val_end = idx_val_start + rel_end;
        let parsed = svg[idx_val_start..idx_val_end].parse::<i64>().ok();
        let tag_start = svg[..marker_start].rfind('<').unwrap_or(marker_start);
        let Some(gt_rel) = svg[idx_val_end..].find('>') else { break };
        let tag_end = idx_val_end + gt_rel + 1;
        if let Some(slot) = parsed {
            out.entry(slot).or_default().push((tag_start, tag_end));
            if tag_name(&svg[tag_start..tag_end]) == "g" {
                if let Some(child_span) = immediate_child_span(svg, tag_end) {
                    out.entry(slot).or_default().push(child_span);
                }
            }
        }
        pos = tag_end;
    }
    out
}

pub(crate) fn tween_svg(from_svg: &str, to_svg: &str, from_order: &[u32], to_order: &[u32], t: f64) -> String {
    if t <= 0.0 {
        return from_svg.to_string();
    }
    if t >= 1.0 {
        return to_svg.to_string();
    }

    let mut from_by_original: HashMap<u32, VecDeque<(usize, usize)>> = HashMap::new();
    for (slot, spans) in find_elements_by_slot(from_svg) {
        if slot < 0 {
            continue;
        }
        if let Some(&orig) = from_order.get(slot as usize) {
            from_by_original.entry(orig).or_default().extend(spans);
        }
    }

    let mut to_entries: Vec<(i64, usize, usize)> =
        find_elements_by_slot(to_svg).into_iter().flat_map(|(slot, spans)| spans.into_iter().map(move |(s, e)| (slot, s, e))).collect();
    to_entries.sort_by_key(|&(_, start, _)| start);

    let mut out = String::with_capacity(to_svg.len());
    let mut cursor = 0usize;
    for (slot, tag_start, tag_end) in to_entries {
        if slot < 0 {
            continue;
        }
        let Some(&orig) = to_order.get(slot as usize) else { continue };
        let Some(queue) = from_by_original.get_mut(&orig) else { continue };
        let Some((f_start, f_end)) = queue.pop_front() else { continue };
        let to_tag = &to_svg[tag_start..tag_end];
        let from_tag = &from_svg[f_start..f_end];
        if tag_name(to_tag) != tag_name(from_tag) {
            continue;
        }
        out.push_str(&to_svg[cursor..tag_start]);
        out.push_str(&lerp_tag(to_tag, from_tag, t));
        cursor = tag_end;
    }
    out.push_str(&to_svg[cursor..]);
    out
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(crate) enum Ease {
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
}

impl Ease {
    pub(crate) fn parse(name: &str) -> Result<Ease, String> {
        match name {
            "linear" => Ok(Ease::Linear),
            "ease_in" => Ok(Ease::EaseIn),
            "ease_out" => Ok(Ease::EaseOut),
            "ease_in_out" => Ok(Ease::EaseInOut),
            other => Err(format!("unknown smooth_ease '{other}' - expected one of 'linear', 'ease_in', 'ease_out', 'ease_in_out'")),
        }
    }

    pub(crate) fn apply(self, t: f64) -> f64 {
        let t = t.clamp(0.0, 1.0);
        match self {
            Ease::Linear => t,
            Ease::EaseIn => t * t,
            Ease::EaseOut => 1.0 - (1.0 - t) * (1.0 - t),
            Ease::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    1.0 - (-2.0 * t + 2.0).powi(2) / 2.0
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn numeric_attrs_extracts_plain_numbers_and_skips_identity_and_style_attrs() {
        let tag = r##"<rect data-idx="3" x="10" y="20.5" width="30" fill="#ff0000" class="cell" id="7"/>"##;
        let attrs = numeric_attrs(tag);
        let names: Vec<&str> = attrs.iter().map(|(n, ..)| *n).collect();
        assert!(names.contains(&"x"));
        assert!(names.contains(&"y"));
        assert!(names.contains(&"width"));
        assert!(!names.contains(&"data-idx"), "data-idx must never be treated as a lerpable geometry attribute");
        assert!(!names.contains(&"id"));
        assert!(!names.contains(&"class"));
        assert!(!names.contains(&"fill"), "a hex color must not parse as f64 and must be excluded");
    }

    #[test]
    fn points_attr_parses_a_space_separated_coordinate_list() {
        let tag = r#"<polyline data-idx="0" points="1,2 3,4 5,6" fill="none"/>"#;
        let (_, _, pairs) = points_attr(tag).unwrap();
        assert_eq!(pairs, vec![(1.0, 2.0), (3.0, 4.0), (5.0, 6.0)]);
    }

    #[test]
    fn lerp_tag_produces_the_exact_midpoint_for_simple_numeric_attributes() {
        let from = r##"<rect data-idx="0" x="0" y="0" width="10" height="10" fill="#38bdf8"/>"##;
        let to = r##"<rect data-idx="0" x="100" y="50" width="10" height="10" fill="#38bdf8"/>"##;
        let mid = lerp_tag(to, from, 0.5);
        assert!(mid.contains("x=\"50.000\""), "got: {mid}");
        assert!(mid.contains("y=\"25.000\""), "got: {mid}");
        assert!(mid.contains("fill=\"#38bdf8\""), "non-numeric attributes must pass through unchanged, got: {mid}");
    }

    #[test]
    fn lerp_tag_at_t_zero_and_one_matches_the_respective_endpoint_value() {
        let from = r#"<rect data-idx="0" x="10"/>"#;
        let to = r#"<rect data-idx="0" x="90"/>"#;
        assert!(lerp_tag(to, from, 0.0).contains("x=\"10.000\""));
        assert!(lerp_tag(to, from, 1.0).contains("x=\"90.000\""));
    }

    #[test]
    fn lerp_tag_interpolates_a_points_list_pairwise_when_lengths_match() {
        let from = r#"<polyline data-idx="0" points="0,0 10,10"/>"#;
        let to = r#"<polyline data-idx="0" points="0,20 10,30"/>"#;
        let mid = lerp_tag(to, from, 0.5);
        assert!(mid.contains("points=\"0.000,10.000 10.000,20.000\""), "got: {mid}");
    }

    #[test]
    fn lerp_tag_leaves_points_untouched_when_point_counts_differ() {
        let from = r#"<polyline data-idx="0" points="0,0 10,10"/>"#;
        let to = r#"<polyline data-idx="0" points="0,0 5,5 10,10"/>"#;
        let mid = lerp_tag(to, from, 0.5);
        assert!(mid.contains("points=\"0,0 5,5 10,10\""), "mismatched point counts must fall back to the to-state unchanged, got: {mid}");
    }

    #[test]
    fn tween_svg_at_the_boundaries_returns_the_endpoint_svgs_unchanged() {
        let from = r#"<svg><rect data-idx="0" x="0"/></svg>"#;
        let to = r#"<svg><rect data-idx="0" x="100"/></svg>"#;
        assert_eq!(tween_svg(from, to, &[0], &[0], 0.0), from);
        assert_eq!(tween_svg(from, to, &[0], &[0], 1.0), to);
    }

    #[test]
    fn tween_svg_interpolates_a_matched_rect_at_the_midpoint() {
        let from = r#"<svg><rect data-idx="0" x="0" width="10"/></svg>"#;
        let to = r#"<svg><rect data-idx="0" x="100" width="10"/></svg>"#;
        let mid = tween_svg(from, to, &[0], &[0], 0.5);
        assert!(mid.contains("x=\"50.000\""), "got: {mid}");
        assert!(mid.starts_with("<svg>") && mid.ends_with("</svg>"), "surrounding structure must be preserved, got: {mid}");
    }

    #[test]
    fn tween_svg_descends_into_a_g_wrapped_marker_and_tweens_its_child() {
        let from = r#"<svg><g data-idx="0" data-x="1.0" data-y="2.0"><circle cx="0" cy="0" r="5"/></g></svg>"#;
        let to = r#"<svg><g data-idx="0" data-x="9.0" data-y="9.0"><circle cx="100" cy="200" r="5"/></g></svg>"#;
        let mid = tween_svg(from, to, &[0], &[0], 0.5);
        assert!(mid.contains("cx=\"50.000\""), "the g-wrapped circle's cx must be tweened, got: {mid}");
        assert!(mid.contains("cy=\"100.000\""), "the g-wrapped circle's cy must be tweened, got: {mid}");
    }

    #[test]
    fn tween_svg_correlates_elements_by_original_identity_not_raw_slot_across_a_reorder() {
        let from = r#"<svg><rect data-idx="0" x="0"/><rect data-idx="1" x="200"/></svg>"#;
        let to = r#"<svg><rect data-idx="0" x="300"/><rect data-idx="1" x="100"/></svg>"#;
        let from_order = [5u32, 7u32];
        let to_order = [7u32, 5u32];
        let mid = tween_svg(from, to, &from_order, &to_order, 0.5);
        assert!(mid.contains("data-idx=\"0\" x=\"250.000\""), "original idx 7 moved from slot 1 (x=200) in FROM to slot 0 (x=300) in TO, midpoint should be 250, got: {mid}");
        assert!(mid.contains("data-idx=\"1\" x=\"50.000\""), "original idx 5 moved from slot 0 (x=0) in FROM to slot 1 (x=100) in TO, midpoint should be 50, got: {mid}");
    }

    #[test]
    fn tween_svg_leaves_a_newly_appeared_element_untouched_when_it_has_no_prior_match() {
        let from = r#"<svg><rect data-idx="0" x="0"/></svg>"#;
        let to = r#"<svg><rect data-idx="0" x="10"/><rect data-idx="1" x="999"/></svg>"#;
        let mid = tween_svg(from, to, &[0], &[9, 1], 0.5);
        assert!(mid.contains("x=\"999\""), "an element with no matching original identity in FROM must pass through with its TO value unmodified, got: {mid}");
    }

    #[test]
    fn tween_svg_pairs_multiple_same_idx_elements_in_encounter_order() {
        let from = r#"<svg><rect data-idx="0" x="0"/><line data-idx="0" x1="0" x2="0"/></svg>"#;
        let to = r#"<svg><rect data-idx="0" x="100"/><line data-idx="0" x1="100" x2="100"/></svg>"#;
        let mid = tween_svg(from, to, &[0], &[0], 0.5);
        assert!(mid.contains("<rect data-idx=\"0\" x=\"50.000\""), "got: {mid}");
        assert!(mid.contains("x1=\"50.000\" x2=\"50.000\""), "got: {mid}");
    }

    fn synthetic_bar_svg(x_offset: f64) -> String {
        let mut svg = String::from(r#"<svg xmlns="http://www.w3.org/2000/svg" width="1280" height="720">"#);
        for i in 0..4 {
            svg.push_str(&format!(
                r##"<rect data-idx="{i}" x="{}" y="10" width="200" height="{}" fill="#38bdf8"/><text x="{}" y="700">Item {i}</text>"##,
                i as f64 * 250.0 + x_offset,
                100.0 + i as f64 * 30.0,
                i as f64 * 250.0
            ));
        }
        svg.push_str("</svg>");
        svg
    }

    #[test]
    fn perf_tween_svg_on_a_representative_four_bar_frame() {
        let from = synthetic_bar_svg(0.0);
        let to = synthetic_bar_svg(15.0);
        let order = [0u32, 1, 2, 3];

        let _ = tween_svg(&from, &to, &order, &order, 0.5);

        let iterations = 2000u32;
        let started = std::time::Instant::now();
        for _ in 0..iterations {
            let _ = tween_svg(&from, &to, &order, &order, 0.5);
        }
        let elapsed = started.elapsed();
        println!("tween_svg (4-element frame) x{iterations}: {elapsed:?} ({:?}/call)", elapsed / iterations);
        assert!(elapsed.as_millis() < 5_000, "tween_svg got implausibly slow: {elapsed:?} for {iterations} calls");
    }

    #[test]
    fn ease_parse_accepts_every_known_name_and_rejects_others() {
        assert_eq!(Ease::parse("linear").unwrap(), Ease::Linear);
        assert_eq!(Ease::parse("ease_in").unwrap(), Ease::EaseIn);
        assert_eq!(Ease::parse("ease_out").unwrap(), Ease::EaseOut);
        assert_eq!(Ease::parse("ease_in_out").unwrap(), Ease::EaseInOut);
        let err = Ease::parse("bounce").unwrap_err();
        assert!(err.contains("linear") && err.contains("ease_in_out"), "error should list the valid names, got: {err}");
    }

    #[test]
    fn every_ease_kind_maps_the_boundaries_to_zero_and_one() {
        for kind in [Ease::Linear, Ease::EaseIn, Ease::EaseOut, Ease::EaseInOut] {
            assert!((kind.apply(0.0) - 0.0).abs() < 1e-9, "{kind:?} at t=0 should be 0");
            assert!((kind.apply(1.0) - 1.0).abs() < 1e-9, "{kind:?} at t=1 should be 1");
        }
    }

    #[test]
    fn ease_in_starts_slower_than_linear_and_ease_out_starts_faster() {
        assert!(Ease::EaseIn.apply(0.25) < 0.25, "ease_in should lag behind linear early in the transition");
        assert!(Ease::EaseOut.apply(0.25) > 0.25, "ease_out should lead ahead of linear early in the transition");
    }

    #[test]
    fn every_ease_kind_is_monotonically_non_decreasing() {
        for kind in [Ease::Linear, Ease::EaseIn, Ease::EaseOut, Ease::EaseInOut] {
            let samples: Vec<f64> = (0..=20).map(|i| kind.apply(i as f64 / 20.0)).collect();
            for w in samples.windows(2) {
                assert!(w[1] >= w[0] - 1e-9, "{kind:?} must never move backwards, got {samples:?}");
            }
        }
    }
}
