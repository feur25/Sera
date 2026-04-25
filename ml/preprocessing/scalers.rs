use rayon::prelude::*;

pub struct StandardScaler {
    pub mean: Vec<f64>,
    pub scale: Vec<f64>,
    pub with_mean: bool,
    pub with_std: bool,
    pub n_samples_seen: u64,
    pub m2: Vec<f64>,
    inv_scale: Vec<f64>,
    p: usize,
}

impl StandardScaler {
    pub fn new(with_mean: bool, with_std: bool) -> Self {
        Self { mean: Vec::new(), scale: Vec::new(), with_mean, with_std, n_samples_seen: 0, m2: Vec::new(), inv_scale: Vec::new(), p: 0 }
    }

    pub fn partial_fit(&mut self, x: &[f64], n: usize, p: usize) {
        if n == 0 { return; }
        if self.p == 0 {
            self.p = p;
            self.mean = vec![0.0; p];
            self.m2 = vec![0.0; p];
            self.scale = vec![1.0; p];
            self.n_samples_seen = 0;
        }
        if self.p != p { return; }
        let mut count = self.n_samples_seen;
        for i in 0..n {
            count += 1;
            let inv_count = 1.0 / count as f64;
            for j in 0..p {
                let v = x[i * p + j];
                let delta = v - self.mean[j];
                self.mean[j] += delta * inv_count;
                let delta2 = v - self.mean[j];
                self.m2[j] += delta * delta2;
            }
        }
        self.n_samples_seen = count;
        if self.with_std && count > 0 {
            let inv = 1.0 / count as f64;
            for j in 0..p {
                self.scale[j] = (self.m2[j] * inv).max(0.0).sqrt().max(1e-15);
            }
        } else {
            for j in 0..p { self.scale[j] = 1.0; }
        }
        self.inv_scale = self.scale.iter().map(|&s| 1.0 / s).collect();
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.p = p;
        self.mean = vec![0.0; p];
        self.scale = vec![1.0; p];
        self.m2 = vec![0.0; p];
        self.n_samples_seen = n as u64;

        if self.with_std {
            let chunk = 4096usize.max(p);
            let nc = (n + chunk - 1) / chunk;
            if n >= 50_000 && nc >= 2 {
                let partials: Vec<(Vec<f64>, Vec<f64>)> = (0..nc).into_par_iter().map(|c| {
                    let s = c * chunk;
                    let e = (s + chunk).min(n);
                    let mut ps = vec![0.0; p];
                    let mut ps2 = vec![0.0; p];
                    for i in s..e {
                        let row = &x[i * p..i * p + p];
                        for j in 0..p { ps[j] += row[j]; ps2[j] += row[j] * row[j]; }
                    }
                    (ps, ps2)
                }).collect();
                let mut sum2 = vec![0.0; p];
                for (ps, ps2) in &partials {
                    for j in 0..p { self.mean[j] += ps[j]; sum2[j] += ps2[j]; }
                }
                let inv = 1.0 / n as f64;
                for j in 0..p {
                    self.mean[j] *= inv;
                    self.scale[j] = (sum2[j] * inv - self.mean[j] * self.mean[j]).max(0.0).sqrt().max(1e-15);
                }
            } else {
                let mut sum2 = vec![0.0; p];
                for i in 0..n {
                    let row = &x[i * p..i * p + p];
                    for j in 0..p { self.mean[j] += row[j]; sum2[j] += row[j] * row[j]; }
                }
                let inv = 1.0 / n as f64;
                for j in 0..p {
                    self.mean[j] *= inv;
                    self.scale[j] = (sum2[j] * inv - self.mean[j] * self.mean[j]).max(0.0).sqrt().max(1e-15);
                }
            }
        } else if self.with_mean {
            for i in 0..n { for j in 0..p { self.mean[j] += x[i * p + j]; } }
            let inv = 1.0 / n as f64;
            for j in 0..p { self.mean[j] *= inv; }
        }
        for j in 0..p {
            let s = self.scale[j];
            self.m2[j] = s * s * (n as f64);
        }
        self.inv_scale = self.scale.iter().map(|&s| 1.0 / s).collect();
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n * p];
        if self.with_mean && self.with_std {
            let mean = &self.mean;
            let inv = &self.inv_scale;
            if n >= 50_000 {
                let grain = 1024usize;
                out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                    let base = ci * grain;
                    let rows = block.len() / p;
                    for r in 0..rows {
                        let i = base + r;
                        let xi = &x[i * p..i * p + p];
                        let row = &mut block[r * p..(r + 1) * p];
                        for j in 0..p { row[j] = (xi[j] - mean[j]) * inv[j]; }
                    }
                });
            } else {
                for i in 0..n {
                    let xi = &x[i * p..i * p + p];
                    let row = &mut out[i * p..i * p + p];
                    for j in 0..p { row[j] = (xi[j] - mean[j]) * inv[j]; }
                }
            }
        } else if self.with_mean {
            for i in 0..n {
                let xi = &x[i * p..i * p + p];
                let row = &mut out[i * p..i * p + p];
                for j in 0..p { row[j] = xi[j] - self.mean[j]; }
            }
        } else if self.with_std {
            let inv = &self.inv_scale;
            for i in 0..n {
                let xi = &x[i * p..i * p + p];
                let row = &mut out[i * p..i * p + p];
                for j in 0..p { row[j] = xi[j] * inv[j]; }
            }
        } else {
            out.copy_from_slice(&x[..n * p]);
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }

    pub fn inverse_transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n * p];
        for i in 0..n {
            for j in 0..p {
                let mut v = x[i * p + j];
                if self.with_std { v *= self.scale[j]; }
                if self.with_mean { v += self.mean[j]; }
                out[i * p + j] = v;
            }
        }
        out
    }
}

