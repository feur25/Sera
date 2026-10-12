use super::push_registry::{self, PushMeta};
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::sync::{Arc, Mutex};
use std::time::Instant;

const CAPACITY: usize = 64;
const MAGIC: &[u8; 4] = b"SPLS";
const VERSION: u8 = 4;

fn write_scale(file: &mut File, meta: &PushMeta) -> std::io::Result<()> {
    match meta {
        PushMeta::Point(p) => {
            file.write_all(&[0u8])?;
            file.write_all(&p.min_x.to_le_bytes())?;
            file.write_all(&p.range_x.to_le_bytes())?;
            file.write_all(&p.min_y.to_le_bytes())?;
            file.write_all(&p.range_y.to_le_bytes())?;
            file.write_all(&p.plot_w.to_le_bytes())?;
            file.write_all(&p.plot_h.to_le_bytes())?;
        }
        PushMeta::Scalar(s) => {
            file.write_all(&[1u8])?;
            file.write_all(&s.min_val.to_le_bytes())?;
            file.write_all(&s.range_val.to_le_bytes())?;
            file.write_all(&s.axis_px.to_le_bytes())?;
        }
        PushMeta::Vector(v) => {
            file.write_all(&[2u8])?;
            file.write_all(&v.min_val.to_le_bytes())?;
            file.write_all(&v.range_val.to_le_bytes())?;
            file.write_all(&v.axis_px.to_le_bytes())?;
            file.write_all(&v.vpp.to_le_bytes())?;
        }
    }
    Ok(())
}

fn write_source(file: &mut File, source: Option<&super::chart_source_registry::ChartSource>) -> std::io::Result<()> {
    match source {
        Some(src) => {
            file.write_all(&[1u8])?;
            let family_bytes = src.family.as_bytes();
            file.write_all(&(family_bytes.len() as u32).to_le_bytes())?;
            file.write_all(family_bytes)?;
            let json_bytes = src.json.as_bytes();
            file.write_all(&(json_bytes.len() as u32).to_le_bytes())?;
            file.write_all(json_bytes)?;
        }
        None => {
            file.write_all(&[0u8])?;
        }
    }
    Ok(())
}

fn f64_at(data: &[u8], pos: &mut usize) -> Option<f64> {
    let end = *pos + 8;
    let v = f64::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
    *pos = end;
    Some(v)
}

fn i32_at(data: &[u8], pos: &mut usize) -> Option<i32> {
    let end = *pos + 4;
    let v = i32::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
    *pos = end;
    Some(v)
}

fn i64_at(data: &[u8], pos: &mut usize) -> Option<i64> {
    let end = *pos + 8;
    let v = i64::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
    *pos = end;
    Some(v)
}

fn u32_at(data: &[u8], pos: &mut usize) -> Option<u32> {
    let end = *pos + 4;
    let v = u32::from_le_bytes(data.get(*pos..end)?.try_into().ok()?);
    *pos = end;
    Some(v)
}

fn read_scale(data: &[u8], pos: &mut usize) -> Option<PushMeta> {
    let tag = *data.get(*pos)?;
    *pos += 1;
    match tag {
        0 => Some(PushMeta::Point(push_registry::PointMeta {
            min_x: f64_at(data, pos)?,
            range_x: f64_at(data, pos)?,
            min_y: f64_at(data, pos)?,
            range_y: f64_at(data, pos)?,
            plot_w: i32_at(data, pos)?,
            plot_h: i32_at(data, pos)?,
        })),
        1 => Some(PushMeta::Scalar(push_registry::ScalarMeta {
            min_val: f64_at(data, pos)?,
            range_val: f64_at(data, pos)?,
            axis_px: i32_at(data, pos)?,
        })),
        2 => Some(PushMeta::Vector(push_registry::VectorMeta {
            min_val: f64_at(data, pos)?,
            range_val: f64_at(data, pos)?,
            axis_px: i32_at(data, pos)?,
            vpp: u32_at(data, pos)?,
        })),
        _ => None,
    }
}

fn read_source(data: &[u8], pos: &mut usize) -> Option<(String, String)> {
    let tag = *data.get(*pos)?;
    *pos += 1;
    if tag == 0 {
        return None;
    }
    let family_len = u32_at(data, pos)? as usize;
    let family = String::from_utf8_lossy(data.get(*pos..*pos + family_len)?).to_string();
    *pos += family_len;
    let json_len = u32_at(data, pos)? as usize;
    let json = String::from_utf8_lossy(data.get(*pos..*pos + json_len)?).to_string();
    *pos += json_len;
    Some((family, json))
}

pub struct RecordState {
    file: Mutex<File>,
    start: Instant,
    path: String,
}

