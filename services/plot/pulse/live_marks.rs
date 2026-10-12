use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 256;
const MAX_MARKS_PER_ROOM: usize = 400;
const MAX_IMAGE_DATA_LEN: usize = 400_000;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
#[serde(tag = "k")]
pub enum Mark {
    #[serde(rename = "pin")]
    Pin { x: i32, y: i32, a: i32, label: String, n: String, c: String },
    #[serde(rename = "stroke")]
    Stroke { points: Vec<(i32, i32)>, a: i32, w: i32, c: String },
    #[serde(rename = "image")]
    Image { x: i32, y: i32, a: i32, w: i32, h: i32, data: String },
}

impl Mark {
    fn recolor(&mut self, color: &str) -> bool {
        match self {
            Mark::Pin { c, .. } | Mark::Stroke { c, .. } => {
                *c = color.to_string();
                true
            }
            Mark::Image { .. } => false,
        }
    }

    fn reposition(&mut self, nx: i32, ny: i32) -> bool {
        match self {
            Mark::Pin { x, y, .. } | Mark::Image { x, y, .. } => {
                *x = nx;
                *y = ny;
                true
            }
            Mark::Stroke { .. } => false,
        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct MarkEntry {
    pub id: u64,
    #[serde(flatten)]
    pub mark: Mark,
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct LayoutEntry {
    pub index: i32,
    pub x: i32,
    pub y: i32,
}

#[derive(Default, serde::Serialize, serde::Deserialize)]
struct SessionFile {
    #[serde(default)]
    marks: Vec<MarkEntry>,
    #[serde(default)]
    layout: Vec<LayoutEntry>,
}

struct Room {
    items: VecDeque<MarkEntry>,
    next_id: u64,
    layout: HashMap<i32, (i32, i32)>,
    persist_path: Option<String>,
}

impl Default for Room {
    fn default() -> Self {
        Room { items: VecDeque::new(), next_id: 1, layout: HashMap::new(), persist_path: None }
    }
}

struct Registry {
    map: HashMap<u64, Room>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

fn evict_room(reg: &mut Registry, hid: u64) {
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
}

fn persist(room: &Room) {
    let Some(path) = &room.persist_path else { return };
    let file = SessionFile {
        marks: room.items.iter().cloned().collect(),
        layout: room.layout.iter().map(|(&index, &(x, y))| LayoutEntry { index, x, y }).collect(),
    };
    let Ok(json) = serde_json::to_string_pretty(&file) else { return };
    let _ = super::frame_codec::atomic_write(path, &json);
}

pub fn add(hid: u64, mark: Mark) -> u64 {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    let id = room.next_id;
    room.next_id += 1;
    room.items.push_back(MarkEntry { id, mark });
    while room.items.len() > MAX_MARKS_PER_ROOM {
        room.items.pop_front();
    }
    persist(room);
    id
}

pub fn add_image(hid: u64, x: i32, y: i32, a: i32, w: i32, h: i32, data: String) -> Option<u64> {
    if data.len() > MAX_IMAGE_DATA_LEN {
        return None;
    }
    Some(add(hid, Mark::Image { x, y, a, w, h, data }))
}

pub fn remove(hid: u64, id: u64) -> bool {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return false };
    let Some(room) = reg.map.get_mut(&hid) else { return false };
    let before = room.items.len();
    room.items.retain(|e| e.id != id);
    let changed = room.items.len() != before;
    if changed {
        persist(room);
    }
    changed
}

pub fn recolor(hid: u64, id: u64, color: &str) -> bool {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return false };
    let Some(room) = reg.map.get_mut(&hid) else { return false };
    let Some(entry) = room.items.iter_mut().find(|e| e.id == id) else { return false };
    let changed = entry.mark.recolor(color);
    if changed {
        persist(room);
    }
    changed
}

pub fn reposition(hid: u64, id: u64, x: i32, y: i32) -> bool {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return false };
    let Some(room) = reg.map.get_mut(&hid) else { return false };
    let Some(entry) = room.items.iter_mut().find(|e| e.id == id) else { return false };
    let changed = entry.mark.reposition(x, y);
    if changed {
        persist(room);
    }
    changed
}

pub fn list(hid: u64) -> Vec<MarkEntry> {
    let guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    guard.as_ref().and_then(|r| r.map.get(&hid)).map(|room| room.items.iter().cloned().collect()).unwrap_or_default()
}

pub fn clear(hid: u64) {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return };
    let Some(room) = reg.map.get_mut(&hid) else { return };
    room.items.clear();
    persist(room);
}

pub fn set_layout(hid: u64, index: i32, x: i32, y: i32) {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    room.layout.insert(index, (x, y));
    persist(room);
}

pub fn list_layout(hid: u64) -> Vec<LayoutEntry> {
    let guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    guard
        .as_ref()
        .and_then(|r| r.map.get(&hid))
        .map(|room| room.layout.iter().map(|(&index, &(x, y))| LayoutEntry { index, x, y }).collect())
        .unwrap_or_default()
}

pub fn enable_persistence(hid: u64, path: &str) {
    let loaded: SessionFile = std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    if room.items.is_empty() && !loaded.marks.is_empty() {
        room.next_id = room.next_id.max(loaded.marks.iter().map(|e| e.id).max().unwrap_or(0) + 1);
        room.items = loaded.marks.into();
    }
    if room.layout.is_empty() && !loaded.layout.is_empty() {
        room.layout = loaded.layout.into_iter().map(|l| (l.index, (l.x, l.y))).collect();
    }
    room.persist_path = Some(path.to_string());
}

pub fn peek_mark_count(path: &str) -> usize {
    std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str::<SessionFile>(&s).ok()).map(|f| f.marks.len()).unwrap_or(0)
}

