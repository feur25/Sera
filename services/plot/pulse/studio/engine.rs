use super::renderers::{render_frame, RenderKind, TouchedRow};
use crate::plot::push_registry::PushMeta;
use crate::plot::record_registry::Recording;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, RgbaImage};
use std::time::Duration;

const MIN_FRAME_DELAY_MS: u64 = 20;
const DEFAULT_SMOOTH_TICK_MS: u32 = 20;
const MAX_SMOOTH_STEPS: u32 = 12;
const DEFAULT_REVEAL_STEPS: u32 = 120;

pub(crate) enum BreakpointTrigger {
    AtMs(u32),
    ValueAtLeast(u32, f64),
}

pub(crate) struct Breakpoint {
    pub trigger: BreakpointTrigger,
    pub pause_ms: u32,
}

pub(crate) struct GifOptions {
    pub width: u32,
    pub height: u32,
    pub max_frames: usize,
    pub color: (u8, u8, u8),
    pub sort_by_value: bool,
    pub smooth: bool,
    pub smooth_tick_ms: u32,
    pub smooth_speed: f64,
    pub smooth_max_steps: u32,
    pub smooth_ease: String,
    pub reveal: bool,
    pub reveal_steps: u32,
    pub reveal_fixed_scale: bool,
    pub breakpoints: Vec<Breakpoint>,
}

impl Default for GifOptions {
    fn default() -> Self {
        Self {
            width: 800,
            height: 450,
            max_frames: 600,
            color: super::canvas::DEFAULT_RGB,
            sort_by_value: false,
            smooth: false,
            smooth_tick_ms: DEFAULT_SMOOTH_TICK_MS,
            smooth_speed: 1.0,
            smooth_max_steps: MAX_SMOOTH_STEPS,
            smooth_ease: "linear".to_string(),
            reveal: false,
            reveal_steps: DEFAULT_REVEAL_STEPS,
            reveal_fixed_scale: false,
            breakpoints: Vec::new(),
        }
    }
}

fn ranked(mut rows: Vec<TouchedRow>, sort_by_value: bool) -> Vec<TouchedRow> {
    if sort_by_value {
        rows.sort_by(|a, b| {
            let va = a.1.first().copied().unwrap_or(0);
            let vb = b.1.first().copied().unwrap_or(0);
            vb.cmp(&va)
        });
    }
    rows
}

fn interpolate_touched_rows(from: &[TouchedRow], to: &[TouchedRow], t: f64) -> Vec<TouchedRow> {
    to.iter()
        .map(|(idx, to_row)| {
            let from_row = from.iter().find(|(i, _)| i == idx).map(|(_, r)| r.as_slice());
            let row = to_row
                .iter()
                .enumerate()
                .map(|(i, &tv)| {
                    let fv = from_row.and_then(|r| r.get(i).copied()).unwrap_or(0) as f64;
                    (fv + (tv as f64 - fv) * t).round() as i16
                })
                .collect();
            (*idx, row)
        })
        .collect()
}

fn tween_steps(gap: u32, opts: &GifOptions) -> u32 {
    let effective_tick_ms = (opts.smooth_tick_ms.max(1) as f64 * opts.smooth_speed.max(0.01)).max(1.0);
    ((gap as f64 / effective_tick_ms).round() as u32).clamp(1, opts.smooth_max_steps.max(1))
}

fn render_kind(scale: &PushMeta) -> RenderKind {
    match scale {
        PushMeta::Scalar(s) => RenderKind::Scalar { axis_px: s.axis_px },
        PushMeta::Point(p) => RenderKind::Point { plot_w: p.plot_w, plot_h: p.plot_h },
        PushMeta::Vector(v) => RenderKind::Vector { axis_px: v.axis_px, vpp: v.vpp },
    }
}

fn upsert(index: &mut Vec<u32>, values: &mut Vec<Vec<i16>>, idx: u32, row: Vec<i16>) {
    match index.iter().position(|&existing| existing == idx) {
        Some(pos) => values[pos] = row,
        None => {
            index.push(idx);
            values.push(row);
        }
    }
}

fn touched_rows(index: &[u32], values: &[Vec<i16>]) -> Vec<TouchedRow> {
    index.iter().copied().zip(values.iter().cloned()).collect()
}

#[cfg(feature = "sera-report")]
fn tracked_value(scale: &PushMeta, touched_index: &[u32], touched_values: &[Vec<i16>], index: u32) -> Option<f64> {
    let pos = touched_index.iter().position(|&i| i == index)?;
    let row = touched_values.get(pos)?;
    match scale {
        PushMeta::Scalar(m) => row.first().map(|&px| super::replay::invert_scalar(px, m.min_val, m.range_val, m.axis_px)),
        PushMeta::Point(m) => row.get(1).map(|&py| super::replay::invert_point(0, py, m).1),
        PushMeta::Vector(m) => row.first().map(|&px| super::replay::invert_scalar(px, m.min_val, m.range_val, m.axis_px)),
    }
}

#[cfg(not(feature = "sera-report"))]
fn tracked_value(_scale: &PushMeta, _touched_index: &[u32], _touched_values: &[Vec<i16>], _index: u32) -> Option<f64> {
    None
}

fn due_breakpoints(opts: &GifOptions, fired: &mut [bool], scale: &PushMeta, touched_index: &[u32], touched_values: &[Vec<i16>], record_dt_ms: u32) -> Vec<u32> {
    let mut due = Vec::new();
    for (i, bp) in opts.breakpoints.iter().enumerate() {
        if fired[i] {
            continue;
        }
        let triggered = match &bp.trigger {
            BreakpointTrigger::AtMs(at) => record_dt_ms >= *at,
            BreakpointTrigger::ValueAtLeast(index, threshold) => tracked_value(scale, touched_index, touched_values, *index).is_some_and(|v| v >= *threshold),
        };
        if triggered {
            fired[i] = true;
            due.push(bp.pause_ms);
        }
    }
    due
}

#[cfg(feature = "sera-report")]
fn try_replay(recording: &Recording, scale: &PushMeta, rows: &[TouchedRow], width: u32, height: u32, race: bool) -> Option<RgbaImage> {
    let (family, json) = recording.source.as_ref()?;
    super::replay::render_frame(family, json, scale, rows, width, height, race).ok()
}

#[cfg(not(feature = "sera-report"))]
fn try_replay(_recording: &Recording, _scale: &PushMeta, _rows: &[TouchedRow], _width: u32, _height: u32, _race: bool) -> Option<RgbaImage> {
    None
}

