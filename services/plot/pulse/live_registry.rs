use super::token_bucket::Bucket;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 256;
const STALE_MS: i64 = 15_000;
const PALETTE: [u32; 8] = [0xEF4444, 0xF59E0B, 0x22C55E, 0x06B6D4, 0x3B82F6, 0x8B5CF6, 0xEC4899, 0xF97316];

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum Action {
    Pin,
    Replay,
    Stroke,
    Unmark,
    Recolor,
    Transform,
    Image,
}

impl Action {
    fn rate_per_sec(self) -> f64 {
        match self {
            Action::Pin => 3.0,
            Action::Replay => 1.0 / 3.0,
            Action::Stroke => 5.0,
            Action::Unmark => 3.0,
            Action::Recolor => 3.0,
            Action::Transform => 4.0,
            Action::Image => 0.5,
        }
    }
}

#[derive(Clone, serde::Serialize)]
pub struct Presence {
    #[serde(rename = "s")]
    pub session: u64,
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "c")]
    pub color_hex: String,
    #[serde(skip)]
    pub last_seen_ms: i64,
}

#[derive(Default)]
struct Room {
    presence: HashMap<u64, Presence>,
    buckets: HashMap<Action, Bucket>,
}

#[derive(Default)]
struct Registry {
    map: HashMap<u64, Room>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn color_for(name: &str) -> String {
    let mut hash: u32 = 2_166_136_261;
    for b in name.bytes() {
        hash ^= b as u32;
        hash = hash.wrapping_mul(16_777_619);
    }
    format!("#{:06x}", PALETTE[(hash as usize) % PALETTE.len()])
}

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

pub fn join_or_touch(hid: u64, session: u64, name: &str, now_ms: i64) -> Presence {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(Registry::default);
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    let color_hex = color_for(name);
    let presence = Presence { session, name: name.to_string(), color_hex, last_seen_ms: now_ms };
    room.presence.insert(session, presence.clone());
    presence
}

pub fn touch(hid: u64, session: u64, now_ms: i64) {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return };
    let Some(room) = reg.map.get_mut(&hid) else { return };
    if let Some(p) = room.presence.get_mut(&session) {
        p.last_seen_ms = now_ms;
    }
}

pub fn leave(hid: u64, session: u64) {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return };
    let Some(room) = reg.map.get_mut(&hid) else { return };
    room.presence.remove(&session);
}

pub fn allow(hid: u64, action: Action) -> bool {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let reg = guard.get_or_insert_with(Registry::default);
    evict_room(reg, hid);
    let room = reg.map.entry(hid).or_default();
    room.buckets.entry(action).or_insert_with(|| Bucket::new(action.rate_per_sec())).try_consume()
}

