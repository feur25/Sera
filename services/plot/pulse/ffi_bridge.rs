use super::frame_codec::{build_binary_frame, build_replay_script};
use super::push_registry::{self, PushMeta};
use crate::plot::canvas_points::{b64_encode, pack_indices_u32_raw, pack_points_i16_raw, pack_scalar_i16_raw, pack_vector_i16_raw};

fn require_meta(html: &str) -> Result<(u64, PushMeta), String> {
    let hid = push_registry::extract_hid(html).ok_or_else(|| {
        "push()/serve() require a chart built via one of the framework's native fast paths (enough points/bars to cross the render threshold)".to_string()
    })?;
    let meta = push_registry::get(hid).ok_or_else(|| {
        "push()/push_vector()/serve() need incremental-update metadata that only charts built via a native fast path register (scatter/line/bubble/area for x/y point data, bar/heatmap for label/value data, candlestick for OHLC data, each above its render threshold); this chart either has a different type, was too small to use that path, or its metadata aged out of the registry".to_string()
    })?;
    Ok((hid, meta))
}

fn record_and_broadcast(hid: u64, idx_raw: &[u8], pts_raw: &[u8], n: u32, vpp: u32) {
    let needs_frame = super::record_registry::get(hid).is_some();
    #[cfg(feature = "webapp")]
    let needs_frame = needs_frame || push_registry::get_push_tx(hid).is_some();
    if !needs_frame {
        return;
    }
    let frame = build_binary_frame(hid, idx_raw, pts_raw, n, vpp);
    if let Some(rec) = super::record_registry::get(hid) {
        rec.append(&frame);
    }
    #[cfg(feature = "webapp")]
    if let Some(tx) = push_registry::get_push_tx(hid) {
        let _ = tx.send((crate::webapp::ws::OPCODE_BINARY, frame));
    }
}

pub fn record(html: &str, path: &str) -> Result<(), String> {
    let (hid, meta) = require_meta(html)?;
    super::record_registry::start(hid, path, &meta).map_err(|e| format!("seraplot: failed to start recording to {path}: {e}"))
}

pub fn stop_record(html: &str) -> Result<(), String> {
    let (hid, _) = require_meta(html)?;
    super::record_registry::stop(hid);
    Ok(())
}

fn require_hid(html: &str) -> Result<u64, String> {
    push_registry::extract_hid(html).ok_or_else(|| "this method requires a chart built via one of the framework's native fast paths".to_string())
}

fn gif_options_from_json(obj: &serde_json::Map<String, serde_json::Value>) -> super::studio::GifOptions {
    let mut opts = super::studio::GifOptions::default();
    if let Some(v) = obj.get("width").and_then(|v| v.as_u64()) {
        opts.width = v as u32;
    }
    if let Some(v) = obj.get("height").and_then(|v| v.as_u64()) {
        opts.height = v as u32;
    }
    if let Some(v) = obj.get("max_frames").and_then(|v| v.as_u64()) {
        opts.max_frames = v as usize;
    }
    if let Some(a) = obj.get("color").and_then(|v| v.as_array()) {
        opts.color = (
            a.first().and_then(|v| v.as_u64()).unwrap_or(opts.color.0 as u64) as u8,
            a.get(1).and_then(|v| v.as_u64()).unwrap_or(opts.color.1 as u64) as u8,
            a.get(2).and_then(|v| v.as_u64()).unwrap_or(opts.color.2 as u64) as u8,
        );
    }
    if let Some(v) = obj.get("sort_by_value").and_then(|v| v.as_bool()) {
        opts.sort_by_value = v;
    }
    if let Some(v) = obj.get("smooth").and_then(|v| v.as_bool()) {
        opts.smooth = v;
    }
    if let Some(v) = obj.get("smooth_speed").and_then(|v| v.as_f64()) {
        opts.smooth_speed = v;
    }
    if let Some(v) = obj.get("smooth_ease").and_then(|v| v.as_str()) {
        opts.smooth_ease = v.to_string();
    }
    if let Some(v) = obj.get("reveal").and_then(|v| v.as_bool()) {
        opts.reveal = v;
    }
    if let Some(v) = obj.get("reveal_steps").and_then(|v| v.as_u64()) {
        opts.reveal_steps = v as u32;
    }
    if let Some(v) = obj.get("reveal_fixed_scale").and_then(|v| v.as_bool()) {
        opts.reveal_fixed_scale = v;
    }
    let at_ms: Vec<u32> = obj.get("breakpoint_at_ms").and_then(|v| v.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64()).map(|n| n as u32).collect()).unwrap_or_default();
    let on_value: Vec<(u32, f64)> = obj
        .get("breakpoint_on_value")
        .and_then(|v| v.as_array())
        .map(|a| {
            a.iter()
                .filter_map(|pair| {
                    let p = pair.as_array()?;
                    Some((p.first()?.as_u64()? as u32, p.get(1)?.as_f64()?))
                })
                .collect()
        })
        .unwrap_or_default();
    if !at_ms.is_empty() || !on_value.is_empty() {
        let pause_ms = obj.get("breakpoint_pause_ms").and_then(|v| v.as_u64()).unwrap_or(1000) as u32;
        opts.breakpoints = super::studio::build_breakpoints(at_ms, on_value, pause_ms);
    }
    opts
}