#[cfg(feature = "sera-report")]
fn try_replay_svg(recording: &Recording, scale: &PushMeta, rows: &[TouchedRow], race: bool) -> Option<(String, Vec<u32>)> {
    let (family, json) = recording.source.as_ref()?;
    let replayed = super::replay::render_svg(family, json, scale, rows, race).ok()?;
    Some((replayed.svg, replayed.order))
}

#[cfg(not(feature = "sera-report"))]
fn try_replay_svg(_recording: &Recording, _scale: &PushMeta, _rows: &[TouchedRow], _race: bool) -> Option<(String, Vec<u32>)> {
    None
}

#[cfg(feature = "sera-report")]
fn tween_and_rasterize(from_svg: &str, to_svg: &str, from_order: &[u32], to_order: &[u32], t: f64, ease: &str, width: u32, height: u32) -> Option<RgbaImage> {
    let eased = super::tween::Ease::parse(ease).unwrap_or(super::tween::Ease::Linear).apply(t);
    let svg = super::tween::tween_svg(from_svg, to_svg, from_order, to_order, eased);
    super::replay::rasterize(&svg, width, height).ok()
}

#[cfg(not(feature = "sera-report"))]
fn tween_and_rasterize(_from_svg: &str, _to_svg: &str, _from_order: &[u32], _to_order: &[u32], _t: f64, _ease: &str, _width: u32, _height: u32) -> Option<RgbaImage> {
    None
}

#[cfg(feature = "sera-report")]
fn mask_and_rasterize(svg: &str, frac: f64, width: u32, height: u32) -> Option<RgbaImage> {
    let masked = super::replay::mask_svg_from_x_fraction(svg, frac);
    super::replay::rasterize(&masked, width, height).ok()
}

#[cfg(not(feature = "sera-report"))]
fn mask_and_rasterize(_svg: &str, _frac: f64, _width: u32, _height: u32) -> Option<RgbaImage> {
    None
}

fn render_one_frame(recording: &Recording, kind: &RenderKind, scale: &PushMeta, opts: &GifOptions, rows: &[TouchedRow], color: image::Rgba<u8>) -> RgbaImage {
    if let Some(img) = try_replay(recording, scale, rows, opts.width, opts.height, opts.sort_by_value) {
        return img;
    }
    render_frame(kind, opts.width, opts.height, rows, color)
}

pub(crate) fn validate<'a>(recording: &'a Recording, opts: &GifOptions) -> Result<&'a PushMeta, String> {
    let scale = recording
        .scale
        .as_ref()
        .ok_or_else(|| "this recording has no scale information (a pre-v2 .spls file) and cannot be rendered".to_string())?;
    if recording.records.is_empty() {
        return Err("this recording has no frames".to_string());
    }
    if recording.records.len() > opts.max_frames {
        return Err(format!(
            "this recording has {} frames, more than max_frames={} - raise max_frames or record a shorter window",
            recording.records.len(),
            opts.max_frames
        ));
    }
    Ok(scale)
}

pub(crate) fn render_timed_frames(recording: &Recording, opts: &GifOptions) -> Result<Vec<(u32, RgbaImage)>, String> {
    if opts.reveal {
        return render_reveal_frames(recording, opts);
    }
    let scale = validate(recording, opts)?;
    let kind = render_kind(scale);
    let vpp = scale.values_per_point().max(1) as usize;
    let color = super::canvas::to_rgba(opts.color);

    let mut touched_index: Vec<u32> = Vec::new();
    let mut touched_values: Vec<Vec<i16>> = Vec::new();
    let mut timed_frames = Vec::with_capacity(recording.records.len());
    let mut prev_dt = 0u32;
    let mut prev_svg: Option<(String, Vec<u32>)> = None;
    let mut prev_rows: Option<Vec<TouchedRow>> = None;
    let ease = super::tween::Ease::parse(&opts.smooth_ease).unwrap_or(super::tween::Ease::Linear);
    let mut time_shift = 0u32;
    let mut fired = vec![false; opts.breakpoints.len()];

    for record in &recording.records {
        for (chunk_i, idx) in record.indices.iter().enumerate() {
            let start = chunk_i * vpp;
            let row: Vec<i16> = record.values.get(start..start + vpp).unwrap_or(&[]).to_vec();
            upsert(&mut touched_index, &mut touched_values, *idx, row);
        }
        let rows = ranked(touched_rows(&touched_index, &touched_values), opts.sort_by_value);

        if opts.smooth && opts.sort_by_value {
            if let Some(from_rows) = &prev_rows {
                let gap = record.dt_ms.saturating_sub(prev_dt);
                let steps = tween_steps(gap, opts);
                for step in 1..steps {
                    let t = ease.apply(step as f64 / steps as f64);
                    let interpolated = ranked(interpolate_touched_rows(from_rows, &rows, t), true);
                    if let Some(img) = try_replay(recording, scale, &interpolated, opts.width, opts.height, true) {
                        timed_frames.push((prev_dt + (gap * step) / steps + time_shift, img));
                    }
                }
            }
            prev_rows = Some(rows.clone());
        } else if opts.smooth {
            let this_svg = try_replay_svg(recording, scale, &rows, opts.sort_by_value);
            if let (Some((from_svg, from_order)), Some((to_svg, to_order))) = (&prev_svg, &this_svg) {
                let gap = record.dt_ms.saturating_sub(prev_dt);
                let steps = tween_steps(gap, opts);
                for step in 1..steps {
                    let t = step as f64 / steps as f64;
                    if let Some(img) = tween_and_rasterize(from_svg, to_svg, from_order, to_order, t, &opts.smooth_ease, opts.width, opts.height) {
                        timed_frames.push((prev_dt + (gap * step) / steps + time_shift, img));
                    }
                }
            }
            prev_svg = this_svg;
        }

        let img = render_one_frame(recording, &kind, scale, opts, &rows, color);
        let dt_ms = record.dt_ms + time_shift;
        let due = due_breakpoints(opts, &mut fired, scale, &touched_index, &touched_values, record.dt_ms);
        if due.is_empty() {
            timed_frames.push((dt_ms, img));
        } else {
            timed_frames.push((dt_ms, img.clone()));
            let mut local_shift = 0u32;
            for pause_ms in due {
                local_shift += pause_ms;
                timed_frames.push((dt_ms + local_shift, img.clone()));
            }
            time_shift += local_shift;
        }
        prev_dt = record.dt_ms;
    }
    Ok(timed_frames)
}

fn reveal_count(step: usize, steps: usize, n: usize) -> usize {
    let min_count = 2.min(n);
    (((step * n) as f64 / steps.max(1) as f64).ceil() as usize).clamp(min_count, n)
}

