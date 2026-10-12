use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 2048;

#[derive(Clone)]
pub struct ChartSource {
    pub family: String,
    pub json: String,
}

struct Registry {
    map: HashMap<u64, ChartSource>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn register(hid: u64, family: &str, json: String) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let reg = guard.get_or_insert_with(|| Registry { map: HashMap::new(), order: VecDeque::new() });
    if !reg.map.contains_key(&hid) {
        reg.order.push_back(hid);
        while reg.order.len() > CAPACITY {
            if let Some(old) = reg.order.pop_front() {
                reg.map.remove(&old);
            }
        }
    }
    reg.map.insert(hid, ChartSource { family: family.to_string(), json });
}

pub fn get(hid: u64) -> Option<ChartSource> {
    let guard = REGISTRY.lock().ok()?;
    let reg = guard.as_ref()?;
    reg.map.get(&hid).cloned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn register_then_get_round_trips_family_and_json() {
        let hid = 3_000_001u64;
        register(hid, "build_bar", "{\"values\":[1,2,3]}".to_string());
        let src = get(hid).expect("should be registered");
        assert_eq!(src.family, "build_bar");
        assert_eq!(src.json, "{\"values\":[1,2,3]}");
    }

    #[test]
    fn unknown_hid_returns_none() {
        assert!(get(999_999_996u64).is_none());
    }

    #[test]
    fn registering_again_for_the_same_hid_overwrites_the_previous_source() {
        let hid = 3_000_002u64;
        register(hid, "build_bar", "first".to_string());
        register(hid, "build_heatmap", "second".to_string());
        let src = get(hid).unwrap();
        assert_eq!(src.family, "build_heatmap");
        assert_eq!(src.json, "second");
    }

    #[test]
    fn registry_stays_bounded_under_a_flood_of_distinct_charts() {
        for i in 0..(CAPACITY + 200) {
            register(i as u64 + 10_000_000, "build_bar", "x".to_string());
        }
        let guard = REGISTRY.lock().unwrap();
        let reg = guard.as_ref().unwrap();
        assert!(reg.map.len() <= CAPACITY);
    }
}