pub fn export_gif(html: &str, gif_path: &str, spls_path: &str, args: &serde_json::Map<String, serde_json::Value>) -> Result<(), String> {
    require_hid(html)?;
    let recording = super::studio::read_recording(spls_path)?;
    let opts = gif_options_from_json(args);
    let gif = super::studio::render_gif(&recording, &opts)?;
    super::frame_codec::atomic_write(gif_path, &gif).map_err(|e| format!("seraplot: failed to write {gif_path}: {e}"))
}

pub fn gif_frame_count(html: &str, spls_path: &str) -> Result<usize, String> {
    require_hid(html)?;
    let recording = super::studio::read_recording(spls_path)?;
    Ok(super::studio::count_frames(&recording))
}

pub fn gif_preview_frame(
    html: &str,
    spls_path: &str,
    frame_index: usize,
    png_path: &str,
    args: &serde_json::Map<String, serde_json::Value>,
) -> Result<(), String> {
    require_hid(html)?;
    let recording = super::studio::read_recording(spls_path)?;
    let opts = gif_options_from_json(args);
    let img = super::studio::render_single_frame(&recording, &opts, frame_index)?;
    img.save(png_path).map_err(|e| format!("seraplot: failed to write {png_path}: {e}"))
}

pub fn export_video(html: &str, video_path: &str, spls_path: &str, format: &str, fps: u32, args: &serde_json::Map<String, serde_json::Value>) -> Result<(), String> {
    require_hid(html)?;
    let video_format = super::studio::parse_video_format(format)?;
    let recording = super::studio::read_recording(spls_path)?;
    let fps = fps.max(1);
    let mut opts = gif_options_from_json(args);
    opts.width = super::studio::even_dim(opts.width);
    opts.height = super::studio::even_dim(opts.height);
    opts.smooth_tick_ms = (1000 / fps).max(1);
    let timed_frames = super::studio::render_timed_frames(&recording, &opts)?;
    super::studio::encode_video(&timed_frames, fps, video_format, video_path).map_err(|e| format!("seraplot: {e}"))
}

pub fn export_standalone(html: &str, html_path: &str, spls_path: &str) -> Result<(), String> {
    let (hid, _) = require_meta(html)?;
    let records = super::record_registry::read_all(spls_path)
        .map_err(|e| format!("seraplot: failed to read recording {spls_path}: {e}"))?
        .records;
    let script = build_replay_script(hid, &records);
    let final_html = crate::html::hover::inject_before_body(html, &script);
    std::fs::write(html_path, final_html).map_err(|e| format!("seraplot: failed to write {html_path}: {e}"))
}

pub fn push(html: &str, indices: &[u32], x: &[f64], y: Option<&[f64]>) -> Result<(String, String), String> {
    let (hid, meta) = require_meta(html)?;
    let idx_raw = pack_indices_u32_raw(indices);
    let (pts_raw, vpp): (Vec<u8>, u32) = match &meta {
        PushMeta::Point(pm) => {
            let Some(y) = y else {
                return Err("this chart's push() needs both x and y (it was built from x/y point data)".to_string());
            };
            (pack_points_i16_raw(x, y, pm.min_x, pm.range_x, pm.min_y, pm.range_y, pm.plot_w, pm.plot_h), 2)
        }
        PushMeta::Scalar(sm) => {
            if y.is_some() {
                return Err("this chart's push() takes a single value array (it was built from label/value data): push(indices, values)".to_string());
            }
            (pack_scalar_i16_raw(x, sm.min_val, sm.range_val, sm.axis_px), 1)
        }
        PushMeta::Vector(_) => {
            return Err(
                "this chart was built from more than two parallel value arrays (e.g. OHLC): use push_vector(indices, [array1, array2, ...]) instead of push()"
                    .to_string(),
            );
        }
    };
    record_and_broadcast(hid, &idx_raw, &pts_raw, indices.len() as u32, vpp);
    Ok((b64_encode(&idx_raw), b64_encode(&pts_raw)))
}

