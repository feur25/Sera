use super::push_registry::bounded_insert;
use super::ring_buffer::RingBuffer;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 2048;

#[derive(Default)]
struct Registry {
    map: HashMap<u64, RingBuffer<()>>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn register_capacity(hid: u64, capacity: usize) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let reg = guard.get_or_insert_with(Registry::default);
    bounded_insert(&mut reg.map, &mut reg.order, hid, RingBuffer::with_capacity(capacity), CAPACITY);
}

pub fn next_indices(hid: u64, n: usize) -> Option<Vec<u32>> {
    let mut guard = REGISTRY.lock().ok()?;
    let reg = guard.as_mut()?;
    let rb = reg.map.get_mut(&hid)?;
    Some((0..n).map(|_| rb.push(()) as u32).collect())
}

pub fn ring_capacity(hid: u64) -> Option<usize> {
    let guard = REGISTRY.lock().ok()?;
    let reg = guard.as_ref()?;
    reg.map.get(&hid).map(|rb| rb.capacity())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn fresh_hid() -> u64 {
        use std::sync::atomic::{AtomicU64, Ordering};
        static NEXT: AtomicU64 = AtomicU64::new(900_000_000);
        NEXT.fetch_add(1, Ordering::Relaxed)
    }

    #[test]
    fn an_unregistered_hid_yields_no_indices() {
        let hid = fresh_hid();
        assert!(next_indices(hid, 3).is_none());
        assert!(ring_capacity(hid).is_none());
    }

    #[test]
    fn indices_wrap_around_the_registered_capacity() {
        let hid = fresh_hid();
        register_capacity(hid, 4);
        assert_eq!(ring_capacity(hid), Some(4));
        let idx = next_indices(hid, 10).unwrap();
        assert_eq!(idx, vec![0, 1, 2, 3, 0, 1, 2, 3, 0, 1]);
    }

    #[test]
    fn consecutive_calls_continue_the_same_running_cursor() {
        let hid = fresh_hid();
        register_capacity(hid, 3);
        assert_eq!(next_indices(hid, 2).unwrap(), vec![0, 1]);
        assert_eq!(next_indices(hid, 2).unwrap(), vec![2, 0]);
        assert_eq!(next_indices(hid, 2).unwrap(), vec![1, 2]);
    }

    #[test]
    fn re_registering_the_same_hid_resets_its_cursor() {
        let hid = fresh_hid();
        register_capacity(hid, 3);
        next_indices(hid, 5).unwrap();
        register_capacity(hid, 3);
        assert_eq!(next_indices(hid, 2).unwrap(), vec![0, 1]);
    }
}