pub(crate) fn render_reveal_frames(recording: &Recording, opts: &GifOptions) -> Result<Vec<(u32, RgbaImage)>, String> {
    let scale = validate(recording, opts)?;
    let kind = render_kind(scale);
    let vpp = scale.values_per_point().max(1) as usize;
    let color = super::canvas::to_rgba(opts.color);

    let mut touched_index: Vec<u32> = Vec::new();
    let mut touched_values: Vec<Vec<i16>> = Vec::new();
    for record in &recording.records {
        for (chunk_i, idx) in record.indices.iter().enumerate() {
            let start = chunk_i * vpp;
            let row: Vec<i16> = record.values.get(start..start + vpp).unwrap_or(&[]).to_vec();
            upsert(&mut touched_index, &mut touched_values, *idx, row);
        }
    }
    let mut final_rows = touched_rows(&touched_index, &touched_values);
    final_rows.sort_by_key(|(idx, _)| *idx);
    let n = final_rows.len();
    if n == 0 {
        return Err("this recording has no touched indices to reveal".to_string());
    }

    let steps = opts.reveal_steps.max(1) as usize;
    let mut timed_frames = Vec::with_capacity(steps);
    let mut fired = vec![false; opts.breakpoints.len()];
    let mut time_shift = 0u32;
    let fixed_svg = if opts.reveal_fixed_scale { try_replay_svg(recording, scale, &final_rows, false).map(|(svg, _)| svg) } else { None };

    for step in 1..=steps {
        let count = reveal_count(step, steps, n);
        let visible = &final_rows[..count];
        let img = fixed_svg
            .as_deref()
            .and_then(|svg| mask_and_rasterize(svg, step as f64 / steps as f64, opts.width, opts.height))
            .or_else(|| try_replay(recording, scale, visible, opts.width, opts.height, true))
            .unwrap_or_else(|| render_frame(&kind, opts.width, opts.height, visible, color));
        let raw_dt_ms = step as u32 * opts.smooth_tick_ms.max(1);
        let dt_ms = raw_dt_ms + time_shift;

        let visible_index: Vec<u32> = visible.iter().map(|(i, _)| *i).collect();
        let visible_values: Vec<Vec<i16>> = visible.iter().map(|(_, v)| v.clone()).collect();
        let due = due_breakpoints(opts, &mut fired, scale, &visible_index, &visible_values, raw_dt_ms);
        if due.is_empty() {
            timed_frames.push((dt_ms, img));
        } else {
            timed_frames.push((dt_ms, img.clone()));
            let mut local_shift = 0u32;
            for pause_ms in due {
                local_shift += pause_ms;
                timed_frames.push((dt_ms + local_shift, img.clone()));
            }
            time_shift += local_shift;
        }
    }
    Ok(timed_frames)
}

pub(crate) fn render_gif(recording: &Recording, opts: &GifOptions) -> Result<Vec<u8>, String> {
    let timed_frames = render_timed_frames(recording, opts)?;
    let mut frames = Vec::with_capacity(timed_frames.len());
    let mut prev_dt = 0u32;
    for (dt_ms, img) in timed_frames {
        let delay_ms = dt_ms.saturating_sub(prev_dt) as u64;
        prev_dt = dt_ms;
        frames.push(Frame::from_parts(img, 0, 0, Delay::from_saturating_duration(Duration::from_millis(delay_ms.max(MIN_FRAME_DELAY_MS)))));
    }
    encode_gif(frames)
}

fn encode_gif(frames: Vec<Frame>) -> Result<Vec<u8>, String> {
    let mut bytes: Vec<u8> = Vec::new();
    {
        let mut encoder = GifEncoder::new(&mut bytes);
        encoder.set_repeat(Repeat::Infinite).map_err(|e| format!("GIF encoder setup failed: {e}"))?;
        for f in frames {
            encoder.encode_frame(f).map_err(|e| format!("GIF frame encoding failed: {e}"))?;
        }
    }
    Ok(bytes)
}

pub(crate) fn count_frames(recording: &Recording) -> usize {
    recording.records.len()
}