pub struct MinMaxScaler {
    pub min: Vec<f64>,
    pub range: Vec<f64>,
    pub feature_range: (f64, f64),
    pub max: Vec<f64>,
    pub n_samples_seen: u64,
    p: usize,
}

impl MinMaxScaler {
    pub fn new(feature_range: (f64, f64)) -> Self {
        Self { min: Vec::new(), range: Vec::new(), feature_range, max: Vec::new(), n_samples_seen: 0, p: 0 }
    }

    pub fn partial_fit(&mut self, x: &[f64], n: usize, p: usize) {
        if n == 0 { return; }
        if self.p == 0 {
            self.p = p;
            self.min = vec![f64::INFINITY; p];
            self.max = vec![f64::NEG_INFINITY; p];
        }
        if self.p != p { return; }
        for i in 0..n {
            for j in 0..p {
                let v = x[i * p + j];
                if v < self.min[j] { self.min[j] = v; }
                if v > self.max[j] { self.max[j] = v; }
            }
        }
        self.n_samples_seen += n as u64;
        self.range = (0..p).map(|j| (self.max[j] - self.min[j]).max(1e-15)).collect();
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.p = p;
        let mut mins = vec![f64::MAX; p];
        let mut maxs = vec![f64::MIN; p];
        if n >= 50_000 {
            let chunk = 4096usize.max(1);
            let nc = (n + chunk - 1) / chunk;
            let partials: Vec<(Vec<f64>, Vec<f64>)> = (0..nc).into_par_iter().map(|c| {
                let s = c * chunk;
                let e = (s + chunk).min(n);
                let mut lmin = vec![f64::MAX; p];
                let mut lmax = vec![f64::MIN; p];
                for i in s..e {
                    for j in 0..p {
                        let v = x[i * p + j];
                        if v < lmin[j] { lmin[j] = v; }
                        if v > lmax[j] { lmax[j] = v; }
                    }
                }
                (lmin, lmax)
            }).collect();
            for (lmin, lmax) in &partials {
                for j in 0..p {
                    if lmin[j] < mins[j] { mins[j] = lmin[j]; }
                    if lmax[j] > maxs[j] { maxs[j] = lmax[j]; }
                }
            }
        } else {
            for i in 0..n {
                for j in 0..p {
                    let v = x[i * p + j];
                    if v < mins[j] { mins[j] = v; }
                    if v > maxs[j] { maxs[j] = v; }
                }
            }
        }
        self.min = mins.clone();
        self.range = (0..p).map(|j| (maxs[j] - mins[j]).max(1e-15)).collect();
        self.max = maxs;
        self.n_samples_seen = n as u64;
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let (lo, hi) = self.feature_range;
        let span = hi - lo;
        let inv_range: Vec<f64> = self.range.iter().map(|&r| span / r).collect();
        let min = &self.min;
        let mut out = vec![0.0; n * p];
        if n >= 50_000 {
            let grain = 2048usize;
            out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                let base = ci * grain;
                let rows = block.len() / p;
                for r in 0..rows {
                    let i = base + r;
                    for j in 0..p {
                        block[r * p + j] = (x[i * p + j] - min[j]) * inv_range[j] + lo;
                    }
                }
            });
        } else {
            for i in 0..n {
                for j in 0..p {
                    out[i * p + j] = (x[i * p + j] - min[j]) * inv_range[j] + lo;
                }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }

    pub fn inverse_transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let (lo, hi) = self.feature_range;
        let span = hi - lo;
        let inv_span: Vec<f64> = self.range.iter().map(|&r| r / span).collect();
        let min = &self.min;
        let mut out = vec![0.0; n * p];
        if n >= 50_000 {
            let grain = 2048usize;
            out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                let base = ci * grain;
                let rows = block.len() / p;
                for r in 0..rows {
                    let i = base + r;
                    for j in 0..p {
                        block[r * p + j] = (x[i * p + j] - lo) * inv_span[j] + min[j];
                    }
                }
            });
        } else {
            for i in 0..n {
                for j in 0..p {
                    out[i * p + j] = (x[i * p + j] - lo) * inv_span[j] + min[j];
                }
            }
        }
        out
    }
}

