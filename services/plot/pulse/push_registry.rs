use std::collections::{HashMap, VecDeque};
use std::sync::Mutex;

const CAPACITY: usize = 2048;
const PAD_L: i32 = 56;
const PAD_T: i32 = 36;
const PAD_B: i32 = 48;
const PAD_R: i32 = 20;

#[derive(Clone)]
pub struct PointMeta {
    pub min_x: f64,
    pub range_x: f64,
    pub min_y: f64,
    pub range_y: f64,
    pub plot_w: i32,
    pub plot_h: i32,
}

#[derive(Clone)]
pub struct ScalarMeta {
    pub min_val: f64,
    pub range_val: f64,
    pub axis_px: i32,
}

#[derive(Clone)]
pub struct VectorMeta {
    pub min_val: f64,
    pub range_val: f64,
    pub axis_px: i32,
    pub vpp: u32,
}

#[derive(Clone)]
pub enum PushMeta {
    Point(PointMeta),
    Scalar(ScalarMeta),
    Vector(VectorMeta),
}

impl PushMeta {
    pub fn values_per_point(&self) -> u32 {
        match self {
            PushMeta::Point(_) => 2,
            PushMeta::Scalar(_) => 1,
            PushMeta::Vector(v) => v.vpp,
        }
    }

    pub fn from_xy(x: &[f64], y: &[f64], width: i32, height: i32) -> PushMeta {
        let (min_x, max_x) = crate::bindings::utils::simd_ops::find_minmax(x);
        let (min_y, max_y) = crate::bindings::utils::simd_ops::find_minmax(y);
        PushMeta::Point(PointMeta {
            min_x,
            range_x: (max_x - min_x).max(1e-12),
            min_y,
            range_y: (max_y - min_y).max(1e-12),
            plot_w: (width - PAD_L - PAD_R).max(10),
            plot_h: (height - PAD_T - PAD_B).max(10),
        })
    }

    pub fn from_values(values: &[f64], axis_px: i32) -> PushMeta {
        let (min_val, max_val) = crate::bindings::utils::simd_ops::find_minmax(values);
        let min_val = min_val.min(0.0);
        PushMeta::Scalar(ScalarMeta {
            min_val,
            range_val: (max_val - min_val).max(1e-12),
            axis_px,
        })
    }

    pub fn from_vector_shared_scale(components: &[&[f64]], axis_px: i32) -> PushMeta {
        let (min_val, range_val) = crate::plot::canvas_points::shared_minmax(components);
        PushMeta::Vector(VectorMeta {
            min_val,
            range_val,
            axis_px,
            vpp: components.len() as u32,
        })
    }
}

pub(crate) fn bounded_insert<V>(map: &mut HashMap<u64, V>, order: &mut VecDeque<u64>, hid: u64, value: V, cap: usize) {
    if !map.contains_key(&hid) {
        order.push_back(hid);
        while order.len() > cap {
            if let Some(old) = order.pop_front() {
                map.remove(&old);
            }
        }
    }
    map.insert(hid, value);
}

#[derive(Default)]
struct Registry {
    map: HashMap<u64, PushMeta>,
    order: VecDeque<u64>,
}

static REGISTRY: Mutex<Option<Registry>> = Mutex::new(None);

pub fn register(hid: u64, meta: PushMeta) {
    let Ok(mut guard) = REGISTRY.lock() else { return };
    let reg = guard.get_or_insert_with(Registry::default);
    bounded_insert(&mut reg.map, &mut reg.order, hid, meta, CAPACITY);
}

pub fn get(hid: u64) -> Option<PushMeta> {
    REGISTRY.lock().ok()?.as_ref()?.map.get(&hid).cloned()
}

#[cfg(feature = "webapp")]
mod serve_channels {
    use super::{bounded_insert, HashMap, Mutex, VecDeque};
    use tokio::sync::broadcast;

    const SERVE_CAPACITY: usize = 256;

    #[derive(Default)]
    struct ServeRegistry {
        map: HashMap<u64, broadcast::Sender<(u8, Vec<u8>)>>,
        order: VecDeque<u64>,
    }

    static SERVE_REGISTRY: Mutex<Option<ServeRegistry>> = Mutex::new(None);

    pub fn get_or_create(hid: u64) -> broadcast::Sender<(u8, Vec<u8>)> {
        let mut guard = SERVE_REGISTRY.lock().unwrap_or_else(|e| e.into_inner());
        let reg = guard.get_or_insert_with(ServeRegistry::default);
        if let Some(tx) = reg.map.get(&hid) {
            return tx.clone();
        }
        let (tx, _) = broadcast::channel(1024);
        bounded_insert(&mut reg.map, &mut reg.order, hid, tx.clone(), SERVE_CAPACITY);
        tx
    }

    pub fn get(hid: u64) -> Option<broadcast::Sender<(u8, Vec<u8>)>> {
        SERVE_REGISTRY.lock().ok()?.as_ref()?.map.get(&hid).cloned()
    }
}

#[cfg(feature = "webapp")]
pub use serve_channels::{get as get_push_tx, get_or_create as get_or_create_push_tx};

pub fn extract_hid(html: &str) -> Option<u64> {
    const MARK: &str = "<div id=\"spp";
    let start = html.find(MARK)? + MARK.len();
    let rest = &html[start..];
    let end = rest.find('"')?;
    rest[..end].parse::<u64>().ok()
}
