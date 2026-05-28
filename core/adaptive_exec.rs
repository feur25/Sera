use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

static RETRY: AtomicBool = AtomicBool::new(true);
static DEGRADE: AtomicUsize = AtomicUsize::new(0);

const MAX_DEGRADE: usize = 4;
const MIN_CHUNK: usize = 64;

pub fn set_adaptive_retry(on: bool) {
    RETRY.store(on, Ordering::Relaxed);
}

pub fn adaptive_retry_enabled() -> bool {
    RETRY.load(Ordering::Relaxed)
}

pub fn reset_perf_state() {
    DEGRADE.store(0, Ordering::Relaxed);
}

pub fn degrade_level() -> usize {
    DEGRADE.load(Ordering::Relaxed)
}

pub fn degrade_once() {
    DEGRADE.fetch_update(Ordering::AcqRel, Ordering::Acquire, |v| {
        if v < MAX_DEGRADE { Some(v + 1) } else { None }
    }).ok();
}

pub fn effective_par_threshold() -> usize {
    let base = crate::core::hw_profile::hw().par_threshold;
    base << DEGRADE.load(Ordering::Relaxed)
}

pub fn effective_chunk(base: usize) -> usize {
    let shift = DEGRADE.load(Ordering::Relaxed);
    (base >> shift).max(MIN_CHUNK)
}

pub fn with_retry<T, F: Fn() -> T>(f: F) -> T {
    if !RETRY.load(Ordering::Relaxed) {
        return f();
    }
    let mut attempt = 0usize;
    loop {
        match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f())) {
            Ok(v) => return v,
            Err(_) if attempt < MAX_DEGRADE => {
                degrade_once();
                attempt += 1;
            }
            Err(p) => std::panic::resume_unwind(p),
        }
    }
}