pub struct RobustScaler {
    pub center: Vec<f64>,
    pub scale: Vec<f64>,
    pub with_centering: bool,
    pub with_scaling: bool,
    pub quantile_range: (f64, f64),
    p: usize,
}

impl RobustScaler {
    pub fn new(with_centering: bool, with_scaling: bool) -> Self {
        Self { center: Vec::new(), scale: Vec::new(), with_centering, with_scaling, quantile_range: (25.0, 75.0), p: 0 }
    }

    pub fn with_quantile_range(with_centering: bool, with_scaling: bool, quantile_range: (f64, f64)) -> Self {
        Self { center: Vec::new(), scale: Vec::new(), with_centering, with_scaling, quantile_range, p: 0 }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.p = p;
        self.center = vec![0.0; p];
        self.scale = vec![1.0; p];
        let (q_lo, q_hi) = self.quantile_range;

        if p >= 4 && n >= 1000 {
            let results: Vec<(f64, f64)> = (0..p).into_par_iter().map(|j| {
                let mut col: Vec<f64> = (0..n).map(|i| x[i * p + j]).collect();
                col.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                let c = if self.with_centering { percentile_sorted(&col, 50.0) } else { 0.0 };
                let s = if self.with_scaling {
                    let qlo = percentile_sorted(&col, q_lo);
                    let qhi = percentile_sorted(&col, q_hi);
                    (qhi - qlo).max(1e-15)
                } else { 1.0 };
                (c, s)
            }).collect();
            for j in 0..p {
                self.center[j] = results[j].0;
                self.scale[j] = results[j].1;
            }
        } else {
            for j in 0..p {
                let mut col: Vec<f64> = (0..n).map(|i| x[i * p + j]).collect();
                col.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
                if self.with_centering {
                    self.center[j] = percentile_sorted(&col, 50.0);
                }
                if self.with_scaling {
                    let qlo = percentile_sorted(&col, q_lo);
                    let qhi = percentile_sorted(&col, q_hi);
                    self.scale[j] = (qhi - qlo).max(1e-15);
                }
            }
        }
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let center = &self.center;
        let inv_scale: Vec<f64> = self.scale.iter().map(|&s| 1.0 / s).collect();
        let wc = self.with_centering;
        let ws = self.with_scaling;
        let mut out = vec![0.0; n * p];
        if n >= 50_000 {
            let grain = 2048usize;
            out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                let base = ci * grain;
                let rows = block.len() / p;
                for r in 0..rows {
                    let i = base + r;
                    for j in 0..p {
                        let mut v = x[i * p + j];
                        if wc { v -= center[j]; }
                        if ws { v *= inv_scale[j]; }
                        block[r * p + j] = v;
                    }
                }
            });
        } else {
            for i in 0..n {
                for j in 0..p {
                    let mut v = x[i * p + j];
                    if wc { v -= center[j]; }
                    if ws { v *= inv_scale[j]; }
                    out[i * p + j] = v;
                }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

pub struct MaxAbsScaler {
    pub max_abs: Vec<f64>,
    pub n_samples_seen: u64,
    p: usize,
}

impl MaxAbsScaler {
    pub fn new() -> Self {
        Self { max_abs: Vec::new(), n_samples_seen: 0, p: 0 }
    }

    pub fn partial_fit(&mut self, x: &[f64], n: usize, p: usize) {
        if n == 0 { return; }
        if self.p == 0 {
            self.p = p;
            self.max_abs = vec![0.0; p];
        }
        if self.p != p { return; }
        for i in 0..n {
            for j in 0..p {
                let v = x[i * p + j].abs();
                if v > self.max_abs[j] { self.max_abs[j] = v; }
            }
        }
        self.n_samples_seen += n as u64;
        for j in 0..p { if self.max_abs[j] < 1e-15 { self.max_abs[j] = 1.0; } }
    }

    pub fn fit(&mut self, x: &[f64], n: usize, p: usize) {
        self.p = p;
        self.max_abs = vec![0.0; p];
        if n >= 50_000 {
            let chunk = 4096usize.max(1);
            let nc = (n + chunk - 1) / chunk;
            let partials: Vec<Vec<f64>> = (0..nc).into_par_iter().map(|c| {
                let s = c * chunk;
                let e = (s + chunk).min(n);
                let mut lmax = vec![0.0; p];
                for i in s..e {
                    for j in 0..p {
                        let v = x[i * p + j].abs();
                        if v > lmax[j] { lmax[j] = v; }
                    }
                }
                lmax
            }).collect();
            for lmax in &partials {
                for j in 0..p {
                    if lmax[j] > self.max_abs[j] { self.max_abs[j] = lmax[j]; }
                }
            }
        } else {
            for i in 0..n {
                for j in 0..p {
                    let v = x[i * p + j].abs();
                    if v > self.max_abs[j] { self.max_abs[j] = v; }
                }
            }
        }
        for j in 0..p { if self.max_abs[j] < 1e-15 { self.max_abs[j] = 1.0; } }
        self.n_samples_seen = n as u64;
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let inv: Vec<f64> = self.max_abs.iter().map(|&m| 1.0 / m).collect();
        let mut out = vec![0.0; n * p];
        if n >= 50_000 {
            let grain = 2048usize;
            out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                let base = ci * grain;
                let rows = block.len() / p;
                for r in 0..rows {
                    let i = base + r;
                    for j in 0..p { block[r * p + j] = x[i * p + j] * inv[j]; }
                }
            });
        } else {
            for i in 0..n {
                for j in 0..p { out[i * p + j] = x[i * p + j] * inv[j]; }
            }
        }
        out
    }

    pub fn fit_transform(&mut self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        self.fit(x, n, p);
        self.transform(x, n, p)
    }
}