pub(crate) fn render_single_frame(recording: &Recording, opts: &GifOptions, frame_index: usize) -> Result<image::RgbaImage, String> {
    let scale = recording
        .scale
        .as_ref()
        .ok_or_else(|| "this recording has no scale information (a pre-v2 .spls file) and cannot be rendered".to_string())?;
    if frame_index >= recording.records.len() {
        return Err(format!("frame_index {frame_index} is out of range - this recording has {} frames", recording.records.len()));
    }
    let kind = render_kind(scale);
    let vpp = scale.values_per_point().max(1) as usize;
    let color = super::canvas::to_rgba(opts.color);

    let mut touched_index: Vec<u32> = Vec::new();
    let mut touched_values: Vec<Vec<i16>> = Vec::new();
    for record in &recording.records[..=frame_index] {
        for (chunk_i, idx) in record.indices.iter().enumerate() {
            let start = chunk_i * vpp;
            let row: Vec<i16> = record.values.get(start..start + vpp).unwrap_or(&[]).to_vec();
            upsert(&mut touched_index, &mut touched_values, *idx, row);
        }
    }
    let rows = ranked(touched_rows(&touched_index, &touched_values), opts.sort_by_value);
    Ok(render_one_frame(recording, &kind, scale, opts, &rows, color))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plot::push_registry::{PointMeta, ScalarMeta, VectorMeta};
    use crate::plot::record_registry::Record;

    fn rec(dt_ms: u32, indices: Vec<u32>, values: Vec<i16>) -> Record {
        Record { dt_ms, n: indices.len() as u32, vpp: 1, indices, values }
    }

    fn scalar_recording(records: Vec<Record>) -> Recording {
        Recording {
            scale: Some(PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 })),
            started_at_ms: Some(0),
            source: None,
            records,
        }
    }

    fn point_recording(records: Vec<Record>) -> Recording {
        Recording {
            scale: Some(PushMeta::Point(PointMeta { min_x: 0.0, range_x: 100.0, min_y: 0.0, range_y: 100.0, plot_w: 400, plot_h: 300 })),
            started_at_ms: Some(0),
            source: None,
            records,
        }
    }

    fn vector_recording(records: Vec<Record>) -> Recording {
        Recording {
            scale: Some(PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 100.0, axis_px: 100, vpp: 4 })),
            started_at_ms: Some(0),
            source: None,
            records,
        }
    }

    fn scalar_recording_with_source(records: Vec<Record>, family: &str, json: &str) -> Recording {
        Recording {
            scale: Some(PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 })),
            started_at_ms: Some(0),
            source: Some((family.to_string(), json.to_string())),
            records,
        }
    }

    const BAR_SOURCE_JSON: &str = r#"{"title":"t","labels":["a","b","c"],"values":[10.0,10.0,10.0]}"#;

    #[test]
    fn rejects_a_recording_with_no_scale_information() {
        let legacy = Recording { scale: None, started_at_ms: None, source: None, records: vec![rec(10, vec![0], vec![5])] };
        let err = render_gif(&legacy, &GifOptions::default()).unwrap_err();
        assert!(err.contains("no scale"));
    }

    #[test]
    fn rejects_an_empty_recording() {
        let empty = scalar_recording(vec![]);
        let err = render_gif(&empty, &GifOptions::default()).unwrap_err();
        assert!(err.contains("no frames"));
    }

    #[test]
    fn ranked_leaves_order_untouched_when_sort_by_value_is_false() {
        let rows: Vec<TouchedRow> = vec![(0, vec![10]), (1, vec![90]), (2, vec![50])];
        assert_eq!(ranked(rows.clone(), false), rows);
    }

    #[test]
    fn ranked_sorts_descending_by_first_value_when_sort_by_value_is_true() {
        let rows: Vec<TouchedRow> = vec![(0, vec![10]), (1, vec![90]), (2, vec![50])];
        let sorted = ranked(rows, true);
        assert_eq!(sorted.iter().map(|(idx, _)| *idx).collect::<Vec<_>>(), vec![1, 2, 0], "index 1 (value 90) should rank first, then index 2 (50), then index 0 (10)");
    }

    #[test]
    fn ranked_treats_a_missing_value_as_zero_rather_than_panicking() {
        let rows: Vec<TouchedRow> = vec![(0, vec![]), (1, vec![5])];
        let sorted = ranked(rows, true);
        assert_eq!(sorted[0].0, 1, "a row with a real value should outrank an empty one");
    }

    #[test]
    fn sort_by_value_changes_the_rendered_gif_when_ranks_differ_across_frames() {
        let recording = scalar_recording(vec![rec(0, vec![0, 1], vec![10, 90]), rec(50, vec![0, 1], vec![95, 20])]);
        let stable = render_gif(&recording, &GifOptions::default()).unwrap();
        let raced = render_gif(&recording, &GifOptions { sort_by_value: true, ..GifOptions::default() }).unwrap();
        assert_ne!(stable, raced, "reordering slots by rank each frame must change the encoded animation from the stable-order version");
    }

    #[test]
    fn rejects_a_recording_larger_than_max_frames() {
        let many = scalar_recording((0..10).map(|i| rec(i * 10, vec![0], vec![i as i16])).collect());
        let opts = GifOptions { max_frames: 5, ..GifOptions::default() };
        let err = render_gif(&many, &opts).unwrap_err();
        assert!(err.contains("max_frames"));
    }

    #[test]
    fn a_valid_scalar_recording_produces_a_structurally_real_gif() {
        let recording = scalar_recording(vec![rec(0, vec![0, 1], vec![20, 40]), rec(120, vec![0], vec![80])]);
        let gif = render_gif(&recording, &GifOptions::default()).expect("should render");
        assert!(gif.starts_with(b"GIF8"));
        assert!(gif.len() > 100);
    }

    #[test]
    fn a_valid_point_recording_produces_a_structurally_real_gif() {
        let recording = point_recording(vec![Record { dt_ms: 0, n: 1, vpp: 2, indices: vec![0], values: vec![10, 20] }]);
        let gif = render_gif(&recording, &GifOptions::default()).expect("should render");
        assert!(gif.starts_with(b"GIF8"), "point-type recordings must now render instead of being rejected");
    }

    #[test]
    fn a_valid_vector_ohlc_recording_produces_a_structurally_real_gif() {
        let recording = vector_recording(vec![Record { dt_ms: 0, n: 1, vpp: 4, indices: vec![0], values: vec![20, 80, 10, 70] }]);
        let gif = render_gif(&recording, &GifOptions::default()).expect("should render");
        assert!(gif.starts_with(b"GIF8"), "vector-type (OHLC) recordings must now render instead of being rejected");
    }

    #[test]
    fn different_recordings_produce_different_gif_bytes() {
        let a = scalar_recording(vec![rec(0, vec![0], vec![20])]);
        let b = scalar_recording(vec![rec(0, vec![0], vec![90])]);
        assert_ne!(render_gif(&a, &GifOptions::default()).unwrap(), render_gif(&b, &GifOptions::default()).unwrap());
    }

    #[test]
    fn a_custom_color_changes_the_output_bytes() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![50])]);
        let default_color = render_gif(&recording, &GifOptions::default()).unwrap();
        let custom = render_gif(&recording, &GifOptions { color: (10, 200, 30), ..GifOptions::default() }).unwrap();
        assert_ne!(default_color, custom, "a different bar color should change the encoded GIF bytes");
    }

    #[test]
    fn decoded_gif_frames_actually_differ_pixel_for_pixel_round_tripped_through_the_real_decoder() {
        use image::codecs::gif::GifDecoder;
        use image::AnimationDecoder;
        use image::RgbaImage;

        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(50, vec![0], vec![90])]);
        let gif_bytes = render_gif(&recording, &GifOptions::default()).expect("should render");

        let decoder = GifDecoder::new(std::io::Cursor::new(&gif_bytes)).expect("decode gif");
        let decoded_frames: Vec<_> = decoder.into_frames().collect_frames().expect("collect frames");
        assert_eq!(decoded_frames.len(), 2);

        let bar_pixels = |img: &RgbaImage| img.pixels().filter(|p| **p == super::super::canvas::BAR_COLOR).count();
        assert_ne!(bar_pixels(decoded_frames[0].buffer()), bar_pixels(decoded_frames[1].buffer()));
    }

    #[test]
    fn the_number_of_touched_slots_grows_only_as_new_indices_are_actually_touched() {
        let recording = scalar_recording(vec![rec(0, vec![7], vec![50]), rec(50, vec![7], vec![60]), rec(100, vec![12], vec![30])]);
        let gif = render_gif(&recording, &GifOptions::default()).expect("should render");
        assert!(gif.starts_with(b"GIF8"));
    }

    #[test]
    fn dt_ms_deltas_are_used_for_frame_delay_not_absolute_timestamps() {
        let fast = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(10, vec![0], vec![20])]);
        let slow = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(5000, vec![0], vec![20])]);
        let gif_fast = render_gif(&fast, &GifOptions::default()).unwrap();
        let gif_slow = render_gif(&slow, &GifOptions::default()).unwrap();
        assert_eq!(gif_fast.len(), gif_slow.len(), "the GIF delay field is a fixed 2-byte width regardless of value");
        assert_ne!(gif_fast, gif_slow, "a much longer delay must still change the encoded delay bytes");
    }

    #[test]
    fn render_timed_frames_returns_one_entry_per_record_with_cumulative_dt_ms() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(50, vec![0], vec![20]), rec(200, vec![0], vec![30])]);
        let timed = render_timed_frames(&recording, &GifOptions::default()).expect("should render");
        assert_eq!(timed.iter().map(|(dt, _)| *dt).collect::<Vec<_>>(), vec![0, 50, 200]);
        assert_ne!(timed[0].1, timed[1].1);
        assert_ne!(timed[1].1, timed[2].1);
    }

    #[test]
    fn count_frames_matches_the_number_of_records_without_rendering_anything() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![1]), rec(10, vec![0], vec![2]), rec(20, vec![0], vec![3])]);
        assert_eq!(count_frames(&recording), 3);
    }

    #[test]
    fn render_single_frame_reflects_only_records_up_to_and_including_the_requested_index() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(10, vec![0], vec![90])]);
        let first = render_single_frame(&recording, &GifOptions::default(), 0).unwrap();
        let second = render_single_frame(&recording, &GifOptions::default(), 1).unwrap();
        assert_ne!(first, second, "frame 0 should reflect only the first push (10), frame 1 should reflect the update to 90");
    }

    #[test]
    fn render_single_frame_rejects_an_out_of_range_index() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10])]);
        let err = render_single_frame(&recording, &GifOptions::default(), 5).unwrap_err();
        assert!(err.contains("out of range"));
    }

    #[test]
    fn smooth_false_keeps_the_original_one_frame_per_record_contract_even_with_a_real_source() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let timed = render_timed_frames(&recording, &GifOptions::default()).expect("should render");
        assert_eq!(timed.len(), 2, "smooth=false (default) must not change frame count even when a replay source is available");
    }

    #[test]
    fn smooth_true_still_renders_without_crashing_when_there_is_no_replay_source() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])]);
        let opts = GifOptions { smooth: true, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert_eq!(timed.len(), 2, "without a replay source, smooth must gracefully fall back to one frame per record");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_inserts_interpolated_frames_into_a_real_gap() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert!(timed.len() > 2, "a 200ms gap at a 20ms tick should produce interpolated frames in between, got {} frames", timed.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_does_not_subdivide_a_gap_smaller_than_one_tick() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(5, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert_eq!(timed.len(), 2, "a gap smaller than one tick should not be subdivided");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_caps_interpolation_steps_for_a_very_large_gap() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(100_000, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert!(timed.len() <= (2 + MAX_SMOOTH_STEPS - 1) as usize, "interpolation steps must be capped even for a huge gap, got {} frames", timed.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_first_and_last_interpolated_frame_differ_from_each_other() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        let mid = timed.len() / 2;
        assert_ne!(timed[0].1, timed[mid].1, "an interpolated mid-transition frame must differ from the starting keyframe");
        assert_ne!(timed[mid].1, timed[timed.len() - 1].1, "an interpolated mid-transition frame must differ from the ending keyframe");
    }

    fn pixel_diff(a: &RgbaImage, b: &RgbaImage) -> i64 {
        a.pixels().zip(b.pixels()).map(|(pa, pb)| pa.0.iter().zip(pb.0.iter()).map(|(&x, &y)| (x as i64 - y as i64).abs()).sum::<i64>()).sum()
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_with_a_rank_swap_produces_a_continuously_progressing_transition_with_no_frozen_or_reversed_step() {
        let json = r#"{"title":"t","labels":["a","b","c"],"values":[0.0,0.0,0.0]}"#;
        let recording = scalar_recording_with_source(
            vec![
                Record { dt_ms: 0, n: 3, vpp: 1, indices: vec![0, 1, 2], values: vec![20, 15, 10] },
                Record { dt_ms: 200, n: 3, vpp: 1, indices: vec![0, 1, 2], values: vec![20, 15, 90] },
            ],
            "build_bar",
            json,
        );
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, sort_by_value: true, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert!(timed.len() > 5, "expected several interpolated frames for a 200ms gap at a 20ms tick, got {}", timed.len());

        let diffs: Vec<i64> = timed.windows(2).map(|w| pixel_diff(&w[0].1, &w[1].1)).collect();
        assert!(diffs.iter().all(|&d| d > 0), "every interpolated step during a rank-swapping transition must actually change the rendered image, got {diffs:?}");

        let spikes: Vec<usize> = diffs.windows(3).enumerate().filter(|(_, w)| w[1] > w[0] && w[1] > w[2]).map(|(i, _)| i + 1).collect();
        assert!(
            spikes.is_empty(),
            "a diff spiking above both of its neighbors would mean the transition froze then jumped, rather than progressing continuously as rank is re-derived from the interpolated value each tick: {diffs:?}"
        );
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_speed_greater_than_one_produces_fewer_interpolated_frames() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(240, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let normal = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() }).unwrap();
        let fast = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_speed: 4.0, ..GifOptions::default() }).unwrap();
        assert!(fast.len() < normal.len(), "a higher smooth_speed must produce fewer interpolated frames for the same gap, got normal={} fast={}", normal.len(), fast.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_speed_less_than_one_produces_more_interpolated_frames() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(80, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let normal = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() }).unwrap();
        let slow = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_speed: 0.25, ..GifOptions::default() }).unwrap();
        assert!(slow.len() > normal.len(), "a lower smooth_speed must produce more interpolated frames for the same gap, got normal={} slow={}", normal.len(), slow.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_max_steps_bounds_interpolated_frame_count_independent_of_speed() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(100_000, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, smooth_speed: 50.0, smooth_max_steps: 5, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).unwrap();
        assert!(timed.len() <= 6, "smooth_max_steps=5 should cap this huge gap to at most 6 frames (5 steps + 1 keyframe boundary handling), got {}", timed.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_ease_changes_the_interpolated_geometry_at_the_same_step() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let linear = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_ease: "linear".to_string(), ..GifOptions::default() }).unwrap();
        let eased = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_ease: "ease_in".to_string(), ..GifOptions::default() }).unwrap();
        assert_eq!(linear.len(), eased.len(), "changing the easing curve must not change how many frames are produced");
        let mid = linear.len() / 2;
        assert_ne!(linear[mid].1, eased[mid].1, "linear and ease_in must reach different intermediate states at the same step index");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_ease_with_an_unknown_name_falls_back_to_linear_without_erroring() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let unknown = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_ease: "bounce".to_string(), ..GifOptions::default() }).unwrap();
        let linear = render_timed_frames(&recording, &GifOptions { smooth: true, smooth_tick_ms: 20, smooth_ease: "linear".to_string(), ..GifOptions::default() }).unwrap();
        assert_eq!(unknown.len(), linear.len());
        assert_eq!(unknown[unknown.len() / 2].1, linear[linear.len() / 2].1, "an unrecognized ease name must silently behave exactly like linear");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn smooth_true_produces_a_renderable_gif() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![10]), rec(200, vec![0], vec![90])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, ..GifOptions::default() };
        let gif = render_gif(&recording, &opts).expect("should render");
        assert!(gif.starts_with(b"GIF8"));
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_true_produces_exactly_reveal_steps_frames_progressing_from_partial_to_full() {
        let recording =
            scalar_recording_with_source(vec![rec(0, vec![0], vec![20]), rec(50, vec![1], vec![40]), rec(100, vec![2], vec![60])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { reveal: true, reveal_steps: 6, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert_eq!(timed.len(), 6, "reveal_steps should directly bound the frame count");
        assert_ne!(timed[0].1, timed[timed.len() - 1].1, "the first reveal step should differ from the fully-revealed last step");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_last_frame_matches_a_full_replay_of_every_touched_index() {
        let recording =
            scalar_recording_with_source(vec![rec(0, vec![0], vec![20]), rec(50, vec![1], vec![40]), rec(100, vec![2], vec![60])], "build_bar", BAR_SOURCE_JSON);
        let revealed = render_timed_frames(&recording, &GifOptions { reveal: true, reveal_steps: 10, ..GifOptions::default() }).expect("should render");
        let normal = render_timed_frames(&recording, &GifOptions::default()).expect("should render");
        assert_eq!(revealed.last().unwrap().1, normal.last().unwrap().1, "the fully-revealed final reveal frame should match the normal final replayed frame");
    }

    fn left_margin_pixels(img: &RgbaImage) -> Vec<u8> {
        let mut px = Vec::new();
        for y in 0..img.height() {
            for x in 0..40.min(img.width()) {
                px.extend_from_slice(&img.get_pixel(x, y).0);
            }
        }
        px
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_fixed_scale_keeps_the_axis_region_pixel_identical_across_steps() {
        let recording = scalar_recording_with_source(
            vec![rec(0, vec![0], vec![10]), rec(50, vec![1], vec![90]), rec(100, vec![2], vec![50])],
            "build_bar",
            BAR_SOURCE_JSON,
        );
        let opts = GifOptions { reveal: true, reveal_steps: 10, reveal_fixed_scale: true, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        let early = &timed[1].1;
        let late = &timed[timed.len() - 2].1;
        assert_eq!(
            left_margin_pixels(early),
            left_margin_pixels(late),
            "with a fixed scale, the axis margin (never covered by the reveal mask) must be pixel-identical across steps, since the underlying chart is built once and never re-scaled"
        );
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_without_fixed_scale_does_rescale_the_axis_as_the_default_baseline_behavior() {
        let recording = scalar_recording_with_source(
            vec![rec(0, vec![0], vec![10]), rec(50, vec![1], vec![90]), rec(100, vec![2], vec![50])],
            "build_bar",
            BAR_SOURCE_JSON,
        );
        let scale = validate(&recording, &GifOptions::default()).unwrap();
        let early_svg = try_replay_svg(&recording, scale, &[(0, vec![10])], true).unwrap().0;
        let late_svg = try_replay_svg(&recording, scale, &[(0, vec![10]), (1, vec![90]), (2, vec![50])], true).unwrap().0;
        assert_ne!(
            early_svg, late_svg,
            "this is the contrasting baseline: without a fixed scale, replaying a growing subset of a wide-swinging dataset legitimately produces a different auto-scaled SVG at each stage, proving the fixed-scale mechanism above (one SVG, reused and only masked) is actually doing something different"
        );
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_fixed_scale_last_frame_matches_a_normal_complete_replay() {
        let recording = scalar_recording_with_source(
            vec![rec(0, vec![0], vec![10]), rec(50, vec![1], vec![90]), rec(100, vec![2], vec![50])],
            "build_bar",
            BAR_SOURCE_JSON,
        );
        let revealed = render_timed_frames(&recording, &GifOptions { reveal: true, reveal_steps: 10, reveal_fixed_scale: true, ..GifOptions::default() }).expect("should render");
        let normal = render_timed_frames(&recording, &GifOptions::default()).expect("should render");
        assert_eq!(revealed.last().unwrap().1, normal.last().unwrap().1, "at full reveal the mask covers nothing, so the last frame must match a normal complete replay exactly");
    }

    #[test]
    fn reveal_fixed_scale_still_renders_without_crashing_when_there_is_no_replay_source() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![20]), rec(50, vec![1], vec![40])]);
        let opts = GifOptions { reveal: true, reveal_steps: 5, reveal_fixed_scale: true, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert_eq!(timed.len(), 5, "without a replay source, reveal_fixed_scale must gracefully fall back to the normal truncated-array reveal rather than failing");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_steps_of_one_still_shows_the_complete_final_state() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![20]), rec(50, vec![1], vec![40])], "build_bar", BAR_SOURCE_JSON);
        let timed = render_timed_frames(&recording, &GifOptions { reveal: true, reveal_steps: 1, ..GifOptions::default() }).expect("should render");
        assert_eq!(timed.len(), 1);
    }

    #[test]
    fn reveal_count_never_drops_to_a_single_touched_index_when_at_least_two_exist() {
        for step in 1..=150 {
            let count = reveal_count(step, 150, 30);
            assert!(count >= 2, "step {step}/150 over 30 touched indices revealed only {count} - a lone touched index makes line-family charts render empty and fall back to a bar-shaped placeholder");
        }
    }

    #[test]
    fn reveal_count_reaches_the_full_touched_set_on_the_last_step() {
        assert_eq!(reveal_count(150, 150, 30), 30);
    }

    #[test]
    fn reveal_count_is_monotonically_non_decreasing_across_steps() {
        let mut prev = 0;
        for step in 1..=150 {
            let count = reveal_count(step, 150, 30);
            assert!(count >= prev, "revealed count must never shrink from one step to the next, got {count} after {prev} at step {step}");
            prev = count;
        }
    }

    #[test]
    fn reveal_count_never_exceeds_a_single_touched_index_dataset() {
        assert_eq!(reveal_count(1, 10, 1), 1, "a dataset with only one touched index has nothing better to clamp up to");
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_of_a_multi_series_line_never_falls_back_to_the_bar_shaped_placeholder_on_its_first_frame() {
        let json = r#"{"title":"t","x_labels":["2000","2001","2002","2003","2004","2005","2006","2007","2008","2009"],"series":[[10.0,11.0,12.0,13.0,14.0,15.0,16.0,17.0,18.0,19.0],[5.0,6.0,7.0,8.0,9.0,10.0,11.0,12.0,13.0,14.0]],"series_names":["A","B"],"variant":"multi"}"#;
        let records = vec![Record { dt_ms: 0, n: 10, vpp: 2, indices: (0..10).collect(), values: (0..10i16).flat_map(|i| [100 + i * 5, 50 + i * 5]).collect() }];
        let recording = scalar_recording_with_source(records, "build_line", json);
        let scale_override = PushMeta::Vector(VectorMeta { min_val: 0.0, range_val: 200.0, axis_px: 300, vpp: 2 });
        let recording = Recording { scale: Some(scale_override), ..recording };
        let opts = GifOptions { reveal: true, reveal_steps: 50, width: 400, height: 300, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        let first = &timed[0].1;
        let non_white = first.pixels().filter(|p| !(p.0[0] > 250 && p.0[1] > 250 && p.0[2] > 250)).count();
        assert!(
            non_white > 50,
            "the first reveal frame of a small multi-series recording must already show a real two-point line, not an empty or bar-shaped placeholder frame, got {non_white} non-white pixels"
        );
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn reveal_timeline_is_paced_by_smooth_tick_ms_not_the_recordings_real_elapsed_time() {
        let recording = scalar_recording_with_source(vec![rec(0, vec![0], vec![20]), rec(5, vec![1], vec![40])], "build_bar", BAR_SOURCE_JSON);
        let opts = GifOptions { reveal: true, reveal_steps: 4, smooth_tick_ms: 25, ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        assert_eq!(
            timed.iter().map(|(dt, _)| *dt).collect::<Vec<_>>(),
            vec![25, 50, 75, 100],
            "a near-instant recording (5ms real gap) must still produce a full-length reveal timeline paced by smooth_tick_ms, not collapse to near-zero duration"
        );
    }

    #[test]
    fn reveal_true_still_renders_without_crashing_when_there_is_no_replay_source() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![20]), rec(50, vec![1], vec![40])]);
        let timed = render_timed_frames(&recording, &GifOptions { reveal: true, reveal_steps: 5, ..GifOptions::default() }).expect("should render");
        assert_eq!(timed.len(), 5);
    }

    #[test]
    fn breakpoint_at_ms_inserts_a_held_frame_and_shifts_subsequent_timestamps() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(100, vec![0], vec![50]), rec(200, vec![0], vec![90])]);
        let without_bp = render_timed_frames(&recording, &GifOptions::default()).unwrap();
        let opts = GifOptions { breakpoints: vec![Breakpoint { trigger: BreakpointTrigger::AtMs(100), pause_ms: 500 }], ..GifOptions::default() };
        let with_bp = render_timed_frames(&recording, &opts).unwrap();
        assert_eq!(with_bp.len(), without_bp.len() + 1, "a triggered breakpoint should insert exactly one extra held frame");
        assert_eq!(with_bp.last().unwrap().0, without_bp.last().unwrap().0 + 500, "every frame after the breakpoint should be shifted forward by the pause duration");
    }

    #[test]
    fn breakpoint_pause_frame_repeats_the_triggering_frames_image() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(100, vec![0], vec![50])]);
        let opts = GifOptions { breakpoints: vec![Breakpoint { trigger: BreakpointTrigger::AtMs(100), pause_ms: 300 }], ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).unwrap();
        let trigger_frame = &timed[timed.len() - 2];
        let pause_frame = &timed[timed.len() - 1];
        assert_eq!(trigger_frame.1, pause_frame.1, "the inserted pause frame must be pixel-identical to the frame that triggered it, a genuine freeze");
        assert_eq!(pause_frame.0, trigger_frame.0 + 300);
    }

    #[test]
    fn breakpoint_on_value_fires_exactly_once_even_though_the_value_stays_above_threshold_afterward() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(100, vec![0], vec![95]), rec(200, vec![0], vec![96]), rec(300, vec![0], vec![97])]);
        let opts = GifOptions { breakpoints: vec![Breakpoint { trigger: BreakpointTrigger::ValueAtLeast(0, 90.0), pause_ms: 400 }], ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).unwrap();
        let without_bp = render_timed_frames(&recording, &GifOptions::default()).unwrap();
        assert_eq!(
            timed.len(),
            without_bp.len() + 1,
            "the value-triggered breakpoint should fire exactly once, not on every subsequent frame where the value stays above threshold"
        );
    }

    #[test]
    fn breakpoint_on_value_never_fires_when_the_threshold_is_never_reached() {
        let recording = scalar_recording(vec![rec(0, vec![0], vec![10]), rec(100, vec![0], vec![20])]);
        let opts = GifOptions { breakpoints: vec![Breakpoint { trigger: BreakpointTrigger::ValueAtLeast(0, 90.0), pause_ms: 400 }], ..GifOptions::default() };
        let timed = render_timed_frames(&recording, &opts).unwrap();
        let without_bp = render_timed_frames(&recording, &GifOptions::default()).unwrap();
        assert_eq!(timed.len(), without_bp.len(), "a breakpoint whose threshold is never reached must not insert any pause frame");
    }

    #[test]
    fn interpolate_touched_rows_lerps_each_shared_identity_independently() {
        let from: Vec<TouchedRow> = vec![(0, vec![20]), (1, vec![80])];
        let to: Vec<TouchedRow> = vec![(0, vec![60]), (1, vec![40])];
        let mid = interpolate_touched_rows(&from, &to, 0.5);
        assert_eq!(mid, vec![(0, vec![40]), (1, vec![60])]);
    }

    #[test]
    fn interpolate_touched_rows_grows_a_newly_touched_identity_in_from_zero() {
        let from: Vec<TouchedRow> = vec![(0, vec![50])];
        let to: Vec<TouchedRow> = vec![(0, vec![50]), (7, vec![100])];
        let mid = interpolate_touched_rows(&from, &to, 0.25);
        assert_eq!(mid, vec![(0, vec![50]), (7, vec![25])], "an identity with no prior state must grow in from zero, not appear at full value immediately");
    }

    #[test]
    fn interpolate_touched_rows_at_t_zero_reproduces_the_from_state() {
        let from: Vec<TouchedRow> = vec![(0, vec![15]), (1, vec![95])];
        let to: Vec<TouchedRow> = vec![(0, vec![70]), (1, vec![30])];
        assert_eq!(interpolate_touched_rows(&from, &to, 0.0), from);
    }

    #[test]
    fn interpolate_touched_rows_at_t_one_reproduces_the_to_state() {
        let from: Vec<TouchedRow> = vec![(0, vec![15]), (1, vec![95])];
        let to: Vec<TouchedRow> = vec![(0, vec![70]), (1, vec![30])];
        assert_eq!(interpolate_touched_rows(&from, &to, 1.0), to);
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn race_mode_interpolation_never_collapses_two_rank_swapping_identities_to_identical_geometry() {
        let json = r#"{"title":"t","labels":["a","b","c","d","e","f"],"values":[0.0,0.0,0.0,0.0,0.0,0.0]}"#;
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let from: Vec<TouchedRow> = vec![(0, vec![90]), (1, vec![70]), (2, vec![50]), (3, vec![40]), (4, vec![20]), (5, vec![10])];
        let to: Vec<TouchedRow> = vec![(0, vec![90]), (1, vec![70]), (2, vec![40]), (3, vec![50]), (4, vec![20]), (5, vec![10])];

        let mid_rows = ranked(interpolate_touched_rows(&from, &to, 0.5), true);
        let mid_svg = super::super::replay::render_svg("build_bar", json, &scale, &mid_rows, true).expect("replay should succeed").svg;

        let rect_for = |idx: u32| -> String {
            let needle = format!("data-idx=\"{idx}\"");
            let hit = mid_svg.find(&needle).expect("rect for this identity should exist in the replayed svg");
            let start = mid_svg[..hit].rfind('<').unwrap_or(0);
            let end = mid_svg[hit..].find("/>").map(|e| hit + e).unwrap_or(mid_svg.len());
            mid_svg[start..end].to_string()
        };
        let geometry_of = |rect: &str| -> Vec<String> {
            ["x=\"", "y=\"", "width=\"", "height=\""]
                .iter()
                .map(|attr| {
                    let s = rect.find(attr).map(|p| p + attr.len()).unwrap_or(0);
                    let e = rect[s..].find('"').map(|e| s + e).unwrap_or(s);
                    rect[s..e].to_string()
                })
                .collect()
        };

        let rect_2 = rect_for(2);
        let rect_3 = rect_for(3);
        assert_ne!(
            geometry_of(&rect_2),
            geometry_of(&rect_3),
            "two identities crossing rank exactly at the interpolation midpoint must still render at distinct positions, not collapse onto identical geometry:\nidx2: {rect_2}\nidx3: {rect_3}"
        );
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn race_mode_interpolation_is_generic_across_every_scalar_shaped_race_eligible_family() {
        let scale = PushMeta::Scalar(ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 100 });
        let from: Vec<TouchedRow> = vec![(0, vec![80]), (1, vec![20])];
        let to: Vec<TouchedRow> = vec![(0, vec![20]), (1, vec![80])];
        let mid_rows = ranked(interpolate_touched_rows(&from, &to, 0.5), true);

        let bar_json = r#"{"title":"t","labels":["a","b"],"values":[0.0,0.0]}"#;
        let heatmap_json = r#"{"title":"t","labels":["a","b"],"col_labels":["x"],"values":[0.0,0.0]}"#;
        let icicle_json = r#"{"title":"t","labels":["a","b"],"parents":["",""],"values":[0.0,0.0]}"#;

        for (family, json) in [("build_bar", bar_json), ("build_heatmap", heatmap_json), ("build_icicle", icicle_json)] {
            super::super::replay::render_svg(family, json, &scale, &mid_rows, true)
                .unwrap_or_else(|e| panic!("race-mode value interpolation must generalize to family '{family}' without any per-family special-casing in the engine: {e}"));
        }
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn perf_render_timed_frames_for_a_six_racer_smooth_battle_at_20ms_ticks() {
        let labels: Vec<String> = (0..6).map(|i| format!("Item {i}")).collect();
        let json = serde_json::json!({"title": "battle", "labels": labels, "values": vec![0.0f64; 6]}).to_string();
        let mut records = Vec::new();
        let mut rng_state: u32 = 7;
        let mut values = [50i16, 45, 40, 35, 30, 25];
        for step in 0..20u32 {
            for v in values.iter_mut() {
                rng_state = rng_state.wrapping_mul(1103515245).wrapping_add(12345);
                let delta = ((rng_state >> 16) % 7) as i16 - 3;
                *v = (*v + delta).clamp(0, 99);
            }
            records.push(Record { dt_ms: step * 200, n: 6, vpp: 1, indices: vec![0, 1, 2, 3, 4, 5], values: values.to_vec() });
        }
        let recording = scalar_recording_with_source(records, "build_bar", &json);
        let opts = GifOptions { smooth: true, smooth_tick_ms: 20, sort_by_value: true, ..GifOptions::default() };

        let started = std::time::Instant::now();
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        let elapsed = started.elapsed();
        println!(
            "render_timed_frames (6-racer smooth battle, 20 real records, 20ms ticks) -> {} frames in {elapsed:?} ({:?}/frame)",
            timed.len(),
            elapsed / timed.len().max(1) as u32
        );
        assert!(elapsed.as_secs() < 120, "the new re-rank-and-rerender-per-tick race path got pathologically slow: {elapsed:?} for {} frames (bound is loose to tolerate an unoptimized debug build)", timed.len());
    }

    #[cfg(feature = "sera-report")]
    #[test]
    fn perf_reveal_frames_for_a_300_point_bar_source_over_90_steps() {
        let n = 300;
        let labels: Vec<String> = (0..n).map(|i| format!("Item {i}")).collect();
        let values = vec![0.0f64; n];
        let json = serde_json::json!({"title": "reveal", "labels": labels, "values": values}).to_string();
        let records = vec![Record { dt_ms: 0, n: n as u32, vpp: 1, indices: (0..n as u32).collect(), values: (0..n).map(|i| (i % 90) as i16).collect() }];
        let recording = scalar_recording_with_source(records, "build_bar", &json);
        let opts = GifOptions { reveal: true, reveal_steps: 90, ..GifOptions::default() };

        let started = std::time::Instant::now();
        let timed = render_timed_frames(&recording, &opts).expect("should render");
        let elapsed = started.elapsed();
        println!(
            "render_reveal_frames (300-point bar source, 90 reveal steps) -> {} frames in {elapsed:?} ({:?}/frame)",
            timed.len(),
            elapsed / timed.len().max(1) as u32
        );
        assert!(elapsed.as_secs() < 300, "reveal mode's per-step re-render got pathologically slow: {elapsed:?} for {} frames (bound is loose to tolerate an unoptimized debug build)", timed.len());
    }
}