pub fn push_vector(html: &str, indices: &[u32], components: &[Vec<f64>]) -> Result<(String, String), String> {
    let (hid, meta) = require_meta(html)?;
    let vm = match &meta {
        PushMeta::Vector(vm) => vm,
        _ => return Err("this chart was not built from more than two parallel value arrays: use push() instead of push_vector()".to_string()),
    };
    if components.len() as u32 != vm.vpp {
        return Err(format!("this chart expects {} parallel value arrays, got {}", vm.vpp, components.len()));
    }
    let idx_raw = pack_indices_u32_raw(indices);
    let refs: Vec<&[f64]> = components.iter().map(|c| c.as_slice()).collect();
    let pts_raw = pack_vector_i16_raw(&refs, vm.min_val, vm.range_val, vm.axis_px);
    let vpp = vm.vpp;
    record_and_broadcast(hid, &idx_raw, &pts_raw, indices.len() as u32, vpp);
    Ok((b64_encode(&idx_raw), b64_encode(&pts_raw)))
}

fn json_f64_vec(v: &serde_json::Value, key: &str) -> Vec<f64> {
    v.get(key).and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default()
}

fn json_u32_vec(v: &serde_json::Value, key: &str) -> Vec<u32> {
    v.get(key).and_then(|a| a.as_array()).map(|a| a.iter().filter_map(|x| x.as_u64()).map(|n| n as u32).collect()).unwrap_or_default()
}

fn json_str<'a>(v: &'a serde_json::Value, key: &str) -> &'a str {
    v.get(key).and_then(|s| s.as_str()).unwrap_or("")
}

fn call_record(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    record(html, json_str(&v, "path"))?;
    Ok("{}".to_string())
}

fn call_stop_record(html: &str, _args: &str) -> Result<String, String> {
    stop_record(html)?;
    Ok("{}".to_string())
}

fn call_export_standalone(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    export_standalone(html, json_str(&v, "html_path"), json_str(&v, "spls_path"))?;
    Ok("{}".to_string())
}

fn call_push(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let indices = json_u32_vec(&v, "indices");
    let x = json_f64_vec(&v, "x");
    let y = json_f64_vec(&v, "y");
    let y_opt = if v.get("y").map(|y| y.is_null()).unwrap_or(true) { None } else { Some(y.as_slice()) };
    let (idx_b64, pts_b64) = push(html, &indices, &x, y_opt)?;
    Ok(format!("{{\"idx_b64\":{:?},\"pts_b64\":{:?}}}", idx_b64, pts_b64))
}

fn call_push_vector(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let indices = json_u32_vec(&v, "indices");
    let components: Vec<Vec<f64>> = v
        .get("components")
        .and_then(|a| a.as_array())
        .map(|outer| outer.iter().map(|inner| inner.as_array().map(|a| a.iter().filter_map(|x| x.as_f64()).collect()).unwrap_or_default()).collect())
        .unwrap_or_default();
    let (idx_b64, pts_b64) = push_vector(html, &indices, &components)?;
    Ok(format!("{{\"idx_b64\":{:?},\"pts_b64\":{:?}}}", idx_b64, pts_b64))
}

fn empty_obj() -> serde_json::Map<String, serde_json::Value> {
    serde_json::Map::new()
}

fn call_export_gif(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let obj = v.as_object().cloned().unwrap_or_else(empty_obj);
    export_gif(html, json_str(&v, "gif_path"), json_str(&v, "spls_path"), &obj)?;
    Ok("{}".to_string())
}

fn call_gif_frame_count(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let n = gif_frame_count(html, json_str(&v, "spls_path"))?;
    Ok(format!("{{\"frame_count\":{n}}}"))
}

fn call_gif_preview_frame(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let obj = v.as_object().cloned().unwrap_or_else(empty_obj);
    let frame_index = v.get("frame_index").and_then(|n| n.as_u64()).unwrap_or(0) as usize;
    gif_preview_frame(html, json_str(&v, "spls_path"), frame_index, json_str(&v, "png_path"), &obj)?;
    Ok("{}".to_string())
}

fn call_export_video(html: &str, args: &str) -> Result<String, String> {
    let v: serde_json::Value = serde_json::from_str(args).map_err(|e| format!("seraplot: invalid args json: {e}"))?;
    let obj = v.as_object().cloned().unwrap_or_else(empty_obj);
    let format = v.get("format").and_then(|s| s.as_str()).unwrap_or("mp4");
    let fps = v.get("fps").and_then(|n| n.as_u64()).unwrap_or(30) as u32;
    export_video(html, json_str(&v, "video_path"), json_str(&v, "spls_path"), format, fps, &obj)?;
    Ok("{}".to_string())
}

inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "record", apply: call_record } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "stop_record", apply: call_stop_record } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "export_standalone", apply: call_export_standalone } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "push", apply: call_push } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "push_vector", apply: call_push_vector } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "export_gif", apply: call_export_gif } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "gif_frame_count", apply: call_gif_frame_count } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "gif_preview_frame", apply: call_gif_preview_frame } }
inventory::submit! { crate::bindings::pulse_registry::PulseMethodEntry { name: "export_video", apply: call_export_video } }

#[cfg(test)]
mod tests {
    use super::*;

    fn html_with_scalar_meta() -> (String, u64) {
        let hid = crate::html::hover::html_id();
        let html = format!("<div id=\"spp{hid}\"></div>");
        push_registry::register(hid, PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 100.0, axis_px: 300 }));
        (html, hid)
    }

    fn html_with_point_meta() -> (String, u64) {
        let hid = crate::html::hover::html_id();
        let html = format!("<div id=\"spp{hid}\"></div>");
        push_registry::register(
            hid,
            PushMeta::Point(push_registry::PointMeta { min_x: 0.0, range_x: 10.0, min_y: 0.0, range_y: 10.0, plot_w: 400, plot_h: 300 }),
        );
        (html, hid)
    }

    fn html_with_vector_meta(vpp: u32) -> (String, u64) {
        let hid = crate::html::hover::html_id();
        let html = format!("<div id=\"spp{hid}\"></div>");
        push_registry::register(hid, PushMeta::Vector(push_registry::VectorMeta { min_val: 0.0, range_val: 100.0, axis_px: 300, vpp }));
        (html, hid)
    }

    #[test]
    fn record_then_push_then_stop_record_round_trips_a_real_spls_file() {
        let (html, _hid) = html_with_scalar_meta();
        let path = std::env::temp_dir().join(format!("ffi_bridge_test_{}.spls", crate::plot::now_ms()));
        let path_s = path.to_string_lossy().to_string();
        record(&html, &path_s).unwrap();
        push(&html, &[3], &[42.0], None).unwrap();
        stop_record(&html).unwrap();
        assert!(std::fs::metadata(&path).unwrap().len() > 0);
        std::fs::remove_file(&path).ok();
    }

    fn record_a_few_pushes(html: &str) -> String {
        let path = std::env::temp_dir().join(format!("ffi_bridge_export_test_{}.spls", crate::plot::now_ms()));
        let path_s = path.to_string_lossy().to_string();
        record(html, &path_s).unwrap();
        for i in 0..5 {
            push(html, &[i], &[10.0 + i as f64], None).unwrap();
        }
        stop_record(html).unwrap();
        path_s
    }

    #[test]
    fn gif_frame_count_matches_the_number_of_pushes_recorded() {
        let (html, _hid) = html_with_scalar_meta();
        let spls_path = record_a_few_pushes(&html);
        let n = gif_frame_count(&html, &spls_path).unwrap();
        assert_eq!(n, 5);
        std::fs::remove_file(&spls_path).ok();
    }

    #[test]
    fn export_gif_writes_a_real_gif_with_a_valid_header() {
        let (html, _hid) = html_with_scalar_meta();
        let spls_path = record_a_few_pushes(&html);
        let gif_path = std::env::temp_dir().join(format!("ffi_bridge_test_{}.gif", crate::plot::now_ms()));
        let gif_path_s = gif_path.to_string_lossy().to_string();
        export_gif(&html, &gif_path_s, &spls_path, &empty_obj()).unwrap();
        let bytes = std::fs::read(&gif_path).unwrap();
        assert!(bytes.len() > 6);
        assert!(&bytes[0..3] == b"GIF");
        std::fs::remove_file(&spls_path).ok();
        std::fs::remove_file(&gif_path).ok();
    }

    #[test]
    fn export_video_writes_a_real_mp4_with_a_valid_ftyp_box() {
        let (html, _hid) = html_with_scalar_meta();
        let spls_path = record_a_few_pushes(&html);
        let video_path = std::env::temp_dir().join(format!("ffi_bridge_test_{}.mp4", crate::plot::now_ms()));
        let video_path_s = video_path.to_string_lossy().to_string();
        let mut args = empty_obj();
        args.insert("width".to_string(), serde_json::json!(320));
        args.insert("height".to_string(), serde_json::json!(240));
        export_video(&html, &video_path_s, &spls_path, "mp4", 30, &args).unwrap();
        let bytes = std::fs::read(&video_path).unwrap();
        assert!(bytes.len() > 16);
        assert!(bytes.windows(4).any(|w| w == b"ftyp"));
        std::fs::remove_file(&spls_path).ok();
        std::fs::remove_file(&video_path).ok();
    }

    #[test]
    fn gif_preview_frame_writes_a_real_png() {
        let (html, _hid) = html_with_scalar_meta();
        let spls_path = record_a_few_pushes(&html);
        let png_path = std::env::temp_dir().join(format!("ffi_bridge_test_{}.png", crate::plot::now_ms()));
        let png_path_s = png_path.to_string_lossy().to_string();
        gif_preview_frame(&html, &spls_path, 0, &png_path_s, &empty_obj()).unwrap();
        let bytes = std::fs::read(&png_path).unwrap();
        assert!(bytes.starts_with(&[0x89, b'P', b'N', b'G']));
        std::fs::remove_file(&spls_path).ok();
        std::fs::remove_file(&png_path).ok();
    }

    #[test]
    fn export_gif_on_a_chart_with_no_hid_marker_is_rejected() {
        let err = export_gif("<div></div>", "x.gif", "x.spls", &empty_obj()).unwrap_err();
        assert!(err.contains("native fast path"));
    }

    #[test]
    fn call_dispatch_reaches_export_gif_and_export_video_through_pulse_registry() {
        assert!(crate::bindings::pulse_registry::find("export_gif").is_some());
        assert!(crate::bindings::pulse_registry::find("export_video").is_some());
        assert!(crate::bindings::pulse_registry::find("gif_frame_count").is_some());
        assert!(crate::bindings::pulse_registry::find("gif_preview_frame").is_some());
    }

    #[test]
    fn push_on_a_chart_with_no_registered_metadata_reports_the_native_fast_path_error() {
        let hid = crate::html::hover::html_id();
        let html = format!("<div id=\"spp{hid}\"></div>");
        let err = push(&html, &[0], &[1.0], None).unwrap_err();
        assert!(err.contains("native fast path"));
    }

    #[test]
    fn push_with_y_on_a_scalar_chart_is_rejected() {
        let (html, _) = html_with_scalar_meta();
        let err = push(&html, &[0], &[1.0], Some(&[2.0])).unwrap_err();
        assert!(err.contains("single value array"));
    }

    #[test]
    fn push_without_y_on_a_point_chart_is_rejected() {
        let (html, _) = html_with_point_meta();
        let err = push(&html, &[0], &[1.0], None).unwrap_err();
        assert!(err.contains("needs both x and y"));
    }

    #[test]
    fn push_on_a_vector_chart_directs_the_caller_to_push_vector() {
        let (html, _) = html_with_vector_meta(4);
        let err = push(&html, &[0], &[1.0], None).unwrap_err();
        assert!(err.contains("push_vector"));
    }

    #[test]
    fn push_vector_on_a_non_vector_chart_directs_the_caller_to_push() {
        let (html, _) = html_with_scalar_meta();
        let err = push_vector(&html, &[0], &[vec![1.0]]).unwrap_err();
        assert!(err.contains("use push() instead"));
    }

    #[test]
    fn push_vector_rejects_a_component_count_mismatch() {
        let (html, _) = html_with_vector_meta(4);
        let err = push_vector(&html, &[0], &[vec![1.0], vec![2.0]]).unwrap_err();
        assert!(err.contains("expects 4"));
    }

    #[test]
    fn push_vector_accepts_the_exact_declared_component_count() {
        let (html, _) = html_with_vector_meta(4);
        let (idx_b64, pts_b64) = push_vector(&html, &[0], &[vec![1.0], vec![2.0], vec![3.0], vec![4.0]]).unwrap();
        assert!(!idx_b64.is_empty());
        assert!(!pts_b64.is_empty());
    }

    #[test]
    fn push_returns_base64_that_decodes_back_to_the_indices_given() {
        let (html, _hid) = html_with_scalar_meta();
        use base64::Engine;
        let (idx_b64, pts_b64) = push(&html, &[7, 9], &[55.0, 12.0], None).unwrap();
        let idx_raw = base64::engine::general_purpose::STANDARD.decode(&idx_b64).unwrap();
        let pts_raw = base64::engine::general_purpose::STANDARD.decode(&pts_b64).unwrap();
        assert_eq!(idx_raw.len(), 8);
        assert_eq!(pts_raw.len(), 4);
    }
}