pub struct Normalizer {
    pub norm: NormType,
}

#[derive(Clone, Copy)]
pub enum NormType { L1, L2, Max }

impl Normalizer {
    pub fn new(norm: NormType) -> Self {
        Self { norm }
    }

    pub fn transform(&self, x: &[f64], n: usize, p: usize) -> Vec<f64> {
        let mut out = vec![0.0; n * p];
        if n >= 50_000 {
            let norm = self.norm;
            let grain = 2048usize;
            out.par_chunks_mut(p * grain).enumerate().for_each(|(ci, block)| {
                let base = ci * grain;
                let rows = block.len() / p;
                for r in 0..rows {
                    let i = base + r;
                    let xi = &x[i * p..(i + 1) * p];
                    let scale = match norm {
                        NormType::L1 => { let s: f64 = xi.iter().map(|v| v.abs()).sum(); s.max(1e-15) }
                        NormType::L2 => { let s: f64 = xi.iter().map(|v| v * v).sum(); s.sqrt().max(1e-15) }
                        NormType::Max => { let s = xi.iter().map(|v| v.abs()).fold(0.0f64, f64::max); s.max(1e-15) }
                    };
                    let inv = 1.0 / scale;
                    for j in 0..p { block[r * p + j] = xi[j] * inv; }
                }
            });
        } else {
            for i in 0..n {
                let row = &x[i * p..(i + 1) * p];
                let scale = match self.norm {
                    NormType::L1 => { let s: f64 = row.iter().map(|v| v.abs()).sum(); s.max(1e-15) }
                    NormType::L2 => { let s: f64 = row.iter().map(|v| v * v).sum(); s.sqrt().max(1e-15) }
                    NormType::Max => { let s = row.iter().map(|v| v.abs()).fold(0.0f64, f64::max); s.max(1e-15) }
                };
                for j in 0..p { out[i * p + j] = row[j] / scale; }
            }
        }
        out
    }
}

fn percentile_sorted(sorted: &[f64], pct: f64) -> f64 {
    let n = sorted.len();
    if n == 0 { return 0.0; }
    let idx = (pct / 100.0) * (n - 1) as f64;
    let lo = idx.floor() as usize;
    let hi = idx.ceil() as usize;
    if lo == hi { sorted[lo.min(n - 1)] }
    else {
        let frac = idx - lo as f64;
        sorted[lo.min(n - 1)] * (1.0 - frac) + sorted[hi.min(n - 1)] * frac
    }
}
