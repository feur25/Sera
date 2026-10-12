use super::token_bucket::Bucket;
use crate::Chart;
use pyo3::prelude::*;
use seraplot_macros::sera_doc_impl;
use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 512;

struct Registry {
    map: HashMap<u64, Bucket>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn set(hid: u64, max_per_sec: f64) {
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
    reg.map.insert(hid, Bucket::new(max_per_sec));
}

pub fn clear(hid: u64) {
    if let Ok(mut guard) = REGISTRY.lock() {
        if let Some(reg) = guard.as_mut() {
            reg.map.remove(&hid);
        }
    }
}

pub fn check(hid: u64) -> bool {
    let Ok(mut guard) = REGISTRY.lock() else { return true };
    let Some(reg) = guard.as_mut() else { return true };
    let Some(bucket) = reg.map.get_mut(&hid) else { return true };
    bucket.try_consume()
}

#[cfg(feature = "python")]
#[sera_doc_impl]
#[pyo3::pymethods]
impl Chart {
    #[pyo3(signature = (max_per_sec, downsample = false))]
    #[sera_doc(
        category = "chart_method",
        module = "rate_limit",
        file = "seraplot/pulse-pricing.md",
        aliases("throttle_push"),
        en = "Caps how many push()/push_vector() calls per second this chart accepts — protects a .serve() shared beyond strictly internal use from being overwhelmed by accident or on purpose. By default, calls beyond the limit raise a clear error instead of being silently dropped. Pass downsample=True to aggregate excess pushes into a mean instead of rejecting them: the next in-budget push() for that index is preceded by one synthetic catch-up push carrying the mean of everything suppressed since - scalar/bar-style charts only (push() calls with a single value array, no y).",
        fr = "Plafonne le nombre d'appels push()/push_vector() par seconde acceptés par ce chart — protège un .serve() partagé au-delà d'un usage strictement interne d'être saturé par accident ou intentionnellement. Par défaut, les appels au-delà de la limite lèvent une erreur claire plutôt que d'être silencieusement ignorés. Passer downsample=True agrège les pushes excédentaires en une moyenne plutôt que de les rejeter : le prochain push() dans le budget pour cet indice est précédé d'un push de rattrapage synthétique portant la moyenne de tout ce qui a été supprimé depuis — charts scalaires/type bar uniquement (appels push() avec un seul tableau de valeurs, sans y).",
        param(name = "max_per_sec", ty = "float", en = "Maximum accepted push()/push_vector() calls per second for this chart.", fr = "Nombre maximal d'appels push()/push_vector() par seconde acceptés pour ce chart."),
        param(name = "downsample", ty = "bool", en = "If True, aggregate excess pushes into a mean instead of rejecting them (scalar/bar-style charts only).", fr = "Si True, agrège les pushes excédentaires en une moyenne plutôt que de les rejeter (charts scalaires/type bar uniquement).")
    )]
    fn set_push_rate_limit(&self, max_per_sec: f64, downsample: bool) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        set(hid, max_per_sec);
        super::downsample::set_enabled(hid, downsample);
        Ok(())
    }

    #[sera_doc(
        category = "chart_method",
        module = "rate_limit",
        file = "seraplot/pulse-pricing.md",
        aliases("unthrottle_push"),
        en = "Removes the push rate limit set by set_push_rate_limit(), if any, including its downsample=True aggregation state.",
        fr = "Retire la limite de débit posée par set_push_rate_limit(), s'il y en a une, ainsi que son état d'agrégation downsample=True."
    )]
    fn clear_push_rate_limit(&self) -> PyResult<()> {
        let hid = super::require_hid(&self.html)?;
        clear(hid);
        super::downsample::set_enabled(hid, false);
        Ok(())
    }

    #[pyo3(signature = (max_per_sec, downsample = false))]
    fn throttle_push(&self, max_per_sec: f64, downsample: bool) -> PyResult<()> {
        self.set_push_rate_limit(max_per_sec, downsample)
    }

    fn unthrottle_push(&self) -> PyResult<()> {
        self.clear_push_rate_limit()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unconfigured_hid_always_allowed() {
        assert!(check(1_000_001u64));
        assert!(check(1_000_001u64));
        assert!(check(1_000_001u64));
    }

    #[test]
    fn caps_at_configured_rate() {
        let hid = 1_000_002u64;
        set(hid, 3.0);
        let mut allowed = 0;
        for _ in 0..10 {
            if check(hid) {
                allowed += 1;
            }
        }
        assert_eq!(allowed, 3);
        clear(hid);
    }

    #[test]
    fn clearing_restores_unlimited() {
        let hid = 1_000_003u64;
        set(hid, 1.0);
        assert!(check(hid));
        assert!(!check(hid));
        clear(hid);
        assert!(check(hid));
        assert!(check(hid));
    }

    #[test]
    fn refills_over_time() {
        let hid = 1_000_004u64;
        set(hid, 5.0);
        for _ in 0..5 {
            check(hid);
        }
        std::thread::sleep(std::time::Duration::from_millis(400));
        assert!(check(hid), "bucket should have refilled at least one token after 400ms at 5/sec");
        clear(hid);
    }

    #[test]
    fn overhead_of_unconfigured_check_is_negligible() {
        let hid = 1_000_005u64;
        let n = 1_000_000u32;
        let started = std::time::Instant::now();
        for _ in 0..n {
            check(hid);
        }
        let elapsed = started.elapsed();
        println!("rate_limit::check (unconfigured) x{n}: {elapsed:?} ({:?}/call)", elapsed / n);
        assert!(elapsed.as_millis() < 1000, "unconfigured check() got too slow: {elapsed:?} for {n} calls");
    }
}