impl RecordState {
    pub fn append(&self, frame: &[u8]) {
        let dt_ms = self.start.elapsed().as_millis().min(u32::MAX as u128) as u32;
        let mut header = [0u8; 8];
        header[0..4].copy_from_slice(&dt_ms.to_le_bytes());
        header[4..8].copy_from_slice(&(frame.len() as u32).to_le_bytes());
        if let Ok(mut f) = self.file.lock() {
            let _ = f.write_all(&header);
            let _ = f.write_all(frame);
        }
    }
}

struct Registry {
    map: HashMap<u64, Arc<RecordState>>,
    last_path: HashMap<u64, String>,
    last_path_order: std::collections::VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn start(hid: u64, path: &str, meta: &PushMeta) -> std::io::Result<()> {
    let mut file = File::create(path)?;
    file.write_all(MAGIC)?;
    file.write_all(&[VERSION, 0, 0, 0])?;
    write_scale(&mut file, meta)?;
    file.write_all(&crate::plot::now_ms().to_le_bytes())?;
    write_source(&mut file, super::chart_source_registry::get(hid).as_ref())?;
    let state = Arc::new(RecordState {
        file: Mutex::new(file),
        start: Instant::now(),
        path: path.to_string(),
    });
    let Ok(mut guard) = REGISTRY.lock() else {
        return Ok(());
    };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), last_path: HashMap::new(), last_path_order: std::collections::VecDeque::new() });
    if reg.map.len() >= CAPACITY && !reg.map.contains_key(&hid) {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "too many concurrent recordings"));
    }
    reg.map.insert(hid, state);
    if !reg.last_path.contains_key(&hid) {
        reg.last_path_order.push_back(hid);
        while reg.last_path_order.len() > CAPACITY {
            if let Some(old) = reg.last_path_order.pop_front() {
                reg.last_path.remove(&old);
            }
        }
    }
    reg.last_path.insert(hid, path.to_string());
    Ok(())
}

pub fn get(hid: u64) -> Option<Arc<RecordState>> {
    REGISTRY.lock().ok()?.as_ref()?.map.get(&hid).cloned()
}

pub fn path_of(hid: u64) -> Option<String> {
    let guard = REGISTRY.lock().ok()?;
    let reg = guard.as_ref()?;
    if let Some(state) = reg.map.get(&hid) {
        return Some(state.path.clone());
    }
    reg.last_path.get(&hid).cloned()
}

pub fn stop(hid: u64) {
    if let Ok(mut guard) = REGISTRY.lock() {
        if let Some(reg) = guard.as_mut() {
            reg.map.remove(&hid);
        }
    }
}

pub struct Record {
    pub dt_ms: u32,
    pub n: u32,
    pub vpp: u32,
    pub indices: Vec<u32>,
    pub values: Vec<i16>,
}

pub struct Recording {
    pub scale: Option<PushMeta>,
    pub started_at_ms: Option<i64>,
    pub source: Option<(String, String)>,
    pub records: Vec<Record>,
}