pub fn switch_session(hid: u64, path: &str) {
    let loaded: SessionFile = std::fs::read_to_string(path).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default();
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    room.next_id = loaded.marks.iter().map(|e| e.id).max().unwrap_or(0) + 1;
    room.items = loaded.marks.into();
    room.layout = loaded.layout.into_iter().map(|l| (l.index, (l.x, l.y))).collect();
    room.persist_path = Some(path.to_string());
}

pub fn disable_persistence(hid: u64) {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return };
    if let Some(room) = reg.map.get_mut(&hid) {
        room.persist_path = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_hid() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(810_000_000);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    fn pin(x: i32) -> Mark {
        Mark::Pin { x, y: 0, a: -1, label: "x".to_string(), n: "a".to_string(), c: "#fff".to_string() }
    }

    #[test]
    fn marks_added_are_returned_in_insertion_order_with_increasing_ids() {
        let hid = fresh_hid();
        let id_a = add(hid, pin(1));
        let id_b = add(hid, Mark::Stroke { points: vec![(0, 0), (1, 1)], a: -1, w: 3, c: "#000".to_string() });
        assert!(id_b > id_a);
        let marks = list(hid);
        assert_eq!(marks.len(), 2);
        assert_eq!(marks[0].id, id_a);
    }

    #[test]
    fn removing_a_mark_by_id_drops_only_that_one() {
        let hid = fresh_hid();
        let id_a = add(hid, pin(1));
        let id_b = add(hid, pin(2));
        assert!(remove(hid, id_a));
        assert!(!remove(hid, id_a));
        let marks = list(hid);
        assert_eq!(marks.len(), 1);
        assert_eq!(marks[0].id, id_b);
    }

    #[test]
    fn different_hids_do_not_leak_marks_into_each_other() {
        let hid_a = fresh_hid();
        let hid_b = fresh_hid();
        add(hid_a, pin(0));
        assert!(list(hid_b).is_empty());
    }

    #[test]
    fn clearing_a_room_removes_all_its_marks() {
        let hid = fresh_hid();
        add(hid, pin(0));
        clear(hid);
        assert!(list(hid).is_empty());
    }

    #[test]
    fn a_room_never_grows_past_the_per_room_cap() {
        let hid = fresh_hid();
        for i in 0..(MAX_MARKS_PER_ROOM + 50) {
            add(hid, pin(i as i32));
        }
        assert_eq!(list(hid).len(), MAX_MARKS_PER_ROOM);
    }

    #[test]
    fn unknown_room_returns_empty_not_panic() {
        assert!(list(999_999_999u64).is_empty());
        clear(999_999_999u64);
        assert!(!remove(999_999_999u64, 1));
    }

    #[test]
    fn persistence_writes_to_disk_on_every_mutation_and_survives_a_reload() {
        let hid = fresh_hid();
        let path = std::env::temp_dir().join(format!("sp_marks_test_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(hid, path.to_str().unwrap());
        add(hid, Mark::Pin { x: 5, y: 6, a: -1, label: "incident start".to_string(), n: "a".to_string(), c: "#fff".to_string() });

        let on_disk = std::fs::read_to_string(&path).expect("file should exist after add");
        assert!(on_disk.contains("incident start"));

        let new_hid = fresh_hid();
        enable_persistence(new_hid, path.to_str().unwrap());
        let recovered = list(new_hid);
        assert_eq!(recovered.len(), 1, "a fresh room pointed at the same session file should recover the saved mark");
        assert_eq!(recovered[0].id, 1);

        let _ = std::fs::remove_file(&path);
        clear(hid);
        clear(new_hid);
    }

    #[test]
    fn disabling_persistence_stops_further_writes_and_clearing_afterward_does_not_resurrect_the_file() {
        let hid = fresh_hid();
        let path = std::env::temp_dir().join(format!("sp_marks_disable_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(hid, path.to_str().unwrap());
        add(hid, pin(1));
        assert!(path.exists());

        let _ = std::fs::remove_file(&path);
        disable_persistence(hid);
        clear(hid);
        assert!(!path.exists(), "clearing after disabling persistence must not recreate the deleted file");

        clear(hid);
    }

    #[test]
    fn disabling_persistence_on_a_room_that_never_enabled_it_is_a_harmless_noop() {
        disable_persistence(999_999_996u64);
    }

    #[test]
    fn enabling_persistence_never_clobbers_marks_already_in_memory() {
        let hid = fresh_hid();
        let path = std::env::temp_dir().join(format!("sp_marks_no_clobber_{}.json", std::process::id()));
        std::fs::write(&path, "{}").unwrap();

        add(hid, pin(1));
        enable_persistence(hid, path.to_str().unwrap());

        assert_eq!(list(hid).len(), 1, "the in-memory mark should survive, not get overwritten by the empty file");

        let _ = std::fs::remove_file(&path);
        clear(hid);
    }

    #[test]
    fn recoloring_a_pin_changes_its_color_but_recoloring_an_image_is_rejected() {
        let hid = fresh_hid();
        let pin_id = add(hid, pin(1));
        let img_id = add_image(hid, 0, 0, -1, 40, 40, "data:image/png;base64,AAAA".to_string()).unwrap();
        assert!(recolor(hid, pin_id, "#ff0000"));
        assert!(!recolor(hid, img_id, "#ff0000"));
        let marks = list(hid);
        let pin_entry = marks.iter().find(|e| e.id == pin_id).unwrap();
        match &pin_entry.mark {
            Mark::Pin { c, .. } => assert_eq!(c, "#ff0000"),
            _ => panic!("expected pin"),
        }
    }

    #[test]
    fn repositioning_an_image_moves_it_but_repositioning_a_stroke_is_rejected() {
        let hid = fresh_hid();
        let img_id = add_image(hid, 0, 0, -1, 40, 40, "data:image/png;base64,AAAA".to_string()).unwrap();
        let stroke_id = add(hid, Mark::Stroke { points: vec![(0, 0), (1, 1)], a: -1, w: 3, c: "#000".to_string() });
        assert!(reposition(hid, img_id, 50, 60));
        assert!(!reposition(hid, stroke_id, 50, 60));
        let marks = list(hid);
        let img_entry = marks.iter().find(|e| e.id == img_id).unwrap();
        match &img_entry.mark {
            Mark::Image { x, y, .. } => {
                assert_eq!(*x, 50);
                assert_eq!(*y, 60);
            }
            _ => panic!("expected image"),
        }
    }

    #[test]
    fn an_oversized_image_payload_is_rejected() {
        let hid = fresh_hid();
        let huge = "a".repeat(MAX_IMAGE_DATA_LEN + 1);
        assert!(add_image(hid, 0, 0, -1, 10, 10, huge).is_none());
        assert!(list(hid).is_empty());
    }

    #[test]
    fn switch_session_force_replaces_marks_and_layout_and_starts_persisting_to_the_new_file() {
        let hid = fresh_hid();
        let path_a = std::env::temp_dir().join(format!("sp_switch_a_{}.json", std::process::id()));
        let path_b = std::env::temp_dir().join(format!("sp_switch_b_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path_a);
        let _ = std::fs::remove_file(&path_b);

        enable_persistence(hid, path_a.to_str().unwrap());
        add(hid, pin(1));
        set_layout(hid, 0, 10, 20);
        assert_eq!(list(hid).len(), 1);

        switch_session(hid, path_b.to_str().unwrap());
        assert!(list(hid).is_empty(), "switching to a session with no file yet should start empty");
        assert!(list_layout(hid).is_empty());

        add(hid, pin(2));
        assert!(path_b.exists(), "further mutations after switching should persist to the new file");
        let on_disk_a = std::fs::read_to_string(&path_a).unwrap();
        assert!(!on_disk_a.contains("\"id\":2"), "the old session file must not receive writes after switching away");

        switch_session(hid, path_a.to_str().unwrap());
        let restored = list(hid);
        assert_eq!(restored.len(), 1, "switching back should restore the first session's own marks");
        let restored_layout = list_layout(hid);
        assert!(restored_layout.iter().any(|l| l.index == 0 && l.x == 10 && l.y == 20));

        let _ = std::fs::remove_file(&path_a);
        let _ = std::fs::remove_file(&path_b);
        clear(hid);
    }

    #[test]
    fn frame_layout_persists_alongside_marks_and_survives_a_reload() {
        let hid = fresh_hid();
        let path = std::env::temp_dir().join(format!("sp_layout_test_{}.json", std::process::id()));
        let _ = std::fs::remove_file(&path);

        enable_persistence(hid, path.to_str().unwrap());
        set_layout(hid, 0, 120, 340);
        set_layout(hid, 1, 800, 40);

        let new_hid = fresh_hid();
        enable_persistence(new_hid, path.to_str().unwrap());
        let layout = list_layout(new_hid);
        assert_eq!(layout.len(), 2);
        assert!(layout.iter().any(|l| l.index == 0 && l.x == 120 && l.y == 340));

        let _ = std::fs::remove_file(&path);
        clear(hid);
        clear(new_hid);
    }
}
