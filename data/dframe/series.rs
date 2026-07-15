use pyo3::prelude::*;
use rayon::prelude::*;
use std::sync::Arc;

const PARALLEL_TAKE_THRESHOLD: usize = 50_000;

pub enum ColView<'a> {
    Num(&'a [f64]),
    Str(&'a [Arc<str>]),
    Bool(&'a [bool]),
}

#[inline]
fn fx_combine(h: u64, v: u64) -> u64 {
    const SEED: u64 = 0x51_7c_c1_b7_27_22_0a_95;
    (h.rotate_left(5) ^ v).wrapping_mul(SEED)
}

#[inline]
fn fx_hash_bytes(bytes: &[u8]) -> u64 {
    let mut h = bytes.len() as u64;
    let mut chunks = bytes.chunks_exact(8);
    for c in &mut chunks {
        h = fx_combine(h, u64::from_ne_bytes(c.try_into().unwrap()));
    }
    let mut rest = [0u8; 8];
    rest[..chunks.remainder().len()].copy_from_slice(chunks.remainder());
    fx_combine(h, u64::from_ne_bytes(rest))
}

#[derive(Default)]
pub struct PassThroughHasher(u64);

impl std::hash::Hasher for PassThroughHasher {
    fn finish(&self) -> u64 {
        self.0
    }
    fn write(&mut self, _bytes: &[u8]) {
        unreachable!("PassThroughHasher only supports pre-hashed u64 keys")
    }
    fn write_u64(&mut self, i: u64) {
        self.0 = i;
    }
}

pub type PassThroughBuildHasher = std::hash::BuildHasherDefault<PassThroughHasher>;

#[inline]
pub fn f64_sort_key(v: f64) -> u64 {
    let bits = v.to_bits();
    if bits & (1u64 << 63) != 0 {
        !bits
    } else {
        bits | (1u64 << 63)
    }
}

pub fn row_hash(cols: &[ColView], i: usize) -> u64 {
    let mut h: u64 = 0;
    for c in cols {
        let bits = match c {
            ColView::Num(v) => v[i].to_bits(),
            ColView::Str(v) => fx_hash_bytes(v[i].as_bytes()),
            ColView::Bool(v) => v[i] as u64,
        };
        h = fx_combine(h, bits);
    }
    h
}

pub fn rows_equal(cols: &[ColView], a: usize, b: usize) -> bool {
    cols.iter().all(|c| match c {
        ColView::Num(v) => v[a].to_bits() == v[b].to_bits(),
        ColView::Str(v) => v[a] == v[b],
        ColView::Bool(v) => v[a] == v[b],
    })
}

pub fn rows_equal_cross(left: &[ColView], right: &[ColView], a: usize, b: usize) -> bool {
    left.iter().zip(right.iter()).all(|(lc, rc)| match (lc, rc) {
        (ColView::Num(lv), ColView::Num(rv)) => lv[a].to_bits() == rv[b].to_bits(),
        (ColView::Str(lv), ColView::Str(rv)) => lv[a] == rv[b],
        (ColView::Bool(lv), ColView::Bool(rv)) => lv[a] == rv[b],
        _ => false,
    })
}


#[derive(Clone)]
pub enum Series {
    Num(Arc<Vec<f64>>),
    Str(Arc<Vec<Arc<str>>>),
    Bool(Arc<Vec<bool>>),
}

fn unwrap_or_clone<T: Clone>(v: Arc<Vec<T>>) -> Vec<T> {
    Arc::try_unwrap(v).unwrap_or_else(|a| (*a).clone())
}

impl Series {
    pub fn len(&self) -> usize {
        match self {
            Series::Num(v) => v.len(),
            Series::Str(v) => v.len(),
            Series::Bool(v) => v.len(),
        }
    }

    pub fn byte_size(&self) -> u64 {
        match self {
            Series::Num(v) => (v.len() * std::mem::size_of::<f64>()) as u64,
            Series::Bool(v) => v.len() as u64,
            Series::Str(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => v.par_iter().map(|s| s.len() as u64 + 24).sum(),
            Series::Str(v) => v.iter().map(|s| s.len() as u64 + 24).sum(),
        }
    }

    pub fn value_str(&self, i: usize) -> String {
        match self {
            Series::Num(v) => format_num(v[i]),
            Series::Str(v) => v[i].to_string(),
            Series::Bool(v) => v[i].to_string(),
        }
    }

    pub fn dtype(&self) -> &'static str {
        match self {
            Series::Num(_) => "float64",
            Series::Str(_) => "object",
            Series::Bool(_) => "bool",
        }
    }

    pub fn as_f64_slice(&self) -> Option<&[f64]> {
        match self {
            Series::Num(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_str_slice(&self) -> Option<&[Arc<str>]> {
        match self {
            Series::Str(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_bool_slice(&self) -> Option<&[bool]> {
        match self {
            Series::Bool(v) => Some(v.as_slice()),
            _ => None,
        }
    }

    pub fn as_view(&self) -> ColView<'_> {
        match self {
            Series::Num(v) => ColView::Num(v.as_slice()),
            Series::Str(v) => ColView::Str(v.as_slice()),
            Series::Bool(v) => ColView::Bool(v.as_slice()),
        }
    }

    pub fn to_f64_vec(&self) -> Vec<f64> {
        match self {
            Series::Num(v) => v.as_ref().clone(),
            Series::Bool(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => {
                v.par_iter().map(|b| if *b { 1.0 } else { 0.0 }).collect()
            }
            Series::Bool(v) => v.iter().map(|b| if *b { 1.0 } else { 0.0 }).collect(),
            Series::Str(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => {
                v.par_iter().map(|s| s.parse().unwrap_or(f64::NAN)).collect()
            }
            Series::Str(v) => v.iter().map(|s| s.parse().unwrap_or(f64::NAN)).collect(),
        }
    }

    pub fn to_str_vec(&self) -> Vec<String> {
        match self {
            Series::Str(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => v.par_iter().map(|s| s.to_string()).collect(),
            Series::Str(v) => v.iter().map(|s| s.to_string()).collect(),
            Series::Num(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => v.par_iter().map(|f| format_num(*f)).collect(),
            Series::Num(v) => v.iter().map(|f| format_num(*f)).collect(),
            Series::Bool(v) if v.len() >= PARALLEL_TAKE_THRESHOLD => v.par_iter().map(|b| b.to_string()).collect(),
            Series::Bool(v) => v.iter().map(|b| b.to_string()).collect(),
        }
    }

    pub fn take(&self, idx: &[usize]) -> Series {
        if idx.len() >= PARALLEL_TAKE_THRESHOLD {
            return match self {
                Series::Num(v) => Series::Num(Arc::new(idx.par_iter().map(|&i| v[i]).collect())),
                Series::Str(v) => Series::Str(Arc::new(idx.par_iter().map(|&i| v[i].clone()).collect())),
                Series::Bool(v) => Series::Bool(Arc::new(idx.par_iter().map(|&i| v[i]).collect())),
            };
        }
        match self {
            Series::Num(v) => Series::Num(Arc::new(idx.iter().map(|&i| v[i]).collect())),
            Series::Str(v) => Series::Str(Arc::new(idx.iter().map(|&i| v[i].clone()).collect())),
            Series::Bool(v) => Series::Bool(Arc::new(idx.iter().map(|&i| v[i]).collect())),
        }
    }

    pub fn into_py(self, py: Python<'_>) -> PyObject {
        match self {
            Series::Num(v) => unwrap_or_clone(v).into_py(py),
            Series::Str(v) => v.iter().map(|s| s.to_string()).collect::<Vec<_>>().into_py(py),
            Series::Bool(v) => unwrap_or_clone(v).into_py(py),
        }
    }

    pub fn from_strings(vals: Vec<String>) -> Series {
        let all_bool = vals
            .iter()
            .all(|s| s.is_empty() || s.eq_ignore_ascii_case("true") || s.eq_ignore_ascii_case("false"));
        if all_bool && vals.iter().any(|s| !s.is_empty()) {
            return Series::Bool(Arc::new(vals.iter().map(|s| s.eq_ignore_ascii_case("true")).collect()));
        }
        let all_num = vals.iter().all(|s| s.is_empty() || s.parse::<f64>().is_ok());
        if all_num {
            return Series::Num(Arc::new(
                vals.iter()
                    .map(|s| if s.is_empty() { f64::NAN } else { s.parse().unwrap() })
                    .collect(),
            ));
        }
        Series::Str(Arc::new(vals.into_iter().map(Arc::from).collect()))
    }
}

fn format_num(f: f64) -> String {
    if f.is_nan() {
        String::new()
    } else if f.fract().abs() < 1e-9 {
        format!("{}", f as i64)
    } else {
        f.to_string()
    }
}

#[derive(FromPyObject)]
pub enum PyCell {
    B(bool),
    F(f64),
    S(String),
}

pub fn column_from_pyobjects(items: Vec<Bound<'_, PyAny>>) -> Series {
    let mut nums = Vec::with_capacity(items.len());
    let mut bools = Vec::with_capacity(items.len());
    let mut all_num = true;
    let mut all_bool = true;
    for item in &items {
        if let Ok(b) = item.extract::<bool>() {
            bools.push(b);
            nums.push(if b { 1.0 } else { 0.0 });
        } else if let Ok(f) = item.extract::<f64>() {
            all_bool = false;
            bools.push(false);
            nums.push(f);
        } else {
            all_num = false;
            all_bool = false;
            break;
        }
    }
    if all_bool && !items.is_empty() {
        return Series::Bool(Arc::new(bools));
    }
    if all_num {
        return Series::Num(Arc::new(nums));
    }
    let strs: Vec<String> = items
        .iter()
        .map(|v| {
            v.extract::<String>()
                .unwrap_or_else(|_| v.str().map(|s| s.to_string()).unwrap_or_default())
        })
        .collect();
    Series::from_strings(strs)
}