pub fn prune_and_list(hid: u64, now_ms: i64) -> Vec<Presence> {
    let mut guard = REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
    let Some(reg) = guard.as_mut() else { return Vec::new() };
    let Some(room) = reg.map.get_mut(&hid) else { return Vec::new() };
    room.presence.retain(|_, p| now_ms - p.last_seen_ms < STALE_MS);
    let mut list: Vec<Presence> = room.presence.values().cloned().collect();
    list.sort_by_key(|p| p.session);
    list
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_hid() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(800_000_000);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    #[test]
    fn color_for_is_deterministic_for_the_same_name() {
        assert_eq!(color_for("alice"), color_for("alice"));
    }

    #[test]
    fn color_for_differs_for_most_distinct_names() {
        assert_ne!(color_for("alice"), color_for("bob"));
    }

    #[test]
    fn joining_then_listing_shows_the_new_session() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        let list = prune_and_list(hid, 1000);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].session, 1);
        assert_eq!(list[0].name, "alice");
    }

    #[test]
    fn a_stale_session_is_pruned_from_the_list() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        let list = prune_and_list(hid, 1000 + STALE_MS + 1);
        assert!(list.is_empty());
    }

    #[test]
    fn touching_a_session_keeps_it_alive_past_the_original_stale_window() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        touch(hid, 1, 1000 + STALE_MS - 1);
        let list = prune_and_list(hid, 1000 + STALE_MS + 1);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn multiple_sessions_in_the_same_room_are_tracked_independently() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        join_or_touch(hid, 2, "bob", 1000);
        let list = prune_and_list(hid, 1000);
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn different_hids_do_not_leak_presence_into_each_other() {
        let hid_a = fresh_hid();
        let hid_b = fresh_hid();
        join_or_touch(hid_a, 1, "alice", 1000);
        assert!(prune_and_list(hid_b, 1000).is_empty());
    }

    #[test]
    fn touch_on_an_unknown_session_or_room_does_not_panic() {
        let hid = fresh_hid();
        touch(hid, 999, 1000);
        touch(fresh_hid(), 1, 1000);
    }

    #[test]
    fn leave_removes_the_session_immediately_without_waiting_for_the_stale_window() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        leave(hid, 1);
        assert!(prune_and_list(hid, 1000).is_empty());
    }

    #[test]
    fn leaving_an_unknown_session_or_room_does_not_panic() {
        leave(fresh_hid(), 999);
    }

    #[test]
    fn leave_does_not_affect_other_sessions_in_the_same_room() {
        let hid = fresh_hid();
        join_or_touch(hid, 1, "alice", 1000);
        join_or_touch(hid, 2, "bob", 1000);
        leave(hid, 1);
        let list = prune_and_list(hid, 1000);
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].name, "bob");
    }

    #[test]
    fn pin_actions_are_capped_and_a_stranger_cannot_flood_them() {
        let hid = fresh_hid();
        let mut allowed = 0;
        for _ in 0..20 {
            if allow(hid, Action::Pin) {
                allowed += 1;
            }
        }
        assert!(allowed < 20, "an unbounded burst of pins must be throttled");
        assert!(allowed >= 1, "the very first pin must still go through immediately");
    }

    #[test]
    fn replay_actions_are_debounced_so_two_simultaneous_clicks_only_fire_once() {
        let hid = fresh_hid();
        assert!(allow(hid, Action::Replay), "the first replay request must be allowed");
        assert!(!allow(hid, Action::Replay), "an immediate second replay request must be debounced");
    }

    #[test]
    fn stroke_actions_are_capped_and_a_stranger_cannot_flood_them() {
        let hid = fresh_hid();
        let mut allowed = 0;
        for _ in 0..20 {
            if allow(hid, Action::Stroke) {
                allowed += 1;
            }
        }
        assert!(allowed < 20, "an unbounded burst of strokes must be throttled");
        assert!(allowed >= 1, "the very first stroke must still go through immediately");
    }

    #[test]
    fn unmark_actions_are_capped_and_a_stranger_cannot_flood_them() {
        let hid = fresh_hid();
        let mut allowed = 0;
        for _ in 0..20 {
            if allow(hid, Action::Unmark) {
                allowed += 1;
            }
        }
        assert!(allowed < 20, "an unbounded burst of unmarks must be throttled");
        assert!(allowed >= 1, "the very first unmark must still go through immediately");
    }

    #[test]
    fn recolor_and_transform_and_image_actions_are_each_capped_independently() {
        let hid = fresh_hid();
        for action in [Action::Recolor, Action::Transform, Action::Image] {
            let mut allowed = 0;
            for _ in 0..20 {
                if allow(hid, action) {
                    allowed += 1;
                }
            }
            assert!(allowed < 20, "an unbounded burst must be throttled");
            assert!(allowed >= 1, "the first action must still go through immediately");
        }
    }

    #[test]
    fn pin_and_replay_throttles_are_independent_per_room() {
        let hid_a = fresh_hid();
        let hid_b = fresh_hid();
        assert!(allow(hid_a, Action::Replay));
        assert!(allow(hid_b, Action::Replay), "a different room must have its own, unaffected budget");
    }

    #[test]
    fn different_actions_in_the_same_room_have_independent_budgets() {
        let hid = fresh_hid();
        assert!(allow(hid, Action::Replay));
        assert!(!allow(hid, Action::Replay));
        assert!(allow(hid, Action::Pin), "a different action in the same room must have its own budget");
    }

    #[test]
    fn perf_cursor_touch_throughput_at_realistic_room_sizes() {
        let hid = fresh_hid();
        let sessions = 50u64;
        for s in 0..sessions {
            join_or_touch(hid, s, &format!("guest{s}"), 0);
        }
        let ticks = 200_000usize;
        let start = std::time::Instant::now();
        for i in 0..ticks {
            touch(hid, i as u64 % sessions, i as i64);
        }
        let elapsed = start.elapsed();
        let per_sec = ticks as f64 / elapsed.as_secs_f64();
        println!("live_registry perf: {ticks} cursor touches across {sessions} sessions in {elapsed:?} -> {per_sec:.0} touches/sec");
        assert!(per_sec > 200_000.0, "expected well over 200k touches/sec, got {per_sec:.0}/sec");
    }
}