pub fn read_all(path: &str) -> std::io::Result<Recording> {
    let data = std::fs::read(path)?;
    if data.len() < 8 || &data[0..4] != MAGIC {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "not a SeraPlot recording file"));
    }
    let version = data[4];
    let mut pos = 8usize;
    let scale = if version >= 2 { read_scale(&data, &mut pos) } else { None };
    let started_at_ms = if version >= 3 { i64_at(&data, &mut pos) } else { None };
    let source = if version >= 4 { read_source(&data, &mut pos) } else { None };
    let mut records = Vec::new();
    while pos + 8 <= data.len() {
        let dt_ms = u32::from_le_bytes(data[pos..pos + 4].try_into().unwrap());
        let frame_len = u32::from_le_bytes(data[pos + 4..pos + 8].try_into().unwrap()) as usize;
        pos += 8;
        if pos + frame_len > data.len() {
            break;
        }
        let frame = &data[pos..pos + frame_len];
        pos += frame_len;
        if frame.len() < 12 {
            continue;
        }
        let n = u32::from_le_bytes(frame[4..8].try_into().unwrap());
        let vpp = u32::from_le_bytes(frame[8..12].try_into().unwrap());
        let idx_end = 12 + (n as usize) * 4;
        if frame.len() < idx_end {
            continue;
        }
        let indices: Vec<u32> = frame[12..idx_end]
            .chunks_exact(4)
            .map(|c| u32::from_le_bytes(c.try_into().unwrap()))
            .collect();
        let vals_end = idx_end + (n as usize) * (vpp as usize) * 2;
        if frame.len() < vals_end {
            continue;
        }
        let values: Vec<i16> = frame[idx_end..vals_end]
            .chunks_exact(2)
            .map(|c| i16::from_le_bytes(c.try_into().unwrap()))
            .collect();
        records.push(Record { dt_ms, n, vpp, indices, values });
    }
    Ok(Recording { scale, started_at_ms, source, records })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tmp_path(name: &str) -> String {
        std::env::temp_dir().join(format!("sp_record_registry_test_{name}_{}.spls", std::process::id())).to_string_lossy().to_string()
    }

    fn frame(hid: u64, indices: &[u32], values: &[i16], vpp: u32) -> Vec<u8> {
        let n = indices.len() as u32;
        let mut f = Vec::with_capacity(12 + indices.len() * 4 + values.len() * 2);
        f.extend_from_slice(&(hid as u32).to_le_bytes());
        f.extend_from_slice(&n.to_le_bytes());
        f.extend_from_slice(&vpp.to_le_bytes());
        for i in indices {
            f.extend_from_slice(&i.to_le_bytes());
        }
        for v in values {
            f.extend_from_slice(&v.to_le_bytes());
        }
        f
    }

    #[test]
    fn scalar_scale_round_trips() {
        let path = tmp_path("scalar");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: -5.0, range_val: 42.0, axis_px: 300 });
        start(1, &path, &meta).expect("start");
        let state = get(1).expect("registered");
        state.append(&frame(1, &[0, 1, 2], &[10, 20, 30], 1));
        stop(1);

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        match recording.scale {
            Some(PushMeta::Scalar(s)) => {
                assert_eq!(s.min_val, -5.0);
                assert_eq!(s.range_val, 42.0);
                assert_eq!(s.axis_px, 300);
            }
            _ => panic!("expected scalar scale"),
        }
        assert_eq!(recording.records.len(), 1);
        assert_eq!(recording.records[0].indices, vec![0, 1, 2]);
        assert_eq!(recording.records[0].values, vec![10, 20, 30]);
    }

    #[test]
    fn vector_scale_round_trips() {
        let path = tmp_path("vector");
        let meta = PushMeta::Vector(push_registry::VectorMeta { min_val: 0.0, range_val: 1000.0, axis_px: 400, vpp: 4 });
        start(2, &path, &meta).expect("start");
        let state = get(2).expect("registered");
        state.append(&frame(2, &[7], &[1, 2, 3, 4], 4));
        stop(2);

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        match recording.scale {
            Some(PushMeta::Vector(v)) => {
                assert_eq!(v.vpp, 4);
                assert_eq!(v.axis_px, 400);
            }
            _ => panic!("expected vector scale"),
        }
        assert_eq!(recording.records[0].vpp, 4);
        assert_eq!(recording.records[0].values, vec![1, 2, 3, 4]);
    }

    #[test]
    fn point_scale_round_trips() {
        let path = tmp_path("point");
        let meta = PushMeta::Point(push_registry::PointMeta { min_x: 1.0, range_x: 9.0, min_y: -2.0, range_y: 8.0, plot_w: 640, plot_h: 480 });
        start(3, &path, &meta).expect("start");
        let state = get(3).expect("registered");
        state.append(&frame(3, &[0], &[100, 200], 2));
        stop(3);

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        match recording.scale {
            Some(PushMeta::Point(p)) => {
                assert_eq!(p.plot_w, 640);
                assert_eq!(p.plot_h, 480);
                assert_eq!(p.min_x, 1.0);
                assert_eq!(p.range_y, 8.0);
            }
            _ => panic!("expected point scale"),
        }
    }

    #[test]
    fn v1_files_without_scale_still_read() {
        let path = tmp_path("v1_legacy");
        {
            let mut f = File::create(&path).expect("create");
            f.write_all(MAGIC).unwrap();
            f.write_all(&[1u8, 0, 0, 0]).unwrap();
            let body = frame(4, &[0, 1], &[5, 6], 1);
            f.write_all(&0u32.to_le_bytes()).unwrap();
            f.write_all(&(body.len() as u32).to_le_bytes()).unwrap();
            f.write_all(&body).unwrap();
        }
        let recording = read_all(&path).expect("read legacy v1");
        let _ = std::fs::remove_file(&path);
        assert!(recording.scale.is_none());
        assert!(recording.started_at_ms.is_none(), "v1 files predate the wall-clock anchor too");
        assert_eq!(recording.records.len(), 1);
        assert_eq!(recording.records[0].values, vec![5, 6]);
    }

    #[test]
    fn v2_files_have_scale_but_no_wall_clock_anchor() {
        let path = tmp_path("v2_legacy");
        {
            let mut f = File::create(&path).expect("create");
            f.write_all(MAGIC).unwrap();
            f.write_all(&[2u8, 0, 0, 0]).unwrap();
            let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
            write_scale(&mut f, &meta).unwrap();
            let body = frame(5, &[0], &[3], 1);
            f.write_all(&0u32.to_le_bytes()).unwrap();
            f.write_all(&(body.len() as u32).to_le_bytes()).unwrap();
            f.write_all(&body).unwrap();
        }
        let recording = read_all(&path).expect("read legacy v2");
        let _ = std::fs::remove_file(&path);
        assert!(recording.scale.is_some(), "v2 files do have scale");
        assert!(recording.started_at_ms.is_none(), "v2 predates the wall-clock anchor added in v3");
    }

    #[test]
    fn new_recordings_capture_a_real_wall_clock_anchor() {
        let path = tmp_path("v3_anchor");
        let before = crate::plot::now_ms();
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(6, &path, &meta).expect("start");
        let state = get(6).expect("registered");
        state.append(&frame(6, &[0], &[1], 1));
        stop(6);
        let after = crate::plot::now_ms();

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        let anchor = recording.started_at_ms.expect("new recordings should carry a wall-clock anchor");
        assert!(anchor >= before && anchor <= after, "anchor {anchor} should fall within [{before}, {after}]");
    }

    #[test]
    fn a_registered_chart_source_is_captured_at_start_and_round_trips_through_read_all() {
        let hid = 7u64;
        super::super::chart_source_registry::register(hid, "build_bar", "{\"title\":\"t\",\"values\":[1,2,3]}".to_string());
        let path = tmp_path("v4_source");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(hid, &path, &meta).expect("start");
        let state = get(hid).expect("registered");
        state.append(&frame(hid, &[0], &[5], 1));
        stop(hid);

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        let (family, json) = recording.source.expect("source should have been captured at start()");
        assert_eq!(family, "build_bar");
        assert_eq!(json, "{\"title\":\"t\",\"values\":[1,2,3]}");
    }

    #[test]
    fn a_chart_with_no_registered_source_records_fine_with_source_absent() {
        let hid = 8u64;
        let path = tmp_path("v4_no_source");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(hid, &path, &meta).expect("start");
        let state = get(hid).expect("registered");
        state.append(&frame(hid, &[0], &[5], 1));
        stop(hid);

        let recording = read_all(&path).expect("read");
        let _ = std::fs::remove_file(&path);
        assert!(recording.source.is_none());
        assert_eq!(recording.records.len(), 1, "absence of a source must not affect reading the actual delta records");
    }

    #[test]
    fn v3_files_without_a_source_section_still_read_with_source_none() {
        let path = tmp_path("v3_no_source_section");
        {
            let mut f = File::create(&path).expect("create");
            f.write_all(MAGIC).unwrap();
            f.write_all(&[3u8, 0, 0, 0]).unwrap();
            let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
            write_scale(&mut f, &meta).unwrap();
            f.write_all(&crate::plot::now_ms().to_le_bytes()).unwrap();
            let body = frame(9, &[0], &[3], 1);
            f.write_all(&0u32.to_le_bytes()).unwrap();
            f.write_all(&(body.len() as u32).to_le_bytes()).unwrap();
            f.write_all(&body).unwrap();
        }
        let recording = read_all(&path).expect("read legacy v3");
        let _ = std::fs::remove_file(&path);
        assert!(recording.source.is_none(), "v3 predates the source section added in v4");
        assert_eq!(recording.records.len(), 1);
    }

    #[test]
    fn path_of_resolves_a_currently_active_recording() {
        let hid = 500;
        let path = tmp_path("path_of_active");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(hid, &path, &meta).expect("start");
        assert_eq!(path_of(hid), Some(path.clone()));
        stop(hid);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn path_of_still_resolves_after_stop_so_a_finished_recording_can_still_be_replayed() {
        let hid = 501;
        let path = tmp_path("path_of_after_stop");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(hid, &path, &meta).expect("start");
        stop(hid);
        assert_eq!(path_of(hid), Some(path.clone()), "the path must still resolve so a finished recording can be shared-replayed later");
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn path_of_an_unknown_hid_is_none() {
        assert_eq!(path_of(999_999), None);
    }

    #[test]
    fn starting_a_new_recording_on_the_same_hid_updates_the_remembered_path() {
        let hid = 502;
        let path_a = tmp_path("path_of_first");
        let path_b = tmp_path("path_of_second");
        let meta = PushMeta::Scalar(push_registry::ScalarMeta { min_val: 0.0, range_val: 10.0, axis_px: 10 });
        start(hid, &path_a, &meta).expect("start a");
        stop(hid);
        start(hid, &path_b, &meta).expect("start b");
        assert_eq!(path_of(hid), Some(path_b.clone()));
        stop(hid);
        let _ = std::fs::remove_file(&path_a);
        let _ = std::fs::remove_file(&path_b);
    }
}
